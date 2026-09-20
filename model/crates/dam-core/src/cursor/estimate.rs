use super::moves::CursorMove;
use super::CursorTree;
use crate::quiz::{ANSWER_SEPARATOR, CHAIN_MARK, NOT_PREFIX, NOTHING};
use patterns::{because, source};

pub struct QuestionWords;
source!(
    QuestionWords,
    "the words of a quiz question that tell the teacher what kind of answer it wants: more, fewer and their superlatives ask for a \
     comparison of counts, than for a difference, alphabet and letter for a choice by where letters stand, the arithmetic signs for their \
     operations, and the choices a question offers stand at its end"
);

pub(crate) const COMPARISON_WORDS: [&str; 6] = ["more", "fewer", "less", "most", "fewest", "least"];
because!(COMPARISON_WORDS, QuestionWords, "the words a question says when it asks which thing has the larger or the smaller count, so such \
     a question is answered only after the counts were compared");

pub(crate) const PLURAL_ENDING: &str = "s";
because!(PLURAL_ENDING, QuestionWords, "the letter a plural ends with, so the last word of a yes or no question counts as a thing it asks \
     about only when it is a plural, as apples, and never when it is an adjective, as right");

pub(crate) fn singular_of(word: &str) -> Option<&str> {
    word.strip_suffix(PLURAL_ENDING).filter(|one| one.chars().count() > 1)
}
because!(singular_of, QuestionWords, "the word a plural said stands for, the word less its plural ending, and none when what is left is \
     one letter, since is and as are no plurals of any thing, while the seeds know a roman numeral called i that holds the number one, and \
     a plan that took that one as a number the question named added its way to the biggest of three numbers by chance");

pub(crate) const ZERO_WORDS: [&str; 4] = ["multiple", "factor", "divisible", "divides"];
because!(ZERO_WORDS, QuestionWords, "the words a question says when it asks whether one number divides another with nothing left, so the \
     check \n of the number for zero is offered only to such a question and never turns a difference or a product of nothing \n into a yes \
     by chance");

pub(crate) const RANGE_FROM: &str = "from";
because!(RANGE_FROM, QuestionWords, "the word a question says when it counts the numbers of a range, from one number to another, so the \
     count between is offered only then, its token the word that says which numbers count and its value the number the range ends at, once \
     the number it starts at is set");
pub(crate) const RANGE_TO: &str = "to";
because!(RANGE_TO, QuestionWords, "the word that ends a range a question counts the numbers of, from one to ten, so such a question is not \
     planned as a chain, since ten over two lands on the even numbers by chance while the count between reads them, and the hours from \
     nine to four are walked round the clock and never taken apart");

pub(crate) const COUNTED_WORD: &str = "numbers";
because!(COUNTED_WORD, QuestionWords, "the word a question says when every number of a range counts, so the count between takes it as its \
     kind beside even and odd");

pub(crate) const WHOLE_WORDS: [&str; 3] = ["full", "whole", "buy"];
because!(WHOLE_WORDS, QuestionWords, "the words a question says when only whole ones count, as full bags or what can be bought, so the \
     step that keeps the whole number of a division is offered only to such a question");

pub(crate) const WHICH: &str = "which";
because!(WHICH, QuestionWords, "the question word that asks for one of several, so a comparative or a superlative after it compares them");

pub(crate) const ROUND_WORDS: [&str; 3] = ["round", "rounded", "nearest"];
because!(ROUND_WORDS, QuestionWords, "the words a question says when it asks for a number rounded, so the rounding step is offered only to \
     such a \n question and never turns a comparison into a yes by chance");

pub(crate) const THAN: &str = "than";

pub(crate) const COMPARATIVE_ENDINGS: [&str; 2] = ["er", "est"];
because!(COMPARATIVE_ENDINGS, QuestionWords, "the endings of a comparative and a superlative, as colder and biggest, by which a question \
     that asks which one or says than is read as one that compares, since the words that compare are too many to list");

pub(crate) const COMPARE_SIGNS: [&str; 2] = ["<", ">"];
because!(COMPARE_SIGNS, QuestionWords, "the signs a question writes for less than and greater than");

pub(crate) const ORDER_WORDS: [&str; 2] = ["first", "last"];
because!(ORDER_WORDS, QuestionWords, "the words a question says when it asks which comes first or last in an order, as the alphabet, which \
     is answered by keeping the smaller or the larger place");

