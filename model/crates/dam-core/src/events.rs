use crate::network::FeatureId;
use crate::words::{form_of, norm, number_of};
use patterns::{because, source};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct ItemFeatures;
source!(
    ItemFeatures,
    "the features of one event as the user listed them: for a word its text and its form; for an action its name; for a node its name, \
     whether it was made, whether it is an unknown, a noun, worth a number, owned, placed and joined, a band of how many relations it \
     stands in, whether it is in focus, and whether the same node already stands in an older slot; each written after its kind and never \
     with a place, so an event means the same in every slot"
);

pub const ITEM_KIND: &str = "kind";
because!(ITEM_KIND, ItemFeatures, "the feature every event has, naming its kind, so events of one kind share something whatever else they \
     carry");
const MARK_TEXT: &str = "mark";
because!(MARK_TEXT, ItemFeatures, "tells the start of a reading from its end, so the network can act differently before the first word and \
     after the last");
pub const INPUT_WORD: &str = "input.word";
because!(INPUT_WORD, ItemFeatures, "the word heard as its text, never for a number, which its form, last digit and digit count stand for, \
     so the network learns an operation and never one number's answer; the cursor takes it off a word that names a node of its tree and \
     puts what the word names in its place");
const INPUT_FORM: &str = "input.form";
because!(INPUT_FORM, ItemFeatures, "a number, a sign, a word for a set or any other word, shared by every word of that shape, so a word \
     never seen in training still reads like the words of its shape");
pub struct NumberItems;
source!(
    NumberItems,
    "the user's design of a number heard on the stack: the number as itself, so three and four are different items, and beside it features \
     a number never seen still shares with the numbers that were, so a count reads the same whichever numbers it runs over: its last \
     digit, how many digits it has, and how it stands to the number heard before it on the stack, the gap from that number and whether it \
     is twice that number"
);

pub const DIGIT_BASE: f32 = 10.0;
because!(DIGIT_BASE, NumberItems, "the base the digits of a number are written in, whose remainder is the last digit");
const INPUT_DIGIT: &str = "input.digit";
because!(INPUT_DIGIT, NumberItems, "the last digit of a number heard, shared by every number ending in it, so seven and seventeen share it");
const INPUT_DIGITS: &str = "input.digits";
because!(INPUT_DIGITS, NumberItems, "how many digits the whole part of a number heard has, a band of its size shared by every number of \
     that many digits");
const INPUT_GAP: &str = "input.gap";
because!(INPUT_GAP, NumberItems, "what a number heard adds to the number heard before it on the stack, as the step of a count where \
     it is one, so every step of a count by one carries the same gap, and as more or less where it is not, since a difference a \
     lesson never said would be a place of its own that no held out line reaches and the slot it stands for would lose what the \
     lesson gave it");
const COUNTING_STEPS: [f32; 11] = [-10.0, -5.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 5.0, 10.0];
because!(COUNTING_STEPS, NumberItems, "the gaps a count is made of, by ones, twos, threes, fives and tens each way and standing still, \
     which a number heard says of itself where the gap to the number before it is one of them");

const GAP_MORE: &str = "more";
because!(GAP_MORE, NumberItems, "that a number heard stands above the one before it by no step of a count");

const GAP_LESS: &str = "less";
because!(GAP_LESS, NumberItems, "that a number heard stands below the one before it by no step of a count");

const INPUT_TWICE: &str = "input.twice";
because!(INPUT_TWICE, NumberItems, "whether a number heard is twice the number heard before it on the stack, so a doubling reads the same \
     whichever numbers it doubles");

pub const ACTION_NAME: &str = "action.name";
because!(ACTION_NAME, ItemFeatures, "the behavior an action event took, the family of its move, shared by every kind of the family");
pub const ACTION_KIND: &str = "action.kind";
because!(ACTION_KIND, ItemFeatures, "the kind of a move within its family, the parent of a step or the add of a number, so the family is \
     learned once and the kind tells the steps of a family apart");

pub struct EventStack;
source!(
    EventStack,
    "the user's design of what a network over the stack is given and nothing else: the stack, an event sequence with the newest on top, a \
     mark before a reading's first word and one after its last, every word heard, every action the network picks and every node of the \
     tree an action makes or touches, the oldest falling off past a fixed number of slots, and a chat that starts from an empty stack"
);

