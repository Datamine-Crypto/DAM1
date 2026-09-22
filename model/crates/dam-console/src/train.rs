use dam::chance::mix;
use dam::events::STACK_SLOTS;
use dam::cursor::place_feature;
use dam::network::{FeatureId, Passed, Stacked, slot};
use dam::numbers::{one, zero};
use dam_card::steps::{CardRow, CardTrainer};
use patterns::{because, source};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct EventTrainer;
source!(
    EventTrainer,
    "the user's way of teaching a network over the events directly: send it the stack, force the action the search found, softmax cross entropy on the behaviors and a logistic loss on each of the settle and carry heads, updated row by row with AdaGrad (Duchi, Hazan and Singer), sparse for the embeddings a row touches and dense for the blocks of the slots it fills, on one thread and seeded, so one seed always gives the same bytes; a network returning several steps is taught the actions at the items after the row as well, the loss summed over the positions; a network predicting the input is taught the input items after the row with no label, a softmax cross entropy for each position weighted by the spec"
);

const WEIGHT_STREAM: &str = "slot weights";
because!(WEIGHT_STREAM, EventTrainer, "the name the slot blocks' starting numbers are drawn under, so they never share draws with the other weights");
const OUT_STREAM: &str = "output weights";
because!(OUT_STREAM, EventTrainer, "the name the output weights' starting numbers are drawn under");
const TABLE_STREAM: &str = "embeddings";
because!(TABLE_STREAM, EventTrainer, "the name the embeddings' starting numbers are drawn under, mixed with each feature place so an embedding starts the same whichever row meets it first");
const ORDER_STREAM: &str = "row order";
because!(ORDER_STREAM, EventTrainer, "the name each epoch's order of rows is drawn under");
const POINT_STREAM: &str = "pointing weights";
because!(POINT_STREAM, EventTrainer, "the name the pointing matrix's starting numbers are drawn under");

#[derive(Clone, Copy, Debug)]
pub struct Training {
    pub item: usize,
    pub hidden: usize,
    pub epochs: usize,
    pub rate: f32,
    pub floor: f32,
    pub scale: f32,
    pub seed: u64,
    pub slices: usize,
    pub threads: usize,
}
because!(
    Training,
    "the spec's values a training run takes: the embedding width, the hidden units, the most epochs, AdaGrad's rate and floor, how wide starting numbers are drawn, the seed, how many rows a step sums, and on how many threads at most"
);

#[derive(Clone, Debug, Default)]
pub struct TrainRow {
    pub file: String,
    pub line: usize,
    pub test: bool,
    pub word: String,
    pub behavior: Option<String>,
    pub depth: usize,
    pub stack: Arc<Vec<Vec<FeatureId>>>,
    pub pointed: Vec<usize>,
    pub pointable: Vec<usize>,
}
because!(
    TrainRow,
    "one row the teacher wrote, as a trainer reads it: the lesson and the quiz line it came from, whether it is held out, the input item it was taken at, the step taught there, how many events the line's stack had held, the events of that line's stack, oldest first and shared by every row of the line, and for a row that copies a token the slots the token stands in and the slots it may be copied from; the row's slots are the newest events up to the stack's size, each with the place of its depth added"
);

impl TrainRow {
    pub fn filled(&self) -> usize {
        self.depth.min(STACK_SLOTS)
    }

    pub fn items(&self) -> Vec<Vec<FeatureId>> {
        let filled = self.filled();
        (0..filled).map(|s| self.stack[self.depth - 1 - s].iter().copied().chain(std::iter::once(place_feature(s))).collect()).collect()
    }
}

#[derive(Deserialize)]
struct WrittenRecord {
    #[serde(default)]
    file: String,
    #[serde(default)]
    line: usize,
    #[serde(default)]
    test: bool,
    #[serde(default)]
    events: Vec<Vec<FeatureId>>,
    #[serde(default)]
    rows: Vec<WrittenRow>,
}
because!(WrittenRecord, "the rows of one quiz line as the teacher wrote them: the lesson, the line, whether it is held out, the events its stack held, oldest first, and the rows over them");

