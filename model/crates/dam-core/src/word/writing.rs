use super::mind::{open_question, flag_of, is_number, landed_on, past_form, singular, verb_base, COPULA, FLAG_INDEFINITE, FLAG_GIVE, FLAG_PROPERTY, FLAG_QUANTITY, FLAG_DEFINITE, FLAG_TIME};
use super::moves::{WordMove, WordStep};
use crate::cursor::{quantity_tag_named, step_item, CursorMind, BRACE_OPEN_TEXT, PAST_TIME, TIME_RELATION};
use crate::quiz::IS_FORM;
use crate::words::number_of;
use super::lookup::{present_children, tag_value, story_node, own_child, kept_flag, person};
use super::lookup::{claim_node, unseeded};
use super::written::{thing_made, added_under, added_value, flagged, traced, group_targets, dated, timed_at};
use super::physics::{WordWorld, OWNER_TAG, TRUE_TAG};
use patterns::because;

pub(super) fn wrote_down(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool, braced: bool) -> bool {
    let word = word.to_string();
    match step.act {
        WordMove::AddRole => {
            let Some(role) = flag_of(mind, super::mind::FLAG_ROLE).map(str::to_string).filter(|_| plain) else {
                landed_on(mind, step.act, None);
                return true;
            };
            mind.flags.retain(|(k, _)| k != super::mind::FLAG_ROLE);
            let node = added_under(mind, at, &step_item(&role), false);
            landed_on(mind, step.act, Some(node));
        }
        WordMove::FlagThe => mind.flags.push((FLAG_DEFINITE.to_string(), TRUE_TAG.to_string())),
        WordMove::FlagA => mind.flags.push((FLAG_INDEFINITE.to_string(), TRUE_TAG.to_string())),
        WordMove::FlagTime => mind.flags.push((FLAG_TIME.to_string(), crate::cursor::bare_name(PAST_TIME))),
        WordMove::FlagLater => mind.flags.push((FLAG_TIME.to_string(), crate::cursor::bare_name(crate::cursor::LATER_TIME))),
        WordMove::FlagFrom => mind.flags.push((super::mind::FLAG_FROM.to_string(), TRUE_TAG.to_string())),
        WordMove::FlagWhen => mind.flags.push((super::mind::FLAG_WHEN.to_string(), word)),
        WordMove::FlagWhose => mind.flags.push((super::mind::FLAG_OWNED.to_string(), word)),
        WordMove::FlagQuantity => match is_number(mind, &word).or_else(|| super::mind::NEGATIONS.contains(&word.as_str()).then(|| 0.to_string())) {
            Some(count) => mind.flags.push((FLAG_QUANTITY.to_string(), count)),
            None => landed_on(mind, step.act, None),
        },
        WordMove::FlagProperty => mind.flags.push((FLAG_PROPERTY.to_string(), word)),
        WordMove::AddProperty | WordMove::AddPropertyPast => {
            if !plain {
                landed_on(mind, step.act, None);
                return true;
            }
            let labels = word == super::mind::LABEL_AS && super::mind::labelling(&mind.before);
            let name = if COPULA.contains(&word.as_str()) || labels { IS_FORM.trim().to_string() } else { step_item(&word) };
            if word == super::mind::BEEN {
                mind.held.clear();
                mind.flags.retain(|(k, _)| k != FLAG_GIVE);
            }
            let mut last = at;
            for target in group_targets(mind, at) {
                last = added_under(mind, target, &name, false);
                if step.act == WordMove::AddPropertyPast {
                    dated(mind, last);
                }
            }
            landed_on(mind, step.act, Some(last));
        }
        WordMove::AddRelation => relation_added(mind, step, &word, at, plain, false),
        WordMove::AddRelationPast => relation_added(mind, step, &word, at, plain, true),

        WordMove::AddQuestion => {
            let node = mind.tree.added(0, &word);
            mind.question_start = Some(node);
            let counted: Vec<String> = mind.flags.iter().filter(|(k, _)| k == FLAG_QUANTITY).map(|(_, v)| v.clone()).collect();
            for count in counted {
                mind.tree.added(node, &count);
            }
            mind.flags.clear();
            landed_on(mind, step.act, Some(node));
        }
        WordMove::AddDeed => {
            if !plain {
                landed_on(mind, step.act, None);
                return true;
            }
            let deed = added_under(mind, at, super::mind::DEED_TAG, false);
            added_under(mind, deed, &word, false);
            landed_on(mind, step.act, Some(at));
        }
        WordMove::AddValue if open_question(mind).is_some() => {
            let question = open_question(mind).unwrap_or_default();
            mind.tree.added(question, &word);
            landed_on(mind, step.act, Some(question));
        }
        WordMove::SetClock => {
            let subject = if braced && *mind.tree.node(at).name == *IS_FORM.trim() { mind.tree.node(at).parent } else { 0 };
            if subject == 0 || flag_of(mind, FLAG_QUANTITY).is_none() {
                landed_on(mind, step.act, None);
                return true;
            }
            if present_children(mind, at).is_empty() {
                mind.tree.moved(at, None);
            }
            let time = added_under(mind, subject, TIME_RELATION, false);
            for old in present_children(mind, time) {
                mind.tree.moved(old, None);
            }
            let hour = added_under(mind, time, &word, true);
            flagged(mind, hour);
            landed_on(mind, step.act, Some(hour));
        }
        WordMove::Measure => {
            let parent = if plain { mind.tree.node(at).parent } else { 0 };
            let subject = if parent != 0 { mind.tree.node(parent).parent } else { 0 };
            if subject == 0 || *mind.tree.node(parent).name != *IS_FORM.trim() {
                landed_on(mind, step.act, None);
                return true;
            }
            let relation = added_under(mind, subject, &step_item(&word), false);
            mind.tree.moved(at, None);
            mind.tree.moved(at, Some(relation));
            if present_children(mind, parent).is_empty() {
                mind.tree.moved(parent, None);
            }
            landed_on(mind, step.act, Some(at));
        }
        WordMove::NameResult => {
            let Some(result) = mind.output.last().cloned() else { landed_on(mind, step.act, None); return true; };
            if let Some(question) = open_question(mind) {
                mind.tree.moved(question, None);
            }
            mind.question_start = None;
            let named = story_node(mind, &word).unwrap_or_else(|| thing_made(mind, &word));
            let is = added_under(mind, named, IS_FORM.trim(), false);
            added_under(mind, is, &result, true);
            landed_on(mind, step.act, Some(named));
        }
        WordMove::AddValue => value_added(mind, step, &word, at, braced),
        _ => quality_set(mind, step, &word, at),
    }
    true
}
because!(wrote_down, WordWorld, "the moves that write what a word says into the world: the flags the words before a thing set, the property, the deed, the value and the relation written under it, and the quality of a kind every move not named above falls through to");

