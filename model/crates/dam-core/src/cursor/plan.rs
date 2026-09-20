use super::estimate::{COUNTED_WORD, RANGE_FROM, RANGE_TO, RANGE_WORDS, ROUND_WORDS, SIGN_MOVES, WHOLE_WORDS, ZERO_WORDS, asks_comparison, singular_of};
use super::mind::{CursorMind, worked_text};
use super::moves::{CursorMove, DEFINED_MOVES, FRACTION_WORDS};
use super::figures::{DEFAULT_PLACES, EVEN_WORD, ODD_WORD, PERCENT_WHOLE};
use super::tree::{below, child_named, holds_relations, named_nodes, named_present, present};
use super::BRACE_OPEN_TEXT;
use crate::events::{is_mark, token_text};
use crate::numbers::zero;
use crate::quiz::{BRACE_CLOSE, BRACE_OPEN, QUESTION_END};
use crate::words::number_of;
use patterns::{because, source};

pub struct NumberPlanning;
source!(
    NumberPlanning,
    "the teacher's plan for a question answered by a number: before the reading is searched, the teacher works out which operations over \
     the numbers the question says or names give the answer expected, as an accumulator works them, operation after operation; the plan \
     orders the search and never reaches the network, which learns the steps the reading takes as it learns any other; the user's rule: \
     the network must learn the correct pattern, so the plan uses every number the question says, as the reading must, and takes an \
     operation the question does not speak of, a percentage or a power, only when the question says so"
);

const PLAN_OPERATIONS: usize = 4;
const PLANS_COLLECTED: usize = 4;
because!(PLANS_COLLECTED, NumberPlanning, "how many plans of one length are collected before the search of chains stops: enough for the \
     chains from every number set first to be seen, since the chain from the whole was found twice, as the whole less the rest and the \
     whole over the taken, before the rest less the taken, the one the question names most, was tried at all, while collecting every chain \
     of a long pool would take too long");
because!(PLAN_OPERATIONS, NumberPlanning, "the most operations a plan holds: enough for the mean of the numbers a question says or a \
     purchase taken from an amount and turned around, while a longer chain would let the plan find the answer by chance from numbers that \
     have nothing to do with it");

const COUNT_WORDS: [&str; 1] = ["many"];
because!(COUNT_WORDS, NumberPlanning, "the word a question says when it asks how many of a thing there are, so the thing's own count joins \
     the numbers a plan may use, as the students of which some are absent");

pub(crate) const MEAN_WORDS: [&str; 2] = ["average", "mean"];
because!(MEAN_WORDS, NumberPlanning, "the words a question says when it asks for the mean of the numbers it says, so a plan for it divides \
     by how many were said and a sum of some of them that lands on the mean by chance is no plan");

const RATE_WORDS: [&str; 2] = ["at", "for"];
because!(RATE_WORDS, NumberPlanning, "the words that state a rate, at so much for so many, at twelve eggs for six people, so a plan for \
     the question divides by the number after the second word, since a sum divided by another number that lands on the answer by chance is \
     no plan");

const EACH_WORDS: [&str; 4] = ["each", "every", "per", "apiece"];
because!(EACH_WORDS, NumberPlanning, "the words a question says when a count of things goes with a price or a share of each, so a plan for \
     it multiplies or divides, since a sum of the amount and the price that lands on the answer by chance is no plan");

pub(crate) const LESS_WORDS: [&str; 6] = ["left", "more", "fewer", "less", "difference", "change"];
because!(LESS_WORDS, NumberPlanning, "the words a question says when it asks what is left, what is more or fewer, or the difference, so a \
     plan for it takes something away, by a subtraction, a negation or a remainder");

const PERCENT_WORDS: [&str; 2] = ["percent", "%"];
const OFF_WORDS: [&str; 4] = ["off", "discount", "reduced", "sale"];
const RISE_WORDS: [&str; 5] = ["rise", "increase", "raise", "tax", "tip"];
because!(OFF_WORDS, NumberPlanning, "the words that ask a percentage taken away from an amount, twenty percent off, without which a plan \
     never takes one away, since forty less fifty percent reached twenty by chance");
because!(RISE_WORDS, NumberPlanning, "the words that ask a percentage added to an amount, a ten percent rise, without which a plan never \
     adds one");
because!(PERCENT_WORDS, NumberPlanning, "the word a question says when a share of a number is asked, so the operations on percentages are \
     tried only then, since a share of a number lands on many answers by chance");

const POWER_WORDS: [&str; 5] = ["power", "squared", "cubed", "square", "cube"];

const SQUARE_WORDS: [&str; 2] = ["square", "squared"];
because!(SQUARE_WORDS, NumberPlanning, "the words a question says when it asks for a number times itself, so the plan multiplies the \
     number by itself once");

const CUBE_WORDS: [&str; 2] = ["cube", "cubed"];
because!(CUBE_WORDS, NumberPlanning, "the words a question says when it asks for a number times itself times itself, so the plan \
     multiplies the number by itself twice");

const CUBE_PRODUCTS: usize = 2;
because!(CUBE_PRODUCTS, NumberPlanning, "how many times a cube multiplies its number by itself: one fewer than the equal sides a cube has, \
     since the first side is set and each side after it is multiplied in");

fn self_product_plan(plain: &[String], answer: &str) -> Option<NumberPlan> {
    let said: Vec<&String> = plain.iter().filter(|w| number_of(w).is_some()).collect();
    let [number] = said[..] else {
        return None;
    };
    let times = if plain.iter().any(|w| SQUARE_WORDS.contains(&w.as_str())) { 1 } else if plain.iter().any(|w| CUBE_WORDS.contains(&w.as_str())) { CUBE_PRODUCTS } else { return None };
    let value = number_of(number)?;
    let worked = (0..times).fold(value, |so_far, _| so_far * value);
    (worked_text(worked, None) == answer).then(|| NumberPlan { first: number.clone(), steps: (0..times).map(|_| (CursorMove::MultiplyNumber, Some(number.clone()))).collect(), named: 0, copies: 0 })
}
because!(self_product_plan, NumberPlanning, "the plan for the square or the cube of the one number a question says: the number multiplied \
     by itself once or twice, since a square is a number times itself and a cube a number times itself times itself, taken before any \
     chain over the numbers the tree holds, since beside a seed that gives a cube six faces and twelve edges the cube of three was three \
     plus twelve plus twelve by chance");
because!(POWER_WORDS, NumberPlanning, "the words a question says when it asks for a power, the square or the cube of a number among them, \
     so raising to a power is tried only then, and the cube of three is three raised to the three and not three plus the twelve edges the \
     seeds give a cube twice");

pub(crate) const REMAINDER_WORDS: [&str; 3] = ["remainder", "over", "remaining"];
because!(REMAINDER_WORDS, NumberPlanning, "the words a question says when it asks what is left over after a division, beside the words \
     that ask whether a number divides another, so the remainder is tried only then, since a remainder lands on small answers by chance");

const ROOT_WORDS: [&str; 1] = ["root"];
because!(ROOT_WORDS, NumberPlanning, "the word a question says when it asks for a square root, so the root is tried only then");

#[derive(Debug, Clone, PartialEq)]
pub struct NumberPlan {
    pub first: String,
    pub steps: Vec<(CursorMove, Option<String>)>,
    pub named: usize,
    pub copies: usize,
}
impl NumberPlan {
    fn numbers(first: &str, steps: &[(CursorMove, Option<String>)]) -> Vec<String> {
        let mut all: Vec<String> = steps.iter().filter_map(|(_, token)| token.clone()).chain(std::iter::once(first.to_string())).collect();
        all.sort();
        all
    }