#[derive(Deserialize)]
struct WrittenRow {
    #[serde(default)]
    word: String,
    #[serde(default)]
    behavior: Option<String>,
    #[serde(default)]
    depth: usize,
    #[serde(default)]
    pointed: Vec<usize>,
    #[serde(default)]
    pointable: Vec<usize>,
}
because!(WrittenRow, "one row of a record as the teacher wrote it: the input item, the step taught, how many events the stack had held before it, and the slots of a copy");

pub fn rows_of(text: &str) -> Result<Vec<TrainRow>, String> {
    let mut rows = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let record: WrittenRecord = serde_json::from_str(line).map_err(|e| format!("a record of rows: {e}"))?;
        let stack = Arc::new(record.events);
        for row in record.rows {
            if row.depth > stack.len() {
                return Err(format!("line {}: a row at depth {} over {} events", record.line, row.depth, stack.len()));
            }
            rows.push(TrainRow { file: record.file.clone(), line: record.line, test: record.test, word: row.word, behavior: row.behavior, depth: row.depth, stack: stack.clone(), pointed: row.pointed, pointable: row.pointable });
        }
    }
    Ok(rows)
}
because!(rows_of, "the rows of a file the teacher wrote, one JSON record a line, each record's events kept once and shared by its rows");

fn drawn(seed: u64, stream: &str, index: u64, scale: f32) -> f32 {
    let v = mix(seed ^ mix(u64::from(slot(stream)) ^ mix(index)));
    (v as i64) as f32 / i64::MAX as f32 * scale
}
because!(drawn, "a starting number drawn evenly between minus and plus the scale from the seed, a stream's name and a place in it, so every number is fixed by the seed alone and not by the order it is drawn in");

fn adagrad(w: &mut f32, sq: &mut f32, g: f32, t: &Training) {
    *sq += g * g;
    *w -= t.rate * g / (sq.sqrt() + t.floor);
}
because!(adagrad, "one AdaGrad update: the squared gradient added to the number's sum, and the number moved against the gradient scaled by the rate over the root of that sum");

pub struct ParallelSteps;
source!(
    ParallelSteps,
    "the user's speed-up of a training run over the machine's cores, after a run over the whole quiz used one thread of twenty four: an epoch's rows in their drawn order cut into steps of a fixed number of slices of a fixed number of rows, every slice's gradient summed against the weights at the start of its step on as many threads as the machine and the spec allow, and the slices applied one after another in their order, so the bytes a seed trains depend on the spec and never on the cores"
);

struct Gradient {
    out: BTreeMap<usize, (Vec<f32>, f32)>,
    hidden_bias: Vec<f32>,
    slot_weights: Vec<f32>,
    fill: Vec<f32>,
    table: BTreeMap<FeatureId, Vec<f32>>,
    point: Vec<f32>,
    loss: f32,
}
because!(
    Gradient,
    ParallelSteps,
    "the gradient a slice of rows sums before the weights move: for every output a loss touched its weights and bias, the hidden biases, the blocks and fill weights of the slots its rows filled, the embeddings of the feature places they hold, the pointing matrix, and the loss"
);

fn empty_gradient(net: &Stacked) -> Gradient {
    Gradient { out: BTreeMap::new(), hidden_bias: vec![zero(); net.hidden], slot_weights: Vec::new(), fill: Vec::new(), table: BTreeMap::new(), point: vec![zero(); net.point.len()], loss: zero() }
}
because!(empty_gradient, ParallelSteps, "a slice's gradient before any row is summed into it, the slot blocks growing only as far as its rows fill slots");

