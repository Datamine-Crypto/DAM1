use super::physics::appeared;
use super::WordReading;
use crate::cursor::{names_class, CursorMind, BRACE_OPEN_TEXT, FOUND_BY, FOUND_TOP, NAMES_FEATURE, TOKEN_NODE, TOKEN_TEXT, WORD_PLACE};
use crate::events::{feature, heard_item, Item, ItemKind, Stack, INPUT_WORD, ITEM_KIND};
use crate::quiz::IS_FORM;
use crate::words::number_of;
use patterns::because;
use super::english::{function_word, WordEnglish, COPULA, LIST_MARK, PARTING_MARKS, WHEN_OPENER, QUOTE, LIST_HALVES, MEASURES, DIRECTIONS, ARTICLE_FLAGS, PLURAL_END, PLURAL_LEAST, HISSING_ENDS, LONG_PLURAL, SOFT_PLURAL, PLAIN_PLURALS, FLAG_DEFINITE, FLAG_INDEFINITE, FLAG_GIVE, VERB_ENDS, TAKING, ORDINALS, TIMES_OF_DAY, FLAG_ROLE, COMPARISON_END, WORD_CLASS, WORD_KIND, MARK_CLASS, SPEAKER_CLASS, OWN_CLASS, FILLER_CLASS, NEGATION_CLASS, LINK_CLASS, OPERATOR_CLASS, POSSESSIVE_CLASS, COPULA_CLASS, HAVING_CLASS, ARTICLE_CLASS, SKIPPED_CLASS, ASKS_CLASS, HELPER_CLASS, THING_PRONOUN_CLASS, PERSON_PRONOUN_CLASS, PLACE_CLASS, VERB_CLASS, MOVING_CLASS, PAST_CLASS, NUMBER_CLASS, THING_CLASS, VALUE_CLASS, OTHER_CLASS, CURSOR_KIND, CURSOR_HOLDING, CURSOR_ASKING, WORD_ASKED, CURSOR_FLAG, CURSOR_BY, FACT_BY, SCENE_BY, SCENE_END, RECORD_IN, WORLD_KIND, THING_KIND, VALUE_KIND, RELATION_KIND, PROPERTY_KIND, QUESTION_KIND, RECORD_OBJECT, RECORD_TYPE, RECORD_FLAGS, RECORD_HOLDING, WORD_ENDING, SEEN_ENDS, REPLY_RELATION, STORY_OPENER, AFTER_SIGHT, CURSOR_SUBJECT, CURSOR_NUMBER, CURSOR_QUOTING, ASKED_NTH, CURSOR_RELATION, CURSOR_UNDER, CURSOR_DONE, CURSOR_SIGHTS, SAID_SIGHTS, OPEN_CLASSES, WORD_SIGHTS, DOING_LETTERS};
use super::lookup::present_children;
use super::mind::{mark_word, place_word, comparison_known, verb_base, past_form, kind_of, is_number, word_classes};

pub fn closing_mark(word: &str) -> bool {
    mark_word(word) && !PARTING_MARKS.contains(&word)
}
because!(closing_mark, WordReading, "whether a word is a mark that ends a sentence, at which an open question is dumped and answered");

pub fn singular(word: &str) -> String {
    if let Some((_, one)) = PLAIN_PLURALS.iter().find(|(many, _)| *many == word) {
        return one.to_string();
    }
    if let Some(stem) = word.strip_suffix(LONG_PLURAL).filter(|stem| stem.len() >= PLURAL_LEAST && HISSING_ENDS.iter().any(|end| stem.ends_with(end))) {
        return stem.to_string();
    }
    if let Some(stem) = word.strip_suffix(SOFT_PLURAL.0).filter(|stem| stem.len() >= PLURAL_LEAST) {
        return format!("{stem}{}", SOFT_PLURAL.1);
    }
    match word.strip_suffix(PLURAL_END) {
        Some(stem) if stem.len() >= PLURAL_LEAST && !stem.ends_with(PLURAL_END) => stem.to_string(),
        _ => word.to_string(),
    }
}
because!(singular, WordEnglish, "the singular of a counted plural: the word without its plural ending when enough letters stay and the \
     stem is no plural itself, boxes as box and cookies as cooky, else the word as said");