    fn same_numbers(&self, first: &str, steps: &[(CursorMove, Option<String>)]) -> bool {
        let operations = |steps: &[(CursorMove, Option<String>)]| {
            let mut all: Vec<&str> = steps.iter().map(|(act, _)| act.name()).collect();
            all.sort_unstable();
            all
        };
        Self::numbers(&self.first, &self.steps) == Self::numbers(first, steps) && operations(&self.steps) == operations(steps)
    }
}
because!(NumberPlan, NumberPlanning, "a plan: the number set first and the operations after it, each with the token it copies when it \
     takes one, the tokens written as the stack holds them so a step of the reading is matched to the plan by its text, and how many of \
     its numbers stand under a relation the question names, and how many of its numbers the tree holds that the question also said, since \
     such a number is the fact restated; a chain with the same numbers and the same operations as another, in another order, is the same \
     chain; a plan over said numbers alone tells the reading that the things the question names are its setting and not its facts");

#[derive(Clone, Debug)]
pub(crate) struct Operand {
    pub(crate) text: String,
    pub(crate) value: f32,
    pub(crate) said: bool,
    pub(crate) named: bool,
    pub(crate) once: bool,
    pub(crate) divisor: bool,
    pub(crate) group: usize,
    pub(crate) worth: bool,
    pub(crate) scales: bool,
}
because!(Operand, NumberPlanning, "a number a plan may use: its text as the reading copies it, its value, whether the question said it, \
     since a said number must be used and a number the tree holds is looked up before it is worked, and whether the question names it, by \
     saying its word, as a fraction word, or the relation it stands under, as the cents of a dollar when the question asks for cents, \
     since such a number is the one the question speaks of; and whether a plan may use it once only, which every number is, since a chain \
     that uses one number twice reaches the answer by chance, as sixty plus sixty plus sixty over four reached the minutes in half an hour \
     and a quarter of an hour; whether it only divides, as a fraction word does, since a half is a division by two and never a two to add, \
     and as what the unit the answer is counted in is worth does, unless the tree also holds it as a plain fact; the group it belongs to, \
     the held thing whose count and worth it is, or the said count whose unit it is worth, so a worth multiplies its own count only; \
     whether it is such a worth; and whether it only scales, as a number of a rate does, since a rate is a ratio");

#[derive(Clone, Copy)]
struct Asked {
    mean: bool,
    percent: bool,
    power: bool,
    root: bool,
    round: bool,
    remainder: bool,
    product: bool,
    rate: Option<f32>,
    less: bool,
    range: bool,
    whole: bool,
    compares: bool,
    off: bool,
    rise: bool,
}
because!(Asked, NumberPlanning, "which of the operations a question has to ask for by a word it says: a mean, a remainder, a percentage, a \
     power, a root or a rounding, so a plan takes those only for a question that asks; and which a question demands by a word it says: a \
     mean its division, a price of each its product, and what is left its taking away; and whether it compares numbers by its words, so a \
     plan keeps the larger or the smaller only for such a question");

const BINARY: [CursorMove; 12] = [
    CursorMove::AddNumber,
    CursorMove::SubtractNumber,
    CursorMove::MultiplyNumber,
    CursorMove::DivideNumber,
    CursorMove::LargerNumber,
    CursorMove::SmallerNumber,
    CursorMove::RemainderNumber,
    CursorMove::PowerNumber,
    CursorMove::TakePercent,
    CursorMove::AddPercent,
    CursorMove::TakeAwayPercent,
    CursorMove::RoundNumber,
];
because!(BINARY, NumberPlanning, "the operations a plan tries with another number, in the order they are tried, so plans of the same \
     length are chosen the same way every time: the rules of arithmetic first, since most questions ask them, then the larger and the \
     smaller of two, which answer which is biggest, smallest, hotter or colder");

const UNARY: [CursorMove; 5] = [CursorMove::NegateNumber, CursorMove::DivideByCount, CursorMove::WholeNumber, CursorMove::RootNumber, CursorMove::RoundDefault];
because!(UNARY, NumberPlanning, "the operations a plan tries without another number: the negation, since what is taken from an amount is \
     worked first and taken by a negation, dividing by how many numbers the question said, for a mean, keeping the whole number of a \
     division, for full bags, the square root, and rounding to the places the physics writes a root to");

fn allowed(act: CursorMove, asked: Asked) -> bool {
    if asked.mean && !matches!(act, CursorMove::AddNumber | CursorMove::DivideByCount) {
        return false;
    }
    match act {
        CursorMove::TakePercent => asked.percent,
        CursorMove::AddPercent => asked.percent && asked.rise,
        CursorMove::TakeAwayPercent => asked.percent && asked.off,
        CursorMove::PowerNumber => asked.power,
        CursorMove::RootNumber => asked.root,
        CursorMove::RemainderNumber => asked.remainder,
        CursorMove::RoundNumber | CursorMove::RoundDefault => asked.round,
        CursorMove::DivideByCount => asked.mean,
        CursorMove::WholeNumber => asked.whole,
        CursorMove::LargerNumber | CursorMove::SmallerNumber => asked.compares,
        _ => true,
    }
}
because!(allowed, NumberPlanning, "whether a plan may take an operation for a question: the rules of arithmetic, the larger and the \
     smaller of two only for a question that compares by its words, the negation always, and the division by the count, the whole number, \
     a remainder, a percentage, a power, a root or a rounding only when the question says the word for it, since those land on an answer \
     by chance too often to be tried unasked, as the half of a difference that lands on the colder of two temperatures; and for a mean \
     nothing but additions before the division, since a mean is the sum of what was said over how many were said");

fn worked(act: CursorMove, (value, places): (f32, Option<usize>), operand: Option<&Operand>, said_count: usize) -> Option<(f32, Option<usize>)> {
    let b = operand.map(|o| o.value);
    match (act, b) {
        (CursorMove::AddNumber, Some(b)) => Some((value + b, places)),
        (CursorMove::SubtractNumber, Some(b)) => Some((value - b, places)),
        (CursorMove::MultiplyNumber, Some(b)) => Some((value * b, places)),
        (CursorMove::DivideNumber, Some(b)) if b != zero() => Some((value / b, places)),
        (CursorMove::PowerNumber, Some(b)) => Some((value.powf(b), places)),
        (CursorMove::RaiseNumber, Some(b)) => Some((b.powf(value), places)),
        (CursorMove::RemainderNumber, Some(b)) if b.is_normal() => Some((value.rem_euclid(b), places)),
        (CursorMove::TakePercent, Some(b)) => Some((value * b / PERCENT_WHOLE, places)),
        (CursorMove::AddPercent, Some(b)) => Some((value + value * b / PERCENT_WHOLE, places)),
        (CursorMove::TakeAwayPercent, Some(b)) => Some((value - value * b / PERCENT_WHOLE, places)),
        (CursorMove::LargerNumber, Some(b)) => Some((value.max(b), places)),
        (CursorMove::SmallerNumber, Some(b)) => Some((value.min(b), places)),
        (CursorMove::RoundNumber, Some(b)) if b >= zero() && b.fract() == zero() => Some((value, Some(b as usize))),
        (CursorMove::NegateNumber, None) => Some((-value, places)),
        (CursorMove::WholeNumber, None) => Some((value.trunc(), places)),
        (CursorMove::DivideByCount, None) if said_count > 0 => Some((value / said_count as f32, places)),
        (CursorMove::RootNumber, None) if value >= zero() => Some((value.sqrt(), places)),
        (CursorMove::LnNumber, None) if value > zero() => Some((value.ln(), places)),
        (CursorMove::RoundDefault, None) => Some((value, Some(DEFAULT_PLACES))),
        _ => None,
    }
}
because!(worked, NumberPlanning, "what an operation of a plan makes of the number so far, worked as the physics works it, or nothing when \
     the physics would refuse the step, as a division by nothing or a root of a negative number; the places set by a rounding step travel \
     with the number, since the answer is compared as the physics writes it");

