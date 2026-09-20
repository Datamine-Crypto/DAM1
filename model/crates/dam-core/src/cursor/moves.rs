use super::{CursorTree};
use crate::quiz::{BRACE_CLOSE, BRACE_OPEN};
use patterns::{because, source};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CursorMove {
    WholeNumber,
    SetNumber,
    AddNumber,
    SubtractNumber,
    MultiplyNumber,
    DivideNumber,
    TakePercent,
    PowerNumber,
    RaiseNumber,
    RootNumber,
    AbsNumber,
    SinNumber,
    CosNumber,
    LnNumber,
    RoundNumber,
    RoundDefault,
    NegateNumber,
    LargerNumber,
    SmallerNumber,
    RemainderNumber,
    TakeAwayPercent,
    AddPercent,
    DivideByCount,
}
because!(
    CursorMove,
    CursorTree,
    "the moves on the number of the mind, each a fixed operation the network chooses and never decides the outcome of: set the number, add, subtract, multiply and divide by a number said or found, divide by a count, keep the larger or the smaller, the remainder, a power, a root, a percent taken, added or taken away, the sign turned, the whole part, a rounding to places said or to the places a question of none gets, and the functions a definition may mean; the teacher plans a number answer as a run of these before the moves of the reading one word at a time say it"
);

pub struct CursorMoves;
source!(
    CursorMoves,
    "the user's set of moves on a number, every one a fixed operation, and the words that name a function or a part of a whole"
);

pub(crate) const DEFINED_MOVES: [(&str, CursorMove); 6] = [("sin", CursorMove::SinNumber), ("cos", CursorMove::CosNumber), ("ln", CursorMove::LnNumber), ("sqrt", CursorMove::RootNumber), ("root", CursorMove::RootNumber), ("abs", CursorMove::AbsNumber)];
because!(DEFINED_MOVES, CursorMoves, "the steps on the number alone a definition may mean by their words, so a name defined as sin runs \
     the sine, and a definition that means another defined name runs that name's meaning first");

pub(crate) const FRACTION_WORDS: [(&str, f32); 12] = [("half", 2.0), ("halves", 2.0), ("third", 3.0), ("thirds", 3.0), ("quarter", 4.0), ("quarters", 4.0), ("fifth", 5.0), ("fifths", 5.0), ("sixth", 6.0), ("sixths", 6.0), ("tenth", 10.0), ("tenths", 10.0)];
because!(FRACTION_WORDS, CursorMoves, "the words that name a part of a whole and what each divides by, so a division by a copied fraction \
     word divides by that, and a third of a number or two thirds of it are worked from the word said, like the extreme words drive the \
     answer of the largest");

impl CursorMove {
    pub fn name(self) -> &'static str {
        match self {
            CursorMove::WholeNumber => "{number whole}",
            CursorMove::SetNumber => "{number set}",
            CursorMove::AddNumber => "{number add}",
            CursorMove::SubtractNumber => "{number subtract}",
            CursorMove::MultiplyNumber => "{number multiply}",
            CursorMove::DivideNumber => "{number divide}",
            CursorMove::TakePercent => "{number percent}",
            CursorMove::PowerNumber => "{number power}",
            CursorMove::RaiseNumber => "{number raise}",
            CursorMove::RootNumber => "{number root}",
            CursorMove::AbsNumber => "{number abs}",
            CursorMove::SinNumber => "{number sin}",
            CursorMove::CosNumber => "{number cos}",
            CursorMove::LnNumber => "{number ln}",
            CursorMove::RoundNumber => "{number round}",
            CursorMove::RoundDefault => "{number roundDefault}",
            CursorMove::NegateNumber => "{number negate}",
            CursorMove::LargerNumber => "{number larger}",
            CursorMove::SmallerNumber => "{number smaller}",
            CursorMove::RemainderNumber => "{number remainder}",
            CursorMove::TakeAwayPercent => "{number takeAwayPercent}",
            CursorMove::AddPercent => "{number addPercent}",
            CursorMove::DivideByCount => "{number byCount}",
        }
    }
}

pub(crate) fn bare_name(name: &str) -> String {
    name.trim_matches(|c| c == BRACE_OPEN || c == BRACE_CLOSE).to_string()
}
because!(bare_name, CursorTree, "a move's name without its braces, the word inside the item a step is written as, since the item's own \
     braces hold the move with its properties");

pub(crate) fn step_item(inside: &str) -> String {
    format!("{BRACE_OPEN}{inside}{BRACE_CLOSE}")
}
because!(step_item, CursorTree, "a step as one braced item: the move's family, its kind within the family when it has one, and, after a \
     gap each, the token and the value it carries, so a step reads as one thing with its properties, as step childOf owns or number add \
     with the copy mark, the way the user asked to see the stack, and the network learns a family once for all its kinds");