pub fn verb_like(mind: &CursorMind, word: &str) -> bool {
    let loose = word_classes(mind, word).iter().all(|c| *c == OTHER_CLASS || *c == VALUE_CLASS || *c == THING_CLASS);
    verb_base(mind, word).is_some() || past_form(mind, word) || (loose && VERB_ENDS.iter().any(|end| word.len() >= end.len() + PLURAL_LEAST && word.ends_with(end)))
}
because!(verb_like, WordReading, "whether a word reads as a verb: one the world or the seeds know, a past form, or a word in no class, or \
     held by the seeds only as a thing or a value, with a verb's ending, sees or painted");

pub fn role_word(mind: &CursorMind, word: &str) -> bool {
    !place_word(word) && !ORDINALS.contains(&word) && !super::moves::KINDS.contains(&word) && mind.tree.named(&crate::cursor::step_item(word)).any(|n| !mind.tree.node(n).gone && mind.tree.node(n).children.iter().any(|&c| !mind.tree.node(c).gone && !mind.tree.node(c).name.starts_with(crate::quiz::BRACE_OPEN))) && verb_base(mind, word).is_none() && !verb_like(mind, word) && kind_of(mind, word).is_none()
}
because!(role_word, WordReading, "whether a word names a role of one thing for another: a relation with a plain value the world or the \
     seeds hold that is no verb, no place, no ordinal, no kind and no bare quality, capital, author or opposite, and not a part, the door \
     of the house");

pub fn noun_word(mind: &CursorMind, word: &str) -> bool {
    let classes = word_classes(mind, word);
    let named = mind.flags.last().is_some_and(|(k, _)| k == FLAG_DEFINITE || k == FLAG_INDEFINITE);
    let past = named && (classes.contains(&PAST_CLASS) || classes.contains(&THING_CLASS));
    let role = mind.held.is_empty() && mind.flags.last().is_some_and(|(k, _)| k == FLAG_ROLE);
    if role && !mark_word(word) && !COPULA.contains(&word) {
        return true;
    }
    let owned = !mind.held.is_empty() && mind.flags.last().is_some_and(|(k, _)| k == FLAG_GIVE) && !classes.contains(&PAST_CLASS) && !COPULA.contains(&word) && verb_base(mind, word).is_none_or(|b| !TAKING.contains(&b.as_str()));
    let classes: Vec<_> = classes.into_iter().filter(|c| !(owned && (*c == VERB_CLASS || *c == MOVING_CLASS))).collect();
    let classes: Vec<_> = classes.into_iter().filter(|c| !(past && (*c == PAST_CLASS || *c == VERB_CLASS)) && !(role && (*c == FILLER_CLASS || *c == PLACE_CLASS || *c == VERB_CLASS || *c == PAST_CLASS))).collect();
    !classes.iter().any(|c| [MARK_CLASS, LINK_CLASS, NEGATION_CLASS, OWN_CLASS, OPERATOR_CLASS, FILLER_CLASS, SPEAKER_CLASS, POSSESSIVE_CLASS, COPULA_CLASS, HAVING_CLASS, ARTICLE_CLASS, SKIPPED_CLASS, ASKS_CLASS, HELPER_CLASS, THING_PRONOUN_CLASS, PERSON_PRONOUN_CLASS, PLACE_CLASS, NUMBER_CLASS, VERB_CLASS, PAST_CLASS].contains(c))
}
because!(noun_word, WordReading, "whether a word can name a thing: any word in no closed class and no verb or number, or a past form or a \
     verb the seeds also hold as a thing right after an article, a saw is a tool, a cook uses a pot, or a verb's form right after having \
     that is no copula and no taking, kim's walk, a tree has leaves, or any word but a copula right after a waiting role, the successor of \
     x, a filler, a place word or a verb's form among them, the opposite of up, the opposite of open, a quality counting too, since red \
     alone at the world is the thing red");

