use patterns::{because, source};

#[derive(Clone, Debug)]
pub struct QuizQuestion {
    pub text: String,
    pub words: Vec<String>,
    pub answer: String,
}
because!(QuizQuestion, "a question asked of the permanent space after a text, with the answer it should get, normalized so two spellings \
     of one answer compare equal");

#[derive(Clone, Debug)]
pub struct QuizItem {
    pub text: String,
    pub words: Vec<String>,
    pub expect: Vec<String>,
    pub questions: Vec<QuizQuestion>,
    pub test: bool,
    pub turn: Option<String>,
    pub next: Vec<Vec<String>>,
    pub line: usize,
    pub seed: Vec<String>,
    pub part: usize,
    pub continues: bool,
    pub world: Vec<String>,
}
because!(
    QuizItem,
    "one line the reader learns from or are scored against, with the line of the text it was written on so a lesson can be chosen by its \
     lines, and the facts files it is read on top of, in the order its quiz file names them, and which part of the quiz it stands in, a \
     part ending at each line that names facts or at the end of a quiz file, so lines of one concept are told apart from another's: a \
     text, the statements its space should hold, its questions, whether it is held out, for a turn the answer the text's own open tail \
     should draw, and for a line asking what comes next the words that may follow, each alternative as its words; whether the line goes on \
     from the stack and the space the line above it left; the statements of a world the tree holds before the line is read, with nothing \
     of them on the stack; its statements are scored when any are expected or when nothing at all is asked, a turn's tail counting as \
     asked, since a turn that tells nothing and is answered with nothing had been taking half its score for the empty space it held"
);

impl QuizItem {
    pub fn scores_statements(&self) -> bool {
        !self.expect.is_empty() || (self.questions.is_empty() && self.turn.is_none())
    }
}

pub struct QuizFormat;
source!(
    QuizFormat,
    "the text format of the quiz: a line is a text, an arrow, statements separated by semicolons, then a bar and questions each with its \
     answer after an equals sign; a hash starts a comment; a test prefix holds a line out; a next prefix asks what follows a text"
);

const COMMENT: &str = "#";
because!(COMMENT, QuizFormat, "the mark that starts a comment line");

pub const TURN: &str = "turn:";
because!(
    TURN,
    QuizFormat,
    "the prefix of a line typed as a person types a chat turn, without its marks: after the arrow the statements the settled part should \
     hold, and after the bar the one answer the open tail, read as a question, should draw; which word ends the statement and starts the \
     question is for the reader to learn"
);

pub const THEN_LINE: &str = "then:";
because!(
    THEN_LINE,
    QuizFormat,
    "the prefix of a line read on the stack and the permanent space the line above it left, so a later sentence can change what an earlier \
     line said and a question can ask about both"
);

pub const NEXT: &str = "next:";
because!(
    NEXT,
    QuizFormat,
    "the prefix of a line that asks what comes next: before the arrow a text read onto the stack with no question, after it the words that \
     should follow, alternatives parted by commas; the network's prediction of the input, rolled forward one item at a time, must match \
     one of them, and a network that predicts no input has nothing to say about the line"
);

pub const SENTENCE_END: &str = ".";
because!(
    SENTENCE_END,
    QuizFormat,
    "the mark every text of the quiz ends a sentence with, so it is the word the reader learned to settle on and the one a turn typed \
     without it is closed with"
);

pub const QUESTION_END: &str = "?";
because!(
    QUESTION_END,
    QuizFormat,
    "the mark every question of the quiz ends with, so a question found inside a turn is closed with it and reads as the quiz's questions \
     do"
);

pub const BRACE_OPEN: char = '{';
because!(BRACE_OPEN, QuizFormat, "what opens a braced word: from it to the brace that closes it the text is one word, whatever marks it \
     holds");

pub const BRACE_CLOSE: char = '}';
because!(BRACE_CLOSE, QuizFormat, "what closes a braced word, the last of them when braces are nested, as in the escaped end word");

pub const SEED_PREFIX: &str = "seed:";
because!(
    SEED_PREFIX,
    QuizFormat,
    "the prefix of a line naming a facts file, without its ending, that every quiz line after it in its quiz file is read on top of, since \
     a line can ask what the facts know, as the day after friday or how many minutes an hour is, while every other line starts from \
     nothing; several such lines stack, each line after them read on the facts of all of them in order, and the prefix alone ends them \
     all, which is what the quiz's files are joined with, so no file's facts reach the next file"
);

