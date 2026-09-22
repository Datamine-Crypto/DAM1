use crate::files::{SEEDS_FILE, from_json, json_text, read_bytes, read_quiz, read_text, seed_names, seed_text, write_bytes, write_text};
use crate::train::{EpochReport, TrainRow, Training, rows_of, trained_acts};
use crate::words::{flag, opt, pct};
use dam::cursor::{CursorMind, CursorRow, cursor_told, told_seeds};
use dam::network::{FEATURE_BITS, FeatureId, NetworkClasses, Stacked};
use dam::numbers::zero;
use dam::state::state_bytes;
use dam::word::WordStep;
use dam::quiz::{QuizItem, parse_quiz};
use dam::words::simple_words;
use patterns::{because, source};
use serde::Serialize;

pub struct Console {
    pub training: Training,
    pub shaping: Shaping,
}
because!(Console, "everything the console needs from the spec in one value: how a network over the stack is trained and the limits it reads and trains under");

#[derive(Clone, Copy, Debug)]
pub struct Shaping {
    pub steps: usize,
    pub hidden: usize,
    pub settle: usize,
    pub enough: f32,
    pub patience: usize,
    pub epochs: usize,
}
because!(
    Shaping,
    "the limits from the spec a network reads and trains under: the most steps at one word, its hidden units, how many more epochs training goes on once enough learned lines are taken, the share of learned lines taken that is enough, how many epochs without fewer rows taken wrongly end a run, and the most epochs a run makes"
);

pub struct Options;
source!(Options, "the options of the console's commands, each starting with two dashes; the ones that take a value take the next word");

pub(crate) const OUT: &str = "--out";

pub const SEEDS_OPT: &str = "--seeds";
because!(SEEDS_OPT, Options, "the seeds a state is made from: the names of seeds files joined by commas, the word for none, or every file of the seeds folder in name order when the option is not given");

pub const NO_SEEDS: &str = "none";
because!(NO_SEEDS, Options, "the word the seeds option takes for a state made from no seeds at all, an empty tree, the state every lesson that names no seeds is read from");

pub const SEED_GAP: &str = ",";

pub const STATE_OPT: &str = "--state";
because!(STATE_OPT, Options, "the state every line of a teach or a report starts from before its own seeds: the names of seeds files joined by commas, the word for every file, or nothing when not given, so a curriculum is taught from the state the chat starts from while each lesson keeps the seeds it names");

pub const STATE_ALL: &str = "all";
because!(STATE_ALL, Options, "the word the state option takes for a state made from every file of the seeds folder, the state the chat starts from");
because!(SEED_GAP, Options, "what joins the names of seeds files in the seeds option");

because!(OUT, Options, "the file a command writes to");
pub(crate) const QUIZ: &str = "--quiz";
because!(QUIZ, Options, "the quiz file a command teaches or reports");
pub(crate) const ALL: &str = "--all";
because!(ALL, Options, "show every input of a report, not only the ones the network gets wrong");
const ROWS: &str = "--rows";
because!(ROWS, Options, "the rows file a network over the stack is trained from, as a teaching run writes it");
const SEED: &str = "--seed";
because!(SEED, Options, "the seed a training run draws from instead of the spec's");
const EPOCHS: &str = "--epochs";
because!(EPOCHS, Options, "the most epochs a training run makes instead of the spec's");
const HIDDEN_OPT: &str = "--hidden";
because!(HIDDEN_OPT, Options, "how many hidden units a training run's network has instead of the spec's");
const PATIENCE: &str = "--patience";
because!(PATIENCE, Options, "how many epochs without fewer rows taken wrongly a training run allows instead of the spec's");

const ENOUGH_OPT: &str = "--enough";
because!(ENOUGH_OPT, Options, "the share of learned lines taken as taught at which a training run stops, instead of the spec's; one carries it to every line");
const LIMIT_OPT: &str = "--limit";
because!(LIMIT_OPT, Options, "the most seconds a training run goes on before it stops after its epoch and writes its network, none when not given, so a run is looked at before it is given more time");
const SLICES_OPT: &str = "--slices";
because!(SLICES_OPT, Options, "how many rows a training step sums before AdaGrad moves the weights, instead of the spec's, so a run on the card can take many rows a step at once");
const CARD_OPT: &str = "--card";
because!(CARD_OPT, Options, "a training run works its steps and checks on the graphics card instead of the processor");