fn summed_into(acc: &mut Gradient, g: &Gradient) {
    for (&c, (ws, bias)) in &g.out {
        let (sums, total) = acc.out.entry(c).or_insert_with(|| (vec![zero(); ws.len()], zero()));
        for (s, w) in sums.iter_mut().zip(ws) {
            *s += w;
        }
        *total += bias;
    }
    for (s, x) in acc.hidden_bias.iter_mut().zip(&g.hidden_bias) {
        *s += x;
    }
    for (s, x) in acc.point.iter_mut().zip(&g.point) {
        *s += x;
    }
    if acc.slot_weights.len() < g.slot_weights.len() {
        acc.slot_weights.resize(g.slot_weights.len(), zero());
    }
    for (s, x) in acc.slot_weights.iter_mut().zip(&g.slot_weights) {
        *s += x;
    }
    if acc.fill.len() < g.fill.len() {
        acc.fill.resize(g.fill.len(), zero());
    }
    for (s, x) in acc.fill.iter_mut().zip(&g.fill) {
        *s += x;
    }
    for (id, v) in &g.table {
        let sums = acc.table.entry(*id).or_insert_with(|| vec![zero(); v.len()]);
        for (s, x) in sums.iter_mut().zip(v) {
            *s += x;
        }
    }
    acc.loss += g.loss;
}
because!(summed_into, ParallelSteps, "one row's gradient added into the gradient of its step, number by number, every group the row touched grown as far as the row reaches");

fn applied_together(net: &mut Stacked, sq: &mut Stacked, gradients: &[Gradient], t: &Training) {
    for g in gradients {
        applied_apart(net, sq, g, t);
    }
    let longest = gradients.iter().map(|g| g.slot_weights.len()).max().unwrap_or(0);
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let part = longest.div_ceil(t.threads.min(cores).max(1)).max(net.hidden * net.item * gradients.len()).max(1);
    std::thread::scope(|scope| {
        for ((weights, sums), start) in net.slot_weights[..longest].chunks_mut(part).zip(sq.slot_weights[..longest].chunks_mut(part)).zip((0..longest).step_by(part)) {
            scope.spawn(move || {
                for g in gradients {
                    for (i, &x) in g.slot_weights.iter().enumerate().skip(start).take(weights.len()) {
                        adagrad(&mut weights[i - start], &mut sums[i - start], x, t);
                    }
                }
            });
        }
    });
}
because!(
    applied_together,
    ParallelSteps,
    "the gradients of a step's rows moved into the weights by AdaGrad in the rows' order: every group but the slot weights row by row, and the slot weights in ranges on the machine's threads, each range at least one slot block for every row of the step, so a step whose rows fill few slots starts few threads, and each range taking every row's gradient in the rows' order, since a number's update reads only that number, its sum and its gradient, so every number gets the same updates in the same order as row by row"
);

fn applied_apart(net: &mut Stacked, sq: &mut Stacked, g: &Gradient, t: &Training) {
    let h = net.hidden;
    for (&c, (ws, bias)) in &g.out {
        for (j, &x) in ws.iter().enumerate() {
            adagrad(&mut net.out[c * h + j], &mut sq.out[c * h + j], x, t);
        }
        adagrad(&mut net.out_bias[c], &mut sq.out_bias[c], *bias, t);
    }
    for (j, &x) in g.hidden_bias.iter().enumerate() {
        adagrad(&mut net.hidden_bias[j], &mut sq.hidden_bias[j], x, t);
    }
    for (j, &x) in g.point.iter().enumerate() {
        adagrad(&mut net.point[j], &mut sq.point[j], x, t);
    }
    for (i, &x) in g.fill.iter().enumerate() {
        adagrad(&mut net.fill[i], &mut sq.fill[i], x, t);
    }
    for (id, v) in &g.table {
        if let (Some(w), Some(q)) = (net.table.get_mut(id), sq.table.get_mut(id)) {
            for ((wi, qi), &x) in w.iter_mut().zip(q.iter_mut()).zip(v) {
                adagrad(wi, qi, x, t);
            }
        }
    }
}
because!(applied_apart, ParallelSteps, "a gradient moved into every weight group but the slot weights by AdaGrad, every number once, the outputs no row touched left as they were");

