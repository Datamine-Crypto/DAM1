use crate::numbers::zero;
use patterns::{because, source};

pub struct CursorPhysics;
source!(
    CursorPhysics,
    "the fixed operations on numbers, which the network chooses between and never decides the outcome of: the figures a worked number is written to, the places it is rounded to, percents, and the words for the even and the odd numbers of a range"
);

const FIGURES: i32 = 6;
because!(FIGURES, CursorPhysics, "how many figures a worked number is written to when no places are set, the most the machine's numbers \
     hold without noise, since four over a thousandth came out a hair under four thousand");

pub(crate) fn to_figures(value: f32) -> f32 {
    if value == zero() || !value.is_finite() {
        return value;
    }
    let scale = f64::from(crate::events::DIGIT_BASE).powi(FIGURES - 1 - f64::from(value.abs()).log10().floor() as i32);
    ((f64::from(value) * scale).round() / scale) as f32
}
because!(to_figures, CursorPhysics, "a number rounded to the figures the machine's numbers hold, counted from its first figure, so the \
     noise of the last binary places never reaches the output");

pub(crate) const EVEN_WORD: &str = "even";
because!(EVEN_WORD, CursorPhysics, "the word a question says when it counts the even numbers of a range, so the count between keeps every \
     second one");
pub(crate) const ODD_WORD: &str = "odd";
because!(ODD_WORD, CursorPhysics, "the word a question says when it counts the odd numbers of a range, so the count between keeps the \
     others");

pub(crate) const DEFAULT_PLACES: usize = 3;
because!(DEFAULT_PLACES, CursorPhysics, "how many decimal places a worked number is rounded to when a question names none: the places the \
     quiz writes a root or a cosine to");

pub(crate) const PERCENT_WHOLE: f32 = 100.0;
because!(PERCENT_WHOLE, CursorPhysics, "what a percent is a share of, by what the word means: hundredths");