const FROM_OPT: &str = "--from";
because!(FROM_OPT, Options, "the network a run starts from: for training a network it grows from instead of a fresh one, for teaching a network whose steps it takes wherever the teacher could have taken them");
const SETTLE_OPT: &str = "--settle";
because!(SETTLE_OPT, Options, "how many more epochs a training run goes on once enough learned lines are taken, instead of the spec's");
pub(crate) const NETWORK: &str = "--network";
because!(NETWORK, Options, "the network a report reads with, the weights file with its classes in the same name followed by the classes ending");
pub(crate) const CLASSES_END: &str = ".json";
because!(CLASSES_END, Options, "the ending added to a network's weights file to name the file listing the step classes its outputs stand for");
const SUMS_END: &str = ".sums";
because!(SUMS_END, Options, "the ending added to a network's weights file to name the file of AdaGrad's sums a run kept, laid out as the weights are, so a run that grows from the network goes on with the steps it had shrunk");

pub struct ConsoleParts;
source!(
    ConsoleParts,
    "what the commands of the console share: the rows a teacher writes and a run reads, the training of a network on those rows, the state file every chat starts from, and the mind a line of a quiz starts from"
);

pub(crate) const LEARN_MODE: &str = "train";
because!(LEARN_MODE, ConsoleParts, "the word that has a network taught the rows a teacher wrote");
pub(crate) const STATE_MODE: &str = "state";
because!(STATE_MODE, ConsoleParts, "the word that asks for the state file every chat starts from");
pub(crate) const ROW_BREAK: &str = "\n";
because!(ROW_BREAK, ConsoleParts, "what ends one row of a rows file, one value a line");

const PERCENT: f32 = 100.0;
because!(PERCENT, ConsoleParts, "a share as a percent is the share times this many");

pub fn usage() -> String {
    format!("usage: dam {} teach|{LEARN_MODE}|report|{STATE_MODE}", crate::word::WORD_COMMAND)
}
because!(usage, ConsoleParts, "the one line printed for a command the console does not know");

pub fn run(args: &[String], console: &Console) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some(crate::word::WORD_COMMAND) => crate::word::word_command(args.get(1..).unwrap_or_default(), console),
        _ => Err(usage()),
    }
}
because!(run, ConsoleParts, "the console: the first word must name the command of the reading one word at a time, and the rest of the line is handed to it");

#[derive(Serialize)]
pub(crate) struct StackRecord {
    pub(crate) file: String,
    pub(crate) line: usize,
    pub(crate) test: bool,
    pub(crate) blanked: bool,
    pub(crate) texts: Vec<String>,
    pub(crate) events: Vec<Vec<FeatureId>>,
    pub(crate) rows: Vec<StackRow>,
}
because!(StackRecord, ConsoleParts, "the rows of one quiz line as a trainer reads them, written once a line and once more with the names blanked: the lesson it came from and the line, whether it is held out, whether the names are blanked, the events the line's stack held, oldest first, each as its text and its feature places without the place of its depth, and the rows, each a window over those events, so a stack of hundreds of events is written once and not once per step, which is what lets the rows of a large curriculum fit a file and the card");

#[derive(Serialize)]
pub(crate) struct StackRow {
    shape: String,
    word: String,
    behavior: String,
    depth: usize,
    pointed: Vec<usize>,
    pointable: Vec<usize>,
}
because!(StackRow, ConsoleParts, "one step the teacher took, as a trainer reads a row of a record: the shape of its input, the input item it was taken at, the step taught there, how many events the line's stack had held before it, so the row's slots are the newest of them up to the stack's size, and for a copy the slots its token stands in and may be copied from");

