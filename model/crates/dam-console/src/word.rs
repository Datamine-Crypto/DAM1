use crate::commands::{cursor_started, loaded_voters, quiz, record_extended, share, state_seeds, word_state, word_train, Console, StackRecord, ALL, LEARN_MODE, NETWORK, OUT, QUIZ, ROW_BREAK, STATE_MODE, STATE_OPT};
use crate::files::{json_text, write_text};
use crate::words::{flag, opt};
use dam::cursor::{output_answers, CursorMind};
use dam::quiz::{sentences, QuizItem};
use dam::network::Stacked;
use dam::word::{function_word, read_by_words, shape_names, taught_words_answered, word_rows, world_built, world_form, world_holds, WordRead, WordStep, POINTED_MARK};
use patterns::{because, source};

pub struct WordModes;
source!(
    WordModes,
    "the user's reading one word at a time on the console: teach writes the deterministic teacher's steps for every sentence of a quiz as rows, and report reads every line word by word with a network and checks the tree and the answers; train is the cursor's, since the rows are the same"
);

pub const WORD_COMMAND: &str = "word";
because!(WORD_COMMAND, WordModes, "the command word of the reading one word at a time");

const WORD_TEACH: &str = "teach";
because!(WORD_TEACH, WordModes, "the word that has the teacher write rows for a quiz, one word an input");

const WORD_REPORT: &str = "report";
because!(WORD_REPORT, WordModes, "the word that has a network read a quiz word by word and every line checked");

const NETWORKS_GAP: &str = ",";
because!(NETWORKS_GAP, WordModes, "what parts the files of several networks that read a quiz together by vote");

const SENTENCE_GAP: &str = " ";
because!(SENTENCE_GAP, WordModes, "what joins the words of a sentence when it is printed");

pub fn word_command(args: &[String], console: &Console) -> Result<(), String> {
    let rest = args.get(1..).unwrap_or_default();
    match args.first().map(String::as_str) {
        Some(WORD_TEACH) => word_teach(rest),
        Some(LEARN_MODE) => word_train(rest, console),
        Some(WORD_REPORT) => word_report(rest, console),
        Some(STATE_MODE) => word_state(rest),
        _ => Err(format!("dam {WORD_COMMAND} takes {WORD_TEACH}, {LEARN_MODE}, {WORD_REPORT} or {STATE_MODE}")),
    }
}
because!(word_command, WordModes, "the command of the reading one word at a time: its first word chooses teach, train, report or state");

fn word_sentences(it: &QuizItem) -> Vec<Vec<String>> {
    sentences(it.words.clone()).into_iter().filter(|s| !s.is_empty()).map(|s| dam::word::closed_sentence(s)).collect()
}
because!(word_sentences, WordModes, "the sentences of a quiz line's text, each as its words with its mark, none of them empty");

fn word_inputs(it: &QuizItem) -> Vec<(Vec<String>, Option<&str>)> {
    let texts = word_sentences(it);
    let last = texts.len().saturating_sub(1);
    let turn = it.turn.as_deref().filter(|answer| !answer.is_empty());
    let mut inputs: Vec<(Vec<String>, Option<&str>)> = texts.into_iter().enumerate().map(|(at, s)| (s, if at == last { turn } else { None })).collect();
    inputs.extend(it.questions.iter().map(|q| (dam::word::asked_sentence(q.words.clone()), Some(q.answer.as_str()))));
    inputs
}
because!(word_inputs, WordModes, "what a line has the reading take in, one input per sentence and then each question with its answer, the last sentence of a turn checked against the turn's answer when it has one, and a question that opens with no word that asks read as a sentence to fill in");

fn word_started(it: &QuizItem, carried: &CursorMind, state: &[String]) -> Result<CursorMind, String> {
    let bare = QuizItem { world: Vec::new(), ..it.clone() };
    let mut mind = cursor_started(&bare, carried, state)?;
    world_built(&mut mind, &it.world);
    Ok(mind)
}
because!(word_started, WordModes, "the mind a line of the reading one word at a time starts from: the cursor's start without the line's world, then the world built in the form a reading leaves, a thing inside its place");

fn world_expected(mind: &CursorMind, expect: &[String]) -> Vec<String> {
    expect.iter().flat_map(|s| world_form(mind, s)).collect()
}
because!(world_expected, WordModes, "the statements a line expects, read as the world holds them, so a lesson written in the older notation of relations is judged by the world");