fn relation_added(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool, date: bool) {
    let word = word.to_string();
    if !plain {
        landed_on(mind, step.act, None);
        return;
    }
    let base = if word == super::mind::MEANS || super::mind::operation_of(&word).is_some() || (flag_of(mind, FLAG_QUANTITY).is_some() && mind.before.first().is_some_and(|first| is_number(mind, first).is_some()) && verb_base(mind, &word).is_none()) { word.clone() } else if verb_base(mind, &word).is_none() && word.ends_with(super::mind::COMPARISON_END) && flag_of(mind, super::mind::FLAG_KIN).is_none() && !super::mind::KIN.contains(&word.as_str()) { format!("{word}{}", super::mind::COMPARED) } else { super::mind::verb_stem(mind, &word) };
    let way: Option<String> = None;
    let mentioned = mind.tree.node(at).parent != 0 && mind.tree.node(mind.tree.node(at).parent).name.starts_with(BRACE_OPEN_TEXT) && verb_base(mind, &word).is_some();
    let done = mind.tree.node(at).link.filter(|&l| *mind.tree.node(mind.tree.node(l).parent).name == *step_item(super::mind::ACTIVITY) || mentioned && mind.tree.story(l));
    let done = if done.is_none() && mentioned && mind.tree.node(at).link.is_none_or(|l| !mind.tree.story(l)) && !claim_node(mind, at) { let name = mind.tree.node(at).name.to_string(); Some(story_node(mind, &name).unwrap_or_else(|| thing_made(mind, &name))) } else { done };
    let mut last = at;
    let mut made = Vec::new();
    let going = base == super::mind::GOING_RELATION;
    for target in group_targets(mind, at) {
        let under = if target == at { done.unwrap_or(target) } else { target };
        last = if going { mind.tree.added(under, &step_item(&base)) } else { added_under(mind, under, &step_item(&base), false) };
        made.push(last);
    }
    if let Some(way) = way.clone() {
        for &one in &made {
            let how = added_under(mind, one, &step_item(super::mind::METHOD_RELATION), false);
            added_under(mind, how, &way, true);
        }
    }
    if date {
        let when: Option<String> = flag_of(mind, super::mind::FLAG_WHEN).map(|one| one.to_string()).filter(|one| super::mind::TIMES_OF_DAY.iter().any(|day| *day == one));
        for one in made {
            match when.clone() {
                Some(day) => timed_at(mind, one, &day),
                None => dated(mind, one),
            }
        }
    } else if (past_form(mind, &word) || word.ends_with(super::mind::PAST_END)) && !super::mind::CLAIMING.contains(&base.as_str()) {
        mind.flags.push((FLAG_TIME.to_string(), crate::cursor::bare_name(PAST_TIME)));
    }
    if flag_of(mind, super::mind::FLAG_KIN).is_none() && super::mind::KIN.contains(&word.as_str()) && mind.held.last() == Some(&at) {
        mind.held.clear();
        mind.flags.clear();
    }
    if flag_of(mind, super::mind::FLAG_KIN).is_some() {
        if let Some(subject) = mind.held.last().copied() {
            if super::mind::KIN.contains(&word.as_str()) || word == super::mind::NAME_ROLE {
                mind.tree.linked(last, subject, false);
            } else {
                mind.tree.moved(last, None);
                let is = added_under(mind, subject, IS_FORM.trim(), false);
                added_under(mind, is, &word, true);
                let from = mind.tree.node(subject).parent;
                mind.tree.moved(subject, None);
                mind.tree.moved(subject, Some(at));
                traced(mind, subject, from);
                let tag = added_under(mind, subject, OWNER_TAG, false);
                mind.tree.linked(tag, at, false);
            }
        }
        mind.held.clear();
        mind.flags.clear();
    }
    landed_on(mind, step.act, Some(last));
}
because!(relation_added, WordWorld, "the relation add: the pointed word is written by its base form as a relation under the thing the cursor stands on and under every thing grouped with it, a past form flags the time, a deed told of a mention is written on the thing it mentions, and with the kin flag the held one is linked under the relation, or for a word that is no kin takes the word under is and goes to the thing with its owner");