struct Growing<'a> {
    pool: &'a [Operand],
    first: &'a str,
    answer: &'a str,
    asked: Asked,
    said_count: usize,
}
because!(Growing, NumberPlanning, "what stays the same while a plan grows: the numbers it may use, the number set first, the answer \
     wanted, what the question asks for by its words, and how many numbers it said");

const PART_OPERATIONS: usize = 2;
const PARTS: usize = 2;
because!(PARTS, NumberPlanning, "how many parts a plan of parts holds: as many as a question joins with one and, as half a day and a \
     quarter of a day, since no question joins more");

const TWO_PART_OPERATIONS: usize = PART_OPERATIONS * PARTS + 1;
because!(TWO_PART_OPERATIONS, NumberPlanning, "the most operations a plan of two parts holds, both parts at their longest and the join, \
     tried after every shorter chain and pair, since the minutes in half an hour and a quarter of an hour are two whole hours in seconds \
     each divided by its fraction and by the seconds of a minute, then added");
because!(PART_OPERATIONS, NumberPlanning, "the most operations one part of a plan of parts holds, enough for a whole divided by a fraction \
     word or a count times a worth, since a longer part joined to another reaches the answer by chance");

struct Part {
    first: String,
    steps: Vec<(CursorMove, Option<String>)>,
    value: f32,
    used: Vec<bool>,
}
because!(Part, NumberPlanning, "one part of a two-part plan: the number it sets first, its operations, the value it works out and which \
     numbers of the pool it used");

fn parts(g: &Growing, state: f32, used: &mut Vec<bool>, steps: &mut Vec<(CursorMove, Option<String>)>, left: usize, out: &mut Vec<Part>) {
    out.push(Part { first: g.first.to_string(), steps: steps.clone(), value: state, used: used.clone() });
    if left == 0 {
        return;
    }
    for act in BINARY.into_iter().filter(|&act| allowed(act, g.asked) && !matches!(act, CursorMove::LargerNumber | CursorMove::SmallerNumber | CursorMove::RoundNumber)) {
        for i in 0..g.pool.len() {
            let operand = &g.pool[i];
            let self_op = steps.is_empty() && matches!(act, CursorMove::SubtractNumber | CursorMove::DivideNumber) && operand.text == g.first;
            if used[i] || (operand.divisor && act != CursorMove::DivideNumber) || (operand.scales && !scaling(act)) || self_op || !own_worth(g, steps, act, operand) {
                continue;
            }
            let Some((next, _)) = worked(act, (state, None), Some(operand), g.said_count) else {
                continue;
            };
            used[i] = true;
            steps.push((act, Some(operand.text.clone())));
            parts(g, next, used, steps, left - 1, out);
            steps.pop();
            used[i] = false;
        }
    }
}
because!(parts, NumberPlanning, "every chain of at most so many operations over the pool from one number set first, with its value and the \
     numbers it used, the empty chain among them, so a part may be one number alone; a comparison or a rounding never makes a part, since \
     a part is a quantity worked out");

fn two_part_plan(pool: &[Operand], asked: Asked, said_count: usize, answer: &str, length: usize) -> Option<NumberPlan> {
    let mut chains: Vec<Part> = Vec::new();
    for (i, first) in pool.iter().enumerate().filter(|(_, first)| !first.divisor) {
        let g = Growing { pool, first: &first.text, answer, asked, said_count };
        let mut used = vec![false; pool.len()];
        used[i] = true;
        let mut steps = Vec::new();
        parts(&g, first.value, &mut used, &mut steps, PART_OPERATIONS, &mut chains);
    }
    let joins = [CursorMove::AddNumber, CursorMove::SubtractNumber, CursorMove::MultiplyNumber, CursorMove::DivideNumber];
    let mut found: Vec<(usize, NumberPlan)> = Vec::new();
    let mut keys: Vec<(Vec<String>, Vec<&str>)> = Vec::new();
    for (ai, a) in chains.iter().enumerate() {
        for (bi, b) in chains.iter().enumerate() {
            let apart = pool.iter().enumerate().all(|(i, o)| !((o.said || o.worth) && a.used[i] && b.used[i]));
            let every_said = pool.iter().enumerate().all(|(i, o)| !o.said || a.used[i] || b.used[i]);
            if !apart || !every_said || (a.steps.is_empty() && b.steps.is_empty()) {
                continue;
            }
            for act in joins.into_iter().filter(|&act| allowed(act, asked) && !(matches!(act, CursorMove::AddNumber | CursorMove::MultiplyNumber) && ai > bi)) {
                let Some((value, _)) = worked(act, (a.value, None), Some(&Operand { text: String::new(), value: b.value, said: false, named: false, once: true, divisor: false, group: 0, worth: false, scales: false }), said_count) else {
                    continue;
                };
                let product_taken = !asked.product || [a, b].iter().any(|part| part.steps.iter().any(|(step, _)| scaling(*step))) || scaling(act);
                let rate_taken = asked.rate.is_none_or(|by| [a, b].iter().any(|part| part.steps.iter().any(|(step, text)| *step == CursorMove::DivideNumber && text.as_deref().and_then(number_of) == Some(by))) || (act == CursorMove::DivideNumber && b.value == by));
                if worked_text(value, None) != answer || !product_taken || !rate_taken || asked.mean || asked.range || (asked.compares && !asked.less) {
                    continue;
                }
                let mut steps = b.steps.clone();
                steps.push((CursorMove::SetNumber, Some(a.first.clone())));
                steps.extend(a.steps.iter().cloned());
                steps.push((act, Some(worked_text(b.value, None))));
                let total = a.steps.len() + b.steps.len() + 1;
                let plan = NumberPlan { first: b.first.clone(), steps, named: 0, copies: 0 };
                let mut numbers: Vec<String> = [a, b].iter().flat_map(|part| std::iter::once(part.first.clone()).chain(part.steps.iter().filter_map(|(_, token)| token.clone()))).collect();
                numbers.sort();
                let mut operations: Vec<&str> = [a, b].iter().flat_map(|part| part.steps.iter().map(|(step, _)| step.name())).chain(std::iter::once(act.name())).collect();
                operations.sort_unstable();
                let key = (numbers, operations);
                let said_first = |text: &str| pool.iter().any(|o| o.said && o.text == text);
                match keys.iter().position(|seen| *seen == key) {
                    Some(seen) if total == length && said_first(&found[seen].1.first) && !said_first(&plan.first) => found[seen].1 = plan,
                    Some(_) => {}
                    None if total == length => {
                        keys.push(key);
                        found.push((total, plan));
                    }
                    None => {}
                }
            }
        }
    }
    let shortest = found.iter().map(|(length, _)| *length).min()?;
    let mut best: Vec<NumberPlan> = found.into_iter().filter(|(length, _)| *length == shortest).map(|(_, plan)| plan).collect();
    (best.len() == 1).then(|| best.remove(0))
}
because!(two_part_plan, NumberPlanning, "a plan of two parts of so many operations in all, the join counted, tried at each length: two \
     chains over the pool each work a part and one sign joins them, the second part worked first, the first part set and worked after it, \
     and the sign taking the second part's value copied from the stack, as the sign plan joins a bracketed part; a said number serves one \
     part only while a number the tree holds may serve both, since an hour's sixty minutes are halved and quartered from the one fact; \
     every said number is used, a product demanded is in one part or the join, and a mean, a range or a comparison that asks for no \
     difference is never made so, since which of three numbers is smallest is one chain of comparisons and never two parts joined, while \
     how many more are present than absent is the present, the whole less the absent, less the absent again, and a join that commutes is \
     planned once whichever part comes first and whichever way a part is worked, the numbers and operations of both parts being its key, \
     the order kept being the one that works a number the tree holds first, since a lookup is made before the said numbers are set, and \
     whichever way its parts are worked; taken only when one plan of the fewest operations exists, since two parts reach an answer by \
     chance far more often than one chain");

