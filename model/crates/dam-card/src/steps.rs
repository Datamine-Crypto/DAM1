use crate::device::{Card, card, compiled, err};
use crate::kernels::train_text;
use cudarc::driver::{CudaFunction, CudaSlice, CudaStream, DeviceRepr, LaunchArgs, LaunchConfig, PushKernelArg};
use dam::network::{FeatureId, Stacked};
use dam::numbers::zero;
use patterns::{because, source};
use dam::cursor::place_feature;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CardTraining;
source!(
    CardTraining,
    "the user's wish to train the network on the graphics card: the loop trainer's step, a summed gradient of a step's rows moved into the weights by AdaGrad once, and its check of every row, worked by kernels on the card over rows copied there once, so an epoch copies only its order to the card and its loss and right rows back"
);

pub struct CardRow<'a> {
    pub stack: &'a Arc<Vec<Vec<FeatureId>>>,
    pub depth: usize,
    pub wanted: usize,
    pub pointed: &'a [usize],
    pub pointable: &'a [usize],
}
because!(CardRow, CardTraining, "one row as the card is taught it: the events of its line's stack, shared by the line's rows, how many of them the stack had held, the output it is taught, and the slots a copied token stands in and may be copied from");

struct Kernels {
    embedded: CudaFunction,
    product: CudaFunction,
    hidden_rest: CudaFunction,
    rectified_rows: CudaFunction,
    embed_point: CudaFunction,
    slot_apply: CudaFunction,
    outputs: CudaFunction,
    point_scores: CudaFunction,
    pointed: CudaFunction,
    hidden_back: CudaFunction,
    out_step: CudaFunction,
    bias_step: CudaFunction,
    slot_step: CudaFunction,
    point_weighted: CudaFunction,
    table_sum: CudaFunction,
    table_apply: CudaFunction,
}
because!(Kernels, CardTraining, "every kernel of the training text, compiled once for a run");

struct Weights {
    table: CudaSlice<f32>,
    slot_weights: CudaSlice<f32>,
    fill: CudaSlice<f32>,
    hidden_bias: CudaSlice<f32>,
    out: CudaSlice<f32>,
    out_bias: CudaSlice<f32>,
    point: CudaSlice<f32>,
}
because!(Weights, CardTraining, "a network's weight groups on the card, the embeddings in one run in rising order of their place; the same groups hold AdaGrad's sums of squares");

struct Rows {
    ids: CudaSlice<i32>,
    slot_ids: CudaSlice<i32>,
    row_slots: CudaSlice<i32>,
    places: CudaSlice<i32>,
    hot_of: CudaSlice<i32>,
    hot_ids: CudaSlice<i32>,
    filled: CudaSlice<i32>,
    wanted: CudaSlice<i32>,
    pointable: CudaSlice<i32>,
    pointable_at: CudaSlice<i32>,
    pointed: CudaSlice<i32>,
    pointed_at: CudaSlice<i32>,
    identity: CudaSlice<i32>,
    reach: Vec<usize>,
}
because!(
    Rows,
    CardTraining,
    "the rows on the card, packed once: the feature places of every event of every stack, each stack once however many rows it serves, where each event's places begin, where each row's oldest slot stands among the events, so a row's slot is an event counted back from its newest, the place of each depth's feature in the table, which every row's slot of that depth adds, the rank among the hot feature places of each table place, or none, and the hot places by rank, the places the most events hold, which a block of the table step gathers in its shared memory before one add each into the sums, since a hundred places take nine tenths of every slot's adds and adds to one number from every thread of the card wait on each other, how many slots each row fills and its output, its pointing lists with where they begin, the rows in their own order for a check, and on the processor how many slots each row fills"
);