pub(super) fn went_along(mind: &mut CursorMind, goer: usize, along: usize) {
    let walked = super::lookup::goings(mind, goer);
    let Some(&place) = walked.last() else { return };
    let name = mind.tree.node(place).name.to_string();
    let when: Option<String> = own_child(mind, mind.tree.node(place).parent, TIME_RELATION)
        .and_then(|time| present_children(mind, time).first().map(|&value| mind.tree.node(value).name.to_string()));
    let going = mind.tree.added(along, &step_item(super::mind::GOING_RELATION));
    match when {
        Some(day) => timed_at(mind, going, &day),
        None => dated(mind, going),
    }
    added_value(mind, going, &name);
}
because!(went_along, WordWorld, "the going of the one that moved written for what went with them, tom went to the park with his dog, so the dog keeps the place and the time of that going as its own");

fn went_with(mind: &mut CursorMind, going: usize, place: &str) {
    let goer = mind.tree.node(going).parent;
    if goer == 0 {
        return;
    }
    let when: Option<String> = own_child(mind, going, TIME_RELATION)
        .and_then(|time| present_children(mind, time).first().map(|&value| mind.tree.node(value).name.to_string()));
    let carried: Vec<usize> = present_children(mind, goer)
        .into_iter()
        .filter(|&thing| !mind.tree.node(thing).name.starts_with(BRACE_OPEN_TEXT))
        .map(|thing| mind.tree.node(thing).link.unwrap_or(thing))
        .filter(|&thing| thing != 0 && !mind.tree.node(thing).gone)
        .collect();
    let way: Option<String> = own_child(mind, going, &step_item(super::mind::METHOD_RELATION))
        .and_then(|how| present_children(mind, how).first().map(|&value| mind.tree.node(value).name.to_string()));
    let stood: Option<String> = super::lookup::place_of(mind, goer).map(|place| mind.tree.node(place).name.to_string());
    for thing in carried {
        if super::lookup::goings(mind, thing).is_empty() {
            if let Some(from) = stood.clone() {
                let first = mind.tree.added(thing, &step_item(super::mind::GOING_RELATION));
                if let Some(day) = when.clone() {
                    timed_at(mind, first, &day);
                }
                added_value(mind, first, &from);
            }
        }
        let relation = mind.tree.added(thing, &step_item(super::mind::GOING_RELATION));
        if let Some(day) = when.clone() {
            timed_at(mind, relation, &day);
        }
        if let Some(one) = way.clone() {
            let how = added_under(mind, relation, &step_item(super::mind::METHOD_RELATION), false);
            added_under(mind, how, &one, true);
        }
        added_value(mind, relation, place);
    }
}
because!(went_with, WordWorld, "a going written for every thing the one that went carries, with the time of that going, so each thing keeps the places it has been and where a carried thing was before is read off the thing itself");

