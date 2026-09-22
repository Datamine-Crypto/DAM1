use super::mind::{open_question, slot_of_text, is_number, singular, slot_of_word, word_classes};
use super::moves::{WordMove, WordStep};
use super::physics::word_stepped;
use crate::cursor::CursorMind;
use crate::quiz::APOSTROPHE;
use patterns::because;
use super::teacher::{WordGame, WalkTarget, ANSWER_TRIES, ANSWERING_MOVES, KeptWalk, KEPT_WALKS, SHAPE_DIGITS, SHAPE_OPEN, SHAPE_PART, SHAPE_GAP};

pub(super) fn answer_walk(mind: &CursorMind, index: usize, answer: &str) -> Option<Vec<(WordMove, Option<String>)>> {
    if answer == crate::quiz::YES || answer == crate::quiz::NO {
        return None;
    }
    let asked = asked_words(mind);
    let mut tries = 0;
    let plan_works = |plan: Vec<(WordMove, Option<String>)>, tries: &mut usize| -> Option<Vec<(WordMove, Option<String>)>> {
        if *tries >= ANSWER_TRIES {
            return None;
        }
        *tries += 1;
        plan_answers(mind, &plan, index, answer).then_some(plan)
    };
    for act in ANSWERING_MOVES {
        if let Some(plan) = plan_works(vec![(act, None)], &mut tries) {
            return Some(plan);
        }
        for said in &asked {
            if let Some(plan) = plan_works(vec![(act, Some(said.clone()))], &mut tries) {
                return Some(plan);
            }
        }
    }
    for found in &asked {
        for act in ANSWERING_MOVES {
            for said in &asked {
                if said == found {
                    continue;
                }
                if let Some(plan) = plan_works(vec![(WordMove::FindAsked, Some(found.clone())), (act, Some(said.clone()))], &mut tries) {
                    return Some(plan);
                }
            }
        }
    }
    None
}
because!(answer_walk, WordGame, "a plan the teacher finds for a question its rules do not answer: every answering move on its own and \
     pointed at each word the question holds, then each of them after finding one of those words, the first that writes the answer the \
     lesson gives; a bounded search, so a wording no rule names is still taught and the rules need not name every way a thing can be asked; a question answered yes or no is left to the rules, since of two answers one is hit by chance and a plan that says yes for reasons of its own would be taught as the reading");

pub(super) fn plan_answers(mind: &CursorMind, plan: &[(WordMove, Option<String>)], index: usize, answer: &str) -> bool {
    let mut trial = mind.clone();
    for (act, target) in plan.iter().cloned().chain(std::iter::once((WordMove::Continue, None))) {
        let at = if !act.points() { None } else if let Some(text) = target { slot_of_text(&trial.stack, &text).or_else(|| super::mind::slot_of_worked(&trial.stack, &text)) } else { slot_of_word(&trial.stack, index) };
        word_stepped(&mut trial, &WordStep { act, at });
    }
    crate::cursor::output_answers(&trial.output, true, answer)
}
because!(plan_answers, WordGame, "whether a plan of moves, run on a copy of the mind, writes the answer the lesson gives, so the teacher \
     keeps the plan its rules make when it is right and looks for a number plan when it is not");