struct Work {
    order: CudaSlice<i32>,
    embedded: CudaSlice<f32>,
    pre: CudaSlice<f32>,
    d_pre: CudaSlice<f32>,
    d_out: CudaSlice<f32>,
    pointer: CudaSlice<f32>,
    d_embedded: CudaSlice<f32>,
    rectified: CudaSlice<f32>,
    pointed_part: CudaSlice<f32>,
    slot_grad: CudaSlice<f32>,
    loss: CudaSlice<f32>,
    right: CudaSlice<i32>,
    sums: CudaSlice<f32>,
    touched: CudaSlice<i32>,
    total: CudaSlice<f32>,
}
because!(
    Work,
    CardTraining,
    "the card's working numbers for a step: the epoch's order, every slot's embedding, the hidden units before rectifying and their gradient, the outputs' gradient, the pointing gradient of every slot, every slot's embedding gradient, every row's loss, whether each row is taken as taught, the summed embedding gradients with the step that last touched each, and the epoch's loss"
);

pub struct CardTrainer {
    card: Card,
    kernels: Kernels,
    weights: Weights,
    squares: Weights,
    rows: Rows,
    work: Work,
    slots: usize,
    item: usize,
    hidden: usize,
    outputs: usize,
    batch: usize,
    rate: f32,
    least: f32,
    mark: i32,
    pointing: bool,
}
because!(CardTrainer, CardTraining, "a run's network, AdaGrad's sums, rows and working numbers on the card, with the network's slots, embedding width, hidden units and outputs, the rows a step takes, AdaGrad's rate and floor, and the count of steps taken, and whether any row copies a token by pointing");

fn int(x: usize) -> Result<i32, String> {
    i32::try_from(x).map_err(err)
}
because!(int, CardTraining, "a count or place as the card's kernels take it, refused when it does not fit");

fn uploaded<T: DeviceRepr + Default + Copy>(card: &Card, xs: &[T]) -> Result<CudaSlice<T>, String> {
    (if xs.is_empty() { card.stream.clone_htod(&[T::default()]) } else { card.stream.clone_htod(xs) }).map_err(err)
}
because!(uploaded, CardTraining, "numbers copied to the card, an empty group held as one number since the card allocates nothing for none");

fn zeros<T: DeviceRepr + cudarc::driver::ValidAsZeroBits>(card: &Card, len: usize) -> Result<CudaSlice<T>, String> {
    card.stream.alloc_zeros(len.max(1)).map_err(err)
}
because!(zeros, CardTraining, "numbers at nothing on the card, at least one");

pub const MILLISECONDS: f32 = 1000.0;
because!(MILLISECONDS, CardTraining, "how many milliseconds a second holds, so a kernel time is read in the unit the epoch report uses");

thread_local! { static KERNEL_TIMES: std::cell::RefCell<std::collections::BTreeMap<&'static str, f64>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

pub fn kernel_times() -> String {
    KERNEL_TIMES.with(|t| { let mut v: Vec<(String, f64)> = t.borrow().iter().map(|(k, v)| (k.to_string(), *v)).collect(); t.borrow_mut().clear(); v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); v.iter().map(|(k, v)| format!("{k} {v:.0}")).collect::<Vec<_>>().join(" ") })
}
because!(kernel_times, CardTraining, "the time every kernel took since the last asking, the longest first, as one line for the epoch report when the kernel trace is on, so the kernel to work on next is the first word; the times are emptied once read");

fn tiles(rows: usize, columns: usize) -> usize {
    rows.div_ceil(SIDE) * columns.div_ceil(SIDE) * TILE * TILE
}
because!(tiles, CardTraining, "how many threads a product over rows and columns takes: one square block of threads per side-long tile of the result, each thread its small square of results, the last tiles padded");

pub const HOT: usize = 256;
because!(HOT, CardTraining, "how many of the hottest feature places a block of the table step gathers in shared memory before adding into the sums once each: as many as a block has threads, which at sixteen numbers each fits the shared memory a block gets, and covers the places nine tenths of every slot's adds go to");