pub(super) fn going_relation(mind: &CursorMind, at: usize) -> bool {
    let named = crate::cursor::bare_name(&mind.tree.node(at).name);
    at != 0 && mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) && (named == super::mind::GOING_RELATION || super::mind::MOVING.contains(&named.as_str()) && named != super::mind::FITTING)
}
because!(going_relation, WordWorld, "whether a node is a going, written under the thing that moved, by the name every going takes or by the verb of moving that made it before its place arrived");

fn value_added(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, braced: bool) {
    let word = word.to_string();
    let done = (!braced).then(|| own_child(mind, at, super::mind::DEED_TAG)).flatten().filter(|&deed| !present_children(mind, deed).is_empty());
    if let Some(deed) = done {
        let said = present_children(mind, deed).last().map(|&value| mind.tree.node(value).name.to_string()).unwrap_or_default();
        if let Some(&value) = present_children(mind, deed).last() {
            mind.tree.moved(value, None);
        }
        if present_children(mind, deed).is_empty() {
            mind.tree.moved(deed, None);
        }
        let relation = added_under(mind, at, &step_item(&super::mind::verb_stem(mind, &said)), false);
        let past = super::mind::past_form(mind, &said) || said.ends_with(super::mind::PAST_END);
        value_added(mind, step, &word, relation, true);
        if past {
            let written = mind.at;
            dated(mind, written);
        }
        return;
    }
    if !braced {
        landed_on(mind, step.act, None);
        return;
    }
    let word = if flag_of(mind, FLAG_QUANTITY).is_some() { singular(&word) } else { word };
    let doer = mind.tree.node(at).parent;
    let word = if super::mind::REFLEXIVES.contains(&word.as_str()) && doer != 0 { mind.tree.node(doer).name.to_string() } else { word };
    let stood_for = (word == super::mind::THING_PRONOUNS[0] && doer != 0).then(|| (mind.tree.state..mind.tree.len()).rev().find(|&n| { let node = mind.tree.node(n); n != doer && !node.gone && !node.name.starts_with(BRACE_OPEN_TEXT) && !tag_value(mind, n) && !person(mind, n) && number_of(&node.name).is_none() && (going_relation(mind, node.parent) || node.link.is_none() && (node.parent == 0 || !mind.tree.node(node.parent).name.starts_with(BRACE_OPEN_TEXT))) })).flatten();
    let word = stood_for.map(|n| mind.tree.node(n).name.to_string()).unwrap_or(word);
    let of_person = super::mind::OBJECT_PRONOUNS.iter().any(|(said, person)| *said == word && *person) || super::mind::PERSON_PRONOUNS.contains(&word.as_str());
    let stands = |n: usize| {
        let node = mind.tree.node(n);
        let above = mind.tree.node(node.parent).name.to_string();
        n != doer && !node.gone && node.link.is_none() && !node.name.starts_with(BRACE_OPEN_TEXT) && *node.name != *word && !above.starts_with(BRACE_OPEN_TEXT) && super::lookup::story_node(mind, &node.name.to_string()) == Some(n)
    };
    let person_said = of_person.then(|| super::lookup::newest_for(mind, true, &word).filter(|&n| stands(n)).or_else(|| (mind.tree.state..mind.tree.len()).rev().find(|&n| stands(n)))).flatten();
    let word = person_said.map(|n| mind.tree.node(n).name.to_string()).unwrap_or(word);
    let named = crate::cursor::bare_name(&mind.tree.node(at).name);
    let toward = mind.before.iter().any(|said| *said == super::mind::INFINITIVE || super::mind::place_word(said));
    if toward && named != super::mind::FITTING && named != super::mind::GOING_RELATION && super::mind::MOVING.contains(&named.as_str()) {
        let way = super::mind::verb_present(mind, &named).unwrap_or(named.clone());
        let deed = step_item(&named);
        let said: Vec<usize> = (mind.sentence_from..mind.tree.len())
            .filter(|&n| !mind.tree.node(n).gone && going_relation(mind, n) && crate::cursor::bare_name(&mind.tree.node(n).name) != super::mind::GOING_RELATION)
            .filter(|&n| present_children(mind, n).into_iter().all(|c| mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)))
            .collect();
        let _ = &deed;
        let mut made = at;
        for one in said {
            let holder = mind.tree.node(one).parent;
            let going = mind.tree.added(holder, &step_item(super::mind::GOING_RELATION));
            for child in present_children(mind, one) {
                mind.tree.moved(child, None);
                mind.tree.moved(child, Some(going));
            }
            mind.tree.moved(one, None);
            if way != super::mind::GOING_RELATION {
                let how = added_under(mind, going, &step_item(super::mind::METHOD_RELATION), false);
                added_under(mind, how, &way, true);
            }
            went_with(mind, going, &word);
            let place = added_value(mind, going, &word);
            flagged(mind, place);
            if one == at {
                made = place;
            }
        }
        landed_on(mind, step.act, Some(made));
        return;
    } else {
        at
    };
    if going_relation(mind, at) && flag_of(mind, super::mind::FLAG_FROM).is_some() {
        let from = added_under(mind, at, &step_item(super::mind::SOURCE), false);
        let came = added_value(mind, from, &word);
        mind.flags.retain(|(k, _)| k != super::mind::FLAG_FROM);
        flagged(mind, came);
        landed_on(mind, step.act, Some(at));
        return;
    }
    let targets = group_targets(mind, at);
    let kept = mind.flags.clone();
    let mut last = at;
    for target in targets {
        mind.flags = kept.clone();
        if *mind.tree.node(target).name == *step_item(super::mind::GOING_RELATION) {
            went_with(mind, target, &word);
        }
        let had = own_child(mind, target, &word).is_some();
        let past_beside = present_children(mind, target).into_iter().any(|v| own_child(mind, v, TIME_RELATION).is_some());
        last = added_value(mind, target, &word);
        if !had && past_beside && flag_of(mind, FLAG_TIME).is_none() && *mind.tree.node(target).name != *IS_FORM.trim() {
            dated(mind, last);
        }
        if had { mind.flags.retain(|(k, _)| kept_flag(k)) } else { flagged(mind, last) }
    }
    let described = mind.before.iter().rev().nth(1).filter(|b| *mind.tree.node(at).name == *IS_FORM.trim() && unseeded(mind, b) && **b != word).and_then(|b| own_child(mind, at, b)).filter(|&q| q != last);
    if let Some(quality) = described {
        mind.tree.moved(quality, None);
        mind.tree.moved(quality, Some(last));
    }
    landed_on(mind, step.act, Some(last));
}
because!(value_added, WordWorld, "the value add: the pointed word is written under the relation the cursor stands on, the singular of a counted word, the doer for a word that points back at them, the person a pronoun of a person names, everyone liked him is everyone liked john, and a word the seeds never state takes the word after it as what it describes");

