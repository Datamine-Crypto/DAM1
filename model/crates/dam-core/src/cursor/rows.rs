use super::{CursorTree, TOKEN_TEXT, VALUE_TEXT};
use crate::events::{INPUT_WORD, feature};
use crate::network::FeatureId;
use patterns::because;

pub struct CursorRow {
    pub word: String,
    pub class: String,
    pub items: Vec<Vec<FeatureId>>,
    pub events: Vec<String>,
    pub pointed: Vec<usize>,
    pub pointable: Vec<usize>,
    pub depth: usize,
    pub blanked: bool,
}
because!(CursorRow, CursorTree, "one step as a network is taught it: the input item it was taken at, the step's class, the stack before it \
     as feature places and as text, newest first, for a copy the slots its token stands in and the slots it may be copied from, how many \
     events the stack has held since the line began, so a writer keeps the events once and the row as a window over them, and whether the \
     row's names are blanked");

pub(crate) fn blanked_items(stack: &crate::events::Stack, items: &[Vec<FeatureId>], blanked: &dyn Fn(&str) -> bool) -> Option<Vec<Vec<FeatureId>>> {
    let mut changed = false;
    let blank: Vec<Vec<FeatureId>> = stack.slots().zip(items).map(|(e, ids)| {
        let texts = [Some(&e.item.text), e.item.token.as_ref(), e.item.value.as_ref()];
        let gone: Vec<FeatureId> = texts.iter().flatten().filter(|t| blanked(t)).flat_map(|t| [feature(INPUT_WORD, t), feature(TOKEN_TEXT, t), feature(VALUE_TEXT, t)]).collect();
        let kept: Vec<FeatureId> = ids.iter().copied().filter(|id| !gone.contains(id)).collect();
        changed |= kept.len() != ids.len();
        kept
    }).collect();
    changed.then_some(blank)
}
because!(blanked_items, CursorTree, "the feature places of a stack's slots with the text of every word the lesson states as a thing taken \
     off, the word heard, the token a step copied or found and the value it set, or nothing when no slot holds such a word");