pub const STACK_SLOTS: usize = 200;
because!(STACK_SLOTS, EventStack, "how many of the newest events the network is given, its short term memory: the user's number; every \
     input of the curriculum with its steps spans at most about a hundred events, so what a line needs beyond its own input is written \
     into the tree and found there, not kept on the stack");

pub const SCENE_SLOTS: usize = 0;
because!(SCENE_SLOTS, EventStack, "how much of what the story holds is laid out before a word, the newest first: what a scene is worth of \
     the stack, left well under the whole so the word and the moves it takes are never crowded off, since the events the network is given \
     are the newest ones and a scene laid out too wide would push the reading itself away");

pub const INPUT_END: &str = "{input end}";
because!(INPUT_END, EventStack, "the mark after a reading's last word, written in braces so no typed word is taken for it; the only mark, \
     since the end mark of the input before tells where the next begins and a start mark would be one more item to learn");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemKind {
    Mark,
    Input,
    Action,
    Argument,
    Found,
}
because!(ItemKind, "what an event on the stack is: a mark around a reading, a word heard, an action the network picked, a node of the tree \
     an action made or touched, or a fact an action wrote into the permanent state, the yes, no or nothing a question's check or look-up \
     came to, or a token said into the output, or a token a step named or what a step found");

impl ItemKind {
    pub fn name(self) -> &'static str {
        match self {
            ItemKind::Mark => "mark",
            ItemKind::Input => "input",
            ItemKind::Action => "action",
            ItemKind::Argument => "argument",
            ItemKind::Found => "found",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub kind: ItemKind,
    pub text: Arc<str>,
    pub ids: Vec<FeatureId>,
    pub token: Option<Arc<str>>,
    pub value: Option<Arc<str>>,
    pub bound: Option<Arc<str>>,
    pub value_bound: Option<Arc<str>>,
    pub node: Option<usize>,
}
because!(
    Item,
    "one event as it is pushed: its kind, the text it is read as, the word, the mark, the action's name or the node's name, the places of \
     its features in the hashed feature space, hashed once when the event is made so the network never hashes a string, for a move the \
     token it named or copied, the value a property step named or copied and the places of the slots each was copied from, for a word read \
     as an item a call predicted, the word that was heard, or for a copied token the place of the slot it came from, and for a token that \
     is a node of the tree the id of that node, so a step may go back to the very node and never to another of the same name"
);

pub fn feature(name: &str, value: impl std::fmt::Display) -> FeatureId {
    crate::network::slot_written(format_args!("{name}={value}"))
}
because!(feature, "the place of one feature of an event, its name and value written as one string and hashed");

#[derive(Debug)]
pub struct Event {
    pub item: Item,
    below: Option<Arc<Event>>,
    depth: usize,
    newest_number: Option<(f32, usize)>,
}
because!(Event, "one event on a stack: the item, the event pushed just before it, shared by every reading that heard it, how many events \
     the stack holds down to the first, and the newest number heard at or below it with the depth it stands at, so a pushed number finds \
     the one before it without walking the stack");

impl Drop for Event {
    fn drop(&mut self) {
        let mut below = self.below.take();
        while let Some(shared) = below {
            match Arc::try_unwrap(shared) {
                Ok(mut alone) => below = alone.below.take(),
                Err(_) => break,
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct Stack {
    top: Option<Arc<Event>>,
}
because!(
    Stack,
    "a reading's events as a list shared between readings: a push makes a new top pointing at the old one, so a reading forked into many \
     copies one pointer, readings that went different ways share what came before they parted, and a long list is let go one event at a \
     time rather than by a call per event"
);

impl std::fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.items().map(|e| e.item.text.clone())).finish()
    }
}

pub struct Items<'a> {
    at: Option<&'a Event>,
}
because!(Items, "the events of a stack walked from the newest down");

impl<'a> Iterator for Items<'a> {
    type Item = &'a Event;
    fn next(&mut self) -> Option<&'a Event> {
        let e = self.at?;
        self.at = e.below.as_deref();
        Some(e)
    }
}