pub(super) fn number_walk(mind: &CursorMind, words: &[String], answer: &str) -> Option<Vec<(WordMove, Option<String>)>> {
    let asked_words: Vec<String> = open_question(mind).map(|q| mind.tree.node(q).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let held: Vec<_> = asked_words.iter().filter(|w| crate::words::number_of(w).is_none()).filter_map(|w| super::physics::thing_number(mind, w, &asked_words)).map(|v| crate::cursor::held_operand(crate::cursor::worked_text(v, None), v)).collect();
    let wanted = crate::words::number_of(answer)?;
    let looked = asked_words.iter().rev().find(|w| crate::words::number_of(w).is_none() && *w != super::mind::CHANGE_ASKED && slot_of_text(&mind.stack, w).is_some() && super::physics::thing_number(mind, w, &asked_words).is_some_and(|n| (n - wanted).abs() < f32::EPSILON));
    let worked = asked_words.iter().any(|w| crate::words::number_of(w).is_some());
    if let Some(word) = looked.filter(|_| !worked) {
        return Some(vec![(WordMove::NumberSet, Some(word.clone())), (WordMove::NumberSay, None)]);
    }
    let plan = crate::cursor::number_plan_over(mind, words, answer, &held);
    let plan = plan?;
    let asked: Vec<String> = open_question(mind).map(|q| mind.tree.node(q).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let results: Vec<String> = {
        let mut shown = Vec::new();
        let mut now = crate::words::number_of(&plan.first);
        for (act, token) in &plan.steps {
            if let Some(n) = now {
                shown.push(crate::cursor::worked_text(n, None));
            }
            let b = token.as_deref().and_then(crate::words::number_of);
            now = match (act, now, b) {
                (crate::cursor::CursorMove::SetNumber, _, b) => b,
                (crate::cursor::CursorMove::AddNumber, Some(a), Some(b)) => Some(a + b),
                (crate::cursor::CursorMove::SubtractNumber, Some(a), Some(b)) => Some(a - b),
                (crate::cursor::CursorMove::MultiplyNumber, Some(a), Some(b)) => Some(a * b),
                (crate::cursor::CursorMove::DivideNumber, Some(a), Some(b)) if b.is_normal() => Some(a / b),
                _ => None,
            };
        }
        shown
    };
    let pointed = |token: &str| -> Option<String> {
        if slot_of_text(&mind.stack, token).is_some() {
            return Some(token.to_string());
        }
        let value = crate::words::number_of(token)?;
        let worth: Vec<&String> = asked.iter().filter(|w| crate::words::number_of(w).is_none() && slot_of_text(&mind.stack, w).is_some() && super::physics::thing_number(mind, w, &asked).is_some_and(|n| (n - value).abs() < f32::EPSILON)).collect();
        let owned = mind.before.iter().any(|b| b == APOSTROPHE);
        worth.iter().find(|w| super::physics::person_named(mind, w) == owned).or(worth.first()).map(|w| w.to_string()).or_else(|| results.iter().find(|r| *r == token).cloned())
    };
    let mut moves = vec![(WordMove::NumberSet, Some(pointed(&plan.first)?))];
    for (act, token) in &plan.steps {
        let (_, word_move) = super::moves::PLANNED_NUMBER_MOVES.iter().find(|(old, _)| old == act)?;
        let target = match token { Some(token) => Some(pointed(token)?), None => None };
        if word_move.points() != target.is_some() {
            return None;
        }
        moves.push((*word_move, target));
    }
    moves.push((WordMove::NumberSay, None));
    Some(moves)
}
because!(number_walk, WordGame, "a number a thing of the question is worth is looked up and said only when the question says no number and names no operation, so ten percent of tom's dollars is never taught as a look at something that happens to be worth the answer; the moves on the number that reach a number answer: the plan the number planner finds for the question \
     and its answer, each number it uses pointed at by the word of the question that says it, or by the word whose thing holds it, a dime \
     for its ten cents, the thing counted before its owner when both are worth it, the coins and not max, but the owner when the question names one with an apostrophe, a third of tom's apples, since only the owner tells whose count it is, or, for the result of a first \
     part, by the number that part showed on the stack, and none when a number of the plan has no such word; an answer that is the number \
     one word's thing holds is set from that word and said, the cents a dime is worth");

fn asked_words(mind: &CursorMind) -> Vec<String> {
    open_question(mind).map(|q| std::iter::once(q).chain(mind.tree.node(q).children.iter().copied()).filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default()
}
because!(asked_words, WordGame, "the words of the open question in the order they were said, its first word with them");

fn asked_shape(mind: &CursorMind, asked: &[String]) -> String {
    let open = [super::mind::THING_CLASS, super::mind::VALUE_CLASS];
    let swapped = |w: &str| [w.to_string(), singular(w)].iter().any(|form| word_classes(mind, form).iter().any(|class| open.contains(class)));
    let part = |w: &str| crate::cursor::FRACTION_WORDS.iter().any(|(f, _)| *f == w);
    asked.iter().map(|w| if crate::words::number_of(w).is_some() { SHAPE_DIGITS } else if part(w) { SHAPE_PART } else if swapped(w) && is_number(mind, w).is_none() { SHAPE_OPEN } else { w.as_str() }).collect::<Vec<_>>().join(SHAPE_GAP)
}
because!(asked_shape, WordGame, "the shape of a question, its words with every number in digits and every thing or value the story or the \
     seeds name, said as one or as many, taken out, so two questions that differ by what a lesson may swap have one shape");

fn word_worth(mind: &CursorMind, word: &str, asked: &[String]) -> Option<f32> {
    crate::words::number_of(word).or_else(|| super::physics::thing_number(mind, word, asked)).or_else(|| is_number(mind, word).and_then(|n| crate::words::number_of(&n)))
}
because!(word_worth, WordGame, "the number a word of a question is worth to a kept walk: the number it says, the number its thing holds, \
     or the number the seeds spell it as");

fn walk_moved(act: WordMove, now: Option<f32>, b: Option<f32>) -> Option<f32> {
    match (act, now, b) {
        (WordMove::NumberSet, _, b) => b,
        (WordMove::NumberAdd, Some(a), Some(b)) => Some(a + b),
        (WordMove::NumberSubtract, Some(a), Some(b)) => Some(a - b),
        (WordMove::NumberMultiply, Some(a), Some(b)) => Some(a * b),
        (WordMove::NumberDivide, Some(a), Some(b)) if b.is_normal() => Some(a / b),
        (WordMove::NumberSay, now, _) => now,
        _ => None,
    }
}
because!(walk_moved, WordGame, "the number a kept walk holds after one move, worked out for the four plain operations and unknown after \
     any other, which is enough to name the result a later move points at");

pub(super) fn kept_walks(mind: &CursorMind) -> Vec<Vec<(WordMove, Option<String>)>> {
    let asked = asked_words(mind);
    let shape = asked_shape(mind, &asked);
    let mut kept: Vec<(KeptWalk, usize)> = KEPT_WALKS.lock().map(|all| all.iter().filter(|(s, _, _)| *s == shape).map(|(_, walk, count)| (walk.clone(), *count)).collect()).unwrap_or_default();
    kept.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    kept.into_iter().filter_map(|(walk, _)| {
        let mut worked: Vec<Option<f32>> = Vec::new();
        let mut now: Option<f32> = None;
        let mut moves = Vec::new();
        for (act, target) in &walk {
            worked.push(now);
            let text = match target {
                Some(WalkTarget::Asked(at)) => Some(asked.get(*at)?.clone()),
                Some(WalkTarget::Worked(at)) => Some(crate::cursor::worked_text((*worked.get(*at)?)?, None)),
                None => None,
            };
            let b = text.as_deref().and_then(|t| word_worth(mind, t, &asked));
            now = walk_moved(*act, now, b);
            moves.push((*act, text));
        }
        Some(moves)
    }).collect()
}
because!(kept_walks, WordGame, "the walks kept for the shape of the open question, the one that answered most questions first, each with \
     its targets read off this question: a word by its place, a result worked out again from this question's numbers");

pub(super) fn walk_kept(mind: &CursorMind, walk: &[(WordMove, Option<String>)]) {
    let asked = asked_words(mind);
    let shape = asked_shape(mind, &asked);
    let mut worked: Vec<Option<String>> = Vec::new();
    let mut now: Option<f32> = None;
    let mut kept: KeptWalk = Vec::new();
    for (act, target) in walk {
        worked.push(now.map(|n| crate::cursor::worked_text(n, None)));
        let place = match target {
            Some(text) => match asked.iter().position(|w| w == text).map(WalkTarget::Asked).or_else(|| worked.iter().rposition(|w| w.as_deref() == Some(text.as_str())).map(WalkTarget::Worked)) {
                Some(place) => Some(place),
                None => return,
            },
            None => None,
        };
        let b = target.as_deref().and_then(|t| word_worth(mind, t, &asked));
        now = walk_moved(*act, now, b);
        kept.push((*act, place));
    }
    if let Ok(mut all) = KEPT_WALKS.lock() {
        match all.iter_mut().find(|(s, w, _)| *s == shape && *w == kept) {
            Some((_, _, count)) => *count += 1,
            None => all.push((shape, kept, 1)),
        }
    }
}
because!(walk_kept, WordGame, "a walk that answered kept under the shape of its question, its targets written as places in the question or \
     as results worked out before, and counted once more when it is kept already; a walk that points at anything else is not kept");
