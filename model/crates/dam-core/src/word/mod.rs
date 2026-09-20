mod answering;
mod appear;
mod asked;
mod english;
mod lookup;
mod moving;
mod mind;
mod moves;
mod physics;
mod rows;
mod shape;
mod writing;
mod teacher;
mod view;
mod walks;
mod written;

pub use mind::{function_word, asked_sentence, closed_sentence, heard_word, node_kind, sentence_slots, word_at, WordEnglish};
pub use moves::{RecordClasses, WordMove, WordStep, ALL as WORD_MOVES, RECORD_MARK};
pub use physics::{word_stepped, WordWorld, OWNER_TAG};
pub use rows::{read_by_words, word_rows, WordRead, WordTrails, POINTED_MARK};
pub use shape::{shape_names, world_built, world_form, world_holds, WorldShape};
pub use teacher::{taught_words, taught_words_answered, WordGame};

use patterns::source;

#[cfg(test)]
mod tests {
    use super::*;
    use patterns::because;
    use crate::cursor::{output_answers, told_seeds, CursorMind};
    use crate::quiz::{parse_quiz, sentences};
    use crate::words::simple_words;

    static SEEDED: std::sync::OnceLock<CursorMind> = std::sync::OnceLock::new();

    const DATA: &str = "../../data";
    because!(DATA, WordReading, "where the seeds and the lessons stand, from the crate the tests run in");

    const WORLD_LESSONS: &str = "train/world";
    because!(WORLD_LESSONS, WordReading, "the folder of the lessons that shape the world and then ask about it, each line a text, the \
     world it must leave and the answers its questions must write");

    fn listed(folder: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut files: Vec<_> = std::fs::read_dir(folder).expect("a folder").flatten().map(|e| e.path()).collect();
        files.sort();
        files
    }
    because!(listed, WordReading, "the entries of a folder in name order");

    fn seeded() -> CursorMind {
        SEEDED.get_or_init(|| {
            let mut mind = CursorMind::default();
            let files: Vec<_> = listed(&std::path::Path::new(DATA).join("seeds")).into_iter().filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
            for file in files {
                let text = std::fs::read_to_string(&file).expect("a seeds file");
                told_seeds(&mut mind, &text).expect("seeds told");
            }
            mind.tree.settled();
            mind.tree.state = mind.tree.len();
            mind
        }).clone()
    }

    #[test]
    fn teacher_shapes_every_world() {
        let mut failures = Vec::new();
        let mut lines = 0;
        for path in listed(&std::path::Path::new(DATA).join(WORLD_LESSONS)).into_iter().filter(|p| p.extension().is_some_and(|x| x == "txt")) {
            let file = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let text = std::fs::read_to_string(&path).expect("a lesson");
            for it in parse_quiz(&text, simple_words).expect("a quiz") {
                lines += 1;
                let mut mind = seeded();
                let mut trails = Vec::new();
                for words in sentences(it.words.clone()).into_iter().filter(|s| !s.is_empty()).map(closed_sentence) {
                    let steps = taught_words(&mut mind, &words);
                    trails.push(words.iter().zip(&steps).map(|(w, taken)| format!("{w}: {}", taken.iter().map(|s| s.class()).collect::<Vec<_>>().join(" "))).collect::<Vec<_>>().join(" | "));
                }
                if !world_holds(&mind.tree, &it.expect) {
                    let story: Vec<String> = mind.tree.paths().into_iter().filter(|p| it.expect.iter().any(|e| e.split(" -> ").next().is_some_and(|head| p.starts_with(&shape::shape_name(head).name)))).collect();
                    failures.push(format!("{file} line {}: shape {:?}
  world: {}
  {}", it.line, it.expect, story.join("; "), trails.join("
  ")));
                }
                for q in &it.questions {
                    let asked = asked_sentence(q.words.clone());
                    let steps = taught_words(&mut mind, &asked);
                    if !output_answers(&mind.output, true, &q.answer) {
                        let trail = asked.iter().zip(&steps).map(|(w, taken)| format!("{w}: {}", taken.iter().map(|s| s.class()).collect::<Vec<_>>().join(" "))).collect::<Vec<_>>().join(" | ");
                        failures.push(format!("{file} line {}: {} expected {} got {:?}
  {trail}", it.line, q.text, q.answer, mind.output));
                    }
                }
            }
        }
        assert!(lines > 0, "no world lessons");
        assert!(failures.is_empty(), "{} of {lines} lines failed:
{}", failures.len(), failures.join("
"));
    }
}

pub struct WordReading;
source!(
    WordReading,
    "the user's reading one word at a time, agreed on the debug page, the world as a space the network shapes word by word: each word is \
     one input, the network takes zero or more steps for it and then continues, a step being a move written as a record with one pointer \
     at a word of the sentence still on the stack, so a count waits until the word it counts arrives and a past form is pointed at from \
     the value it dates; the stack holds one sentence and is emptied at the next, the tree is the memory; a deterministic teacher writes \
     the steps for every word and the network learns them as rows, and the goal is that the pattern holds on words never seen"
);
