mod estimate;
mod figures;
mod mind;
mod moves;
mod plan;
mod rows;
mod statements;
mod tree;

pub use estimate::{QuestionWords, output_answers};
pub use figures::CursorPhysics;
pub use mind::{CursorMind, InputLooks, settled};
pub use moves::CursorMoves;
pub use plan::{NumberPlanning, SearchTrace};
pub use rows::CursorRow;
pub use statements::{ConceptSeeds, SeedsFiles, cursor_told, told_seeds};
pub use tree::{ConceptTags, NodeTree, TimeSeeds, TreeIndexes, TreeNode};
pub(crate) use figures::PERCENT_WHOLE;
pub(crate) use mind::{names_class, worked_text};
pub(crate) use moves::{CursorMove, FRACTION_WORDS, bare_name, step_item};
pub(crate) use plan::{held_operand, number_plan_over};
pub(crate) use rows::blanked_items;
pub(crate) use statements::EQUAL_RELATION;
pub(crate) use tree::{child_named, told_past, verb_of};

use crate::events::{Stack, feature};
use crate::network::FeatureId;
use patterns::{because, source};

pub struct CursorTree;
source!(
    CursorTree,
    "the user's mind of the network: a tree of nodes that starts at the world, the network's permanent memory, a cursor that stands on one of its nodes, and a stack of the newest events, the words heard and every step with what it found, which is all the network sees of what it did; a teacher writes the steps for every input down as the rows the network learns from"
);

pub(crate) const BRACE_OPEN_TEXT: &str = "{";
because!(BRACE_OPEN_TEXT, CursorTree, "the brace a relation's name opens with, as text, so the estimate tells a relation word from a thing \
     word by the nodes the tree holds without the list of relations the console passes");

pub(crate) const CURSOR_WORLD: &str = "{world}";
because!(CURSOR_WORLD, CursorTree, "the name of the tree's root, where the cursor starts, written in braces so no word is taken for it");

pub(crate) const CURSOR_NOTHING: &str = crate::quiz::NOTHING;
because!(CURSOR_NOTHING, CursorTree, "what a step that finds no node puts on the stack, written in braces so no word is taken for it");

pub(crate) const QUANTITY: &str = "{quantity}";

pub(crate) const QUANTITY_TAG: &str = "{quantity ";
because!(QUANTITY_TAG, CursorTree, "the opening of a quantity tag, the count of a thing as one node with its number inside, so the tree \
     holds how many of a thing there are as a property of the count and never as a child of it, as the user asked");

pub(crate) fn is_quantity_tag(name: &str) -> bool {
    name.starts_with(QUANTITY_TAG) && crate::words::number_of(name).is_some()
}
because!(is_quantity_tag, CursorTree, "whether a name is a quantity tag, a quantity with its number");

pub(crate) fn quantity_tag_named(count: f32) -> String {
    format!("{QUANTITY_TAG}{}{}", mind::worked_text(count, None), crate::quiz::BRACE_CLOSE)
}
because!(quantity_tag_named, CursorTree, "the quantity tag of a count, the quantity word and the number in one braced token");
because!(QUANTITY, CursorTree, "the relation under a thing that holds how many of it there are, its count as a property, a braced token so \
     no word of a text is taken for it");

pub(crate) const TIME_RELATION: &str = "{time}";
because!(TIME_RELATION, CursorTree, "the relation of a value that holds when the fact was so: past for what is no longer so or happened at \
     a time the text never names, or the named time the text says, held on the value's own node even when the value links to a thing, \
     since the time belongs to the fact and not to the thing; a value with no time is so now, so a fact that is no longer so stays where \
     it happened on the same node and the ball someone had is the ball someone else has");

pub(crate) const PAST_TIME: &str = "{past}";
because!(PAST_TIME, CursorTree, "the time of a fact that was so and is no longer, or of a deed a text tells in the past without naming its \
     time, since such a fact is earlier than now while its distance is unknown");

pub(crate) const LATER_TIME: &str = "{later}";
because!(LATER_TIME, CursorTree, "the time of a fact a text tells will be so, since such a fact is later than now while its distance is \
     unknown, so what will be is never met by a question about now");

pub(crate) const FUTURE_FORM: &str = "{future}";
because!(FUTURE_FORM, CursorTree, "the relation under a verb the seeds state, holding the form a text tells the future with, will under \
     is, so a sentence or a question that says such a form is known to tell or ask about what will be");

pub(crate) const PAST_FORM: &str = "{past}";
because!(PAST_FORM, CursorTree, "the relation under a verb the seeds state, holding the form a story tells the past with, met under meet \
     and had under has, so a sentence or a question that says such a form is known to tell or ask about the past");

pub(crate) const QUOTING: [&str; 4] = ["{say}", "{think}", "{believe}", "{want}"];
because!(QUOTING, CursorTree, "the relations under a sayer that hold what the sayer said, thinks, believes or wants as a path of plain \
     values, never linked to the things the values name, so a claim about a thing is held by the sayer while the fact about the thing \
     stands on its own, a question of whether the sayer was right compares the two, and a belief or a wish is the same structure as a \
     claim");

pub(crate) const ANSWER_RELATION: &str = "{answer}";
because!(ANSWER_RELATION, CursorTree, "the relation under a question node that holds what the network answered, written by the answering \
     step, so a later turn finds the last answer in the tree once the stack is emptied, as a bare sign and number typed after a sum carry \
     the sum on from the number just answered");

pub(crate) const QUESTION_NODE: &str = "question";
because!(QUESTION_NODE, CursorTree, "the thing under the world that holds what the newest question asks, written by the network as the \
     words come in and dropped at the next input's start, so a question is a structure in the tree like any fact, the stack may be cleared \
     once it is written, and an input of any length reads through the tree");