fn quality_set(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize) {
    let word = word.to_string();
    let kind = super::mind::quality_kind(mind, &word).unwrap_or_default();
    let on_is = at != 0 && *mind.tree.node(at).name == *IS_FORM.trim();
    if !on_is || kind.is_empty() {
        landed_on(mind, step.act, None);
        return;
    }
    let denied = flag_of(mind, FLAG_QUANTITY).is_some_and(|q| number_of(q) == Some(f32::default()));
    let mut last = at;
    for target in group_targets(mind, at) {
        let under = added_under(mind, target, &step_item(&kind), false);
        last = added_under(mind, under, &word, !denied);
        for tag in present_children(mind, last).into_iter().filter(|&q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG)).collect::<Vec<_>>() {
            mind.tree.moved(tag, None);
        }
        if denied {
            added_under(mind, last, &quantity_tag_named(f32::default()), false);
        }
        if let Some(degree) = flag_of(mind, super::mind::FLAG_DEGREE).map(str::to_string).filter(|degree| *degree != word) {
            let how = added_under(mind, last, &step_item(super::mind::FLAG_DEGREE), false);
            added_under(mind, how, &degree, true);
        }
    }
    mind.flags.retain(|(k, _)| k != super::mind::FLAG_DEGREE);
    if denied {
        mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
    }
    landed_on(mind, step.act, Some(last));
}
because!(quality_set, WordWorld, "the set of a quality: standing on is, a word of the kind the move names is written under that kind, red under color, and any other word or place is refused");
