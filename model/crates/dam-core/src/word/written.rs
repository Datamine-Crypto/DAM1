use super::mind::{open_question, flag_of, kind_of, landed_on, FLAG_INDEFINITE, FLAG_PROPERTY, FLAG_QUANTITY, FLAG_DEFINITE, FLAG_TIME};
use super::moves::WordMove;
use super::WordReading;
use crate::cursor::{quantity_tag_named, step_item, CursorMind, BRACE_OPEN_TEXT, PAST_TIME, TIME_RELATION};
use crate::quiz::{BRACE_CLOSE, BRACE_OPEN, IS_FORM};
use crate::words::number_of;
use patterns::because;
use super::lookup::story_node;
use super::physics::OWNER_TAG;
use super::physics::{TRUE_TAG, ANSWER_TAG, PAIR_LEAST};
use super::lookup::{present_children, own_child, kept_flag, person, seen, owner_of, time_of_day};

pub(super) fn thing_made(mind: &mut CursorMind, name: &str) -> usize {
    let thing = mind.tree.added(0, name);
    mind.tree.adopted(thing);
    thing
}
because!(thing_made, WordReading, "a thing made under the world, its own node even when the seeds know a thing of that name, since a story \
     that says the car is red states its own car");

pub(super) fn added_under(mind: &mut CursorMind, under: usize, name: &str, alone: bool) -> usize {
    if let Some(had) = own_child(mind, under, name) {
        return had;
    }
    if alone {
        for old in present_children(mind, under) {
            mind.tree.moved(old, None);
        }
    }
    mind.tree.added(under, name)
}
because!(added_under, WordReading, "a child added under a node, or the child of that name it already holds, and for a value that stands \
     alone, a time, the values it replaces taken out");

pub(super) fn added_value(mind: &mut CursorMind, under: usize, name: &str) -> usize {
    if let Some(had) = own_child(mind, under, name) {
        return had;
    }
    match mind.tree.referred(name) {
        Some(thing) if *mind.tree.node(under).name != *TIME_RELATION => mind.tree.linked(under, thing, false),
        _ => mind.tree.added(under, name),
    }
}
because!(added_value, WordReading, "a value added under a relation, or the value of that name it already holds: a name that is a thing \
     under the world is written as a mention linked to that thing, so a fact about a thing said as a value is held by the thing");

pub(super) fn flagged(mind: &mut CursorMind, node: usize) {
    let flags = std::mem::take(&mut mind.flags);
    mind.flags = flags.iter().filter(|(k, _)| kept_flag(k)).cloned().collect();
    for (key, value) in &flags {
        match key.as_str() {
            key if key == super::mind::FLAG_OWNED => {
                let of = super::mind::OWN_WORDS.iter().find(|(own, _)| *own == value).map_or_else(|| value.clone(), |(_, of)| (*of).to_string());
                let owner = if super::mind::SELF_WORDS.contains(&of.as_str()) || super::mind::YOU_WORDS.contains(&of.as_str()) { super::mind::USER_NAME.to_string() } else { of };
                let person = story_node(mind, &owner).unwrap_or_else(|| thing_made(mind, &owner));
                let tag = added_under(mind, node, OWNER_TAG, false);
                mind.tree.linked(tag, person, false);
            }
            FLAG_DEFINITE | FLAG_INDEFINITE => {
                let tag = added_under(mind, node, &step_item(key), false);
                added_under(mind, tag, TRUE_TAG, true);
            }
            FLAG_TIME => {
                let tag = added_under(mind, node, TIME_RELATION, false);
                added_under(mind, tag, &format!("{BRACE_OPEN}{value}{BRACE_CLOSE}"), true);
            }
            FLAG_QUANTITY => {
                if let Some(count) = number_of(value) {
                    added_under(mind, node, &quantity_tag_named(count), false);
                }
            }
            super::mind::FLAG_DEGREE if !flags.iter().any(|(k, _)| k == FLAG_PROPERTY) => {
                let degree = added_under(mind, node, &step_item(super::mind::FLAG_DEGREE), false);
                added_under(mind, degree, value, true);
            }
            FLAG_PROPERTY => {
                let is = added_under(mind, node, IS_FORM.trim(), false);
                let under = match kind_of(mind, value) {
                    Some(kind) => added_under(mind, is, &step_item(&kind), false),
                    None => is,
                };
                let quality = added_under(mind, under, value, false);
                if let Some((_, degree)) = flags.iter().find(|(k, _)| k == super::mind::FLAG_DEGREE).filter(|(_, degree)| *degree != *value) {
                    let how = added_under(mind, quality, &step_item(super::mind::FLAG_DEGREE), false);
                    added_under(mind, how, degree, true);
                }
            }
            _ => {}
        }
    }
}
because!(flagged, WordReading, "a node made takes the flags and clears them, all but the give, which the drop still reads: the article as \
     a tag with true under it, the time as its time relation with the past braced, the count as its quantity tag, and each quality said \
     before the noun as a value under is, under the kind the seeds class it by when they class it");