fn own_worth(g: &Growing, steps: &[(CursorMove, Option<String>)], act: CursorMove, operand: &Operand) -> bool {
    let before_text: &str = steps.last().and_then(|(_, text)| text.as_deref()).unwrap_or(g.first);
    let Some(before) = g.pool.iter().find(|o| o.text == before_text) else {
        return true;
    };
    if before.scales && !scaling(act) {
        return false;
    }
    if operand.worth {
        return act == CursorMove::MultiplyNumber && !before.worth && before.group == operand.group;
    }
    if operand.group != 0 && g.pool.iter().any(|o| o.worth && o.group == operand.group) {
        return act == CursorMove::MultiplyNumber && before.worth && before.group == operand.group;
    }
    true
}
because!(own_worth, NumberPlanning, "whether an operand may follow the one before it: a worth, what a kind of held thing is worth in the \
     unit asked, only by multiplying the count of that very thing, and a count after a worth only as that worth's own count, a number the \
     question says right before a unit's name being the count of that unit, since three nickels are worth three times five cents and never \
     three times a quarter, nor a count divided by a worth, which reached the cents of two owners by chance, nor a worth divided by its \
     count, which reaches the minutes of one hour by chance");

fn unfolded(g: &Growing, state: (f32, Option<usize>), (used, named): (&mut Vec<bool>, usize), steps: &mut Vec<(CursorMove, Option<String>)>, left: usize, found: &mut Vec<NumberPlan>) {
    let every_said_used = g.pool.iter().zip(used.iter()).all(|(o, u)| !o.said || *u);
    let taken = |acts: &[CursorMove]| steps.iter().any(|(act, _)| acts.contains(act));
    let mean_taken = !g.asked.mean || taken(&[CursorMove::DivideByCount]);
    let product_taken = !g.asked.product || taken(&[CursorMove::MultiplyNumber, CursorMove::DivideNumber]);
    let less_taken = !g.asked.less || taken(&[CursorMove::SubtractNumber, CursorMove::NegateNumber, CursorMove::RemainderNumber]);
    let rate_taken = g.asked.rate.is_none_or(|by| steps.iter().any(|(act, text)| *act == CursorMove::DivideNumber && text.as_deref().and_then(number_of) == Some(by)));
    let whole_taken = !g.asked.whole || taken(&[CursorMove::DivideNumber]);
    let reached = every_said_used && mean_taken && product_taken && rate_taken && less_taken && whole_taken && !g.asked.range && worked_text(state.0, state.1) == g.answer;
    if reached && left == 0 && !found.iter().any(|plan| plan.same_numbers(g.first, steps)) {
        let copies = g.pool.iter().zip(used.iter()).filter(|(o, u)| **u && !o.said && g.pool.iter().any(|said| said.said && said.text == o.text)).count();
        found.push(NumberPlan { first: g.first.to_string(), steps: steps.clone(), named, copies });
    }
    if (reached && !steps.is_empty()) || left == 0 || found.len() > PLANS_COLLECTED {
        return;
    }
    let only_units = reached;
    let negated = steps.last().is_some_and(|(act, _)| *act == CursorMove::NegateNumber);
    for act in BINARY.into_iter().filter(|&act| allowed(act, g.asked) && !negated) {
        for i in 0..g.pool.len() {
            let operand = &g.pool[i];
            let self_op = steps.is_empty() && matches!(act, CursorMove::SubtractNumber | CursorMove::DivideNumber) && operand.text == g.first;
            let again = operand.named && !operand.said && g.asked.less && act == CursorMove::SubtractNumber;
            if (operand.once && used[i] && !again) || (act == CursorMove::RoundNumber && !operand.said) || (operand.divisor && act != CursorMove::DivideNumber) || (operand.scales && !scaling(act)) || self_op || !own_worth(g, steps, act, operand) || (only_units && !(operand.divisor || operand.worth)) {
                continue;
            }
            let Some(next) = worked(act, state, Some(operand), g.said_count) else {
                continue;
            };
            let was = used[i];
            used[i] = true;
            steps.push((act, Some(operand.text.clone())));
            unfolded(g, next, (used, named + usize::from(operand.named)), steps, left - 1, found);
            steps.pop();
            used[i] = was;
        }
    }
    for act in UNARY.into_iter().filter(|&act| allowed(act, g.asked) && (act != CursorMove::DivideByCount || every_said_used) && !only_units && !(negated && act == CursorMove::NegateNumber)) {
        let Some(next) = worked(act, state, None, g.said_count) else {
            continue;
        };
        steps.push((act, None));
        unfolded(g, next, (used, named), steps, left - 1, found);
        steps.pop();
    }
}
because!(unfolded, NumberPlanning, "the plans of exactly the length allowed that reach the answer, a shorter chain having been tried \
     before and a chain that reached the answer never grown further, unless the number set first is the answer itself and the operand is a \
     unit's worth, since the thirty that thirty ones make is still worked by dividing by one while one plus nothing is no plan, collected \
     until a few are found, every operation with another number tried before those without, so the plan found first at a length is the \
     same every time; a said number is used once, in any order, as is a number the tree holds, unless it stands under a relation the \
     question names and the question asks for a difference, when it is taken away again, since how many more are present than absent takes \
     the absent from the whole and then from the present, a fraction word only divides, and the number set first is never taken from \
     itself or divided by itself, since that makes nothing or one whatever the number and fourteen over fourteen plus three reached four \
     by chance, since a reading that leaves a said number unread is refused while the hours from an earlier hour to a later take the later \
     number first; no operation with another number follows a negation, so what is taken from an amount is worked, taken, and turned \
     around last, and the same purchase is not planned again as a negation before the sum; a plan is done only once every said number was \
     used and the operations the question's words demand were taken: for a mean the division by the count, which comes only once every \
     said number was, since a mean divides their sum, for a price of each a product or a rate, for what is left a taking away, and for \
     whole ones, as what can be bought or the full bags, a division, since what can be bought is money over the price while four plus four \
     reached the eight balls by chance; a chain with the same numbers and operations as one found already is the same chain, so a sum of \
     counts is planned once; and a rounding to a said number of places takes only a said number");

