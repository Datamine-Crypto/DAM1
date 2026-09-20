use crate::hub::pulled;
use dam::cursor::CursorMind;
use dam::quiz::parse_quiz;
use dam::state::state_bytes;
use dam::word::WordStep;
use dam::words::simple_words;
use dam_console::commands::{Console, SEEDS_OPT, STATE_ALL, seeded_mind, state_seeds};
use dam_console::files::{SEEDS_FILE, read_bytes, read_quiz, read_text};
use dam_console::word::line_read;
use dam_console::words::{opt, pct};
use dam1::check::{network_differences, source_tensors, state_differences, tensor_differences};
use dam1::layout::{CLASSES_END, WEIGHTS_FILE};
use dam1::model::{Build, Model, NetworkSource, Reading, loaded, written};
use patterns::{because, source};
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct ModelTool;
source!(
    ModelTool,
    "the commands of the model tool: export a word network and the permanent state as a model folder, check a folder against the network and the seeds it was made from, pull a model from the hub, and talk with a model"
);

pub const EXPORT_COMMAND: &str = "export";
because!(EXPORT_COMMAND, ModelTool, "writes a word network, one or the several that read by vote, the reading limit and the state the seeds make as a model folder");

pub const CHECK_COMMAND: &str = "check";
because!(CHECK_COMMAND, ModelTool, "compares a model folder with the network and the seeds it was made from, byte for byte, and grades it on every quiz file");

pub const PULL_COMMAND: &str = "pull";
because!(PULL_COMMAND, ModelTool, "fetches a model from the hub and prints the folder it sits in");

pub const TALK_COMMAND: &str = "talk";
because!(TALK_COMMAND, ModelTool, "says each text given to a model as one turn of a conversation and prints the reply");

pub const NETWORK_OPTION: &str = "--network";
because!(NETWORK_OPTION, ModelTool, "the network weights file a model is exported from or checked against, its classes beside it under the same name with the classes ending");

pub const OUT_OPTION: &str = "--out";
because!(OUT_OPTION, ModelTool, "the folder a model is exported to");

pub const COMMIT_OPTION: &str = "--commit";
because!(COMMIT_OPTION, ModelTool, "the commit a model is exported at, written into its config for whoever traces it back");

pub const MODEL_OPTION: &str = "--model";
because!(MODEL_OPTION, ModelTool, "a folder already written by the export command or fetched from the hub, read instead of fetching the model again");

pub const QUIZ_OPTION: &str = "--quiz";
because!(QUIZ_OPTION, ModelTool, "the folder of quiz files a check grades the model on, the training grades when not given");

pub const SEEDS_MARK: &str = ": ";
because!(SEEDS_MARK, ModelTool, "what stands between the seeds folder and the seeds named, in the record of what a model's state was made from, so a check reads the names back and makes the same state");

pub const QUIZ_FOLDER: &str = "data/train";
because!(QUIZ_FOLDER, ModelTool, "the folder the grades' quiz files stand in, one folder a grade, which a check grades a model on when no other is named");

pub const QUIZ_END: &str = "txt";
because!(QUIZ_END, ModelTool, "the ending of a quiz file, so a note that stands beside the lessons of a folder is not read as one");

pub const REPO_OPTION: &str = "--repo";
because!(REPO_OPTION, ModelTool, "a model repository on the hub, as owner and name");

pub const REVISION_OPTION: &str = "--revision";
because!(REVISION_OPTION, ModelTool, "the branch, tag or commit of a hub repository to fetch");

pub const DEFAULT_REVISION: &str = "main";
because!(DEFAULT_REVISION, ModelTool, "the branch a hub repository is fetched from when no revision is given");

const PERCENT: f32 = 100.0;
because!(PERCENT, ModelTool, "what a share is multiplied by to be said as a percentage, by what the word means");

pub const OPTION_MARK: &str = "--";
because!(OPTION_MARK, ModelTool, "what an option starts with, so the words after a command that are not options or their values are the texts to say");

pub const USAGE: &str = "commands:
  export --network FILE --out DIR [--commit HASH] [--seeds NAMES|none]
  check --model DIR --network FILE [--quiz DIR]
  pull --repo OWNER/NAME [--revision REVISION]
  talk (--model DIR | --repo OWNER/NAME [--revision REVISION]) TEXT...";
because!(USAGE, ModelTool, "what the tool prints when it is started without a command it knows");

