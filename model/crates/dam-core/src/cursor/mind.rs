use super::tree::{NodeTree, asked_node, is_kind};
use super::figures::to_figures;
use super::{BRACE_OPEN_TEXT, CursorTree, NAMES_KIND, NAMES_NUMBER, NAMES_RELATION, NAMES_THING, NAMES_VALUE};
use crate::events::Stack;
use crate::words::number_of;
use crate::words::INFINITY_WORD;
use patterns::because;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct InputLooks {
    pub looked: bool,
    pub opened_at: Option<usize>,
    pub by_find: bool,
    pub moved: bool,
    pub nothing_found: bool,
    pub pointed: bool,
    pub worked: Vec<usize>,
    pub listed: Vec<usize>,
    pub counted: Vec<usize>,
}
because!(
    InputLooks,
    CursorTree,
    "what the steps of the newest input did, kept as the steps are taken instead of read back from the stack for every step the teacher \
     tries: whether a step looked for something; the stack depth of the newest step that opened the output, a lookup, a write, a walk into \
     a hidden node, a step to a slot or to the newest node, a step on numbers or letters, a listing, a check, a count of the rest or an \
     answer that found something; whether a find, a step to a slot or to the newest node, a write or a discovery found a node, since the \
     newest node is the thing the story told last and a question that names no thing asks of it, since a node the input just wrote is one \
     it knows without looking, so a turn answers from what it stored without a lookup for nothing; whether the cursor stands on a value; \
     whether the newest step pointed at a node by its name and found it; whether the newest lookup, a find, a step to the child of a \
     relation or a step back through the links, found nothing; the slots steps on a number took; the depths of what listings put on the \
     stack; and the nodes the cursor stood at when a step on a number took a count, so a comparison takes each count once"
);

impl InputLooks {
    pub fn looks(&self, depth: usize) -> (bool, Option<usize>, bool) {
        (self.looked, self.opened_at.map(|at| depth - at), self.by_find)
    }
}

pub struct Seen {
    pub(crate) node: usize,
    pub(crate) before: Option<Arc<Seen>>,
}
because!(Seen, CursorTree, "one node a reading discovered, pointing at the one discovered before it, as a list shared between the readings \
     of a search, so a discovering walk adds one link and copies no tree");

pub(crate) fn seen_nodes(seen: &Option<Arc<Seen>>) -> impl Iterator<Item = usize> + '_ {
    std::iter::successors(seen.as_ref(), |s| s.before.as_ref()).map(|s| s.node)
}
because!(seen_nodes, CursorTree, "the nodes a reading discovered, the newest first");

#[derive(Clone, Default)]
pub struct CursorMind {
    pub tree: NodeTree,
    pub seen: Option<Arc<Seen>>,
    pub at: usize,
    pub first_mark: Option<usize>,
    pub second_mark: Option<usize>,
    pub stack: Stack,
    pub output: Vec<String>,
    pub ended: bool,
    pub number: Option<f32>,
    pub places: Option<usize>,
    pub held: Vec<usize>,
    pub flags: Vec<(String, String)>,
    pub steps: usize,
    pub looks: InputLooks,
    pub known: Arc<HashSet<String>>,
    pub asked: Vec<(usize, usize)>,
    pub question_start: Option<usize>,
    pub preferred: Arc<Vec<String>>,
    pub own_relations: Arc<HashSet<String>>,
    pub before: Vec<String>,
    pub ahead: Vec<String>,
    pub plan: Option<Arc<super::plan::NumberPlan>>,
    pub last_topic: Option<usize>,
    pub sentence_from: usize,
    pub cause_of: Option<usize>,
}
because!(
    CursorMind,
    CursorTree,
    "what the network runs on: the tree; the nodes the reading discovered, kept apart from the tree until the reading is kept; the \
     node the cursor points at; the nodes its two pointers memorized; the stack, which is never cleared; the output of the newest input \
     and whether it ended; the number the input works on and the decimal places it is shown to; the nodes grabbed and not yet dropped, the \
     newest last; the flags set by the words before a thing and taken by the thing that appears next, each a name and a value, as the \
     reading one word at a time carries them; how many steps were taken; the record of what the newest input's steps did; and what the \
     console tells it for the teacher: the words the quiz and the seeds say name things, the stretches of the stack that earlier questions \
     filled, which no later question copies from, the start of the newest input while it is a question, the classes of the program taught \
     before for the same shape of input, which a reading follows at a tie, and the relations the quiz file itself states, the only ones a \
     question names outright beside the words it says, and the words of the sentences of the same lesson text read before this input and \
     still to come, since a text is read one sentence per input as the chat page sends it, and a statement is due by the sentence that \
     names its things; the thing a because in the newest sentence gives the cause of, until the sentence ends and the cause is tied to it; how many nodes the tree had when the newest sentence began, so the reading one word at a time tells a thing the sentence made from one it found; and the thing the sentence before this one was about, which the reading one word at a time keeps for the word it"
);

pub fn settled(mind: &mut CursorMind) {
    let seen: Vec<usize> = seen_nodes(&mind.seen).collect();
    mind.tree.unhidden(seen.into_iter());
    mind.seen = None;
}
because!(settled, CursorTree, "the nodes a reading discovered marked seen in its tree, once, when the reading is kept, so the tree carries \
     what the network has seen into the next line and the list is emptied");

pub(crate) fn names_class(mind: &CursorMind, word: &str) -> Option<&'static str> {
    if number_of(word).is_some() {
        return Some(NAMES_NUMBER);
    }
    if word.starts_with(BRACE_OPEN_TEXT) {
        return None;
    }
    if super::tree::named_braced(&mind.tree, word).any(|n| !mind.tree.node(n).gone && !asked_node(&mind.tree, n)) {
        Some(NAMES_RELATION)
    } else if is_kind(mind, word) {
        Some(NAMES_KIND)
    } else if mind.tree.thing_named(word).is_some() {
        Some(NAMES_THING)
    } else if super::tree::named_present(mind, word).next().is_some() {
        Some(NAMES_VALUE)
    } else {
        None
    }
}
because!(names_class, CursorTree, "what a word names in the tree the mind holds when it is heard or found: a relation by the word a braced \
     relation is written from, a kind when every node of the name stands under is, a thing when one stands under the world, a value when \
     the tree holds the name anywhere else, a number for a number, and nothing for a braced token or a name the tree lacks");

pub(crate) fn worked_text(value: f32, places: Option<usize>) -> String {
    if value.is_infinite() {
        return INFINITY_WORD.to_string();
    }
    match places {
        Some(p) => format!("{value:.p$}"),
        None => to_figures(value).to_string(),
    }
}
because!(worked_text, CursorTree, "a worked number as the stack and the output write it: to the decimal places set, keeping their zeros, \
     or as its form writes it to the figures the machine's numbers hold, and a number divided by nothing as the word for it");