fn feature_places<'a>(rows: &'a [&'a TrainRow]) -> impl Iterator<Item = FeatureId> + 'a {
    rows.iter().flat_map(|r| r.stack.iter().flatten().copied()).chain((0..STACK_SLOTS).map(place_feature))
}
because!(feature_places, "every feature place the rows hold: the places of every event of their stacks and the place of every depth a slot may stand at");

pub fn started_with(rows: &[&TrainRow], t: &Training, outputs: usize) -> Stacked {
    let mut table = BTreeMap::new();
    for id in feature_places(rows) {
        table.entry(id).or_insert_with(|| (0..t.item).map(|i| drawn(t.seed ^ id, TABLE_STREAM, i as u64, t.scale)).collect());
    }
    let weights = STACK_SLOTS * t.hidden * t.item;
    Stacked {
        slots: STACK_SLOTS,
        item: t.item,
        hidden: t.hidden,
        table,
        slot_weights: (0..weights).map(|i| drawn(t.seed, WEIGHT_STREAM, i as u64, t.scale)).collect(),
        fill: vec![zero(); STACK_SLOTS * t.hidden],
        hidden_bias: vec![zero(); t.hidden],
        out: (0..outputs * t.hidden).map(|i| drawn(t.seed, OUT_STREAM, i as u64, t.scale)).collect(),
        out_bias: vec![zero(); outputs],
        point: if rows.iter().any(|r| !r.pointed.is_empty()) { (0..t.hidden * t.item).map(|i| drawn(t.seed, POINT_STREAM, i as u64, t.scale)).collect() } else { Vec::new() },
    }
}
because!(
    started_with,
    "a network over the events before training with the number of outputs given: an embedding for every feature place a learned row holds, the slot blocks and output weights drawn from the seed for every output of every position, the fill weights and biases at nothing, and a pointing matrix drawn from the seed when some row copies a word"
);

fn strongest(block: &[f32]) -> Option<usize> {
    (0..block.len()).max_by(|&a, &b| block[a].total_cmp(&block[b]))
}
because!(strongest, "the column of the strongest output of a block");

fn pointable_among(row: &TrainRow, filled: usize) -> Vec<usize> {
    row.pointable.iter().copied().filter(|&s| s < filled).collect()
}
because!(pointable_among, "the slots a row's copied token may be copied from that the row's stack fills");

fn taken_as_taught(net: &Stacked, (row, wanted): (&TrainRow, usize)) -> bool {
    let items = row.items();
    let passed = net.passed(items.iter().map(Vec::as_slice));
    let out: Vec<f32> = (0..net.out_bias.len()).map(|c| net.output_at(&passed.hidden, c)).collect();
    let among = pointable_among(row, passed.filled);
    let pointed_right = row.pointed.is_empty() || net.point.is_empty() || among.is_empty() || strongest(&net.pointing(&passed, &among)).is_some_and(|k| row.pointed.contains(&among[k]));
    strongest(&out) == Some(wanted) && pointed_right
}
because!(
    taken_as_taught,
    "whether a network takes a row as taught: its strongest output is the step taught, and for a row that copies a token its strongest pointable slot is one the token stands in, since a step that names the right class but copies from the wrong slot reads the line wrongly"
);

fn softmax_loss(out: &[f32], d_out: &mut [f32], wanted: usize) -> f32 {
    let top = out.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let total: f32 = out.iter().map(|x| (x - top).exp()).sum();
    let mut loss = zero();
    for (c, (x, d)) in out.iter().zip(d_out.iter_mut()).enumerate() {
        let p = (x - top).exp() / total;
        *d = p - if c == wanted { one() } else { zero() };
        if c == wanted {
            loss -= p.max(f32::MIN_POSITIVE).ln();
        }
    }
    loss
}
because!(softmax_loss, "the cross entropy of one softmax over a block of outputs against the class taught, its gradient written into the block's place in the output gradient");

fn hidden_of(net: &Stacked, d_out: &[(usize, f32)]) -> Vec<f32> {
    let h = net.hidden;
    let mut d_hidden = vec![zero(); h];
    for &(c, g) in d_out {
        for (d, w) in d_hidden.iter_mut().zip(&net.out[c * h..c * h + h]) {
            *d += g * w;
        }
    }
    d_hidden
}
because!(hidden_of, "the gradient reaching the hidden units from the outputs a row's losses touched");

fn summed_back(net: &Stacked, items: &[Vec<FeatureId>], f: &Passed, (d_out, d_hidden, d_embedded): (&[(usize, f32)], &[f32], &[f32]), acc: &mut Gradient) {
    let (h, item) = (net.hidden, net.item);
    for &(c, g) in d_out {
        let (ws, bias) = acc.out.entry(c).or_insert_with(|| (vec![zero(); h], zero()));
        for (w, x) in ws.iter_mut().zip(&f.hidden) {
            *w += g * x;
        }
        *bias += g;
    }
    let d_pre: Vec<f32> = d_hidden.iter().zip(&f.pre).map(|(d, &z)| if z > zero() { *d } else { zero() }).collect();
    for (b, d) in acc.hidden_bias.iter_mut().zip(&d_pre) {
        *b += d;
    }
    let block = h * item;
    if acc.slot_weights.len() < f.filled * block {
        acc.slot_weights.resize(f.filled * block, zero());
        acc.fill.resize(f.filled * h, zero());
    }
    let mut d_e = vec![zero(); item];
    for s in 0..f.filled {
        let e = &f.embedded[s * item..s * item + item];
        let base = s * block;
        match d_embedded.get(s * item..s * item + item) {
            Some(shared) => d_e.copy_from_slice(shared),
            None => d_e.fill(zero()),
        }
        for (j, &g) in d_pre.iter().enumerate() {
            for (i, d) in d_e.iter_mut().enumerate() {
                *d += g * net.slot_weights[base + j * item + i];
            }
            for (i, x) in e.iter().enumerate() {
                acc.slot_weights[base + j * item + i] += g * x;
            }
            acc.fill[s * h + j] += g;
        }
        for id in items[s].iter().filter(|id| net.table.contains_key(id)) {
            let v = acc.table.entry(*id).or_insert_with(|| vec![zero(); item]);
            for (w, d) in v.iter_mut().zip(&d_e) {
                *w += d;
            }
        }
    }
}
because!(
    summed_back,
    "the gradient of a row's outputs carried back into its slice's sums: the output weights and biases the losses touched, the hidden biases through the rectified units, the blocks and fill weights of the filled slots, and the embeddings of the feature places the row holds, a slot with no gradient of its own from a pointer taking nothing but what reaches it through its block"
);

fn squares_of(net: &Stacked, kept: Option<&Stacked>) -> Stacked {
    let mut sq = kept.cloned().unwrap_or_else(|| Stacked {
        table: std::collections::BTreeMap::new(),
        slot_weights: vec![zero(); net.slot_weights.len()],
        fill: vec![zero(); net.fill.len()],
        hidden_bias: vec![zero(); net.hidden_bias.len()],
        out: Vec::new(),
        out_bias: Vec::new(),
        point: vec![zero(); net.point.len()],
        ..net.clone()
    });
    for (id, v) in &net.table {
        sq.table.entry(*id).or_insert_with(|| vec![zero(); v.len()]);
    }
    sq.out.resize(net.out.len(), zero());
    sq.out_bias.resize(net.out_bias.len(), zero());
    sq
}
because!(
    squares_of,
    "AdaGrad's sums of squared gradients, one beside every number of the network and laid out as the network is: the sums a run kept when it has them, and a sum at nothing for every number they lack, as the embeddings of new feature places and the outputs a grown network added at its end"
);

fn drawn_order(order: &mut [usize], seed: u64, epoch: usize) {
    for i in (1..order.len()).rev() {
        let j = (mix(seed ^ mix(u64::from(slot(ORDER_STREAM)) ^ mix(epoch as u64) ^ i as u64)) % (i as u64 + 1)) as usize;
        order.swap(i, j);
    }
}
because!(drawn_order, "an epoch's order of rows shuffled from the seed and the epoch, so one seed gives the same order on any machine");

const REACH_BAND: usize = 64;
because!(REACH_BAND, EventTrainer, "how many slots apart rows may fill and still be batched together: rows are laid out by the band their reach falls in and shuffled within it, since batches of rows that fill exactly alike are rows of the same step of the same shapes and learn slowly, while a band this wide costs a batch a third of the slots at most beyond its longest row and learns as fast an epoch as rows drawn at random, measured");

const BATCH_STREAM: &str = "batch order";
because!(BATCH_STREAM, EventTrainer, "the name each epoch's order of batches is drawn under, apart from the rows' own");

fn batched_order(order: &mut [usize], seed: u64, epoch: usize, reach: &[usize], batch: usize) {
    drawn_order(order, seed, epoch);
    order.sort_by_key(|&r| reach[r] / REACH_BAND);
    let batches = order.len().div_ceil(batch.max(1));
    let mut places: Vec<usize> = (0..batches).collect();
    for i in (1..places.len()).rev() {
        let j = (mix(seed ^ mix(u64::from(slot(BATCH_STREAM)) ^ mix(epoch as u64) ^ i as u64)) % (i as u64 + 1)) as usize;
        places.swap(i, j);
    }
    let sorted = order.to_vec();
    let mut at = 0;
    for place in places {
        let chunk = &sorted[place * batch.max(1)..((place + 1) * batch.max(1)).min(sorted.len())];
        order[at..at + chunk.len()].copy_from_slice(chunk);
        at += chunk.len();
    }
}
because!(batched_order, "an epoch's order of rows, the same on the card and on the processor: the rows shuffled from the seed, then laid out by the band of slots they fill, the shortest first, so that each batch holds rows that fill about as many slots, and the batches shuffled again from the seed, since a batch is worked out to the most slots any of its rows fills and a batch of rows drawn at random nearly always holds one that fills them all, while the rows fill under half on average; one seed still gives one order on any machine");

pub struct LoopTrainer;
source!(
    LoopTrainer,
    "the user's self-loop network taught directly: send it the stack, force the action the loop's teacher found, softmax cross entropy over the loop's actions alone with no settle, carry or later heads, row by row with AdaGrad from the seed, until every learned row's strongest output is its action"
);

fn grown(base: &Stacked, rows: &[&TrainRow], t: &Training, outputs: usize) -> Stacked {
    let mut net = base.clone();
    let item = net.item;
    for id in feature_places(rows) {
        net.table.entry(id).or_insert_with(|| (0..item).map(|i| drawn(t.seed ^ id, TABLE_STREAM, i as u64, t.scale)).collect());
    }
    for c in net.out_bias.len()..outputs {
        net.out.extend((0..net.hidden).map(|i| drawn(t.seed, OUT_STREAM, (c * net.hidden + i) as u64, t.scale)));
        net.out_bias.push(zero());
    }
    net
}
because!(
    grown,
    LoopTrainer,
    "the network a grade is taught from when it starts from the network of the grade before: every weight kept, an embedding drawn from the seed for every feature place the new rows hold that it lacks, and the weights of every output added since drawn as a new network's are"
);

pub fn trained_acts(rows: &[TrainRow], outputs: &[String], (t, (settle, t_enough, patience, limit), (from, on_card)): (&Training, (usize, f32, usize, Option<f32>), (Option<(&Stacked, Option<&Stacked>)>, bool)), report: &mut dyn FnMut(&EpochReport)) -> Result<(Stacked, Stacked), String> {
    let mut taught: Vec<(&TrainRow, usize)> = Vec::new();
    for row in rows.iter().filter(|r| !r.test) {
        let name = row.behavior.as_deref().unwrap_or_default();
        let wanted = outputs.iter().position(|o| o == name).ok_or(format!("line {}: {name} is not one of the network's outputs", row.line))?;
        taught.push((row, wanted));
    }
    let learned: Vec<&TrainRow> = taught.iter().map(|&(r, _)| r).collect();
    let mut net = match from {
        Some((base, _)) => grown(base, &learned, t, outputs.len()),
        None => started_with(&learned, t, outputs.len()),
    };
    let mut sq = squares_of(&net, from.and_then(|(_, kept)| kept));
    let card_rows: Vec<CardRow> = taught.iter().map(|&(row, wanted)| CardRow { stack: &row.stack, depth: row.depth, wanted, pointed: &row.pointed, pointable: &row.pointable }).collect();
    let mut card = if on_card { Some(CardTrainer::of((&net, &sq), &card_rows, (t.rate, t.floor), t.slices)?) } else { None };
    let mut order: Vec<usize> = (0..taught.len()).collect();
    let reaches: Vec<usize> = taught.iter().map(|(row, _)| row.filled().min(net.slots)).collect();
    let groups: Vec<usize> = taught
        .iter()
        .scan((usize::MAX, 0usize), |(last, count), (row, _)| {
            if row.line != *last {
                *last = row.line;
                *count += 1;
            }
            Some(*count)
        })
        .collect();
    let (mut settled, mut best, mut stale) = (0, usize::MAX, 0);
    let began = std::time::Instant::now();
    for epoch in 0..t.epochs {
        let clock = std::time::Instant::now();
        batched_order(&mut order, t.seed, epoch, &reaches, t.slices);
        let mut total = zero();
        let right: Vec<bool> = match card.as_mut() {
            Some(c) => {
                total = c.epoch(&order)?;
                c.taken(&order)?
            }
            None => {
                for step in order.chunks(t.slices.max(1)) {
                    let gradients = dam::threads::in_line_order(step.len(), t.threads, |k| {
                        let mut g = empty_gradient(&net);
                        act_gradient(&net, taught[step[k]], &mut g);
                        g
                    });
                    let mut summed = empty_gradient(&net);
                    for g in &gradients {
                        summed_into(&mut summed, g);
                    }
                    total += summed.loss;
                    applied_together(&mut net, &mut sq, std::slice::from_ref(&summed), t);
                }
                dam::threads::in_line_order(taught.len(), t.threads, |k| taken_as_taught(&net, taught[k]))
            }
        };
        let wrong = right.iter().filter(|taken| !**taken).count();
        let mut lines: BTreeMap<usize, bool> = BTreeMap::new();
        for (group, taken) in groups.iter().zip(&right) {
            let whole = lines.entry(*group).or_insert(true);
            *whole = *whole && *taken;
        }
        let whole_lines = lines.values().filter(|whole| **whole).count();
        report(&EpochReport { epoch: epoch + 1, loss: total / taught.len().max(1) as f32, wrong, seconds: clock.elapsed().as_secs_f32() });
        let enough = whole_lines as f32 >= t_enough * lines.len() as f32;
        settled = if enough { settled + 1 } else { 0 };
        stale = if wrong < best { 0 } else { stale + 1 };
        best = best.min(wrong);
        if settled > settle || stale >= patience || limit.is_some_and(|seconds| began.elapsed().as_secs_f32() >= seconds) {
            break;
        }
    }
    if let Some(c) = &card {
        c.written_into((&mut net, &mut sq))?;
    }
    let stacked: BTreeMap<u64, Vec<&TrainRow>> = taught.iter().fold(BTreeMap::new(), |mut all, (row, _)| {
        all.entry(stack_alike(row)).or_default().push(row);
        all
    });
    for (row, _) in taught.iter().filter(|&&taken| !taken_as_taught(&net, taken)) {
        let wanted = row.behavior.as_deref().unwrap_or_default();
        let rivals: Vec<String> = stacked
            .get(&stack_alike(row))
            .map(|alike| {
                let mut said: Vec<String> = alike.iter().filter(|other| other.behavior.as_deref().unwrap_or_default() != wanted).map(|other| format!("{} line {} at {:?} wants {}", other.file, other.line, other.word, other.behavior.as_deref().unwrap_or_default())).collect();
                said.sort();
                said.dedup();
                said
            })
            .unwrap_or_default();
        if rivals.is_empty() {
            println!("row not learned: {} line {} at \"{}\" wants {wanted}", row.file, row.line, row.word);
        } else {
            println!("row not learned: {} line {} at \"{}\" wants {wanted}, on the stack of {}", row.file, row.line, row.word, rivals.join(" and "));
        }
    }
    Ok((net, sq))
}
because!(
    trained_acts,
    LoopTrainer,
    "a network over the stack taught the loop's rows, from a fresh start or from the network of an earlier grade, the held out ones left aside, with one output for every action named, refused when a row's action is not one of them, every epoch reported, stopping once the share of learned lines whose every row is taken as taught has reached the share that is enough, a line being right only whole and the lines told apart by where a line of rows begins, so rows of several files never share a line, for as many epochs in a row as the settling allows beyond the first, or once the rows taken wrongly have gone the patience of epochs without a new low, since rows taught two actions on one stack can never all be right, each such row said at the end by the line it came from, the item it was taken at and the step it wanted, so the lines that teach one stack two things can be found and said again, or once the run has used the seconds it was given, since the user wants a run cut short and looked at before it is given more time; each step worked on the graphics card when the run asks for it, else each step's rows worked out on the machine's threads against the weights at the start of the step, added into one gradient in their order and applied once, and every row checked on threads, since the user wants a run fast and stopped as soon as it is good enough"
);

fn stack_alike(row: &TrainRow) -> u64 {
    let mut seen = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&row.items(), &mut seen);
    std::hash::Hasher::finish(&seen)
}
because!(stack_alike, LoopTrainer, "what tells two rows apart by their stacks alone: the events each is given, hashed, so the rows a \
     network is asked to take two ways from one stack are found by grouping on it");