fn needed(args: &[String], name: &str) -> Result<String, String> {
    let value: String = opt(args, name, String::new())?;
    if value.is_empty() { Err(format!("{name} is needed\n{USAGE}")) } else { Ok(value) }
}

fn network_from(file: &str) -> Result<NetworkSource, String> {
    Ok(NetworkSource { weights: read_bytes(file)?, classes: read_text(&format!("{file}{CLASSES_END}"))? })
}

fn seeded_state(seeds: &str) -> Result<Vec<u8>, String> {
    let (mind, _) = seeded_mind(seeds)?;
    state_bytes(&mind)
}
because!(seeded_state, ModelTool, "the permanent state as the console's state command makes it from the seeds named, as bytes, so an exported state and a checked one are the state the chat page loads");

fn state_record(seeds: &str) -> String {
    if seeds.is_empty() { SEEDS_FILE.to_string() } else { format!("{SEEDS_FILE}{SEEDS_MARK}{seeds}") }
}
because!(state_record, ModelTool, "what a model records its state was made from: the seeds folder alone when every file of it was told, else the folder and the seeds named");

fn seeds_of(record: &str) -> Result<String, String> {
    match record.strip_prefix(SEEDS_FILE) {
        Some("") => Ok(String::new()),
        Some(rest) => rest.strip_prefix(SEEDS_MARK).map(str::to_string).ok_or(format!("the state record {record:?} is not one this tool wrote")),
        None => Err(format!("the state record {record:?} is not one this tool wrote")),
    }
}
because!(seeds_of, ModelTool, "the seeds named in a model's state record, read back the way the record was written, refused when the record is not one this tool writes");

fn reading_of(console: &Console) -> Reading {
    Reading { steps: console.shaping.steps }
}

fn texts(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut value_next = false;
    for word in args {
        if value_next {
            value_next = false;
        } else if word.starts_with(OPTION_MARK) {
            value_next = true;
        } else {
            out.push(word.clone());
        }
    }
    out
}

fn export(args: &[String], console: &Console) -> Result<(), String> {
    let out = needed(args, OUT_OPTION)?;
    let network_file = needed(args, NETWORK_OPTION)?;
    let commit: String = opt(args, COMMIT_OPTION, String::new())?;
    let seeds: String = opt(args, SEEDS_OPT, String::new())?;
    let network = network_from(&network_file)?;
    let (named, nets) = network.voters().map_err(|e| format!("{network_file}: {e}"))?;
    let started = Instant::now();
    let state = seeded_state(&seeds)?;
    println!("made the state, {} bytes, from the seeds in {:?}", state.len(), started.elapsed());
    let model = Model { network, state, reading: reading_of(console), build: Build { commit, network: network_file, state: state_record(&seeds) } };
    written(&model, Path::new(&out))?;
    println!("exported {} networks that read by vote, {} numbers and {} step classes, to {out}", nets.len(), nets.iter().map(dam::network::Stacked::numbers).sum::<usize>(), named.classes.len());
    Ok(())
}

fn quiz_files(folder: &str) -> Result<Vec<PathBuf>, String> {
    let listed = |dir: &Path| -> Result<Vec<PathBuf>, String> {
        let mut found: Vec<PathBuf> = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?.filter_map(|entry| entry.ok().map(|e| e.path())).collect();
        found.sort();
        Ok(found)
    };
    let mut files = Vec::new();
    for path in listed(Path::new(folder))? {
        if path.is_dir() {
            files.extend(listed(&path)?.into_iter().filter(|p| p.is_file()));
        } else {
            files.push(path);
        }
    }
    files.retain(|file| file.extension().is_some_and(|end| end == QUIZ_END));
    Ok(files)
}
because!(quiz_files, ModelTool, "every quiz file under a folder, the files of each grade's folder in name order and no file of another ending, so a check grades a model on the whole curriculum");

fn graded(file: &Path, network: (&[dam::network::Stacked], &[WordStep]), (reading, state): (Reading, &[String])) -> Result<Vec<(bool, bool)>, String> {
    let items = parse_quiz(&read_quiz(&file.to_string_lossy())?, simple_words)?;
    let mut lines = Vec::new();
    let mut carried = CursorMind::default();
    for it in items {
        let (mind, inputs) = line_read(&it, &carried, state, network, reading.steps)?;
        carried = mind;
        lines.push((it.test, inputs.iter().all(|input| input.right)));
    }
    Ok(lines)
}
because!(
    graded,
    "every line of one quiz file read by a model's networks one word at a time, by the console's own reading of a line so the two grade alike, from the state the model ships with and then each lesson's own world, each sentence of its text and then each question on the same tree, a line right when every input continued within the step limit and the tree holds its shape or the output says its answer, with whether the line is held out"
);