fn known_names(items: &[QuizItem]) -> std::collections::HashSet<String> {
    items.iter().flat_map(|it| shape_names(&it.expect).into_iter().chain(shape_names(&it.world))).filter(|name| !function_word(name)).collect()
}
because!(known_names, WordModes, "the names the lessons state as things and values, the words of a closed class left out, whose text is taken off the twin rows, so a name never seen reads as they do while a word that says what to do by being itself keeps its text");

fn word_teach(args: &[String]) -> Result<(), String> {
    let out: String = opt(args, OUT, String::new())?;
    if out.is_empty() || !flag(args, QUIZ) {
        return Err(format!("dam {WORD_COMMAND} {WORD_TEACH} needs {QUIZ} and {OUT}"));
    }
    let items = quiz(args)?;
    let state = state_seeds(&opt::<String>(args, STATE_OPT, String::new())?)?;
    let known = known_names(&items);
    let mut text = String::new();
    let (mut learned, mut learned_all, mut held, mut held_all) = (0usize, 0usize, 0usize, 0usize);
    let mut missed = Vec::new();
    for last in [false, true] {
        text.clear();
        missed.clear();
        (learned, learned_all, held, held_all) = Default::default();
        let mut carried = CursorMind::default();
        for it in &items {
            let mut mind = word_started(it, &carried, &state)?;
            let mut failed = None;
            let mut written = String::new();
            let inputs = word_inputs(it);
            let last_sentence = word_sentences(it).len().saturating_sub(1);
            for (at, (words, answer)) in inputs.iter().enumerate() {
                let start = mind.clone();
                let steps = taught_words_answered(&mut mind, words, *answer);
                let trail: Vec<String> = words.iter().zip(&steps).map(|(w, taken)| format!("{w} {}", taken.iter().map(|s| format!("{}{}", s.class(), s.at.and_then(|d| dam::word::word_at(&mind.stack, d)).map(|p| format!("{POINTED_MARK}{p}")).unwrap_or_default())).collect::<Vec<_>>().join(SENTENCE_GAP))).collect();
                let reached = match answer {
                    Some(answer) => output_answers(&mind.output, true, answer),
                    None => at != last_sentence || world_holds(&mind.tree, &world_expected(&mind, &it.expect)),
                };
                if !reached {
                    let lacking: Vec<String> = world_expected(&mind, &it.expect).into_iter().filter(|s| !world_holds(&mind.tree, std::slice::from_ref(s))).collect();
                    failed = Some(format!("line {}: {}: not reached; lacking: {}; steps: {}; tree: {}; output: {}", it.line, words.join(SENTENCE_GAP), lacking.join("; "), trail.join(" | "), mind.tree.paths().join("; "), mind.output.join(SENTENCE_GAP)));
                    break;
                }
                let mut replay = start;
                for rows in word_rows(&mut replay, words, &steps, &|word| known.contains(word)) {
                    let mut records = [false, true].map(|blanked| StackRecord { line: it.line, test: it.test, blanked, texts: Vec::new(), events: Vec::new(), rows: Vec::new() });
                    for row in rows {
                        record_extended(&mut records, &words.join(SENTENCE_GAP), &row)?;
                    }
                    for record in records.iter().filter(|record| !record.rows.is_empty()) {
                        written.push_str(&json_text(record)?);
                        written.push_str(ROW_BREAK);
                    }
                }
            }
            carried = mind;
            if it.test { held_all += 1 } else { learned_all += 1 }
            if let Some(failed) = failed {
                if last {
                    println!("{failed}");
                }
                missed.push(it.line);
                continue;
            }
            text.push_str(&written);
            if it.test { held += 1 } else { learned += 1 }
        }
    }
    write_text(&out, &text)?;
    println!("taught: learned lines {learned} of {learned_all}, held out lines {held} of {held_all}; the teacher does not reach lines {missed:?}");
    Ok(())
}
because!(
    word_teach,
    WordModes,
    "every line of a quiz taught one word at a time, twice over, the first pass only so the teacher keeps for each shape of number question the walk that answers most of them and the second pass teaches that one, a then line on the tree the line above it left and any other line on the tree its seeds make: each sentence of its text and then each question read by the deterministic teacher, the text checked against the shape it expects and every question against its answer, the steps of a line the teacher does not reach printed with its tree and the line left out of the rows; the rows of the lines reached written, one record per word since the stack is emptied at each, twice when the stack holds a name the quiz states as a thing, and the counts printed"
);