pub const PER: usize = 2;
because!(PER, CardTraining, "how many results a thread of a product works out each way, a small square, so each number read from shared memory is used that many times, which a card's threads need since they read far slower than they multiply; kept small since the batches are a few hundred rows and a wider square would leave most of the card idle");

pub const SIDE: usize = TILE * PER;
because!(SIDE, CardTraining, "the side of the tile of results one block of a product works out: the tile of threads times the results a thread works out each way");

const PLAIN: i32 = 0;
because!(PLAIN, CardTraining, "the flag a product kernel takes for an operand read as it lies, row by row");

const TURNED: i32 = 1;
because!(TURNED, CardTraining, "the flag a product kernel takes for an operand read turned, column by column, so a transpose is never made");

pub const TILE: usize = 16;
because!(TILE, CardTraining, "the side of the square tile a product kernel works in shared memory, the size the kernel text defines, so a warp of threads reads a row of the tile at once");

fn launched_as(launch: LaunchArgs<'_>, threads: usize, name: &'static str, stream: &CudaStream) -> Result<(), String> {
    if threads == 0 {
        return Ok(());
    }
    let count = u32::try_from(threads).map_err(err)?;
    launched_with(launch, LaunchConfig::for_num_elems(count), name, stream)
}

fn launched_tiled(launch: LaunchArgs<'_>, threads: usize, name: &'static str, stream: &CudaStream) -> Result<(), String> {
    let square = u32::try_from(TILE * TILE).map_err(err)?;
    let blocks = u32::try_from(threads / (TILE * TILE)).map_err(err)?;
    launched_with(launch, LaunchConfig { grid_dim: (blocks, 1, 1), block_dim: (square, 1, 1), shared_mem_bytes: 0 }, name, stream)
}
because!(launched_tiled, CardTraining, "a product kernel launched with one block of threads per tile of the result, a tile of threads each way, since the tile shares its memory within its block and no other");

fn launched_with(mut launch: LaunchArgs<'_>, config: LaunchConfig, name: &'static str, stream: &CudaStream) -> Result<(), String> {
    let tracing = std::env::var("DAM_TRACE_K").is_ok();
    let started = std::time::Instant::now();
    unsafe { launch.launch(config) }.map(|_| ()).map_err(err)?;
    if tracing {
        stream.synchronize().map_err(err)?;
        KERNEL_TIMES.with(|t| *t.borrow_mut().entry(name).or_default() += started.elapsed().as_secs_f64() * f64::from(MILLISECONDS));
    }
    Ok(())
}
because!(launched_as, CardTraining, "a kernel launched over as many threads as asked in blocks the runtime sizes, nothing launched for none");
because!(launched_with, CardTraining, "a kernel launched with a given shape; when the kernel trace is on, the stream waits for it and its time is kept under its name");

fn turned(weights: &[f32], (slots, hidden, item): (usize, usize, usize)) -> Vec<f32> {
    let mut out = vec![zero(); weights.len()];
    for s in 0..slots {
        for j in 0..hidden {
            for i in 0..item {
                out[(s * item + i) * hidden + j] = weights[(s * hidden + j) * item + i];
            }
        }
    }
    out
}
because!(turned, CardTraining, "the slot weights laid out for the card with the hidden unit innermost, slot by slot and place by place, so the threads of a warp, which take neighbouring hidden units, read neighbouring numbers; the network keeps its own order, hidden unit by hidden unit, and the card turns the weights on the way in and back on the way out");

fn turned_back(weights: &[f32], (slots, hidden, item): (usize, usize, usize)) -> Vec<f32> {
    let mut out = vec![zero(); weights.len()];
    for s in 0..slots {
        for j in 0..hidden {
            for i in 0..item {
                out[(s * hidden + j) * item + i] = weights[(s * item + i) * hidden + j];
            }
        }
    }
    out
}
because!(turned_back, CardTraining, "the slot weights in the network's own order again, from the card's");