pub(crate) fn pool_of(mind: &CursorMind, words: &[String]) -> Vec<Operand> {
    let plain: Vec<String> = words.iter().map(|w| token_text(w)).filter(|w| !is_mark(w) && *w != QUESTION_END).collect();
    let mut pool: Vec<Operand> = Vec::new();
    let said_group = |at: usize| mind.tree.len() + at;
    let rate = rate_positions(&plain);
    for (at, w) in plain.iter().enumerate().filter(|(_, w)| number_of(w).is_some()) {
        let value = number_of(w).unwrap_or_default();
        {
            pool.push(Operand { text: w.clone(), value, said: true, named: false, once: true, divisor: false, group: said_group(at), worth: false, scales: rate.is_some_and(|(n, m)| at == n || at == m) });
        }
    }
    for word in &plain {
        if let Some((_, by)) = FRACTION_WORDS.iter().find(|(f, _)| *f == word.as_str()) {
            pool.push(Operand { text: word.clone(), value: *by, said: true, named: true, once: true, divisor: true, group: 0, worth: false, scales: false });
        }
    }
    if plain.first().is_some_and(|w| SIGN_MOVES.iter().any(|(sign, _)| *sign == w)) {
        if let Some(text) = super::tree::last_answered(mind).map(|n| mind.tree.node(n).name.to_string()).filter(|t| number_of(t).is_some() && !pool.iter().any(|o| o.text == *t)) {
            let value = number_of(&text).unwrap_or_default();
            pool.push(Operand { text, value, said: false, named: true, once: true, divisor: false, group: 0, worth: false, scales: false });
        }
    }
    let singular = |w: &str| singular_of(w).map(str::to_string);
    let names_word = |name: &str| plain.iter().any(|w| *w == name || singular(w).as_deref() == Some(name));
    let relation_word = |relation: &str| relation.strip_prefix(BRACE_OPEN).and_then(|r| r.strip_suffix(BRACE_CLOSE)).is_some_and(names_word);
    let push = |pool: &mut Vec<Operand>, node: usize, relation: usize, (group, worth, divisor): (usize, bool, bool)| {
        let text = mind.tree.node(node).name.to_string();
        if let Some(value) = number_of(&text) {
            let named = relation_word(&mind.tree.node(relation).name);
            match pool.iter_mut().find(|o| o.text == text && !o.said) {
                Some(seen) => {
                    seen.named |= named;
                    if worth && named && seen.divisor {
                        *seen = Operand { divisor: false, group, worth, ..seen.clone() };
                    } else if !worth && !divisor && seen.divisor {
                        seen.divisor = false;
                    }
                }
                None => pool.push(Operand { text, value, said: false, named, once: true, divisor, group, worth, scales: false }),
            }
        }
    };
    let owners: Vec<usize> = plain.iter().filter_map(|w| mind.tree.thing_named(w)).filter(|&o| holds_relations(mind, o)).collect();
    let worth_thing = |w: &String| [Some(w.clone()), singular(w)].into_iter().flatten().flat_map(|name| named_present(mind, &name).collect::<Vec<_>>()).any(|n| mind.tree.node(n).parent == 0 && mind.tree.node(n).children.iter().any(|&r| !super::is_quantity_tag(&mind.tree.node(r).name) && mind.tree.node(r).children.iter().any(|&c| number_of(&mind.tree.node(c).name).is_some())));
    let asks_unit = plain.iter().zip(plain.iter().skip(1)).any(|(w, next)| COUNT_WORDS.contains(&w.as_str()) && worth_thing(next));
    for (at, word) in plain.iter().enumerate().filter(|(_, w)| number_of(w).is_none() && w.chars().all(char::is_alphabetic) && !FRACTION_WORDS.iter().any(|(f, _)| *f == w.as_str())) {
        let before = at.checked_sub(1).map(|b| plain[b].as_str());
        let asked_unit = before.is_some_and(|b| COUNT_WORDS.contains(&b));
        let counted_unit = before.filter(|b| number_of(b).is_some()).map(|_| said_group(at - 1));
        let names = [Some(word.clone()), singular(word)];
        for thing in names.into_iter().flatten().flat_map(|name| named_nodes(mind, &name)).filter(|&n| (holds_relations(mind, n) || super::tree::has_worth(&mind.tree, n)) && (mind.tree.node(n).parent == 0 || owners.is_empty() || owners.iter().any(|&o| below(&mind.tree, o).contains(&n)))) {
            let relations: Vec<usize> = mind.tree.node(thing).children.iter().copied().filter(|&r| present(mind, r)).collect();
            let counted = plain.iter().any(|w| COUNT_WORDS.contains(&w.as_str())) && !super::tree::has_worth(&mind.tree, thing);
            let number_under = |relation: usize| super::tree::number_below(mind, relation);
            let whole = counted.then(|| relations.iter().copied().find(|&r| super::is_quantity_tag(&mind.tree.node(r).name)).and_then(number_under)).flatten();
            for &relation in relations.iter().filter(|&&r| counted || !super::is_quantity_tag(&mind.tree.node(r).name)) {
                if let Some((whole, taken)) = whole.zip(number_under(relation)).filter(|_| !super::is_quantity_tag(&mind.tree.node(relation).name)) {
                    let text = worked_text(whole - taken, None);
                    if !pool.iter().any(|o| o.text == text) {
                        pool.push(Operand { text, value: whole - taken, said: false, named: relation_word(&mind.tree.node(relation).name), once: true, divisor: false, group: 0, worth: false, scales: false });
                    }
                }
                let in_units = mind.tree.node(relation).name.strip_prefix(BRACE_OPEN).and_then(|r| r.strip_suffix(BRACE_CLOSE)).is_some_and(|r| mind.tree.thing_named(r).is_some() || singular(r).is_some_and(|one| mind.tree.thing_named(&one).is_some()));
                let unit = mind.tree.node(thing).parent == 0 && !super::is_quantity_tag(&mind.tree.node(relation).name) && in_units;
                let worth = unit && counted_unit.is_some() && (asks_unit || relation_word(&mind.tree.node(relation).name));
                for node in mind.tree.node(relation).children.iter().copied().filter(|&n| present(mind, n)) {
                    push(&mut pool, node, relation, (counted_unit.filter(|_| worth).unwrap_or(0), worth, unit && asked_unit));
                }
            }
            let any_named = relations.iter().any(|&r| mind.tree.node(r).children.iter().any(|&n| present(mind, n) && names_word(&mind.tree.node(n).name)));
            for &relation in &relations {
                for held in mind.tree.node(relation).children.iter().copied().filter(|&n| present(mind, n) && (names_word(&mind.tree.node(n).name) || (counted && !any_named))) {
                    let kind = mind.tree.node(held).link.or_else(|| mind.tree.thing_named(&mind.tree.node(held).name)).filter(|&k| k != held);
                    let deepers: Vec<usize> = std::iter::once(held).chain(kind).flat_map(|h| mind.tree.node(h).children.iter().copied()).filter(|&r| present(mind, r)).collect();
                    let unit_named = |r: usize| plain.iter().any(|w| mind.tree.node(r).name.strip_prefix(BRACE_OPEN_TEXT).and_then(|rest| rest.strip_suffix(BRACE_CLOSE)) == Some(w.as_str()));
                    let any_unit_named = deepers.iter().any(|&r| unit_named(r));
                    for deeper in deepers.into_iter().filter(|&r| !any_unit_named || unit_named(r) || super::is_quantity_tag(&mind.tree.node(r).name)) {
                        for node in mind.tree.node(deeper).children.iter().copied().filter(|&n| present(mind, n)) {
                            push(&mut pool, node, deeper, (held, mind.tree.node(deeper).parent != held, false));
                        }
                    }
                }
            }
        }
    }
    pool
}
because!(pool_of, NumberPlanning, "the numbers a plan may use for a question: every number it says, in the order said, each a number the \
     reading must use, and the number last answered when the question starts with a sign, since a sign and a number typed alone carry the \
     sum on from the last answer; reading must use, and never a number that names a node holding relations of its own, as an hour on the \
     clock, which is a thing to walk to; then what a fraction word it says divides by, and never a thing of that word's name, since a \
     quarter of a day is no coin, and a number the tree holds that the question also said is another operand, since the reading copies it \
     from the node while the said one comes from the word, as the sixty a minute holds under the sixty seconds asked about; then every \
     number that stands right under a relation of a node of a name the question says, or of its singular, that holds relations of its own \
     or a worth, wherever it stands, since the cookies ben shares are things under ben and ten holds only its worth, and under an owner \
     the question names when it names one, since the apples of joe are the ones asked about when kim has apples too, its own count aside \
     unless the question asks how many and the thing is no unit with a worth, and beside its own count what is left of it once the number \
     under each of its other relations is taken away, the students present being the students less the absent, since a question about the \
     rest is answered by the step that counts it and the plan takes that count as one number, since a conversion reads what a unit is \
     worth from the seeds, a worth being a number under a relation named for another unit, while a count a story states of a unit under \
     is, as the hundred centimeters a meter has, is no worth of the asked unit and multiplies as any said number, and the count of a unit \
     under the world is the count of someone's mention of it, and a count question about a counted thing works the whole and what is said \
     of some of it; then the numbers under what such a thing holds when the question names that too, as the legs of a spider, or asks how \
     many and names none of what the thing holds, as the dollars joe holds when asked how many cups joe can buy, while the pens asked \
     about are the ones that count when ann has apples too, with the numbers the held thing's kind holds, the thing it links to or the \
     root thing of its name, as the cents a nickel is worth, each text once, while what stands deeper belongs to other things; and a unit \
     under the world has a role by its place in \n the question, after how many it is the unit the answer is counted in, so what it is \
     worth only divides, and \n after a number it is the unit of that number, so what it is worth only multiplies, since the minutes in \
     two \n hours are two times the seconds of an hour over the seconds of a minute and never the seconds of a minute \n times two, which \
     lands on the answer by chance");