pub fn node_kind(mind: &CursorMind, at: usize) -> &'static str {
    if at == 0 {
        return WORLD_KIND;
    }
    if open_question(mind) == Some(at) {
        return QUESTION_KIND;
    }
    let name = &*mind.tree.node(at).name;
    let parent = mind.tree.node(at).parent;
    let is = IS_FORM.trim();
    if name.starts_with(BRACE_OPEN_TEXT) {
        if *name == *is || *mind.tree.node(parent).name == *is {
            PROPERTY_KIND
        } else {
            RELATION_KIND
        }
    } else if parent == 0 || !mind.tree.node(parent).name.starts_with(BRACE_OPEN_TEXT) {
        THING_KIND
    } else {
        VALUE_KIND
    }
}
because!(node_kind, WordReading, "what kind of node the cursor stands on: the world at the root, the question for the open question node, \
     a property for is or a kind under it, a relation for any other braced node, a thing for a plain node under the world or inside \
     another thing, and a value under a relation");

fn scene_nodes(mind: &CursorMind, word: &str) -> Vec<usize> {
    let asked = open_question(mind);
    let said = |n: usize| {
        let name = crate::cursor::bare_name(&mind.tree.node(n).name);
        std::iter::once(word).chain(mind.before.iter().map(String::as_str)).any(|w| *w == *name || singular(w) == singular(&name))
    };
    let told: Vec<usize> = (mind.tree.state..mind.tree.len())
        .filter(|&n| !mind.tree.node(n).gone && mind.tree.story(n))
        .filter(|&n| asked.is_none_or(|q| n != q && mind.tree.node(n).parent != q))
        .collect();
    let about: Vec<usize> = told.iter().copied().filter(|&n| said(n)).collect();
    let mut near: Vec<usize> = Vec::new();
    for &n in &about {
        let up = mind.tree.node(n).parent;
        for step in std::iter::once(n).chain((up != 0).then_some(up)).chain(present_children(mind, n)) {
            if !near.contains(&step) && step >= mind.tree.state {
                near.push(step);
            }
        }
        for kid in present_children(mind, n) {
            for under in present_children(mind, kid) {
                if !near.contains(&under) {
                    near.push(under);
                }
            }
        }
    }
    near.sort_unstable();
    near.into_iter().rev().take(crate::events::SCENE_SLOTS).rev().collect()
}
because!(scene_nodes, WordReading, "the nodes of the scene laid out before a word: the things the sentence has named so far, each with \
     what holds it, what it holds and the values under those, so the reader sees the shape it must choose a get by, whether the thing \
     holds the relation asked, whether what it holds is an activity, whether the answer stands two steps up; what the story wrote and \
     this sentence never named is left out, since a scene of everything written lately shows the network which lesson line it is on and \
     it reads the line instead of the shape, and the words of an open question are left out, being what he is asked");

fn scene_item(mind: &CursorMind, at: usize, word: &str) -> Item {
    let name = mind.tree.node(at).name.trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE).to_string();
    let kind = node_kind(mind, at);
    let parent = mind.tree.node(at).parent;
    let holder = if parent == 0 { WORLD_KIND.to_string() } else { mind.tree.node(parent).name.trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE).to_string() };
    let mut ids = vec![feature(ITEM_KIND, SCENE_BY), feature(WORD_KIND, kind), feature(RECORD_IN, node_kind(mind, parent))];
    if mind.at == at {
        ids.push(feature(SCENE_BY, CURSOR_BY));
    }
    if mind.first_mark == Some(at) {
        ids.push(feature(SCENE_BY, CURSOR_SUBJECT));
    }
    if mind.held.contains(&at) {
        ids.push(feature(SCENE_BY, CURSOR_HOLDING));
    }
    if mind.tree.node(at).link.is_some() {
        ids.push(feature(SCENE_BY, NAMES_FEATURE));
    }
    let same = |said: &str| *said == *name || singular(said) == singular(&name);
    if same(word) {
        ids.push(feature(SCENE_BY, INPUT_WORD));
    }
    if mind.before.iter().any(|said| same(said)) {
        ids.push(feature(SCENE_BY, SAID_SIGHTS[0]));
    }
    let record = format!(
        "{}{SCENE_BY} {RECORD_OBJECT}: {name} {RECORD_TYPE}: {kind} {RECORD_IN}: {holder}{}",
        crate::quiz::BRACE_OPEN,
        crate::quiz::BRACE_CLOSE
    );
    Item { kind: ItemKind::Found, ids, text: record.into(), token: None, value: None, bound: None, value_bound: None, node: Some(at) }
}
because!(scene_item, WordReading, "one node of the scene as the network reads it: its kind, the kind of what holds it, whether the cursor \
     stands on it, whether it is the subject of the sentence, whether he holds it and whether it names another node, whether the word now heard names it and whether a word already said in this sentence names it, written for the user \
     as the record scene object: corn type: thing in: bag, and carrying the node itself so a step may go to the very thing he is talking \
     about; what a node is called is no feature of its event, since a scene that says john would be read by the name and not by the shape, \
     and every held-out line names things the learned lines never name");