impl Stack {
    pub fn push(&mut self, mut item: Item) {
        if let Some(value) = heard_number(&item) {
            let depth = self.depth();
            if let Some((previous, _)) = self.top.as_ref().and_then(|e| e.newest_number).filter(|&(_, at)| depth - at < STACK_SLOTS - 1) {
                item.ids.extend(step_ids(value, previous));
            }
        }
        self.place(item);
    }

    fn place(&mut self, item: Item) {
        let depth = self.depth() + 1;
        let newest_number = heard_number(&item).map(|value| (value, depth)).or_else(|| self.top.as_ref().and_then(|e| e.newest_number));
        self.top = Some(Arc::new(Event { item, below: self.top.take(), depth, newest_number }));
    }

    pub fn depth(&self) -> usize {
        self.top.as_ref().map_or(0, |e| e.depth)
    }

    pub fn items(&self) -> Items<'_> {
        Items { at: self.top.as_deref() }
    }

    pub fn slots(&self) -> std::iter::Take<Items<'_>> {
        self.items().take(STACK_SLOTS)
    }

}

pub fn is_mark(word: &str) -> bool {
    word == INPUT_END
}
because!(is_mark, "whether a word is the mark that ends a reading, which the network acts at like a word but which is no word of the text");

pub fn token_text(word: &str) -> String {
    if is_mark(word) { word.to_string() } else { norm(word) }
}
because!(
    token_text,
    "the text an input item is read as: a mark as it is written and a word as the reader normalises it, the same text the item carries on \
     the stack, so a predicted next item and the one that arrives compare as text"
);

pub fn heard_item(word: &str) -> Item {
    let plain = token_text(word);
    if is_mark(word) {
        return marked_item(&plain);
    }
    let mut ids = vec![feature(ITEM_KIND, ItemKind::Input.name()), feature(INPUT_FORM, form_of(&plain))];
    if number_of(&plain).is_none() {
        ids.push(feature(INPUT_WORD, &plain));
    }
    if let Some(value) = number_of(&plain) {
        let whole = value.abs().trunc();
        ids.push(feature(INPUT_DIGIT, whole % DIGIT_BASE));
        ids.push(feature(INPUT_DIGITS, whole.to_string().len()));
    }
    Item { kind: ItemKind::Input, text: plain.into(), ids, token: None, value: None, bound: None, value_bound: None, node: None }
}
because!(heard_item, "the event of a word heard, as the reader normalises it, with its form, its text unless it is a number, and for a \
     number its last digit and how many digits it has, or of a mark");

pub fn named_item(kind: ItemKind, (name, text): (&str, &str)) -> Item {
    Item { kind, text: text.into(), ids: vec![feature(ITEM_KIND, kind.name()), feature(name, text)], token: None, value: None, bound: None, value_bound: None, node: None }
}
because!(named_item, "the event of an item known by one feature beside its kind: its kind, its text, and that text as the feature named, \
     as an action by its name, an answer token by its answer or a token of a step by its text");

pub fn marked_item(mark: &str) -> Item {
    let ids = vec![feature(ITEM_KIND, ItemKind::Mark.name()), feature(MARK_TEXT, mark)];
    Item { kind: ItemKind::Mark, text: mark.into(), ids, token: None, value: None, bound: None, value_bound: None, node: None }
}
because!(marked_item, "the event of a mark as it is written, around a reading or around a reply, which no typed word is taken for");

fn heard_number(item: &Item) -> Option<f32> {
    if item.kind == ItemKind::Input { number_of(&item.text) } else { None }
}
because!(heard_number, NumberItems, "the value of an event that is a number heard, none for any other event");

pub fn step_ids(value: f32, previous: f32) -> Vec<FeatureId> {
    let gap = value - previous;
    let told = match COUNTING_STEPS.iter().find(|&&step| step == gap) {
        Some(step) => step.to_string(),
        None if gap > crate::numbers::zero() => GAP_MORE.to_string(),
        None => GAP_LESS.to_string(),
    };
    vec![feature(INPUT_GAP, told), feature(INPUT_TWICE, previous + previous == value)]
}
because!(
    step_ids,
    NumberItems,
    "the places of the features of how a number heard stands to the number heard before it on the stack: the gap between them and whether \
     it is twice that number, added when the number is pushed"
);