fn holds_worth_in(mind: &CursorMind, thing: usize, plain: &[String]) -> bool {
    let unit_relation = |r: usize| plain.iter().any(|w| mind.tree.node(r).name.strip_prefix(BRACE_OPEN_TEXT).and_then(|rest| rest.strip_suffix(BRACE_CLOSE)) == Some(w.as_str()));
    let kind_of = |v: usize| mind.tree.node(v).link.or_else(|| mind.tree.thing_named(&mind.tree.node(v).name)).filter(|&k| k != v);
    mind.tree.node(thing).children.iter().any(|&r| mind.tree.node(r).children.iter().any(|&v| kind_of(v).is_some_and(|k| mind.tree.node(k).children.iter().any(|&u| unit_relation(u)))))
}
because!(holds_worth_in, NumberPlanning, "whether a thing holds a value whose kind is worth something in a unit the question names, as tom \
     holding nickels when the cents are asked, so the question is answered by the step that sums what the owners hold and not planned as a \
     chain, while a minute, which holds its seconds and no such value, is no such owner");

fn rate_positions(plain: &[String]) -> Option<(usize, usize)> {
    let after = |word: &str, from: usize| plain.iter().enumerate().skip(from).zip(plain.iter().skip(from + 1)).find(|((_, w), next)| *w == word && number_of(next).is_some()).map(|((i, _), _)| i + 1);
    let so_much = after(RATE_WORDS[0], 0)?;
    let so_many = after(RATE_WORDS[1], so_much)?;
    Some((so_much, so_many))
}
because!(rate_positions, NumberPlanning, "where a question states a rate, at so much for so many: the place of the number after at and of \
     the number after the for that follows it, so the plan divides by the second and both only scale, since the two numbers of a rate are \
     a ratio, and eight over four plus two lands on the eggs for eight people by chance");

fn scaling(act: CursorMove) -> bool {
    matches!(act, CursorMove::MultiplyNumber | CursorMove::DivideNumber)
}
because!(scaling, NumberPlanning, "whether an operation scales, multiplies or divides, the only operations a number of a rate takes");

fn asked_of(plain: &[String]) -> Asked {
    let says = |words: &[&str]| plain.iter().any(|w| words.contains(&w.as_str()));
    let rate = rate_positions(plain).and_then(|(_, by)| number_of(&plain[by]));
    Asked { off: says(&OFF_WORDS), rise: says(&RISE_WORDS), mean: says(&MEAN_WORDS), percent: says(&PERCENT_WORDS), power: says(&POWER_WORDS), root: says(&ROOT_WORDS), round: says(&ROUND_WORDS), remainder: says(&REMAINDER_WORDS) || says(&ZERO_WORDS), product: says(&EACH_WORDS), rate, less: says(&LESS_WORDS), range: says(&RANGE_WORDS), compares: asks_comparison(plain.iter().map(String::as_str)), whole: says(&WHOLE_WORDS) && (plain.iter().any(|w| number_of(w).is_some()) || !says(&[FULL_WORD])) }
}
because!(asked_of, NumberPlanning, "what a question asks for by its words: a mean, a remainder, a percentage, a power, a root or a \
     rounding, and what it demands, a product for a price of each, a division for a rate stated as at a number, at twelve eggs for six \
     people, and a taking away for what is left, and whole ones only when a number is said or buying is asked, since the cups that are \
     full ask for no division, whether it asks for a range, which no plan serves, and whether it compares, read from the words that name \
     each");

const FULL_WORD: &str = "full";
because!(FULL_WORD, NumberPlanning, "the whole word that also names a state of things, so a question that says it and no number, how many \
     more \n cups are full than empty, asks for no division, while buying always does");

const AND_WORD: &str = "and";
because!(AND_WORD, NumberPlanning, "the word a question says between two parts it asks the sum of, as half a day and a quarter of a day, \
     so such a question is planned as two parts before one chain");

pub(crate) fn held_operand(text: String, value: f32) -> Operand {
    Operand { text, value, said: false, named: true, once: true, divisor: false, group: 0, worth: false, scales: false }
}
because!(held_operand, NumberPlanning, "a number a reading found held by a thing the question names, as an operand a plan may use once");