fn class_or_text<'a>(mind: &CursorMind, word: &'a str) -> &'a str {
    let classes = word_classes(mind, word);
    if classes.iter().any(|class| OPEN_CLASSES.contains(class)) { classes[0] } else { word }
}
because!(class_or_text, WordReading, "a word as the reader is told of it where it is not the word being read: by its class where a \
     lesson may swap it for another, else by its text, so a lesson that says one word of a time of day is read as the next lesson \
     that says another; the relation the cursor stands on keeps its text, a verb of the seeds being a thing of theirs as well and to go \
     meaning no what to love means");

fn cursor_sight(mind: &CursorMind) -> [bool; CURSOR_SIGHTS.len()] {
    [super::physics::holds_asked(mind), super::physics::holds_doing(mind), super::physics::held_under(mind), super::physics::owned_thing(mind), super::physics::reaches_through(mind), super::physics::manners_said(mind), super::physics::holds_said(mind), super::physics::owned_twice(mind), super::physics::ranked_kind(mind)]
}
because!(cursor_sight, WordReading, "what the story shows of the thing the cursor stands on, whether it holds a relation the sentence \
     names, holds a doing, stands under a relation the sentence names, has an owner, is two steps from the relation the \
     sentence names, or stands where a word of manners was said, 
     so the network chooses a get of a relation by \
     what the teacher's rules choose it by");

pub(super) fn cursor_item(mind: &CursorMind) -> Item {
    let at = mind.at;
    let text = if at == 0 { crate::cursor::CURSOR_NOTHING.to_string() } else { mind.tree.node(at).name.trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE).to_string() };
    let mut ids = vec![feature(ITEM_KIND, ItemKind::Found.name()), feature(FOUND_BY, CURSOR_BY), feature(CURSOR_KIND, node_kind(mind, at))];
    if at != 0 {
        ids.push(feature(TOKEN_NODE, true));
        if let Some(named) = names_class(mind, &text) {
            ids.push(feature(NAMES_FEATURE, named));
        }
        if mind.tree.node(at).parent == 0 {
            ids.push(feature(FOUND_TOP, true));
        }
        if mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) {
            ids.push(feature(CURSOR_RELATION, &text));
        }
        let above = mind.tree.node(at).parent;
        if above != 0 && mind.tree.node(above).name.starts_with(BRACE_OPEN_TEXT) {
            ids.push(feature(CURSOR_UNDER, crate::cursor::bare_name(&mind.tree.node(above).name)));
        }
        if super::physics::clause_done(mind) {
            ids.push(feature(CURSOR_DONE, true));
        }
        if mind.first_mark == Some(at) {
            ids.push(feature(CURSOR_SUBJECT, true));
        }
    }
    let held = mind.held.last().copied().filter(|&h| !mind.tree.node(h).gone);
    if held.is_some() {
        ids.push(feature(CURSOR_HOLDING, true));
    }
    if mind.number.is_some() {
        ids.push(feature(CURSOR_NUMBER, true));
    }
    if let Some(question) = open_question(mind) {
        ids.push(feature(CURSOR_ASKING, true));
        let quotes = mind.tree.node(question).children.iter().filter(|&&c| !mind.tree.node(c).gone && *mind.tree.node(c).name == *QUOTE).count();
        if quotes % LIST_HALVES == 1 {
            ids.push(feature(CURSOR_QUOTING, true));
        }
    }
    let mut flags: Vec<&str> = mind.flags.iter().map(|(k, _)| k.as_str()).collect();
    flags.dedup();
    for flag in &flags {
        ids.push(feature(CURSOR_FLAG, *flag));
    }
    for (place, seen) in CURSOR_SIGHTS.iter().zip(cursor_sight(mind)) {
        if seen {
            ids.push(feature(place, true));
        }
    }
    let nothing = crate::cursor::CURSOR_NOTHING;
    let record = format!(
        "{}{} {RECORD_OBJECT}: {text} {RECORD_TYPE}: {} {RECORD_FLAGS}: {} {RECORD_HOLDING}: {}{}",
        crate::quiz::BRACE_OPEN,
        CURSOR_BY,
        node_kind(mind, at),
        if flags.is_empty() { nothing.to_string() } else { flags.join(" ") },
        held.map_or_else(|| nothing.to_string(), |h| mind.tree.node(h).name.to_string()),
        crate::quiz::BRACE_CLOSE
    );
    Item { kind: ItemKind::Found, ids, text: record.into(), token: None, value: None, bound: None, value_bound: None, node: (at != 0).then_some(at) }
}
because!(cursor_item, WordReading, "the event that says where the cursor stands when a word arrives: its kind, what its node names, \
     whether it is a thing under the world, whether he holds something, whether a question is open and which flags are set, written as the \
     user's record, cursor object: corn type: thing flags: in holding: corn, since the stack is emptied at every word and this is all the \
     network is told of the tree");