pub(super) fn written_out(mind: &mut CursorMind, by: WordMove, text: Option<String>) {
    let text = text.unwrap_or_else(|| crate::cursor::CURSOR_NOTHING.to_string());
    if text != crate::cursor::CURSOR_NOTHING {
        let repeats = by == WordMove::GetRelation && open_question(mind).is_some_and(|q| present_children(mind, q).into_iter().any(|c| super::mind::reply_word(mind, &mind.tree.node(c).name)));
        if !repeats {
            let last = added_under(mind, 0, ANSWER_TAG, false);
            if mind.output.is_empty() {
                for old in present_children(mind, last) {
                    mind.tree.moved(old, None);
                }
            }
            mind.tree.added(last, &text);
        }
        mind.output.push(text.clone());
        if let Some(said) = number_of(&text) {
            mind.number = Some(said);
        }
    }
    let item = super::mind::found_word_item(mind, &text, by.family(), None);
    mind.stack.push(item);
}
because!(written_out, WordReading, "an answer written into the output and under the last answer tag of the world, in place of the answer \
     before it, unless the question only asks that answer back, a number among them kept as the number of the mind so the next sum may \
     carry it on, and put on the stack as what the step found; the nothing token goes on the stack and never into the output");

pub(super) fn landed_seen(mind: &mut CursorMind, by: WordMove, node: Option<usize>) {
    let Some(n) = node else { landed_on(mind, by, None); return };
    let ids = seen(mind, n);
    mind.at = n;
    let text = crate::cursor::bare_name(&mind.tree.node(n).name);
    let mut item = super::mind::found_word_item(mind, &text, by.family(), Some(n));
    item.ids.extend(ids);
    mind.stack.push(item);
}
because!(landed_seen, WordReading, "where a find landed, the node put on the stack with what the find sees of it, or the nothing token");

pub(super) fn traced(mind: &mut CursorMind, thing: usize, from: usize) {
    if from == 0 || mind.tree.node(from).name.starts_with(BRACE_OPEN_TEXT) || Some(from) == open_question(mind) {
        return;
    }
    let mention = mind.tree.linked(from, thing, false);
    match time_of_day(mind, thing) {
        Some(when) => timed_at(mind, mention, &when),
        None => dated(mind, mention),
    }
    added_under(mind, mention, &quantity_tag_named(f32::default()), false);
    let carries = person(mind, thing);
    {
        let carried: Vec<usize> = present_children(mind, thing).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && mind.tree.node(c).link.is_none() && (carries || owner_of(mind, c) == Some(thing))).collect();
        for c in carried {
            let trace_of = mind.tree.linked(from, c, false);
            dated(mind, trace_of);
        }
    }
}
because!(traced, WordReading, "the trace a thing leaves where it was when it moves or changes hands, with a trace for each thing it \
     carries, all of a person's and of any other mover what it owns: a mention of it under the old place or owner, dated past, or with the \
     time of day it was placed at, and counted none, since the old holder no longer has it, so where was the cat and who gave the car are \
     read from it; a person who moves leaves the traces of what they carry too");

pub(super) fn group_targets(mind: &mut CursorMind, at: usize) -> Vec<usize> {
    let members: Vec<usize> = mind.held.iter().copied().filter(|&h| !mind.tree.node(h).gone).collect();
    if flag_of(mind, super::mind::FLAG_GROUP).is_none() || members.len() < PAIR_LEAST || flag_of(mind, super::mind::FLAG_PLACE).is_some() {
        return vec![at];
    }
    if !mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) {
        return members;
    }
    let mut chain = Vec::new();
    let mut n = at;
    while n != 0 && mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) {
        chain.push(mind.tree.node(n).name.to_string());
        n = mind.tree.node(n).parent;
    }
    chain.reverse();
    members.into_iter().map(|m| chain.iter().fold(m, |under, name| added_under(mind, under, name, false))).collect()
}
because!(group_targets, WordReading, "where a write goes when the things held are a group: on each member, or under each member's relation \
     of the same path as the cursor's, so the cat and the dog are black writes black on both");

pub(super) fn timed_at(mind: &mut CursorMind, node: usize, when: &str) {
    if let Some(old) = own_child(mind, node, TIME_RELATION) {
        mind.tree.moved(old, None);
    }
    let time = added_under(mind, node, TIME_RELATION, false);
    added_under(mind, time, when, true);
}
because!(timed_at, WordReading, "a node given the time of day of its move under its time relation, in place of the one it had");

pub(super) fn dated(mind: &mut CursorMind, node: usize) {
    let time = added_under(mind, node, TIME_RELATION, false);
    added_under(mind, time, PAST_TIME, true);
}
because!(dated, WordReading, "a node given the time past under its time relation, as a past form of the verb or copula says");