fn act_gradient(net: &Stacked, (row, wanted): (&TrainRow, usize), acc: &mut Gradient) {
    let items = row.items();
    let f = net.passed(items.iter().map(Vec::as_slice));
    let out: Vec<f32> = (0..net.out_bias.len()).map(|c| net.output_at(&f.hidden, c)).collect();
    let mut d = vec![zero(); out.len()];
    let loss = softmax_loss(&out, &mut d, wanted);
    let d_out: Vec<(usize, f32)> = d.into_iter().enumerate().collect();
    let mut d_hidden = hidden_of(net, &d_out);
    let mut d_embedded = vec![zero(); if row.pointed.is_empty() { 0 } else { f.embedded.len() }];
    let pointed = pointed_loss(net, (&f, row), one(), (&mut d_hidden, &mut d_embedded), acc);
    summed_back(net, &items, &f, (&d_out, &d_hidden, &d_embedded), acc);
    acc.loss += loss + pointed;
}
because!(act_gradient, LoopTrainer, "one row of the loop summed into its gradient: the loss of a softmax over every action against the action taught, and for a row that copies a token the loss of pointing at a slot it stands in, carried back through the network");

fn pointed_loss(net: &Stacked, (f, row): (&Passed, &TrainRow), weight: f32, (d_hidden, d_embedded): (&mut [f32], &mut [f32]), acc: &mut Gradient) -> f32 {
    let item = net.item;
    let among = pointable_among(row, f.filled);
    if row.pointed.is_empty() || net.point.is_empty() || among.is_empty() {
        return zero();
    }
    let scores = net.pointing(f, &among);
    let top = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let total: f32 = scores.iter().map(|x| (x - top).exp()).sum();
    let chances: Vec<f32> = scores.iter().map(|x| (x - top).exp() / total).collect();
    let right = among.iter().zip(&chances).filter(|(s, _)| row.pointed.contains(s)).map(|(_, p)| *p).sum::<f32>().max(f32::MIN_POSITIVE);
    for (&s, &p) in among.iter().zip(&chances) {
        let g = weight * (p - if row.pointed.contains(&s) { p / right } else { zero() });
        let e = &f.embedded[s * item..s * item + item];
        for (j, (weights, x_h)) in net.point.chunks_exact(item).zip(&f.hidden).enumerate() {
            d_hidden[j] += g * weights.iter().zip(e).map(|(w, x)| w * x).sum::<f32>();
            for (i, (w, x)) in weights.iter().zip(e).enumerate() {
                acc.point[j * item + i] += g * x_h * x;
                d_embedded[s * item + i] += g * x_h * w;
            }
        }
    }
    -weight * right.ln()
}
because!(
    pointed_loss,
    "the loss of a copied token's slot, weighted as asked: a softmax over the slots a row may copy from against every slot the token stands in, its gradient added to the hidden units, the embeddings of those slots and the pointing matrix; nothing for a row that copies nothing or a network that does not point"
);

pub struct EpochReport {
    pub epoch: usize,
    pub loss: f32,
    pub wrong: usize,
    pub seconds: f32,
}
because!(
    EpochReport,
    "one epoch of a training run as it is said while the run goes on, since a whole quiz run said nothing for over an hour: the epoch counted from one, its mean loss, the rows it still takes wrongly, and the seconds the epoch took with its check"
);