impl Weights {
    fn of(card: &Card, net: &Stacked) -> Result<Weights, String> {
        let table: Vec<f32> = net.table.values().flat_map(|v| v.iter().copied().chain(std::iter::repeat(zero())).take(net.item)).collect();
        Ok(Weights {
            table: uploaded(card, &table)?,
            slot_weights: uploaded(card, &turned(&net.slot_weights, (net.slots, net.hidden, net.item)))?,
            fill: uploaded(card, &net.fill)?,
            hidden_bias: uploaded(card, &net.hidden_bias)?,
            out: uploaded(card, &net.out)?,
            out_bias: uploaded(card, &net.out_bias)?,
            point: uploaded(card, &net.point)?,
        })
    }

    fn written_into(&self, card: &Card, net: &mut Stacked) -> Result<(), String> {
        let back = |x: &CudaSlice<f32>, len: usize| {
            card.stream.clone_dtoh(x).map_err(err).map(|mut v| {
                v.truncate(len);
                v
            })
        };
        let table = back(&self.table, net.table.len() * net.item)?;
        for (v, laid) in net.table.values_mut().zip(table.chunks_exact(net.item.max(1))) {
            let len = v.len();
            v.copy_from_slice(&laid[..len]);
        }
        net.slot_weights = turned_back(&back(&self.slot_weights, net.slot_weights.len())?, (net.slots, net.hidden, net.item));
        net.fill = back(&self.fill, net.fill.len())?;
        net.hidden_bias = back(&self.hidden_bias, net.hidden_bias.len())?;
        net.out = back(&self.out, net.out.len())?;
        net.out_bias = back(&self.out_bias, net.out_bias.len())?;
        net.point = back(&self.point, net.point.len())?;
        Ok(())
    }
}

impl Rows {
    fn of(card: &Card, net: &Stacked, rows: &[CardRow]) -> Result<Rows, String> {
        let places: HashMap<FeatureId, usize> = net.table.keys().enumerate().map(|(k, id)| (*id, k)).collect();
        let (mut ids, mut slot_ids, mut row_slots, mut filled, mut wanted) = (Vec::new(), vec![0], Vec::new(), Vec::new(), Vec::new());
        let (mut pointable, mut pointable_at, mut pointed, mut pointed_at, mut reach) = (Vec::new(), vec![0], Vec::new(), vec![0], Vec::new());
        let mut packed: HashMap<*const Vec<Vec<FeatureId>>, usize> = HashMap::new();
        let mut counts = vec![0usize; net.table.len()];
        for row in rows {
            let base = match packed.get(&Arc::as_ptr(row.stack)) {
                Some(&base) => base,
                None => {
                    let base = slot_ids.len() - 1;
                    for event in row.stack.iter() {
                        for k in event.iter().filter_map(|id| places.get(id)) {
                            ids.push(int(*k)?);
                            counts[*k] += 1;
                        }
                        slot_ids.push(int(ids.len())?);
                    }
                    packed.insert(Arc::as_ptr(row.stack), base);
                    base
                }
            };
            let fills = row.depth.min(net.slots).min(row.stack.len());
            row_slots.push(int(base + row.depth - fills)?);
            filled.push(int(fills)?);
            reach.push(fills);
            wanted.push(int(row.wanted)?);
            for s in row.pointable {
                pointable.push(int(*s)?);
            }
            pointable_at.push(int(pointable.len())?);
            for s in row.pointed {
                pointed.push(int(*s)?);
            }
            pointed_at.push(int(pointed.len())?);
        }
        let identity = (0..rows.len()).map(int).collect::<Result<Vec<i32>, String>>()?;
        let place_of = (0..net.slots).map(|s| places.get(&place_feature(s)).map_or(Ok(-1), |&k| int(k))).collect::<Result<Vec<i32>, String>>()?;
        for &p in place_of.iter().filter(|&&p| p >= 0) {
            counts[p as usize] += rows.len();
        }
        let mut ranked: Vec<usize> = (0..counts.len()).collect();
        ranked.sort_by_key(|&k| std::cmp::Reverse(counts[k]));
        ranked.truncate(HOT);
        let mut hot_of = vec![-1i32; counts.len()];
        for (rank, &k) in ranked.iter().enumerate() {
            hot_of[k] = int(rank)?;
        }
        let hot_ids = ranked.iter().map(|&k| int(k)).chain(std::iter::repeat(Ok(0))).take(HOT).collect::<Result<Vec<i32>, String>>()?;
        Ok(Rows {
            ids: uploaded(card, &ids)?,
            slot_ids: uploaded(card, &slot_ids)?,
            row_slots: uploaded(card, &row_slots)?,
            places: uploaded(card, &place_of)?,
            hot_of: uploaded(card, &hot_of)?,
            hot_ids: uploaded(card, &hot_ids)?,
            filled: uploaded(card, &filled)?,
            wanted: uploaded(card, &wanted)?,
            pointable: uploaded(card, &pointable)?,
            pointable_at: uploaded(card, &pointable_at)?,
            pointed: uploaded(card, &pointed)?,
            pointed_at: uploaded(card, &pointed_at)?,
            identity: uploaded(card, &identity)?,
            reach,
        })
    }
}