pub const WORLD: &str = "world:";
because!(
    WORLD,
    QuizFormat,
    "the prefix of a line that states a world instead of telling a text: the statements before the bar are in the tree before the line \
     starts and the stack holds none of them, as a person who looked around earlier and no longer holds the telling in mind, so each \
     question after the bar is solved by searching the tree; the user's puzzles, as a ball in one of three boxes, which a network learns \
     like a game"
);

const TEST: &str = "test:";
because!(TEST, QuizFormat, "the prefix that holds a line out: reported beside the learned lines, never counted in their score");

const ARROW: &str = "=>";
because!(ARROW, QuizFormat, "what separates a text from the statements its space should hold");

const BAR: &str = "|";
because!(BAR, QuizFormat, "what separates the statements from the questions");

const SEMICOLON: &str = ";";
because!(SEMICOLON, QuizFormat, "what separates one statement or one question from the next");

const EQUALS: &str = "=";
because!(EQUALS, QuizFormat, "what separates a question from its answer");

const COLON: &str = ":";
const PROPERTY_WORDS: [&str; 5] = ["quantity", "value", "time", "the", "a"];
because!(PROPERTY_WORDS, QuizFormat, "the properties written inside a name with a colon, an object with its quantity, a value with its \
     time, the number tag with its value and a thing with the article it was said with, cat:the(true), which a statement keeps tight \
     against the name, since the colon that opens a property list is spaced and this one is part of the name");
because!(COLON, QuizFormat, "what separates a group from a fact about it in a statement");

const COMMA: &str = ",";
because!(COMMA, QuizFormat, "what separates the names in an answer as it is written");

pub const ANSWER_SEPARATOR: &str = ", ";
because!(ANSWER_SEPARATOR, QuizFormat, "what separates the names in an answer's normal form, the comma with one space, so two spellings of \
     one answer compare equal");

use crate::cursor::PATH_MARK;

pub const PROPERTY_OPEN: char = '(';
because!(PROPERTY_OPEN, QuizFormat, "what opens the properties of a node after its name in a statement, each property a relation and a \
     value, so a test reads as objects with properties");

pub const PROPERTY_CLOSE: char = ')';
because!(PROPERTY_CLOSE, QuizFormat, "what closes the properties of a node");

const PROPERTY_GAP: char = ',';
because!(PROPERTY_GAP, QuizFormat, "what separates one property of a node from the next inside its parentheses");

pub const WORD_GAP: char = ' ';
because!(WORD_GAP, QuizFormat, "what separates the words of a statement");