pub struct InputRead {
    pub words: Vec<String>,
    pub answer: Option<String>,
    pub right: bool,
    pub read: WordRead,
}
because!(InputRead, WordModes, "one input of a line as a network read it: its words, the answer it must say when it has one, whether it passed, and the reading with its mind, its steps and whether every word continued");

pub fn line_read(it: &QuizItem, carried: &CursorMind, state: &[String], network: (&[Stacked], &[WordStep]), steps: usize) -> Result<(CursorMind, Vec<InputRead>), String> {
    let started = word_started(it, carried, state)?;
    let last_sentence = word_sentences(it).len().saturating_sub(1);
    let mut inputs: Vec<InputRead> = Vec::new();
    for (at, (words, answer)) in word_inputs(it).into_iter().enumerate() {
        let read = read_by_words(inputs.last().map_or(&started, |input| &input.read.mind), &words, network, steps);
        let right = read.ended
            && match answer {
                Some(answer) => output_answers(&read.mind.output, read.ended, answer),
                None => at != last_sentence || world_holds(&read.mind.tree, &world_expected(&read.mind, &it.expect)),
            };
        inputs.push(InputRead { words, answer: answer.map(str::to_string), right, read });
    }
    let mind = inputs.last().map_or(started, |input| input.read.mind.clone());
    Ok((mind, inputs))
}
because!(
    line_read,
    WordModes,
    "one line of a quiz read by a network one word at a time, from the mind the line starts from, each sentence of its text and then each question on the same tree: an input passes when every word continued within the step limit and the tree holds the shape or the output says the answer; the inputs as read and the mind the line leaves, so the report and the model tool grade a line the same way"
);

fn word_report(args: &[String], console: &Console) -> Result<(), String> {
    let network: String = opt(args, NETWORK, String::new())?;
    if network.is_empty() || !flag(args, QUIZ) {
        return Err(format!("dam {WORD_COMMAND} {WORD_REPORT} needs {QUIZ} and {NETWORK}"));
    }
    let mut nets = Vec::new();
    let mut classes: Vec<WordStep> = Vec::new();
    for path in network.split(NETWORKS_GAP) {
        let (voters, of_net) = loaded_voters(path, WordStep::of_class)?;
        if !classes.is_empty() && classes != of_net {
            return Err(format!("{path} was trained on other classes than the networks before it"));
        }
        classes = of_net;
        nets.extend(voters);
    }
    let state = state_seeds(&opt::<String>(args, STATE_OPT, String::new())?)?;
    let mut results: Vec<(bool, bool, bool)> = Vec::new();
    let mut carried = CursorMind::default();
    for it in quiz(args)? {
        let (mind, inputs) = line_read(&it, &carried, &state, (&nets, &classes), console.shaping.steps)?;
        for InputRead { words, answer, right, read } in &inputs {
            if answer.is_some() {
                results.push((it.test, true, *right));
            }
            if !right || flag(args, ALL) {
                println!("{} line {}: {}", if *right { "passed" } else { "failed" }, it.line, words.join(SENTENCE_GAP));
                match answer {
                    Some(answer) => println!("  expected output: {answer}; output: {}; ended: {}", read.mind.output.join(", "), read.ended),
                    None => println!("  expected shape: {}", it.expect.join("; ")),
                }
                println!("  tree: {}", read.mind.tree.paths().join("; "));
                println!("  steps: {}", read.trail.iter().map(|(w, taken)| format!("{w} {}", taken.join(SENTENCE_GAP))).collect::<Vec<_>>().join(" | "));
            }
        }
        results.push((it.test, false, inputs.iter().all(|input| input.right)));
        carried = mind;
    }
    let rate = |test: bool, question: bool| {
        let all: Vec<bool> = results.iter().filter(|r| r.0 == test && r.1 == question).map(|r| r.2).collect();
        let got = all.iter().filter(|&&r| r).count();
        format!("{got} of {} ({})", all.len(), share(got as f32 / all.len().max(1) as f32))
    };
    println!("word: learned lines {}, questions {}; held out lines {}, questions {}", rate(false, false), rate(false, true), rate(true, false), rate(true, true));
    Ok(())
}
because!(
    word_report,
    WordModes,
    "every line of a quiz read by a network one word at a time, each sentence of its text and then each question on the same tree: an input passes when every word continued within the step limit and the tree holds the shape or the output says the answer, a line when all its inputs pass; the failed inputs, or every input when all are asked for, printed with the goal, the output, the tree and the steps, and the learned and held out lines and questions tallied"
);