impl CardTrainer {
    pub fn of((net, sums): (&Stacked, &Stacked), rows: &[CardRow], (rate, least): (f32, f32), batch: usize) -> Result<CardTrainer, String> {
        let card = card()?;
        let [embedded, product, hidden_rest, rectified_rows, embed_point, slot_apply, outputs, point_scores, pointed, hidden_back, out_step, bias_step, slot_step, point_weighted, table_sum, table_apply] = compiled(
            &card,
            &train_text(net.item),
            ["embedded", "product", "hidden_rest", "rectified_rows", "embed_point", "slot_apply", "outputs", "point_scores", "pointed", "hidden_back", "out_step", "bias_step", "slot_step", "point_weighted", "table_sum", "table_apply"],
        )?;
        let kernels = Kernels { embedded, product, hidden_rest, rectified_rows, embed_point, slot_apply, outputs, point_scores, pointed, hidden_back, out_step, bias_step, slot_step, point_weighted, table_sum, table_apply };
        let batch = batch.max(1);
        let (slots, item, h, c_n) = (net.slots, net.item, net.hidden, net.out_bias.len());
        let work = Work {
            order: zeros(&card, rows.len())?,
            embedded: zeros(&card, batch * slots * item)?,
            pre: zeros(&card, batch * h)?,
            d_pre: zeros(&card, batch * h)?,
            d_out: zeros(&card, batch * c_n)?,
            pointer: zeros(&card, batch * slots)?,
            d_embedded: zeros(&card, batch * slots * item)?,
            rectified: zeros(&card, batch * h)?,
            pointed_part: zeros(&card, batch * item)?,
            slot_grad: zeros(&card, slots * item * h)?,
            loss: zeros(&card, batch)?,
            right: zeros(&card, rows.len())?,
            sums: zeros(&card, net.table.len() * item)?,
            touched: zeros(&card, net.table.len())?,
            total: zeros(&card, 1)?,
        };
        let weights = Weights::of(&card, net)?;
        let squares = Weights::of(&card, sums)?;
        let pointing = !net.point.is_empty() && rows.iter().any(|r| !r.pointed.is_empty());
        let rows = Rows::of(&card, net, rows)?;
        Ok(CardTrainer { card, kernels, weights, squares, rows, work, slots, item, hidden: h, outputs: c_n, batch, rate, least, mark: 0, pointing })
    }