pub fn heard_word(mind: &mut CursorMind, word: &str, index: usize) {
    if index == 0 {
        mind.before.clear();
        mind.last_topic = mind.first_mark.filter(|&t| t < mind.tree.len() && !mind.tree.node(t).gone);
    }
    let opener_said = mind.before.iter().filter(|said| *said != LIST_MARK).map(String::as_str).eq(STORY_OPENER);
    if opener_said && word != LIST_MARK {
        mind.before.clear();
    }
    let clause_opened = mind.before.last().is_some_and(|b| b == WHEN_OPENER) && (ARTICLE_FLAGS.contains(&word) || noun_word(mind, word) && !COPULA.contains(&word));
    if let Some(question) = open_question(mind).filter(|&q| clause_opened && *mind.tree.node(q).name == *WHEN_OPENER) {
        mind.tree.moved(question, None);
        mind.question_start = None;
        mind.at = 0;
    }
    if index == 0 {
        mind.tree.settled();
        mind.sentence_from = mind.tree.len();
        mind.cause_of = None;
        mind.output.clear();
        mind.places = None;
        mind.ended = false;
        mind.at = 0;
        mind.held.clear();
        mind.flags.clear();
        mind.second_mark = None;
        if let Some(question) = open_question(mind) {
            mind.tree.moved(question, None);
        }
        mind.question_start = None;
    }
    mind.stack = Stack::default();
    for node in scene_nodes(mind, word) {
        let told = scene_item(mind, node, word);
        mind.stack.push(told);
    }
    mind.stack.push(Item { kind: ItemKind::Mark, text: SCENE_END.into(), ids: vec![feature(ITEM_KIND, SCENE_END)], token: None, value: None, bound: None, value_bound: None, node: None });
    let cursor = cursor_item(mind);
    mind.stack.push(cursor);
    let mut item = sighted_item(mind, word);
    let last_said = mind.before.len().checked_sub(1);
    for (nth, said) in mind.before.iter().enumerate() {
        let seen: &str = class_or_text(mind, said);
        let places = [Some(feature(SAID_SIGHTS[0], seen)), (Some(nth) == last_said).then(|| feature(crate::cursor::WORD_AFTER, seen))];
        for place in places.into_iter().flatten() {
            if !item.ids.contains(&place) {
                item.ids.push(place);
            }
        }
        if Some(nth) == last_said && !mark_word(said) {
            for (place, seen) in WORD_SIGHTS.iter().zip(word_sight(mind, said)) {
                if seen {
                    item.ids.push(feature(AFTER_SIGHT, place));
                }
            }
        }
    }
    item.ids.push(feature(WORD_PLACE, index));
    mind.stack.push(item);
    if closing_mark(word) {
        if let Some(question) = open_question(mind) {
            let asked: Vec<usize> = std::iter::once(question).chain(mind.tree.node(question).children.iter().copied().filter(|&c| !mind.tree.node(c).gone)).collect();
            for node in asked {
                let text = mind.tree.node(node).name.to_string();
                let mut dumped = sighted_item(mind, &text);
                dumped.ids.push(feature(WORD_ASKED, true));
                dumped.ids.push(feature(ASKED_NTH, mind.stack.items().filter(|e| e.item.kind == ItemKind::Input).count()));
                mind.stack.push(dumped);
            }
        }
    }
    if let Some(node) = appeared(mind, word) {
        let text = mind.tree.node(node).name.to_string();
        let fact = found_word_item(mind, &text, FACT_BY, Some(node));
        mind.stack.push(fact);
        mind.at = node;
    }
    mind.before.push(word.to_string());
}
because!(heard_word, WordReading, "one word put on the stack, the opening of a tale forgotten at the first word after it so the sentence is read as begun there, the topic of the sentence before kept at the first word of a new one, and \
     kept among the words said before in the sentence once its facts are done: an article or a noun right after when shows the when opened \
     a clause and asked nothing, so its question is taken away first and the sentence is read as told; the stack is emptied at every word, \
     as the user asked, so the tree, the cursor, what he holds and the flags are all that carries from one word to the next; the cursor's \
     event goes on first, then the word with its form, its text, every class it is in, the kind of a quality and how many words came \
     before it; then the physics of the word, a thing word making the thing appear, which is pushed after the word and the cursor moved \
     onto it; at the question mark of an open question the question node and the words under it are dumped on the stack, each with its \
     classes, so the network sees the question it must answer; at the first word of a sentence the output is cleared, the cursor put back \
     at the world, what he held and the flags dropped and the question of the sentence before taken out");

