use super::mind::{heard_word, sentence_slots, word_at};
use super::moves::{WordMove, WordStep};
use super::physics::word_stepped;
use super::WordReading;
use crate::cursor::{blanked_items, slot_features, CursorMind, CursorRow};
use crate::network::Stacked;
use patterns::{because, source};

pub struct WordTrails;
source!(WordTrails, "how the steps of a reading are printed for a person: each class with the word it pointed at, so a trail is read as \
     the debug page shows it");

pub const POINTED_MARK: &str = "->";
because!(POINTED_MARK, WordTrails, "what joins a step's class to the word it pointed at in a printed trail");

pub fn word_rows(mind: &mut CursorMind, words: &[String], steps: &[Vec<WordStep>], blanked: &dyn Fn(&str) -> bool) -> Vec<Vec<CursorRow>> {
    let mut all = Vec::new();
    for (index, (word, taken)) in words.iter().zip(steps).enumerate() {
        heard_word(mind, word, index);
        let mut rows = Vec::new();
        for step in taken {
            let pointable = if step.act.points() { sentence_slots(&mind.stack) } else { Vec::new() };
            let pointed: Vec<usize> = match step.at.and_then(|depth| word_at(&mind.stack, depth)) {
                Some(text) => pointable.iter().copied().filter(|&s| word_at(&mind.stack, s).is_some_and(|w| w == text)).collect(),
                None => Vec::new(),
            };
            let items = slot_features(&mind.stack);
            let events: Vec<String> = mind.stack.slots().map(|e| e.item.text.to_string()).collect();
            let blank = blanked_items(&mind.stack, &items, blanked);
            let depth = mind.stack.depth();
            rows.push(CursorRow { word: word.clone(), class: step.class(), items, events: events.clone(), pointed: pointed.clone(), pointable: pointable.clone(), depth, blanked: false });
            if let Some(items) = blank {
                rows.push(CursorRow { word: word.clone(), class: step.class(), items, events, pointed, pointable, depth, blanked: true });
            }
            word_stepped(mind, step);
        }
        all.push(rows);
    }
    all
}
because!(
    word_rows,
    WordReading,
    "the steps of a sentence written as the rows a network learns, one list per word since every word has its own stack: every step with \
     the stack it was taken on, its class, and for a step that points the slots of the sentence's words it may point at and among them \
     every slot that holds the word pointed at, so the pointer is taught the word and not one place; and every row whose stack holds a \
     word the lesson states as a thing written twice, once as it is and once with the text of every such word taken off, so a name never \
     seen reads as the learned names do"
);

pub struct WordRead {
    pub mind: CursorMind,
    pub ended: bool,
    pub trail: Vec<(String, Vec<String>)>,
}
because!(WordRead, WordReading, "a sentence read by a network one word at a time: the mind after it, whether every word continued within \
     the step limit, and the steps taken at each word by their classes with the word each pointed at");

fn shares(scores: &[f32]) -> Vec<f32> {
    let top = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let raised: Vec<f32> = scores.iter().map(|s| (s - top).exp()).collect();
    let all: f32 = raised.iter().sum();
    raised.into_iter().map(|r| r / all).collect()
}
because!(shares, WordReading, "scores turned into shares of one whole, so the votes of several networks add up on one scale");

fn word_choice(mind: &CursorMind, (nets, classes): (&[Stacked], &[WordStep]), among: &[usize]) -> Option<WordStep> {
    let slots = slot_features(&mind.stack);
    let passes: Vec<_> = nets.iter().map(|net| net.passed(slots.iter().map(Vec::as_slice))).collect();
    let mut votes = vec![f32::default(); classes.len()];
    for (net, passed) in nets.iter().zip(&passes) {
        let scores: Vec<f32> = (0..classes.len()).map(|c| net.output_at(&passed.hidden, c)).collect();
        for (vote, share) in votes.iter_mut().zip(shares(&scores)) {
            *vote += share;
        }
    }
    let best = (0..classes.len()).max_by(|&a, &b| votes[a].total_cmp(&votes[b]))?;
    let mut step = classes[best];
    if step.act.points() && !among.is_empty() {
        let mut pointed = vec![f32::default(); among.len()];
        for (net, passed) in nets.iter().zip(&passes).filter(|(net, _)| !net.point.is_empty()) {
            for (vote, share) in pointed.iter_mut().zip(shares(&net.pointing(passed, among))) {
                *vote += share;
            }
        }
        step.at = (0..among.len()).max_by(|&a, &b| pointed[a].total_cmp(&pointed[b])).map(|k| among[k]);
    }
    Some(step)
}
because!(word_choice, WordReading, "the step the networks pick on a mind by vote, each one's scores turned into shares and added, so what \
     one network trained from one start gets wrong by chance the others outvote: the strongest class, and for a class that points the slot \
     its pointer scores highest among the words of the sentence; none for a network with no classes");

pub fn read_by_words(start: &CursorMind, words: &[String], (nets, classes): (&[Stacked], &[WordStep]), limit: usize) -> WordRead {
    let mut mind = start.clone();
    let mut trail = Vec::new();
    let mut ended = true;
    for (index, word) in words.iter().enumerate() {
        heard_word(&mut mind, word, index);
        let mut taken = Vec::new();
        let mut continued = false;
        for _ in 0..=limit {
            let among = sentence_slots(&mind.stack);
            let Some(step) = word_choice(&mind, (nets, classes), &among) else { break };
            let pointed = step.at.and_then(|depth| word_at(&mind.stack, depth)).map(|w| format!("{}{w}", POINTED_MARK)).unwrap_or_default();
            taken.push(format!("{}{pointed}", step.class()));
            if !word_stepped(&mut mind, &step) {
                break;
            }
            if step.act == WordMove::Continue {
                continued = true;
                break;
            }
        }
        ended &= continued;
        trail.push((word.clone(), taken));
    }
    WordRead { mind, ended, trail }
}
because!(read_by_words, WordReading, "a sentence read on the network one word at a time, the network choosing every step, its strongest \
     class and for a pointing class the word its pointer scores highest, until it continues or has taken as many steps as the limit \
     allows; the trail records each step's class with the word it pointed at");