pub(crate) fn number_plan_over(mind: &CursorMind, words: &[String], answer: &str, held: &[Operand]) -> Option<NumberPlan> {
    let plain: Vec<String> = words.iter().map(|w| token_text(w)).collect();
    if number_of(answer).is_none() {
        return None;
    }
    let unit_named = |w: &String| super::tree::named_braced(&mind.tree, w).filter(|&r| !mind.tree.node(r).gone && mind.tree.node(r).parent != 0 && mind.tree.node(mind.tree.node(r).parent).parent == 0 && !mind.tree.node(mind.tree.node(r).parent).gone).any(|r| mind.tree.node(r).children.iter().any(|&c| number_of(&mind.tree.node(c).name).is_some()));
    let owner_named = |w: &String| mind.tree.thing_named(w).is_some_and(|t| holds_worth_in(mind, t, &plain));
    if held.is_empty() && !plain.iter().any(|w| number_of(w).is_some()) && plain.iter().any(unit_named) && plain.iter().any(owner_named) {
        return None;
    }
    if plain.iter().any(|w| *w == RANGE_FROM) && plain.iter().any(|w| *w == RANGE_TO) && plain.iter().any(|w| *w == EVEN_WORD || *w == ODD_WORD || *w == COUNTED_WORD) {
        return None;
    }
    let written = |w: &String| SIGN_MOVES.iter().any(|(sign, _)| *sign == w) || DEFINED_MOVES.iter().any(|(name, _)| *name == w) || *w == BRACKET_OPEN;
    if plain.iter().any(written) {
        if let Some(plan) = sign_plan(mind, &plain, answer) {
            return Some(plan);
        }
    }
    if let Some(plan) = self_product_plan(&plain, answer) {
        return Some(plan);
    }
    let asked = asked_of(&plain);
    if asked.compares && plain.iter().filter(|w| number_of(w).is_some()).count() > 1 && plain.iter().any(|w| w == answer) {
        return None;
    }
    let mut pool = pool_of(mind, words);
    if !held.is_empty() {
        pool.retain(|o| o.said || o.named);
    }
    for operand in held {
        if !pool.iter().any(|o| o.text == operand.text && !o.said) {
            pool.push(operand.clone());
        }
    }
    if std::env::var(TRACE_VARIABLE).is_ok() {
        eprintln!("pool: {}", pool.iter().map(|o| format!("{}{}{}{}", o.text, if o.said { "!" } else { "" }, if o.named { "*" } else { "" }, if o.divisor { "/" } else { "" })).collect::<Vec<_>>().join(" "));
    }
    let said_count = pool.iter().filter(|o| o.said).count();
    let joined = plain.iter().any(|w| *w == AND_WORD);
    for length in 1..=PLAN_OPERATIONS {
        if joined {
            if let Some(plan) = two_part_plan(&pool, asked, said_count, answer, length) {
                return Some(plan);
            }
        }
        let mut found = Vec::new();
        for (i, first) in pool.iter().enumerate().filter(|(_, first)| !first.divisor) {
            let g = Growing { pool: &pool, first: &first.text, answer, asked, said_count };
            let mut used = vec![false; pool.len()];
            used[i] = true;
            let mut steps = Vec::new();
            unfolded(&g, (first.value, None), (&mut used, usize::from(first.named)), &mut steps, length, &mut found);
        }
        let least = found.iter().map(|plan| plan.copies).min().unwrap_or(0);
        let found: Vec<NumberPlan> = found.into_iter().filter(|plan| plan.copies == least).collect();
        let most = found.iter().map(|plan| plan.named).max().unwrap_or(0);
        let best: Vec<&NumberPlan> = found.iter().filter(|plan| plan.named == most).collect();
        let operations = |plan: &NumberPlan| { let mut ops: Vec<CursorMove> = plan.steps.iter().map(|(act, _)| *act).collect(); ops.sort_by_key(|act| act.name()); ops };
        let one_set = best.iter().all(|plan| NumberPlan::numbers(&plan.first, &plan.steps) == NumberPlan::numbers(&best[0].first, &best[0].steps) && operations(plan) == operations(best[0]));
        let said_first = |plan: &NumberPlan| pool.iter().any(|o| o.said && o.text == plan.first);
        if let Some(plan) = best.iter().find(|plan| said_first(plan)).or(best.first()).filter(|_| best.len() == 1 || most > 0 || one_set) {
            return Some((*plan).clone());
        }
        let place = |token: &str| plain.iter().position(|w| w == token);
        let in_order = |plan: &&NumberPlan| {
            let places: Vec<Option<usize>> = std::iter::once(plan.first.as_str()).chain(plan.steps.iter().filter_map(|(_, token)| token.as_deref())).map(place).collect();
            places.iter().all(Option::is_some) && places.windows(PARTS).all(|pair| pair[0] < pair[1])
        };
        let ordered: Vec<&NumberPlan> = best.iter().copied().filter(in_order).collect();
        if let [one] = ordered[..] {
            return Some(one.clone());
        }
        let whole = |plan: &&NumberPlan| std::iter::once(plan.first.as_str()).chain(plan.steps.iter().filter_map(|(_, token)| token.as_deref())).all(|token| number_of(token).is_some_and(|n| n.fract() == zero()));
        let wholes: Vec<&NumberPlan> = best.iter().copied().filter(whole).collect();
        if let [one] = wholes[..] {
            return Some(one.clone());
        }
        if !joined {
            if let Some(plan) = two_part_plan(&pool, asked, said_count, answer, length) {
                return Some(plan);
            }
        }
    }
    (PLAN_OPERATIONS + 1..=TWO_PART_OPERATIONS).find_map(|length| two_part_plan(&pool, asked, said_count, answer, length))
}
because!(number_plan_over, NumberPlanning, "a caller that holds operands of its own, the word world, is planned for with them, the numbers \
     said and the numbers of relations the question names, and no worth question is turned away from it; among several chains of one \
     length that reach the answer, the one chain that takes its numbers in the order the question says them, fifty percent of eighty and \
     not eighty less fifty percent, or the one chain over whole numbers only, fourteen days over the seven of a week and not times a \
     seventh; else the shortest plan that works the answer out of the numbers the question says or names, none for a question that \
     compares and is answered by one of the several numbers it says, since such a question chooses among its numbers and the smallest of \
     six, twenty and fourteen is the twenty less the fourteen by chance, taken only when it is the one plan of its length among those that \
     read the fewest said numbers from the tree as well, since the question restates the fact it asks about and fifty over fifteen less \
     two times fifteen reached the money left by chance, the first of those whose numbers the question names most, or the first of those \
     of one operation that use the same two numbers and differ only in the operation, never one that starts from a fraction word, since a \
     fraction word only divides, the one that starts from a number the question said before the rest, as thirty divided by one before one \
     times thirty, since the accumulator starts from what the question says and works with what was looked up, and an answer that more \
     than one chain of operations reaches at that length is reached by chance at that length unless the chain speaks of what the question \
     asked, and a longer length is tried then, where the chains that reached the answer by chance are not grown; none for an answer that \
     is no number, for a question written with the signs of arithmetic, whose signs order its steps already, or when no plan of the length \
     allowed reaches the answer; a plan of no operation is never made, since an answer that is a number said or held is copied or looked \
     up, which the estimate guides without a plan; a question written with signs, a function word or a bracket gets its plan from the \
     signs, and the plan over its numbers when the signs do not read as one expression, as half of ten plus two; at each length a plan of \
     two parts, its join counted as an operation, is tried before one chain of that length when the question says and between its parts \
     and after it otherwise, since half of ten plus two is one chain while half a day and a quarter of a day are two");

enum Piece {
    Number(String, f32),
    Sign(CursorMove),
    Alone(CursorMove),
    Open,
    Close,
}
because!(Piece, NumberPlanning, "one piece of an expression written with signs: a number, said or the worth of a name the tree holds, a \
     sign between two numbers, a function word that works the number alone, as a root or a sine, and an opening or a closing bracket; \
     every other word is left out");

pub(crate) const BRACKET_OPEN: &str = "(";
because!(BRACKET_OPEN, NumberPlanning, "the bracket that opens a part of an expression worked first, and a path of a bracket statement of \
     the seeds");

pub(crate) const BRACKET_CLOSE: &str = ")";
because!(BRACKET_CLOSE, NumberPlanning, "the bracket that closes such a part or such a path");

struct Grown {
    steps: Vec<(CursorMove, Option<String>)>,
    value: f32,
    plain: Option<String>,
}
because!(Grown, NumberPlanning, "a part of an expression read: the steps that work it, its value, and its text when it is one number that \
     no step has to work, so a sign takes it as its operand instead of setting it first");

fn pieces_of(mind: &CursorMind, plain: &[String]) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for word in plain {
        if let Some(value) = number_of(word) {
            pieces.push(Piece::Number(word.clone(), value));
        } else if *word == BRACKET_OPEN {
            pieces.push(Piece::Open);
        } else if *word == BRACKET_CLOSE {
            pieces.push(Piece::Close);
        } else if let Some((_, act)) = SIGN_MOVES.iter().find(|(sign, _)| *sign == word) {
            pieces.push(Piece::Sign(*act));
        } else if let Some((_, act)) = DEFINED_MOVES.iter().find(|(name, _)| *name == word) {
            pieces.push(Piece::Alone(*act));
        } else if let Some(worth) = mind.tree.thing_named(word).and_then(|thing| super::tree::worth_of(&mind.tree, thing)).filter(|&n| present(mind, n)) {
            let text = mind.tree.node(worth).name.to_string();
            if let Some(value) = number_of(&text) {
                pieces.push(Piece::Number(text, value));
            }
        }
    }
    pieces
}
because!(pieces_of, NumberPlanning, "the pieces of a question written with signs, in the order said: its numbers, its brackets, its signs, \
     its function words, and for a name the tree holds with a worth, that worth as a number the reading looks up");

fn as_set(grown: Grown) -> (Vec<(CursorMove, Option<String>)>, f32) {
    match grown.plain {
        Some(text) => (vec![(CursorMove::SetNumber, Some(text))], grown.value),
        None => (grown.steps, grown.value),
    }
}
because!(as_set, NumberPlanning, "the steps that leave a part of an expression as the number set: a bare number is set, and a worked part \
     is as its steps left it");