    pub fn epoch(&mut self, order: &[usize]) -> Result<f32, String> {
        let ints = order.iter().map(|&r| int(r)).collect::<Result<Vec<i32>, String>>()?;
        self.card.stream.memcpy_htod(&ints, &mut self.work.order).map_err(err)?;
        self.card.stream.memcpy_htod(&[zero()], &mut self.work.total).map_err(err)?;
        for (k, step) in order.chunks(self.batch).enumerate() {
            let reach = step.iter().map(|&r| self.rows.reach[r]).max().unwrap_or(0);
            self.passed(false, k * self.batch, step.len(), reach)?;
            self.stepped(k * self.batch, step.len(), reach)?;
        }
        let total = self.card.stream.clone_dtoh(&self.work.total).map_err(err)?;
        Ok(total.first().copied().unwrap_or_else(zero))
    }

    pub fn taken(&self, order: &[usize]) -> Result<Vec<bool>, String> {
        let right = self.card.stream.clone_dtoh(&self.work.right).map_err(err)?;
        let mut by_row = vec![false; order.len()];
        for (place, &row) in order.iter().enumerate() {
            by_row[row] = right.get(place).is_some_and(|&x| x != 0);
        }
        Ok(by_row)
    }

    pub fn right(&mut self) -> Result<Vec<bool>, String> {
        let count = self.rows.reach.len();
        for offset in (0..count).step_by(self.batch) {
            let n = self.batch.min(count - offset);
            let reach = self.rows.reach[offset..offset + n].iter().copied().max().unwrap_or(0);
            self.passed(true, offset, n, reach)?;
            if self.pointing {
                self.pointer_pass(true, offset, n)?;
            }
        }
        let right = self.card.stream.clone_dtoh(&self.work.right).map_err(err)?;
        Ok(right.iter().take(count).map(|&x| x != 0).collect())
    }