pub(crate) const ASKS_FORM: &str = "{form}";
because!(ASKS_FORM, CursorTree, "the relation of the question node that holds the question's first word, who, what, where, how, is or \
     does, which says what kind of answer it wants, and its second when the first is how or which");

pub(crate) const NAMES_NUMBER: &str = "number";
because!(NAMES_NUMBER, CursorTree, "the names feature of a word or token that is a number, whose text the network never sees, since its \
     form, its last digit and how many digits it has are the pattern and the physics works the value");

pub(crate) const FOUND_TOP: &str = "cursor.top";
because!(FOUND_TOP, CursorTree, "whether a node a step found stands right under the world, as a feature of its event, so the network tells \
     a thing from a value");

pub(crate) const TOKEN_TEXT: &str = "cursor.token";
because!(TOKEN_TEXT, CursorTree, "the text of a token a step named, copied or found, as a feature of its event, so a network sees which \
     token it was; a copied or found token that names a node carries what it names instead");

pub(crate) const VALUE_TEXT: &str = "cursor.value";
because!(VALUE_TEXT, CursorTree, "the text of the value a property step named or copied, as a feature of its event, so a network sees \
     which value it was; a copied value that names a node carries what it names instead");

pub(crate) const TOKEN_NODE: &str = "cursor.node";
because!(TOKEN_NODE, CursorTree, "whether a token a step found is a node of the tree, as a feature of its event, so the network sees which \
     slots a step that goes to a node may take");

pub(crate) const FOUND_BY: &str = "cursor.found_by";
because!(FOUND_BY, CursorTree, "the move that found a token, as a feature of its event, so a pointer tells apart tokens it never learned \
     by the step that brought them");

const SLOT_PLACE: &str = "cursor.slot";
because!(SLOT_PLACE, CursorTree, "how deep a slot stands on the stack when the network reads it, the newest at nothing, as a feature of \
     the slot and not of its event, since an event goes deeper with every push, so a pointer tells apart tokens it never learned by how \
     recent they are");

pub fn place_feature(depth: usize) -> FeatureId {
    feature(SLOT_PLACE, depth)
}
because!(place_feature, CursorTree, "the feature place of a slot's depth alone, so a trainer that keeps the events of a stack once and \
     reads each row as a window over them adds the depth of every slot itself");

pub(crate) fn slot_features(stack: &Stack) -> Vec<Vec<FeatureId>> {
    stack.slots().enumerate().map(|(depth, e)| e.item.ids.iter().copied().chain(std::iter::once(feature(SLOT_PLACE, depth))).collect()).collect()
}
because!(slot_features, CursorTree, "the feature places a network reads for every slot of a stack, the newest first: the places of the \
     slot's event and the place of the slot's depth, the same for a taught row and for a network choosing a step");

pub(crate) const WORD_PLACE: &str = "cursor.place";
because!(WORD_PLACE, CursorTree, "how many words of the input came before a word, as a feature of its event, so a pointer tells apart \
     words it never learned by where they stand");

pub(crate) const WORD_AFTER: &str = "cursor.after";
because!(WORD_AFTER, CursorTree, "the word heard just before a word in its input, or what that word names when it names a node, as a \
     feature of its event, so a pointer tells apart words it never learned by the word they follow, as a thing follows the or a");

pub(crate) const NAMES_FEATURE: &str = "cursor.names";
because!(NAMES_FEATURE, CursorTree, "what a word heard or a token copied or found names in the tree, as a feature of its event in place of \
     its text: a thing, a value, a kind or a relation, so the network learns the pattern of a kind word or a thing word and never one \
     word's context, as color and never that cars have a color; a word that names nothing keeps its text, since the question words and the \
     function words are the pattern");

pub(crate) const NAMES_THING: &str = "thing";
because!(NAMES_THING, CursorTree, "the names feature of a word or token whose name stands right under the world");

pub(crate) const NAMES_VALUE: &str = "value";
because!(NAMES_VALUE, CursorTree, "the names feature of a word or token whose name the tree holds only as a value under a relation");

pub(crate) const NAMES_KIND: &str = "kind";
because!(NAMES_KIND, CursorTree, "the names feature of a word or token whose every node stands under is, as color under red");

pub(crate) const NAMES_RELATION: &str = "relation";
because!(NAMES_RELATION, CursorTree, "the names feature of a word a braced relation of the tree is written from, as older for {older}");

pub const PATH_MARK: &str = " -> ";
because!(PATH_MARK, CursorTree, "what joins the names along a path of the tree when a person reads it, as the user writes one");

pub(crate) const NODE_KINDS: usize = 2;
because!(NODE_KINDS, CursorTree, "how many kinds of node a path alternates between, a thing or value and a relation, so every other name \
     of a path names a thing and a node an odd number of nodes below the world is a relation");

pub(crate) const BARE_STATEMENT_NODES: usize = 2;
because!(BARE_STATEMENT_NODES, CursorTree, "how many names a statement of a bare quality takes, the thing and the relation with nothing \
     under it, as the dog to big, since a quality is its own relation and needs no value, as the user decided");

pub(crate) const STATEMENT_NODES: usize = 3;
because!(STATEMENT_NODES, CursorTree, "how many nodes an expected statement of one relation takes in the tree, one after another: the \
     older name, the relation and the newer name, which is also the fewest a path statement names");

pub(crate) const SHARED_STEM: usize = 3;
because!(SHARED_STEM, CursorTree, "how many letters an answer must at least share with the start of a word the question heard before the \
     teacher offers the letters to drop from that word and the ending to add: as many as the shortest word of the quiz shares with its \
     plural, as cat with cats and bus with buses, so a name that only starts like a word, as kim like kite, is never offered as built from \
     it");