fn combined(a: Grown, act: CursorMove, b: Grown) -> Option<Grown> {
    let value = match act {
        CursorMove::AddNumber => a.value + b.value,
        CursorMove::SubtractNumber => a.value - b.value,
        CursorMove::MultiplyNumber => a.value * b.value,
        CursorMove::DivideNumber if b.value != zero() => a.value / b.value,
        CursorMove::PowerNumber => a.value.powf(b.value),
        _ => return None,
    };
    let commutes = matches!(act, CursorMove::AddNumber | CursorMove::MultiplyNumber);
    let steps = match (b.plain.clone(), a.plain.clone()) {
        (Some(text), _) => {
            let (mut steps, _) = as_set(a);
            steps.push((act, Some(text)));
            steps
        }
        (None, Some(text)) if commutes => {
            let (mut steps, _) = as_set(b);
            steps.push((act, Some(text)));
            steps
        }
        (None, Some(text)) if act == CursorMove::PowerNumber => {
            let (mut steps, _) = as_set(b);
            steps.push((CursorMove::RaiseNumber, Some(text)));
            steps
        }
        (None, _) => {
            let (mut steps, worked) = as_set(b);
            let (later, _) = as_set(a);
            steps.extend(later);
            steps.push((act, Some(worked_text(worked, None))));
            steps
        }
    };
    Some(Grown { steps, value, plain: None })
}
because!(combined, NumberPlanning, "two parts of an expression joined by a sign: when the second is one number the first is worked and the \
     sign takes that number; when the second is worked and the first is one number under a sign that commutes, the second is worked and \
     the sign takes the first; when the second is worked and the first is one number under a power, the second is worked and the first is \
     raised to it, so a chain of powers copies no result; and otherwise the second is worked first, the first is worked after it, and the \
     sign takes the second's result copied from the stack, so a bracketed part is an operand like any number");

fn alone(act: CursorMove, x: Grown) -> Option<Grown> {
    let value = match act {
        CursorMove::NegateNumber => -x.value,
        CursorMove::RootNumber if x.value >= zero() => x.value.sqrt(),
        CursorMove::SinNumber => x.value.sin(),
        CursorMove::CosNumber => x.value.cos(),
        CursorMove::LnNumber if x.value > zero() => x.value.ln(),
        CursorMove::AbsNumber => x.value.abs(),
        _ => return None,
    };
    let (mut steps, _) = as_set(x);
    steps.push((act, None));
    Some(Grown { steps, value, plain: None })
}
because!(alone, NumberPlanning, "a part of an expression worked by a step on the number alone: the negation, a root, a sine, a cosine or \
     the absolute value, taken after the part is set");

fn sum(pieces: &[Piece], at: &mut usize) -> Option<Grown> {
    let mut left = product(pieces, at)?;
    while let Some(Piece::Sign(act @ (CursorMove::AddNumber | CursorMove::SubtractNumber))) = pieces.get(*at) {
        *at += 1;
        let right = product(pieces, at)?;
        left = combined(left, *act, right)?;
    }
    Some(left)
}
because!(sum, NumberPlanning, "a sum of products, joined by plus and minus from the left, the loosest binding");

fn product(pieces: &[Piece], at: &mut usize) -> Option<Grown> {
    let mut left = power(pieces, at)?;
    while let Some(Piece::Sign(act @ (CursorMove::MultiplyNumber | CursorMove::DivideNumber))) = pieces.get(*at) {
        *at += 1;
        let right = power(pieces, at)?;
        left = combined(left, *act, right)?;
    }
    Some(left)
}
because!(product, NumberPlanning, "a product of powers, joined by times and divided by from the left, binding tighter than a sum");

fn power(pieces: &[Piece], at: &mut usize) -> Option<Grown> {
    let base = primary(pieces, at)?;
    if let Some(Piece::Sign(CursorMove::PowerNumber)) = pieces.get(*at) {
        *at += 1;
        let exponent = power(pieces, at)?;
        return combined(base, CursorMove::PowerNumber, exponent);
    }
    Some(base)
}
because!(power, NumberPlanning, "a power, binding tightest and taken from the right, so a power of a power raises to the power worked \
     first");

fn primary(pieces: &[Piece], at: &mut usize) -> Option<Grown> {
    let piece = pieces.get(*at)?;
    *at += 1;
    match piece {
        Piece::Number(text, value) => Some(Grown { steps: Vec::new(), value: *value, plain: Some(text.clone()) }),
        Piece::Sign(CursorMove::SubtractNumber) => alone(CursorMove::NegateNumber, primary(pieces, at)?),
        Piece::Alone(act) => alone(*act, primary(pieces, at)?),
        Piece::Open => {
            let inner = sum(pieces, at)?;
            matches!(pieces.get(*at), Some(Piece::Close)).then(|| *at += 1)?;
            Some(inner)
        }
        _ => None,
    }
}
because!(primary, NumberPlanning, "one operand of an expression: a number, a minus before an operand, which negates it, a function word \
     before its operand, or a bracketed expression, which is worked as a whole");

const PLACES_RELATION: &str = "{in}";
because!(PLACES_RELATION, NumberPlanning, "the relation under the thing named round that holds the decimal places a text asked for, as \
     round to two decimal places writes them");

fn round_places(mind: &CursorMind) -> Option<String> {
    let first_value = |thing: usize, relation: &str| child_named(&mind.tree, thing, relation).and_then(|r| mind.tree.node(r).children.iter().copied().find(|&c| present(mind, c))).map(|v| mind.tree.node(v).name.to_string());
    let places = first_value(mind.tree.thing_named(ROUND_WORDS[0])?, PLACES_RELATION)?;
    if number_of(&places).is_some() {
        return Some(places);
    }
    super::tree::worth_of(&mind.tree, mind.tree.thing_named(&places)?).map(|n| mind.tree.node(n).name.to_string())
}
because!(round_places, NumberPlanning, "the decimal places the tree asks answers to be rounded to: the value under round's places, as a \
     number, or the worth of that value's word when the text said the places as a word, so two decimal places reads as the two the reading \
     copies; none when no rounding was asked");

fn sign_plan(mind: &CursorMind, plain: &[String], answer: &str) -> Option<NumberPlan> {
    let pieces = pieces_of(mind, plain);
    let mut at = 0;
    let grown = sum(&pieces, &mut at)?;
    if at != pieces.len() {
        return None;
    }
    let exact = worked_text(grown.value, None) == answer;
    let rounding = (!exact).then(|| round_places(mind)).flatten().filter(|places| number_of(places).map(|p| p as usize).is_some_and(|p| worked_text(grown.value, Some(p)) == answer));
    let by_default = !exact && rounding.is_none() && round_places(mind).is_none() && worked_text(grown.value, Some(DEFAULT_PLACES)) == answer;
    if !exact && rounding.is_none() && !by_default {
        return None;
    }
    let (mut steps, _) = as_set(grown);
    if let Some(places) = rounding {
        steps.push((CursorMove::RoundNumber, Some(places)));
    } else if by_default {
        steps.push((CursorMove::RoundDefault, None));
    }
    let (first, rest) = steps.split_first()?;
    match first {
        (CursorMove::SetNumber, Some(text)) => Some(NumberPlan { first: text.clone(), steps: rest.to_vec(), named: 0, copies: 0 }),
        _ => None,
    }
}
because!(sign_plan, NumberPlanning, "the plan of a question written with signs: its pieces read as a sum of products of powers with \
     brackets and functions, into the steps an accumulator takes, the first the number set first and the rest the operations in order, a \
     bracketed or tighter-binding part worked first and copied back as the operand of the step that uses it; none when the pieces do not \
     read as one expression or its value is not the answer expected, so a rounded answer ends the plan with a rounding to the places the \
     tree holds when that makes it the answer, since a text that asked for two decimal places is answered by the division rounded so, or \
     with the quiz's default rounding when the tree asks for no places and that makes it the answer, as the cosine of two to three places");

pub struct SearchTrace;
source!(SearchTrace, "the user watching a number plan that finds nothing and asking what it had to plan with, answered by a trace the environment switches on");

const TRACE_VARIABLE: &str = "DAM_TRACE";
because!(TRACE_VARIABLE, SearchTrace, "the environment variable that, when set, makes the number plan print the operands it gathered, each marked as said, named or a divisor, so a plan that finds nothing shows what it had to work with");