pub(crate) fn record_extended(records: &mut [StackRecord], shape: &str, row: &CursorRow) -> Result<(), String> {
    let which = usize::from(row.blanked);
    let recorded = records[which].events.len();
    let behind = row.depth.checked_sub(recorded).ok_or_else(|| format!("line {}: the stack shrank from {} to {} events", records[which].line, recorded, row.depth))?;
    let gap = behind.saturating_sub(row.items.len());
    if gap > 0 && (!row.blanked || records[0].events.len() < recorded + gap) {
        return Err(format!("line {}: {behind} events pushed at once, more than the stack shows, at {shape}", records[which].line));
    }
    let older: Vec<(Vec<FeatureId>, String)> = (recorded..recorded + gap).map(|at| (records[0].events[at].clone(), records[0].texts[at].clone())).collect();
    let record = &mut records[which];
    for (ids, text) in older {
        record.events.push(ids);
        record.texts.push(text);
    }
    let fresh = row.depth - record.events.len();
    for (ids, text) in row.items[..fresh].iter().zip(&row.events[..fresh]).rev() {
        record.events.push(ids[..ids.len().saturating_sub(1)].to_vec());
        record.texts.push(text.clone());
    }
    record.rows.push(StackRow { shape: shape.to_string(), word: row.word.clone(), behavior: row.class.clone(), depth: row.depth, pointed: row.pointed.clone(), pointable: row.pointable.clone() });
    Ok(())
}
because!(record_extended, ConsoleParts, "a row added to its line's record, the plain record or the record of the blanked twins: the events the stack gained since the record's last row, read off the row's newest slots, oldest first, each without its last feature place, the place of its depth, which the trainer adds by slot, and the row as its depth and its step; the twin record takes the events older than the row shows from the plain record, since a twin is written only for a step with a name to blank, so the twins fall behind the stack by whole inputs beside a state that knows most names, and a row reads only the slots it shows");

pub(crate) fn quiz(args: &[String]) -> Result<Vec<QuizItem>, String> {
    let file: String = opt(args, QUIZ, String::new())?;
    parse_quiz(&read_quiz(&file)?, simple_words)
}
because!(quiz, ConsoleParts, "the quiz file the command names, read into its lines");

pub(crate) fn share(x: f32) -> String {
    pct(x, PERCENT)
}
because!(share, ConsoleParts, "a share as a percent with one decimal");

fn named_rows(args: &[String]) -> Result<Vec<TrainRow>, String> {
    let rows_path: String = opt(args, ROWS, String::new())?;
    let out: String = opt(args, OUT, String::new())?;
    if rows_path.is_empty() || out.is_empty() {
        return Err(format!("dam {} {LEARN_MODE} needs {ROWS} and {OUT}", crate::word::WORD_COMMAND));
    }
    rows_of(&read_text(&rows_path)?)
}
because!(named_rows, ConsoleParts, "the rows a training run names, refused when the run names no rows file or no file to write the network to");

fn written_network(args: &[String], console: &Console, rows: &[TrainRow], (outputs, base): (Vec<String>, Option<(&Stacked, Option<&Stacked>)>)) -> Result<(), String> {
    let out: String = opt(args, OUT, String::new())?;
    let training = Training { seed: opt(args, SEED, console.training.seed)?, epochs: opt(args, EPOCHS, console.shaping.epochs)?, hidden: opt(args, HIDDEN_OPT, console.shaping.hidden)?, slices: opt(args, SLICES_OPT, console.training.slices)?, ..console.training };
    let learned = rows.iter().filter(|r| !r.test).count();
    let settle: usize = opt(args, SETTLE_OPT, console.shaping.settle)?;
    let enough: f32 = opt(args, ENOUGH_OPT, console.shaping.enough)?;
    let patience: usize = opt(args, PATIENCE, console.shaping.patience)?;
    let limit: f32 = opt(args, LIMIT_OPT, zero())?;
    let (net, sums) = trained_acts(rows, &outputs, (&training, (settle, enough, patience, (limit > zero()).then_some(limit)), (base, flag(args, CARD_OPT))), &mut |e: &EpochReport| {
        println!("epoch {}: loss {:.4}, rows wrong {} of {learned}, {:.1} seconds", e.epoch, e.loss, e.wrong, e.seconds);
        if std::env::var("DAM_TRACE_K").is_ok() { eprintln!("  kernels ms: {}", dam_card::steps::kernel_times()); }
    })?;
    let bytes = net.bytes();
    write_bytes(&out, &bytes)?;
    write_bytes(&format!("{out}{SUMS_END}"), &sums.bytes())?;
    write_text(&format!("{out}{CLASSES_END}"), &json_text(&NetworkClasses { bits: FEATURE_BITS, half: false, classes: outputs, stacked: net.shape(), voters: usize::from(true) })?)?;
    println!("wrote {out} ({} bytes, {} numbers), seed {}", bytes.len(), net.numbers(), training.seed);
    Ok(())
}
because!(written_network, ConsoleParts, "a network taught rows with the spec's values or the seed, epochs, hidden units, step size, settling, share, patience and time limit a run names, on the card when it asks, grown from a base with the sums it kept when there is one, and written with its classes file and its AdaGrad sums beside it");

