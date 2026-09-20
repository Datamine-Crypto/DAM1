use crate::quiz::{PROPERTY_CLOSE, PROPERTY_OPEN};
use crate::quiz::{BRACE_CLOSE, BRACE_OPEN, SENTENCE_END};
use patterns::{because, source};

pub struct WordShapes;
source!(
    WordShapes,
    "how the quiz and the console turn text into words: split on spaces with each punctuation mark its own word, every plain word \
     lowercased, so I and i are one word as the seeds write it, a run in braces one word, a sentence mark between two digits kept as a \
     decimal point, and a word taken as a key lowercased and trimmed"
);

pub fn norm(word: &str) -> String {
    word.trim().to_lowercase()
}
because!(norm, WordShapes, "a word as a key: trimmed and lowercased, so a word is the same whatever the text's spelling of case and \
     spacing");

const PLUS: &str = "+";
because!(PLUS, WordShapes, "the sign a number may be written with in front of it, which is no part of its value");

const MINUS: &str = "-";
because!(MINUS, WordShapes, "the sign in front of a number below nothing");

pub const INFINITY_WORD: &str = "infinity";
because!(INFINITY_WORD, WordShapes, "the word for the number beyond every number, read as that number and how a number divided by nothing \
     is written, the word the quiz answers with");

const TAG_OPEN: &str = "{";
because!(TAG_OPEN, WordShapes, "what opens a tag, a braced token whose last word may be a number, as a number or a quantity written as a \
     concept");
const TAG_CLOSE: &str = "}";
because!(TAG_CLOSE, WordShapes, "the brace after the last word of a braced token, so the number read is the whole last word");
const TAG_GAP: char = ' ';
because!(TAG_GAP, WordShapes, "what parts a tag's words, so its last word is read as its number");

pub fn number_of(word: &str) -> Option<f32> {
    if word == INFINITY_WORD {
        return Some(f32::INFINITY);
    }
    if let Some(inner) = word.strip_prefix(TAG_OPEN).and_then(|rest| rest.strip_suffix(TAG_CLOSE)) {
        if let Some((_, value)) = inner.split_once(PROPERTY_OPEN) {
            return value.strip_suffix(PROPERTY_CLOSE).and_then(number_of);
        }
        return inner.rsplit_once(TAG_GAP).and_then(|(_, last)| number_of(last));
    }
    let unsigned = word.strip_prefix(MINUS).or_else(|| word.strip_prefix(PLUS)).unwrap_or(word);
    let (whole, part) = unsigned.split_once(SENTENCE_END).unwrap_or((unsigned, ""));
    let digits = |s: &str| s.chars().all(|c| c.is_ascii_digit());
    let written = !(whole.is_empty() && part.is_empty()) && digits(whole) && digits(part);
    written.then(|| word.parse::<f32>().ok()).flatten()
}
because!(number_of, WordShapes, "the value of a word written as a number, its digits with an optional sign and one decimal point, the word \
     infinity as the number beyond every number, and none for any other word");

pub fn simple_words(text: &str) -> Vec<String> {
    let mut s = String::new();
    let mut braced = 0usize;
    let chars: Vec<char> = text.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        let between_digits = i > 0 && i + 1 < chars.len() && chars[i - 1].is_ascii_digit() && chars[i + 1].is_ascii_digit() && c.to_string() == SENTENCE_END;
        let signs_number = c.to_string() == MINUS && chars.get(i + 1).is_some_and(char::is_ascii_digit) && (i == 0 || !chars[i - 1].is_alphanumeric());
        if c == BRACE_OPEN {
            if braced == 0 {
                s.push_str(" ");
            }
            braced += 1;
            s.push(c);
        } else if c == BRACE_CLOSE && braced > 0 {
            braced -= 1;
            s.push(c);
            if braced == 0 {
                s.push_str(" ");
            }
        } else if braced == 0 && c.is_ascii_punctuation() && !between_digits && !signs_number {
            s.push_str(" ");
            s.push(c);
            s.push_str(" ");
        } else {
            s.push(c);
        }
    }
    s.split_whitespace().map(|w| if w.starts_with(BRACE_OPEN) { w.to_string() } else { w.to_lowercase() }).collect()
}
because!(
    simple_words,
    WordShapes,
    "a text split on spaces with each punctuation mark its own word, except that a run in braces is one word with its braces, so the end \
     word of a message stays one word, a sentence mark between two digits is the point of a decimal and stays in its number, and a minus \
     right before a digit with no letter or digit right before it is the sign of that number and stays in it, so a number below nothing is \
     one word and the minus between two numbers, which has a space after it, stays the sign of taking away"
);

pub struct WordForm;
source!(
    WordForm,
    "the test lines a network over the stack lost when it saw only the literal word: every unseen number, where the rows had read any \
     number by its form; a number is one kind of word whatever digits it has"
);

const NUMBER_FORM: &str = "number";
because!(NUMBER_FORM, WordForm, "the form of a word made of digits");

const PLAIN_FORM: &str = "plain";
because!(PLAIN_FORM, WordForm, "the form of every other word");

pub fn form_of(plain: &str) -> &'static str {
    if number_of(plain).is_some() { NUMBER_FORM } else { PLAIN_FORM }
}
because!(form_of, WordForm, "the kind of a normalised word by its form alone: a number or any other word");