    fn passed(&mut self, own_order: bool, offset: usize, n: usize, reach: usize) -> Result<(), String> {
        let CardTrainer { card, kernels: k, weights: w, rows, work, slots, item, hidden: h, outputs: c_n, .. } = self;
        let order = if own_order { &rows.identity } else { &work.order };
        let (offset32, n32, reach32, slots32, item32, h32, c32) = (int(offset)?, int(n)?, int(reach)?, int(*slots)?, int(*item)?, int(*h)?, int(*c_n)?);
        let mut l = card.stream.launch_builder(&k.embedded);
        l.arg(&mut work.embedded).arg(&w.table).arg(&rows.ids).arg(&rows.slot_ids).arg(&rows.row_slots).arg(&rows.places).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&reach32).arg(&slots32).arg(&item32);
        launched_as(l, n * reach, "embedded", &card.stream)?;
        let inner = int(reach * *item)?;
        let width = int(*slots * *item)?;
        let mut l = card.stream.launch_builder(&k.product);
        l.arg(&mut work.pre).arg(&work.embedded).arg(&w.slot_weights);
        l.arg(&n32).arg(&h32).arg(&inner).arg(&width).arg(&h32).arg(&h32).arg(&PLAIN).arg(&PLAIN);
        launched_tiled(l, tiles(n, *h), "hidden product", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.hidden_rest);
        l.arg(&mut work.pre).arg(&w.fill).arg(&w.hidden_bias).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&h32);
        launched_as(l, n * *h, "hidden rest", &card.stream)?;
        let count = int(n * *h)?;
        let mut l = card.stream.launch_builder(&k.rectified_rows);
        l.arg(&mut work.rectified).arg(&work.pre).arg(&count);
        launched_as(l, n * *h, "rectified", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.product);
        l.arg(&mut work.d_out).arg(&work.rectified).arg(&w.out);
        l.arg(&n32).arg(&c32).arg(&h32).arg(&h32).arg(&h32).arg(&c32).arg(&PLAIN).arg(&TURNED);
        launched_tiled(l, tiles(n, *c_n), "output product", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.outputs);
        l.arg(&mut work.d_out).arg(&mut work.loss).arg(&mut work.right).arg(&w.out_bias).arg(&rows.wanted).arg(order);
        l.arg(&offset32).arg(&n32).arg(&c32).arg(&f32::MIN_POSITIVE);
        launched_tiled(l, n * TILE * TILE, "outputs", &card.stream)
    }

    fn pointer_pass(&mut self, own_order: bool, offset: usize, n: usize) -> Result<(), String> {
        let CardTrainer { card, kernels: k, weights: w, rows, work, slots, item, hidden: h, .. } = self;
        let order = if own_order { &rows.identity } else { &work.order };
        let (offset32, n32, slots32, item32, h32) = (int(offset)?, int(n)?, int(*slots)?, int(*item)?, int(*h)?);
        let mut l = card.stream.launch_builder(&k.product);
        l.arg(&mut work.pointed_part).arg(&work.rectified).arg(&w.point);
        l.arg(&n32).arg(&item32).arg(&h32).arg(&h32).arg(&item32).arg(&item32).arg(&PLAIN).arg(&PLAIN);
        launched_tiled(l, tiles(n, *item), "pointed product", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.point_scores);
        l.arg(&mut work.pointer).arg(&work.pointed_part).arg(&work.embedded).arg(&rows.pointable).arg(&rows.pointable_at).arg(&rows.pointed_at).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&slots32).arg(&item32);
        launched_as(l, n * *slots, "point_scores", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.pointed);
        l.arg(&mut work.pointer).arg(&mut work.loss).arg(&mut work.right).arg(&rows.pointable).arg(&rows.pointable_at).arg(&rows.pointed).arg(&rows.pointed_at).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&slots32).arg(&f32::MIN_POSITIVE);
        launched_as(l, n, "pointed", &card.stream)
    }

    fn stepped(&mut self, offset: usize, n: usize, reach: usize) -> Result<(), String> {
        self.mark += 1;
        if self.pointing {
            self.pointer_pass(false, offset, n)?;
        }
        let (rate, least, mark, pointing) = (self.rate, self.least, self.mark, self.pointing);
        let CardTrainer { card, kernels: k, weights: w, squares: q, rows, work, slots, item, hidden: h, outputs: c_n, .. } = self;
        let (offset32, n32, reach32, slots32, item32, h32, c32, pointing32) = (int(offset)?, int(n)?, int(reach)?, int(*slots)?, int(*item)?, int(*h)?, int(*c_n)?, i32::from(pointing));
        let order = &work.order;
        let mut l = card.stream.launch_builder(&k.hidden_back);
        l.arg(&mut work.d_pre).arg(&work.d_out).arg(&work.pointer).arg(&work.pre).arg(&work.embedded).arg(&w.out).arg(&w.point).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&slots32).arg(&item32).arg(&h32).arg(&c32).arg(&pointing32);
        launched_as(l, n * *h, "hidden_back", &card.stream)?;
        let inner = int(reach * *item)?;
        let width = int(*slots * *item)?;
        let mut l = card.stream.launch_builder(&k.product);
        l.arg(&mut work.d_embedded).arg(&work.d_pre).arg(&w.slot_weights);
        l.arg(&n32).arg(&inner).arg(&h32).arg(&h32).arg(&h32).arg(&width).arg(&PLAIN).arg(&TURNED);
        launched_tiled(l, tiles(n, reach * *item), "embed product", &card.stream)?;
        if pointing {
            let mut l = card.stream.launch_builder(&k.embed_point);
            l.arg(&mut work.d_embedded).arg(&work.pointer).arg(&work.pointed_part).arg(&rows.filled).arg(order);
            l.arg(&offset32).arg(&n32).arg(&reach32).arg(&slots32).arg(&item32);
            launched_as(l, n * reach * *item, "embed point", &card.stream)?;
        }
        let mut l = card.stream.launch_builder(&k.product);
        l.arg(&mut work.slot_grad).arg(&work.embedded).arg(&work.d_pre);
        l.arg(&inner).arg(&h32).arg(&n32).arg(&width).arg(&h32).arg(&h32).arg(&TURNED).arg(&PLAIN);
        launched_tiled(l, tiles(reach * *item, *h), "slot product", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.slot_apply);
        let grown = int(reach * *item * *h)?;
        l.arg(&mut w.slot_weights).arg(&mut q.slot_weights).arg(&work.slot_grad).arg(&grown).arg(&rate).arg(&least);
        launched_as(l, reach * *item * *h, "slot apply", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.out_step);
        l.arg(&mut w.out).arg(&mut q.out).arg(&mut w.out_bias).arg(&mut q.out_bias).arg(&work.d_out).arg(&work.pre);
        l.arg(&n32).arg(&h32).arg(&c32).arg(&rate).arg(&least);
        launched_as(l, *c_n * *h, "out_step", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.bias_step);
        l.arg(&mut w.hidden_bias).arg(&mut q.hidden_bias).arg(&work.d_pre);
        l.arg(&n32).arg(&h32).arg(&rate).arg(&least);
        launched_as(l, *h, "bias_step", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.slot_step);
        l.arg(&mut w.slot_weights).arg(&mut q.slot_weights).arg(&mut w.fill).arg(&mut q.fill).arg(&work.d_pre).arg(&work.embedded).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&reach32).arg(&slots32).arg(&item32).arg(&h32).arg(&rate).arg(&least);
        launched_as(l, reach * *h, "fill step", &card.stream)?;
        if pointing {
            let mut l = card.stream.launch_builder(&k.point_weighted);
            l.arg(&mut work.pointed_part).arg(&work.pointer).arg(&work.embedded).arg(&rows.filled).arg(order);
            l.arg(&offset32).arg(&n32).arg(&slots32).arg(&item32);
            launched_as(l, n * *item, "point weighted", &card.stream)?;
            let mut l = card.stream.launch_builder(&k.product);
            l.arg(&mut work.slot_grad).arg(&work.rectified).arg(&work.pointed_part);
            l.arg(&h32).arg(&item32).arg(&n32).arg(&h32).arg(&item32).arg(&item32).arg(&TURNED).arg(&PLAIN);
            launched_tiled(l, tiles(*h, *item), "point product", &card.stream)?;
            let mut l = card.stream.launch_builder(&k.slot_apply);
            let count = int(*h * *item)?;
            l.arg(&mut w.point).arg(&mut q.point).arg(&work.slot_grad).arg(&count).arg(&rate).arg(&least);
            launched_as(l, *h * *item, "point apply", &card.stream)?;
        }
        let mut l = card.stream.launch_builder(&k.table_sum);
        l.arg(&mut work.sums).arg(&mut work.touched).arg(&mut work.total).arg(&work.d_embedded).arg(&work.loss).arg(&rows.ids).arg(&rows.slot_ids).arg(&rows.row_slots).arg(&rows.places).arg(&rows.hot_of).arg(&rows.hot_ids).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&slots32).arg(&item32).arg(&mark);
        launched_tiled(l, (n * *slots).div_ceil(TILE * TILE) * TILE * TILE, "table_sum", &card.stream)?;
        let mut l = card.stream.launch_builder(&k.table_apply);
        l.arg(&mut w.table).arg(&mut q.table).arg(&mut work.sums).arg(&mut work.touched).arg(&rows.ids).arg(&rows.slot_ids).arg(&rows.row_slots).arg(&rows.places).arg(&rows.filled).arg(order);
        l.arg(&offset32).arg(&n32).arg(&slots32).arg(&item32).arg(&mark).arg(&rate).arg(&least);
        launched_as(l, n * *slots, "table_apply", &card.stream)
    }

    pub fn written_into(&self, (net, sums): (&mut Stacked, &mut Stacked)) -> Result<(), String> {
        self.weights.written_into(&self.card, net)?;
        self.squares.written_into(&self.card, sums)
    }
}