fn split_top(s: &str, at: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for c in s.chars() {
        match c {
            PROPERTY_OPEN => depth += 1,
            PROPERTY_CLOSE => depth = depth.saturating_sub(1),
            _ => {}
        }
        if c == at && depth == 0 {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    parts.push(current);
    parts
}
because!(split_top, QuizFormat, "a statement split at a mark outside any parentheses, so a property list inside them stays whole");

fn split_path(s: &str) -> Vec<String> {
    let arrow = PATH_MARK.trim();
    let mut names = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for word in split_top(s, WORD_GAP).into_iter().filter(|w| !w.is_empty()) {
        if word == arrow {
            names.push(current.join(&WORD_GAP.to_string()));
            current.clear();
        } else {
            current.push(word);
        }
    }
    names.push(current.join(&WORD_GAP.to_string()));
    names
}
because!(split_path, QuizFormat, "the nodes of a path split at the arrows outside any parentheses, so a value that carries a path of its \
     own inside them stays whole");

fn expand_node(prefix: &[String], node: &str, out: &mut Vec<String>) {
    let node = node.trim();
    let property_word = |open: usize| PROPERTY_WORDS.iter().any(|w| node[..open].ends_with(&format!("{COLON}{w}")));
    let (name, properties) = match node.find(PROPERTY_OPEN) {
        Some(open) if !property_word(open) && node.ends_with(PROPERTY_CLOSE) && node[open + 1..].contains(COLON) => (node[..open].trim(), Some(&node[open + 1..node.len() - 1])),
        _ => (node, None),
    };
    let mut path: Vec<String> = prefix.to_vec();
    path.push(name.to_string());
    let timed = properties.is_some_and(|p| split_top(p, PROPERTY_GAP).iter().any(|property| property.split_once(COLON).is_some_and(|(relation, _)| crate::cursor::TIME_RELATION.trim_matches(|c| c == BRACE_OPEN || c == BRACE_CLOSE) == relation.trim() || crate::cursor::TIME_RELATION == relation.trim())));
    if path.len() > 1 && !timed {
        out.push(path.join(PATH_MARK));
    }
    let Some(properties) = properties else {
        return;
    };
    for property in split_top(properties, PROPERTY_GAP) {
        let Some((relation, value)) = property.split_once(COLON) else {
            continue;
        };
        let relation = relation.trim();
        let braced = if relation.starts_with(BRACE_OPEN) || relation == WORTH_FORM.trim() { relation.to_string() } else { format!("{BRACE_OPEN}{relation}{BRACE_CLOSE}") };
        let mut deeper = path.clone();
        deeper.push(braced);
        let along = split_path(value);
        for (at, node) in along.iter().enumerate() {
            if at + 1 < along.len() {
                deeper.push(node.trim().to_string());
            } else {
                expand_node(&deeper, node, out);
            }
        }
    }
}
because!(expand_node, QuizFormat, "a node whose parentheses hold no colon, or whose parenthesis follows a property word, is left whole, \
     since apple with its quantity in the user's form is one name the cursor reads, and two amounts joined by a relation hold a colon \
     between the parentheses while neither is a list of properties; the paths a node with properties stands for: the path down to the \
     node, unless the node carries a time, since a bare path states what is so now and a value with a time is not, then for every property \
     the path on through the relation, bare names taken as braced relations, to the value, which may be a path and may carry properties of \
     its own");

pub fn expand_properties(statement: &str) -> Vec<String> {
    if !statement.contains(PROPERTY_OPEN) {
        return vec![statement.to_string()];
    }
    let nots = statement.trim().split(WORD_GAP).take_while(|w| *w == NOT_PREFIX.trim()).count();
    let body = statement.trim().splitn(nots + 1, WORD_GAP).last().unwrap_or("");
    let names: Vec<String> = split_path(body).into_iter().map(|n| n.trim().to_string()).collect();
    let mut out = Vec::new();
    let mut prefix: Vec<String> = Vec::new();
    for (at, node) in names.iter().enumerate() {
        if at + 1 < names.len() {
            prefix.push(node.clone());
            continue;
        }
        expand_node(&prefix, node, &mut out);
    }
    let prefixed: String = std::iter::repeat_n(NOT_PREFIX, nots).collect();
    out.into_iter().map(|s| format!("{prefixed}{s}")).collect()
}
because!(expand_properties, QuizFormat, "every path a statement stands for: itself when it holds no parentheses, else the paths of its \
     last node with the properties in its parentheses, each keeping the statement's nots, so the checker and the teacher see paths while \
     the test reads as an object with properties");

pub fn norm_statement(s: &str) -> String {
    let spaced = format!(" {COLON} ");
    let s = s.trim().to_lowercase().replace(COLON, &spaced);
    let spaced_out = s.split_whitespace().collect::<Vec<_>>().join(" ").replace(&format!(" {COLON}"), COLON);
    PROPERTY_WORDS.iter().fold(spaced_out, |text, word| text.replace(&format!("{COLON} {word}{PROPERTY_OPEN}"), &format!("{COLON}{word}{PROPERTY_OPEN}")))
}
because!(norm_statement, "one statement in a fixed form, lowercase with single spaces and the colon tight to its group, so a statement \
     written by hand and one the reader made compare equal");

pub fn norm_answer(s: &str) -> String {
    let mut parts: Vec<String> = s
        .split(COMMA)
        .map(|p| p.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase())
        .filter(|p| !p.is_empty())
        .collect();
    parts.sort();
    parts.join(ANSWER_SEPARATOR)
}
because!(norm_answer, "an answer in a fixed form, its names lowercased and sorted, so the order the reader found them in does not matter");

pub fn parse_quiz(src: &str, mut split: impl FnMut(&str) -> Vec<String>) -> Result<Vec<QuizItem>, String> {
    let mut items: Vec<QuizItem> = Vec::new();
    let mut seed: Vec<String> = Vec::new();
    let mut part = 0;
    for (at, line) in src.lines().enumerate() {
        let n = at + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with(COMMENT) {
            continue;
        }
        if let Some(rest) = line.strip_prefix(SEED_PREFIX) {
            match rest.trim() {
                "" => seed.clear(),
                name if !seed.iter().any(|s| s == name) => seed.push(name.to_string()),
                _ => {}
            }
            part += 1;
            continue;
        }
        let (test, line) = match line.strip_prefix(TEST) {
            Some(rest) => (true, rest.trim()),
            None => (false, line),
        };
        let (continues, line) = match line.strip_prefix(THEN_LINE) {
            Some(rest) => (true, rest.trim()),
            None => (false, line),
        };
        let (is_turn, line) = match line.strip_prefix(TURN) {
            Some(rest) => (true, rest),
            None => (false, line),
        };
        let (is_next, line) = match line.strip_prefix(NEXT) {
            Some(rest) => (true, rest),
            None => (false, line),
        };
        if let Some(rest) = line.strip_prefix(WORLD) {
            let (world, asked) = rest.split_once(BAR).unwrap_or((rest, ""));
            let world: Vec<String> = world.split(SEMICOLON).flat_map(expand_properties).map(|s| norm_statement(&s)).filter(|s| !s.is_empty()).collect();
            let questions = questions_of(asked, n, &mut split)?;
            items.push(QuizItem { words: Vec::new(), text: String::new(), expect: Vec::new(), questions, test, turn: None, next: Vec::new(), line: n, seed: seed.clone(), part, continues, world });
            continue;
        }
        let (text, rest) = line.split_once(ARROW).ok_or(format!("line {n}: no {ARROW}"))?;
        if is_next {
            let next: Vec<Vec<String>> = rest.split(COMMA).map(|alt| split(alt.trim())).filter(|alt| !alt.is_empty()).collect();
            if next.is_empty() {
                return Err(format!("line {n}: {NEXT} needs the words that should follow after {ARROW}"));
            }
            let text = text.trim().to_string();
            items.push(QuizItem { words: split(&text), text, expect: Vec::new(), questions: Vec::new(), test, turn: None, next, line: n, seed: seed.clone(), part, continues, world: Vec::new() });
            continue;
        }
        let (expect, asked) = rest.split_once(BAR).unwrap_or((rest, ""));
        let mut expect: Vec<String> = expect.split(SEMICOLON).flat_map(expand_properties).map(|s| norm_statement(&s)).filter(|s| !s.is_empty()).collect();
        let mut kept = std::collections::HashSet::new();
        expect.retain(|statement| kept.insert(statement.clone()));
        let (questions, turn) = if is_turn { (Vec::new(), Some(norm_answer(asked))) } else { (questions_of(asked, n, &mut split)?, None) };
        let text = text.trim().to_string();
        items.push(QuizItem { words: split(&text), text, expect, questions, test, turn, next: Vec::new(), line: n, seed: seed.clone(), part, continues, world: Vec::new() });
    }
    if items.iter().all(|it| it.test) {
        return Err(format!("the quiz has no lines to learn from (lines without {TEST})"));
    }
    Ok(items)
}
because!(
    parse_quiz,
    "the quiz text read into its lines, each statement and answer normalized, a line's statements kept in the order it lists them, which \
     is the order its text says them, a repeated one once, its seeds named, a world line read as its world and its questions with no text, \
     and a quiz with nothing to learn from refused"
);

fn questions_of(asked: &str, n: usize, split: &mut impl FnMut(&str) -> Vec<String>) -> Result<Vec<QuizQuestion>, String> {
    let mut questions = Vec::new();
    for qa in asked.split(SEMICOLON).map(str::trim).filter(|s| !s.is_empty()) {
        let (q, a) = qa.split_once(EQUALS).ok_or(format!("line {n}: question {qa:?} has no {EQUALS} answer"))?;
        let q = q.trim().to_string();
        questions.push(QuizQuestion { words: split(&q), text: q, answer: norm_answer(a) });
    }
    Ok(questions)
}
because!(questions_of, QuizFormat, "the questions after a line's bar, each with its answer normalized, refused at a question with no \
     answer");

pub struct AnswerWords;
source!(AnswerWords, "the words a question of the quiz is answered with when no thing of the tree is the answer: yes and no for a check, \
     nothing when the tree holds nothing the question asks for");

pub const NOTHING: &str = "{nothing}";
because!(NOTHING, AnswerWords, "the answer when the tree holds nothing the question asks for, so a question never fails, it says it found \
     nothing");

pub const YES: &str = "yes";
because!(YES, AnswerWords, "the answer of a check the tree holds");

pub const NO: &str = "no";
because!(NO, AnswerWords, "the answer of a check the tree does not hold, which is an answer and not a failure");

pub const NOT_PREFIX: &str = "not ";
because!(NOT_PREFIX, QuizFormat, "what starts a token of an answer the output must not hold, as not ann, and a statement the tree must not \
     hold, since extra content fails only when a line says so");

pub const CHAIN_MARK: &str = "->";

pub const APOSTROPHE: &str = "'";
because!(APOSTROPHE, QuizFormat, "the mark a text glues to a word for a possessive or a contraction, as in tom's and o'clock; a word glued \
     to it is no thing word, since the s of a possessive and the o of the clock name nothing, whatever the seeds hold under those letters");

because!(CHAIN_MARK, QuizFormat, "what joins the tokens of a chain an answer expects in the output, each saying something about the one \
     before it, as color->red");

pub struct StatementForms;
source!(StatementForms, "the forms of the three relations every quiz statement may use without naming them anywhere else: a thing in a \
     container, a member of a group, and an owner of a thing, and the form of a name worth a number");

pub const IN_FORM: &str = " {in} ";

pub const PLACE_RELATIONS: [&str; 13] = ["{in}", "{inside}", "{on}", "{under}", "{at}", "{behind}", "{beside}", "{next}", "{above}", "{below}", "{near}", "{between}", "{over}"];
because!(PLACE_RELATIONS, QuizFormat, "the relations that put a thing at a place, each the word the sentence said, in, inside, on, under \
     and the rest, as the user asked that a place word is written as said; a statement with one of them moves a thing rather than giving \
     it away");
because!(IN_FORM, StatementForms, "the form of a thing inside another: the thing, then its container");

pub const IS_FORM: &str = " {is} ";
because!(IS_FORM, StatementForms, "the form of a thing that joined a group: the member, then the group it joined");

pub const OWNS_FORM: &str = " {has} ";
because!(OWNS_FORM, StatementForms, "the form of having: the one who has first, then the thing had, the word the text says, since a person \
     who has a car may not own it");

pub const WORTH_FORM: &str = " = ";
because!(WORTH_FORM, StatementForms, "the form of a name worth a number: the name, then the number");

pub const ASKING_WORDS: [&str; 20] = ["who", "what", "where", "when", "why", "how", "which", "whose", "is", "are", "was", "were", "do", "does", "did", "can", "could", "will", "has", "have"];
because!(ASKING_WORDS, QuizFormat, "the words a turn starts with when it asks, the question words and the verbs a yes or no question puts \
     first, so a turn typed without its mark is closed as a question when it starts with one");

pub const ARITHMETIC_MARKS: [&str; 7] = ["+", "-", "*", "/", "=", "(", ")"];
because!(ARITHMETIC_MARKS, QuizFormat, "the marks a bare sum is typed with between its numbers, the four signs, the equals mark and the \
     brackets, so a turn of numbers and these marks alone asks for its result");

pub fn punctuated(mut words: Vec<String>) -> Vec<String> {
    let closed = words.last().is_some_and(|w| w == SENTENCE_END || w == QUESTION_END);
    if !closed && !words.is_empty() {
        let bare_sum = words.iter().all(|w| crate::words::number_of(w).is_some() || ARITHMETIC_MARKS.contains(&w.as_str())) && words.iter().any(|w| crate::words::number_of(w).is_some());
        let asks = bare_sum || words.first().is_some_and(|w| ASKING_WORDS.contains(&w.as_str()));
        words.push(if asks { QUESTION_END } else { SENTENCE_END }.to_string());
    }
    words
}
because!(punctuated, QuizFormat, "the words of a typed turn closed with the mark the lessons end a sentence with when the turn has none: \
     the question mark when it starts with an asking word or is a bare sum of numbers and arithmetic marks, since eight times four typed \
     alone asks for thirty-two, and the full stop otherwise, since the lessons taught statements that end with a full stop and questions \
     that end with a question mark, and a turn without its mark reads as neither");

pub fn sentences(words: Vec<String>) -> Vec<Vec<String>> {
    let mut pieces: Vec<Vec<String>> = vec![Vec::new()];
    for word in words {
        let closes = word == SENTENCE_END || word == QUESTION_END;
        pieces.last_mut().expect("a piece is always open").push(word);
        if closes {
            pieces.push(Vec::new());
        }
    }
    pieces.into_iter().filter(|piece| !piece.is_empty()).collect()
}
because!(sentences, QuizFormat, "the words of a typed message cut into its sentences, each closed by the full stop or the question mark it \
     ends with and the last kept open when it has none, so a message of a story and two questions is read as the lessons teach it, one \
     input per sentence, each with its own actions, and every question gets its own answer");