pub(crate) fn word_state(args: &[String]) -> Result<(), String> {
    let out: String = opt(args, OUT, String::new())?;
    if out.is_empty() {
        return Err(format!("{STATE_MODE} needs {OUT}"));
    }
    let seeds: String = opt(args, SEEDS_OPT, String::new())?;
    let (mind, told) = seeded_mind(&seeds)?;
    let bytes = state_bytes(&mind)?;
    write_bytes(&out, &bytes)?;
    println!("wrote {out} ({} bytes, {} nodes from {told} statements)", bytes.len(), mind.tree.len());
    Ok(())
}
pub fn seeded_mind(seeds: &str) -> Result<(CursorMind, usize), String> {
    let mut mind = CursorMind::default();
    let mut told = 0;
    let names: Vec<String> = if seeds.is_empty() || seeds == STATE_ALL {
        seed_names()?
    } else if seeds == NO_SEEDS {
        Vec::new()
    } else {
        seeds.split(SEED_GAP).map(str::to_string).collect()
    };
    let parts: Vec<(String, String)> = names.into_iter().map(|name| seed_text(&name).map(|text| (name, text))).collect::<Result<Vec<_>, _>>()?;
    for (name, text) in parts {
        told += told_seeds(&mut mind, &text).map_err(|e| format!("{SEEDS_FILE}/{name}: {e}"))?;
    }
    Ok((mind, told))
}
because!(seeded_mind, ConsoleParts, "a fresh mind told the seeds a state is made from, with how many statements it was told: every seeds file in name order when no seeds are named or the word for all is, none for the word for none, else each named file, refused when a named file is missing, so the chat, the model export and its check make the same state from the same words");

because!(word_state, ConsoleParts, "the state file every chat starts from: the seeds the run names told into a fresh mind in name order and its tree written as bytes to the file the run names, so the page loads the state whole");

pub(crate) fn loaded_network<T>(path: &str, of_class: impl Fn(&str) -> Option<T>) -> Result<(Stacked, Vec<T>), String> {
    let named: NetworkClasses = from_json(&read_text(&format!("{path}{CLASSES_END}"))?, path)?;
    let steps = named.classes.iter().map(|c| of_class(c).ok_or(format!("{path}: {c} is no step the network reads with"))).collect::<Result<Vec<_>, _>>()?;
    Ok((Stacked::read(&read_bytes(path)?, named.stacked, named.classes.len())?, steps))
}
because!(loaded_network, ConsoleParts, "a trained network from its weights and its classes file, each class read back as the step it names by the reading given, the cursor's or the one word at a time, refused when one names no step");

pub(crate) fn loaded_voters<T>(path: &str, of_class: impl Fn(&str) -> Option<T>) -> Result<(Vec<Stacked>, Vec<T>), String> {
    let named: NetworkClasses = from_json(&read_text(&format!("{path}{CLASSES_END}"))?, path)?;
    let steps = named.classes.iter().map(|c| of_class(c).ok_or(format!("{path}: {c} is no step the network reads with"))).collect::<Result<Vec<_>, _>>()?;
    Ok((Stacked::read_voters(&read_bytes(path)?, named.stacked, named.classes.len(), named.voters.max(usize::from(true)))?, steps))
}
because!(loaded_voters, ConsoleParts, "the networks one weights file holds, one or the several its classes file counts as voters, with the steps the classes name");