fn share_of(lines: &[(bool, bool)], test: bool) -> String {
    let all: Vec<bool> = lines.iter().filter(|l| l.0 == test).map(|l| l.1).collect();
    let right = all.iter().filter(|&&r| r).count();
    if all.is_empty() { "none".to_string() } else { format!("{right} of {} ({})", all.len(), pct(right as f32 / all.len() as f32, PERCENT)) }
}

fn check(args: &[String], console: &Console) -> Result<(), String> {
    let dir = needed(args, MODEL_OPTION)?;
    let network_file = needed(args, NETWORK_OPTION)?;
    let quiz: String = opt(args, QUIZ_OPTION, QUIZ_FOLDER.to_string())?;
    let started = Instant::now();
    let model = loaded(Path::new(&dir))?;
    println!("loaded {dir} in {:?}", started.elapsed());
    let network = network_from(&network_file)?;
    let mut found = network_differences(&network, &model.network)?;
    let weights = std::fs::read(Path::new(&dir).join(WEIGHTS_FILE)).map_err(|e| format!("{WEIGHTS_FILE}: {e}"))?;
    let tensors = source_tensors(&network)?;
    found.extend(tensor_differences(&weights, &tensors)?);
    println!("compared {} tensors byte for byte with the source files", tensors.len());
    found.extend(state_differences(&seeded_state(&seeds_of(&model.build.state)?)?, &model.state));
    if model.reading != reading_of(console) {
        found.push("the reading limit differs from the spec this tool was built with".to_string());
    }
    let (named, nets) = model.network.voters()?;
    let recorded = seeds_of(&model.build.state)?;
    let state = state_seeds(if recorded.is_empty() { STATE_ALL } else { &recorded })?;
    let classes: Vec<WordStep> = named.classes.iter().filter_map(|class| WordStep::of_class(class)).collect();
    let mut everything = Vec::new();
    for file in quiz_files(&quiz)? {
        let lines = graded(&file, (&nets, &classes), (model.reading, &state))?;
        println!("{}: learned {}, held out {}", file.display(), share_of(&lines, false), share_of(&lines, true));
        everything.extend(lines);
    }
    println!("every file: learned {}, held out {}", share_of(&everything, false), share_of(&everything, true));
    println!("checked in {:?}", started.elapsed());
    if found.is_empty() {
        println!("the model matches its source: every tensor byte for byte, the network's classes and shape, the reading limit and the state");
        Ok(())
    } else {
        Err(found.join("\n"))
    }
}

fn folder_from(args: &[String]) -> Result<PathBuf, String> {
    let dir: String = opt(args, MODEL_OPTION, String::new())?;
    if !dir.is_empty() {
        return Ok(PathBuf::from(dir));
    }
    let repo = needed(args, REPO_OPTION)?;
    let revision: String = opt(args, REVISION_OPTION, DEFAULT_REVISION.to_string())?;
    pulled(&repo, &revision)
}

fn pull(args: &[String]) -> Result<(), String> {
    needed(args, REPO_OPTION)?;
    println!("{}", folder_from(args)?.display());
    Ok(())
}

fn talk(args: &[String]) -> Result<(), String> {
    let model = loaded(&folder_from(args)?)?;
    dam_page::load(&model.network.weights, &model.network.classes)?;
    dam_page::state(&model.state)?;
    for text in texts(args) {
        let started = Instant::now();
        let reply = dam_page::read(&text, model.reading.steps)?;
        println!("> {text}\n{reply}\n({:?})", started.elapsed());
    }
    Ok(())
}

pub fn run(args: &[String], console: &Console) -> Result<(), String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(USAGE.to_string());
    };
    match command.as_str() {
        EXPORT_COMMAND => export(rest, console),
        CHECK_COMMAND => check(rest, console),
        PULL_COMMAND => pull(rest),
        TALK_COMMAND => talk(rest),
        _ => Err(USAGE.to_string()),
    }
}
because!(run, "the model tool's commands dispatched on the first word it was started with, the console's settings passed in by the binary that wires them from the spec");