pub fn open_question(mind: &CursorMind) -> Option<usize> {
    mind.question_start.filter(|&q| q < mind.tree.len() && !mind.tree.node(q).gone)
}
because!(open_question, WordReading, "the open question node, kept on the mind since it is named by its first word, what or where, and \
     nothing else in the tree marks it; none when no question is open");

fn asked_now(mind: &CursorMind) -> Vec<String> {
    open_question(mind).map(|q| mind.tree.node(q).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default()
}
because!(asked_now, WordReading, "the words the open question holds so far, or none");

fn unseen_doing(word: &str) -> bool {
    word.len() > DOING_LETTERS && word.ends_with(PLURAL_END) && word.chars().all(char::is_alphabetic) && !function_word(word) && !TIMES_OF_DAY.contains(&word)
}
because!(unseen_doing, WordReading, "whether a word in no class looks like a word of doing the seeds never state, orbits or hunts: long enough, letters \
     only, with the ending of doing, and no word of a closed class or time of day, since means defines a name by being itself and \
     a plural noun with its name taken away would read as it does; such a word loses its text in the view, since a learned one would \
     be read by its text and the next unseen one could not be, while every other word in no class keeps its text, which the teacher may read");

fn sighted_item(mind: &CursorMind, word: &str) -> Item {
    let mut item = heard_item(word);
    let classes = word_classes(mind, word);
    let many_of = { let one = singular(word); one != word && mind.tree.named(word).all(|n| n == 0 || mind.tree.node(n).gone || mind.tree.node(n).parent != 0) };
    let plain_thing = classes == [THING_CLASS] && !function_word(word) && !ORDINALS.contains(&word) && !comparison_known(mind, word) && !MEASURES.contains(&word) && !word.ends_with(COMPARISON_END) && !DIRECTIONS.contains(&word) && !many_of && !super::physics::told_thing(mind, word) && !mind.before.iter().any(|said| is_number(mind, said).is_some() || ORDINALS.contains(&said.as_str()));
    if (classes == [OTHER_CLASS] && unseen_doing(word)) || plain_thing {
        let written = feature(crate::events::INPUT_WORD, &*item.text);
        item.ids.retain(|id| *id != written);
    }
    for class in classes {
        item.ids.push(feature(WORD_CLASS, class));
    }
    if let Some(kind) = kind_of(mind, word) {
        item.ids.push(feature(WORD_KIND, &*kind));
    }
    if mark_word(word) {
        return item;
    }
    if let Some(end) = SEEN_ENDS.iter().find(|end| word.len() > end.len() + 1 && word.ends_with(**end) && number_of(word).is_none()) {
        item.ids.push(feature(WORD_ENDING, end));
    }
    for (place, seen) in WORD_SIGHTS.iter().zip(word_sight(mind, word)) {
        if seen {
            item.ids.push(feature(place, true));
        }
    }
    item
}
because!(sighted_item, WordReading, "the event of a word with what the reader sees of it: its classes, its kind, and what the story shows \
     of it, so the network reads a word by what the teacher's rules read it by; a word of doing never seen loses its text, and so does a word that is only a thing the seeds know, not a word of a closed class or an ordinal, which the teacher reads by its text, a bit hot and first tom went, and so must the network, not a comparison, a measure, a direction or a plural the question counts, not a thing the story told, and not in a sentence that says a number or a place in an order, where a walk of numbers may turn on the word, the cube of three, so the network reads each by its class and sights as it must read the next unseen one, what is cheese as what is bread");

fn word_sight(mind: &CursorMind, word: &str) -> [bool; WORD_SIGHTS.len()] {
    [super::physics::told_thing(mind, word), super::physics::value_told(mind, word), super::physics::told_relation(mind, word), super::physics::person_named(mind, word), super::physics::activity_named(mind, word), super::physics::named_result(mind, word).is_some(), super::physics::measure_asked(mind, word).is_some_and(|(measure, _)| super::physics::told_relation(mind, &measure)), super::physics::thing_number(mind, word, &asked_now(mind)).is_some(), role_word(mind, word), super::physics::told_with_article(mind, word), super::physics::has_relation(mind, word, REPLY_RELATION), super::physics::seeded_class(mind, word)]
}
because!(word_sight, WordReading, "what the story shows of a word, whether it told a thing of that name, holds it as a value, told it as a \
     relation, knows it as a person or a holder, as an activity, as a result a sum named, as a comparison of a measure it told, as a thing \
     that is worth a number, as a role of one thing for another, as a thing the story told with an article or told what it is, or as a \
     word of manners the seeds give a reply, or as a thing the seeds say what it is, so the network reads a word by what the teacher's rules read it by");

pub fn found_word_item(mind: &CursorMind, text: &str, by: &str, node: Option<usize>) -> Item {
    let mut ids = vec![feature(ITEM_KIND, ItemKind::Found.name()), feature(FOUND_BY, by)];
    match node.and_then(|_| names_class(mind, text)) {
        Some(named) => ids.push(feature(NAMES_FEATURE, named)),
        None => ids.push(feature(TOKEN_TEXT, text)),
    }
    if let Some(n) = node {
        ids.push(feature(TOKEN_NODE, true));
        let tree = &mind.tree;
        if tree.node(n).parent == 0 {
            ids.push(feature(FOUND_TOP, true));
        }
        if tree.node(n).name.starts_with(BRACE_OPEN_TEXT) {
            ids.push(feature(NAMES_FEATURE, crate::cursor::NAMES_RELATION));
        } else if n != 0 {
            for (place, seen) in super::physics::FOUND_SIGHTS.iter().zip(super::physics::found_sight(mind, n)) {
                if seen {
                    ids.push(feature(place, true));
                }
            }
        }
    }
    Item { kind: ItemKind::Found, ids, text: text.into(), token: None, value: None, bound: None, value_bound: None, node }
}
because!(found_word_item, WordReading, "the event of what a step found or a word made appear: what brought it, the text or what it names, \
     and for a node of the tree that it is one, whether it stands under the world and whether it is a relation, and for a thing what is \
     seen of it, its place, its owner and whether the question asks of it");