pub(crate) const RANGE_WORDS: [&str; 1] = ["range"];
because!(RANGE_WORDS, QuestionWords, "the word a question says when it asks for the range of the numbers it says, the largest less the \
     smallest, which no chain of an accumulator over each number once works out, so no plan is made and a chain that lands on the range by \
     chance is refused");

pub(crate) fn asks_comparison<'a>(words: impl Iterator<Item = &'a str> + Clone) -> bool {
    let says = |word: &str| words.clone().any(|w| w == word);
    let listed = |w: &str| COMPARISON_WORDS.contains(&w) || COMPARE_SIGNS.contains(&w) || ORDER_WORDS.contains(&w) || RANGE_WORDS.contains(&w);
    let choosing = says(WHICH) || says(THAN);
    words.clone().any(listed) || (choosing && words.clone().any(|w| COMPARATIVE_ENDINGS.iter().any(|ending| w.ends_with(ending) && w.len() > ending.len())))
}
because!(asks_comparison, QuestionWords, "whether a question compares numbers by its words: it says more or fewer, first or last, the \
     range, the sign of less or greater than, or a comparative or a superlative while it asks which one or says than, so the larger or the \
     smaller of two numbers is kept only for such a question, since twelve o'clock in three hours reached three as the smaller of twelve \
     and three, and a question that never compares is answered by chance so");
because!(THAN, QuestionWords, "the word that makes a question of more or fewer a question of the difference between two counts, which is \
     worked out and never read off one of them");

pub(crate) const SIGN_MOVES: [(&str, CursorMove); 5] = [("+", CursorMove::AddNumber), ("-", CursorMove::SubtractNumber), ("*", CursorMove::MultiplyNumber), ("/", CursorMove::DivideNumber), ("^", CursorMove::PowerNumber)];
because!(SIGN_MOVES, QuestionWords, "the arithmetic signs and the step each one writes: a sign means what it writes, so an input that says \
     one is worked by that step and no other, while a word such as plus names no step and is left to the network to learn");

#[allow(dead_code)]
pub struct SpelledNumbers;
patterns::source!(SpelledNumbers, "the English words for the numbers from none to twenty, which the lessons write in place of digits, four \
     hens and three ducks");

pub const SPELLED_NUMBERS: [&str; 21] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty"];
because!(SPELLED_NUMBERS, SpelledNumbers, "the numbers a lesson may write as words, each at the place of the number it names, so an answer \
     of four and an output of the digit agree");

fn answer_value(text: &str) -> Option<f32> {
    crate::words::number_of(text).or_else(|| SPELLED_NUMBERS.iter().position(|w| *w == text).map(|at| at as f32))
}
because!(answer_value, CursorTree, "the number an answer names, written in digits or spelled as a word");

fn same_answer(said: &str, wanted: &str) -> bool {
    said == wanted || answer_value(said).zip(answer_value(wanted)).is_some_and(|(a, b)| a == b)
}
because!(same_answer, CursorTree, "whether an output says what an answer wants: the same words, or the same number whether written in \
     digits or spelled");

pub fn output_answers(output: &[String], ended: bool, answer: &str) -> bool {
    let wanted = answer.split(ANSWER_SEPARATOR).filter(|part| !part.is_empty() && *part != NOTHING && part.strip_prefix(NOT_PREFIX).is_none()).flat_map(|part| part.split(CHAIN_MARK)).count();
    ended
        && output.len() == wanted
        && answer.split(ANSWER_SEPARATOR).filter(|part| !part.is_empty()).all(|part| {
            if part == NOTHING {
                return output.is_empty();
            }
            match part.strip_prefix(NOT_PREFIX) {
                Some(banned) => !output.iter().any(|s| s == banned),
                None => {
                    let chain: Vec<&str> = part.split(CHAIN_MARK).map(str::trim).collect();
                    output.windows(chain.len()).any(|run| run.iter().zip(&chain).all(|(s, c)| same_answer(s, c)))
                }
            }
        })
}
because!(
    output_answers,
    CursorTree,
    "whether an ended output answers a question: nothing is answered by an empty output, a token after not by its absence, and every other \
     part by its tokens standing in the output one after another, the output holding exactly as many tokens as the parts want and nothing \
     else, so no reading says the answer and then says more"
);