pub(crate) fn word_train(args: &[String], console: &Console) -> Result<(), String> {
    let rows = named_rows(args)?;
    let from_path: String = opt(args, FROM_OPT, String::new())?;
    let base = if from_path.is_empty() { None } else { Some(loaded_network(&from_path, WordStep::of_class)?) };
    let sums_path = format!("{from_path}{SUMS_END}");
    let kept = match &base {
        Some((net, _)) if std::path::Path::new(&sums_path).exists() => Some(Stacked::read(&read_bytes(&sums_path)?, net.shape(), net.out_bias.len())?),
        _ => None,
    };
    let mut outputs: Vec<String> = base.as_ref().map(|(_, steps)| steps.iter().map(WordStep::class).collect()).unwrap_or_default();
    for name in rows.iter().filter_map(|r| r.behavior.clone()) {
        if !outputs.contains(&name) {
            outputs.push(name);
        }
    }
    written_network(args, console, &rows, (outputs, base.as_ref().map(|(net, _)| (net, kept.as_ref()))))
}
because!(
    word_train,
    ConsoleParts,
    "a network taught the rows a teacher wrote, its outputs every step class the rows take, after the classes of the network it grows from when a run names one, so a grown network keeps the meaning of its outputs, and with the AdaGrad sums that network's run kept when they are beside it, so a run cut short goes on where it stopped"
);

const KNOWN_SEED: &str = "number-words";
because!(KNOWN_SEED, ConsoleParts, "the seeds file every line on the cursor starts from: the number words, which any reader of the grades already knows, so a text that says two cats never has to work out that two is worth two");

static SEEDED: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<Vec<String>, CursorMind>>> = std::sync::OnceLock::new();
because!(SEEDED, ConsoleParts, "the mind each set of seeds makes, kept once made, so every line that starts from the same seeds clones it, which copies only the pointer to its tree, instead of telling the seeds again; a state of millions of nodes is told once a run");

pub fn state_seeds(state: &str) -> Result<Vec<String>, String> {
    Ok(if state.is_empty() || state == NO_SEEDS {
        Vec::new()
    } else if state == STATE_ALL {
        seed_names()?
    } else {
        state.split(SEED_GAP).map(str::to_string).collect()
    })
}
because!(state_seeds, ConsoleParts, "the seeds files a state option names, in name order: none when the option is not given or names none, every file for the word for all, else the names listed");

fn seeded_start(state: &[String], seeds: &[String]) -> Result<CursorMind, String> {
    let mut key: Vec<String> = vec![KNOWN_SEED.to_string()];
    for name in state.iter().chain(seeds.iter()) {
        if !key.contains(name) {
            key.push(name.clone());
        }
    }
    let kept = SEEDED.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Some(mind) = kept.lock().map_err(|e| e.to_string())?.get(&key) {
        return Ok(mind.clone());
    }
    let mut fresh = CursorMind::default();
    for name in &key {
        told_seeds(&mut fresh, &seed_text(name)?).map_err(|e| format!("{SEEDS_FILE}/{name}: {e}"))?;
    }
    fresh.tree.settled();
    fresh.tree.state = fresh.tree.len();
    kept.lock().map_err(|e| e.to_string())?.insert(key, fresh.clone());
    Ok(fresh)
}
because!(seeded_start, ConsoleParts, "a fresh mind told the number words, the seeds of the state a run names and the seeds a line names, each once, made once for each set of seeds and cloned after, its tree settled so every clone shares the base, and marked as the state the story begins from, so the story's nodes come first by name");

pub fn cursor_started(it: &QuizItem, carried: &CursorMind, state: &[String]) -> Result<CursorMind, String> {
    let mut mind = if it.continues { carried.clone() } else { seeded_start(state, &it.seed)? };
    if let Some(refused) = it.world.iter().find(|statement| !cursor_told(&mut mind, statement, true)) {
        return Err(format!("line {}: {refused:?} is no statement the cursor can hold", it.line));
    }
    Ok(mind)
}
because!(
    cursor_started,
    ConsoleParts,
    "the cursor a line starts from: the tree and stack the line above left when the line carries on from it, else a tree with the number words every reader knows, the seeds of the state the run names and the statements of every seeds file the line names; then the statements of the line's world written into that tree as paths of hidden nodes, with nothing on the stack, which no find reaches until a walk lands on them, so the network looks for what a world holds, and a world can change after the network looked at it, as things move while nobody tells"
);
