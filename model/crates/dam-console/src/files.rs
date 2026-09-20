use patterns::{because, source};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::process::ExitCode;

pub struct RepositoryLayout;
source!(
    RepositoryLayout,
    "where the repository keeps what the console reads, under the data directory, so a run from the model folder finds it"
);

pub const QUIZ_PART: &str = ".txt";
because!(QUIZ_PART, RepositoryLayout, "the ending of the files in the quiz directory that are part of the quiz; anything else there is left alone");

const PART_BREAK: &str = "\n";
because!(PART_BREAK, RepositoryLayout, "the line break put after each quiz file as they are joined, so the last line of one file and the first of the next never run together");

pub const SEEDS_FILE: &str = "data/seeds";
because!(
    SEEDS_FILE,
    RepositoryLayout,
    "the seeds a line on the self-loop may name: files of statements in the forms the permanent state holds, one a line, written into the state before the line is read, so a line can rely on what a person already knows, as three is worth three, without the words for it being heard"
);

pub fn seed_names() -> Result<Vec<String>, String> {
    Ok(quiz_parts(SEEDS_FILE)?.into_iter().map(|(name, _)| name).collect())
}
because!(seed_names, RepositoryLayout, "every seed name, the files of the seeds directory in name order");

pub fn seed_text(name: &str) -> Result<String, String> {
    let path = format!("{SEEDS_FILE}/{name}{QUIZ_PART}");
    if !std::path::Path::new(&path).exists() {
        return Ok(String::new());
    }
    read_text(&path)
}
because!(seed_text, RepositoryLayout, "the text of one seeds file, or none for a name with no file");

pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}
because!(args, "the words the console was started with, the program's own name left out");

pub fn read_text(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))
}
because!(read_text, "a file as text, found from wherever the console was started, or the path and what went wrong");

pub fn quiz_parts(path: &str) -> Result<Vec<(String, String)>, String> {
    let at = std::path::PathBuf::from(path);
    if !at.is_dir() {
        let text = read_text(path)?;
        return Ok(vec![(stem_of(&at), text)]);
    }
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&at)
        .map_err(|e| format!("{path}: {e}"))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().is_some_and(|x| format!(".{}", x.to_string_lossy()) == QUIZ_PART))
        .collect();
    files.sort();
    files.iter().map(|file| Ok((stem_of(file), std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?))).collect()
}
because!(
    quiz_parts,
    "the quiz files of a directory in name order, each with the name it goes by, its ending left off, or a single file as the one part, so a test can score each file as an area of its own and the quiz can read them as one"
);

fn stem_of(path: &std::path::Path) -> String {
    path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn read_quiz(path: &str) -> Result<String, String> {
    let mut text = String::new();
    for (_, part) in quiz_parts(path)? {
        text.push_str(&part);
        text.push_str(PART_BREAK);
        text.push_str(dam::quiz::SEED_PREFIX);
        text.push_str(PART_BREAK);
    }
    Ok(text)
}
because!(
    read_quiz,
    "the quiz as one text: a single file as it is, or every quiz file of a directory in name order, one after the other, so the quiz can be kept as files by concept and read as before"
);

fn folder_made(path: &str) -> Result<(), String> {
    match std::path::Path::new(path).parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(folder) => std::fs::create_dir_all(folder).map_err(|e| format!("{path}: {e}")),
        None => Ok(()),
    }
}

pub fn write_text(path: &str, text: &str) -> Result<(), String> {
    folder_made(path)?;
    std::fs::write(path, text).map_err(|e| format!("{path}: {e}"))
}
because!(write_text, "text written to a file, its folder made first when it is not there, so a fresh checkout without the generated data folder can still write, or the path and what went wrong");

pub fn write_bytes(path: &str, bytes: &[u8]) -> Result<(), String> {
    folder_made(path)?;
    std::fs::write(path, bytes).map_err(|e| format!("{path}: {e}"))
}
because!(write_bytes, "bytes written to a file, its folder made first as for text, or the path and what went wrong; beside write_text, since a packed memory is not text");

pub fn read_bytes(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("{path}: {e}"))
}
because!(read_bytes, "the bytes of a file, or the path and what went wrong, so the console can start a turn from the packed memory the page loads");

pub fn json_text<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| e.to_string())
}
because!(json_text, "a value as one line of JSON, the form every file the console writes takes");

pub fn from_json<T: DeserializeOwned>(text: &str, what: &str) -> Result<T, String> {
    serde_json::from_str(text).map_err(|e| format!("{what}: {e}"))
}
because!(from_json, "a value read back from JSON, naming the file it came from when it does not fit");

pub fn exit_code(done: Result<(), String>) -> ExitCode {
    match done {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
because!(exit_code, "a command's result as the process's exit code, printing what went wrong, so a script can tell success from failure");
