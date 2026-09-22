use super::mind::{mark_word, FLAG_HAND, HAVING, open_question, flag_of, heard_text, landed_on, past_form, singular, verb_base, FLAG_INDEFINITE, FLAG_GIVE, FLAG_CONTAIN, FLAG_QUANTITY, FLAG_DEFINITE, FLAG_TIME};
use super::moves::{WordMove, WordStep};
use crate::cursor::{quantity_tag_named, step_item, CursorMind, BRACE_OPEN_TEXT, PAST_TIME, TIME_RELATION};
use crate::quiz::IS_FORM;
use crate::words::number_of;
use super::lookup::{present_children, story_node, story_nodes, claims_asked, claimed_named, own_child, mention_thing, moved_without_place, person, newest_told, newest_for, inside, story_nodes_all, quantity_tag_of, count_of, owner_of, held_or_owned, holds_value, told_of, group_pronoun};
use super::lookup::{question_match, things_named, claim_node, activity_word, place_of, who_has};
use super::written::{thing_made, added_under, flagged, landed_seen, traced, timed_at, dated};
use super::physics::{WordWorld, ASSUMED_TAG, OWNER_TAG, TRUE_TAG};
use patterns::because;

pub(super) fn carried(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool, braced: bool) -> bool {
    let word = word.to_string();
    match step.act {
        WordMove::Continue => {}
        WordMove::PointNothing => mind.flags.retain(|(k, _)| k != super::mind::FLAG_FROM),
        WordMove::Hand => {
            if plain {
                let giver = mind.tree.node(at).name.to_string();
                mind.flags.push((FLAG_HAND.to_string(), giver));
            }
            landed_on(mind, step.act, plain.then_some(at));
        }
        WordMove::Leave => {
            if plain {
                mind.held = vec![at];
                mind.flags.push((super::mind::FLAG_LEAVE.to_string(), TRUE_TAG.to_string()));
            }
            landed_on(mind, step.act, plain.then_some(at));
        }
        WordMove::Accompany => {
            if plain {
                let holder = mind.tree.node(at).parent;
                let goer = (holder != 0 && super::writing::going_relation(mind, holder)).then(|| mind.tree.node(holder).parent).filter(|&one| one != 0);
                mind.held = vec![goer.unwrap_or(at)];
                mind.flags.push((super::mind::FLAG_WITH.to_string(), TRUE_TAG.to_string()));
            }
            landed_on(mind, step.act, plain.then_some(at));
        }
        WordMove::Belong => {
            if plain {
                mind.held = vec![mention_thing(mind, at)];
                mind.flags.push((super::mind::FLAG_PLACE.to_string(), TRUE_TAG.to_string()));
                mind.flags.push((FLAG_HAND.to_string(), TRUE_TAG.to_string()));
            }
            landed_on(mind, step.act, plain.then_some(at));
        }
        WordMove::Relate => {
            if !plain {
                landed_on(mind, step.act, None);
                return true;
            }
            let parent = mind.tree.node(at).parent;
            let on_is = parent != 0 && *mind.tree.node(parent).name == *IS_FORM.trim();
            let role = mind.tree.node(at).name.to_string();
            if !on_is {
                mind.tree.moved(at, None);
            }
            mind.flags.retain(|(k, _)| k != FLAG_DEFINITE && k != FLAG_INDEFINITE);
            mind.flags.push((super::mind::FLAG_ROLE.to_string(), role));
            if on_is {
                let subject = mind.tree.node(parent).parent;
                mind.held = vec![subject];
                landed_on(mind, step.act, Some(subject));
            } else {
                landed_on(mind, step.act, None);
                mind.at = 0;
            }
        }
        WordMove::Together => {
            let members: Vec<usize> = mind.held.iter().copied().filter(|&member| !mind.tree.node(member).gone).collect();
            if members.len() < super::physics::PAIR_LEAST {
                landed_on(mind, step.act, None);
                return true;
            }
            let group = mind.tree.added(0, &step_item(super::english::GROUP_NAME));
            mind.tree.adopted(group);
            let holds = added_under(mind, group, &step_item(super::english::ITEMS_NAME), false);
            let said: Vec<String> = members
                .iter()
                .filter_map(|&member| own_child(mind, member, IS_FORM.trim()))
                .flat_map(|is| present_children(mind, is))
                .filter(|&value| value >= mind.sentence_from && !mind.tree.node(value).name.starts_with(BRACE_OPEN_TEXT))
                .map(|value| mind.tree.node(value).name.to_string())
                .collect();
            for member in &members {
                if let Some(is) = own_child(mind, *member, IS_FORM.trim()) {
                    let told: Vec<usize> = present_children(mind, is).into_iter().filter(|&value| value >= mind.sentence_from).collect();
                    for value in told {
                        mind.tree.moved(value, None);
                    }
                    if present_children(mind, is).is_empty() {
                        mind.tree.moved(is, None);
                    }
                }
            }
            for member in members {
                mind.tree.linked(holds, member, false);
            }
            let theirs = added_under(mind, group, IS_FORM.trim(), false);
            for name in said {
                added_under(mind, theirs, &name, false);
            }
            mind.at = theirs;
            mind.held = vec![group];
            mind.flags.retain(|(key, _)| key != super::mind::FLAG_GROUP);
            landed_on(mind, step.act, Some(theirs));
        }
        WordMove::Join => joined(mind, step, at, plain),
        WordMove::ChangeState => {
            let said = super::mind::STATES.iter().find(|(v, _, _)| verb_base(mind, &word).as_deref() == Some(*v)).copied();
            if let Some((verb, _, _)) = said {
                mind.flags.push((super::mind::FLAG_STATE.to_string(), verb.to_string()));
                landed_on(mind, step.act, plain.then_some(at));
                return true;
            }
            let flagged_verb = flag_of(mind, super::mind::FLAG_STATE).map(str::to_string);
            mind.flags.retain(|(k, _)| k != super::mind::FLAG_STATE);
            let state = super::mind::STATES.iter().find(|(v, _, _)| flagged_verb.as_deref() == Some(*v)).copied();
            let Some((_, set, gone)) = state.filter(|_| plain) else { landed_on(mind, step.act, None); return true; };
            let is = added_under(mind, at, IS_FORM.trim(), false);
            if let Some(old) = own_child(mind, is, gone) {
                mind.tree.moved(old, None);
            }
            added_under(mind, is, set, false);
            landed_on(mind, step.act, Some(at));
        }
        WordMove::Take | WordMove::Release => {
            if !plain {
                landed_on(mind, step.act, None);
                return true;
            }
            mind.held = vec![at];
            mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
            let flag = if step.act == WordMove::Take { super::mind::FLAG_TAKE } else { super::mind::FLAG_RELEASE };
            let said = super::mind::verb_stem(mind, &heard_text(mind));
            mind.flags.push((flag.to_string(), if step.act == WordMove::Release && !said.is_empty() { said } else { TRUE_TAG.to_string() }));
            landed_on(mind, step.act, Some(at));
        }
        WordMove::Grab | WordMove::Give | WordMove::Contain => {
            if step.act == WordMove::Grab && plain {
                let said = heard_text(mind);
                mind.flags.push((super::mind::FLAG_PLACE.to_string(), if said == super::mind::HOUR_OPENER { said } else { TRUE_TAG.to_string() }));
            }
            if !plain {
                landed_on(mind, step.act, None);
                return true;
            }
            if flag_of(mind, super::mind::FLAG_GROUP).is_some() && step.act == WordMove::Grab {
                if !mind.held.contains(&at) {
                    mind.held.push(at);
                }
            } else if !(step.act == WordMove::Give && flag_of(mind, super::mind::FLAG_GROUP).is_some() && mind.held.contains(&at)) {
                let holder = mind.tree.node(at).parent;
                let goer = (step.act == WordMove::Give && holder != 0 && super::writing::going_relation(mind, holder)).then(|| mind.tree.node(holder).parent).filter(|&one| one != 0);
                mind.held = vec![goer.unwrap_or(at)];
            }
            if step.act == WordMove::Give {
                let said = heard_text(mind);
                mind.flags.push((FLAG_GIVE.to_string(), if HAVING.contains(&said.as_str()) { said } else { TRUE_TAG.to_string() }));
            }
            if step.act == WordMove::Contain {
                mind.flags.push((FLAG_CONTAIN.to_string(), TRUE_TAG.to_string()));
            }
            landed_on(mind, step.act, Some(at));
        }
        WordMove::Drop => dropped(mind, step, &word, at, plain),
        WordMove::FindNext => {
            let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
            let next = story_nodes(mind, &name).into_iter().find(|&n| n < at);
            if next.is_none() {
                mind.at = 0;
            }
            landed_seen(mind, step.act, next);
        }
        WordMove::FindWith => {
            let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
            let holding: Vec<usize> = story_nodes_all(mind).into_iter().filter(|&n| holds_value(mind, n, &word) && mind.tree.node(n).name.as_ref() != word).collect();
            let value = holding.iter().copied().find(|&n| who_has(mind, n).is_some_and(|owner| asked.iter().any(|a| **a == *mind.tree.node(owner).name))).or_else(|| holding.first().copied().filter(|_| !asked.iter().any(|a| *a != word && story_node(mind, a).is_some())));
            if value.is_none() {
                mind.at = 0;
            }
            landed_on(mind, step.act, value);
        }
        WordMove::Possess => {
            let parent = if plain { mind.tree.node(at).parent } else { 0 };
            let subject = if parent != 0 { mind.tree.node(parent).parent } else { 0 };
            if subject == 0 || *mind.tree.node(parent).name != *IS_FORM.trim() {
                landed_on(mind, step.act, None);
                return true;
            }
            let name = mind.tree.node(at).name.to_string();
            mind.tree.moved(at, None);
            if present_children(mind, parent).is_empty() {
                mind.tree.moved(parent, None);
            }
            let speaker_named = super::mind::OWN_WORDS.iter().find(|(own, _)| *own == name).map(|(_, owner)| if super::mind::YOU_WORDS.contains(owner) { super::mind::ASSISTANT_NAME } else { super::mind::USER_NAME }).filter(|_| super::mind::OWN_WORDS.iter().any(|(own, owner)| *own == name && (super::mind::SELF_WORDS.contains(owner) || super::mind::YOU_WORDS.contains(owner))));
            let name = speaker_named.map_or(name, str::to_string);
            let owner = story_node(mind, &name).unwrap_or_else(|| thing_made(mind, &name));
            mind.held = vec![subject];
            mind.flags.push((super::mind::FLAG_KIN.to_string(), TRUE_TAG.to_string()));
            landed_on(mind, step.act, Some(owner));
        }
        WordMove::ActivityNamed => {
            let deed = own_child(mind, at, super::mind::DEED_TAG).unwrap_or(at);
            let said = present_children(mind, deed).last().map(|&value| mind.tree.node(value).name.to_string()).unwrap_or_default();
            if let Some(&value) = present_children(mind, deed).last() {
                mind.tree.moved(value, None);
            }
            if present_children(mind, deed).is_empty() {
                mind.tree.moved(deed, None);
            }
            added_under(mind, at, &step_item(&super::mind::verb_stem(mind, &said)), false);
            let activity = added_under(mind, at, &step_item(super::mind::ACTIVITY), false);
            landed_on(mind, step.act, Some(activity));
        }
        WordMove::ActivityDone => {
            let deed = own_child(mind, at, super::mind::DEED_TAG).unwrap_or(at);
            let thing = at;
            let at = present_children(mind, deed).into_iter().last().unwrap_or(at);
            let said = mind.tree.node(at).name.to_string();
            let name = super::mind::verb_stem(mind, &said);
            mind.tree.moved(at, None);
            if present_children(mind, deed).is_empty() {
                mind.tree.moved(deed, None);
            }
            let activity = added_under(mind, thing, &step_item(super::mind::ACTIVITY), false);
            let value = added_under(mind, activity, &name, true);
            if past_form(mind, &said) || said.ends_with(super::mind::PAST_END) {
                dated(mind, value);
            }
            landed_on(mind, step.act, Some(value));
        }
        WordMove::ActivityUnder => {
            let deed = mind.tree.node(at).parent;
            let doer = mind.tree.node(deed).parent;
            let name = crate::cursor::bare_name(&mind.tree.node(deed).name);
            let activity = added_under(mind, doer, &step_item(super::mind::ACTIVITY), false);
            let value = added_under(mind, activity, &name, true);
            let of = added_under(mind, value, &step_item(super::mind::TOWARD), false);
            mind.tree.moved(at, None);
            mind.tree.moved(at, Some(of));
            mind.tree.moved(deed, None);
            landed_on(mind, step.act, Some(value));
        }
        WordMove::Activity => {
            let thing = if braced { mind.tree.node(at).parent } else { 0 };
            if thing == 0 || !present_children(mind, at).is_empty() {
                landed_on(mind, step.act, None);
                return true;
            }
            let under = if heard_text(mind) == super::mind::INFINITIVE { at } else { thing };
            let activity = added_under(mind, under, &step_item(super::mind::ACTIVITY), false);
            if heard_text(mind) != super::mind::INFINITIVE {
                let name = crate::cursor::bare_name(&mind.tree.node(at).name);
                mind.tree.moved(at, None);
                let value = added_under(mind, activity, &name, true);
                flagged(mind, value);
                landed_on(mind, step.act, Some(value));
            } else {
                landed_on(mind, step.act, Some(activity));
            }
        }
        WordMove::Regard => {
            if !plain {
                landed_on(mind, step.act, None);
                return true;
            }
            let name = mind.tree.node(at).name.to_string();
            let mut above = mind.tree.node(at).parent;
            mind.tree.moved(at, None);
            while above != 0 && mind.tree.node(above).name.starts_with(BRACE_OPEN_TEXT) {
                let next = mind.tree.node(above).parent;
                if present_children(mind, above).is_empty() {
                    mind.tree.moved(above, None);
                }
                above = next;
            }
            if above == 0 {
                landed_on(mind, step.act, None);
                return true;
            }
            let relation = added_under(mind, above, &step_item(&name), false);
            landed_on(mind, step.act, Some(relation));
        }
        WordMove::GrabAll => {
            let things = if plain { held_or_owned(mind, at) } else { Vec::new() };
            if things.is_empty() {
                landed_on(mind, step.act, None);
                return true;
            }
            mind.held = things.clone();
            mind.flags.push((super::mind::FLAG_GROUP.to_string(), TRUE_TAG.to_string()));
            mind.flags.push((super::mind::FLAG_PLACE.to_string(), TRUE_TAG.to_string()));
            landed_on(mind, step.act, things.last().copied());
        }
        WordMove::StepUser => {
            let yours = |said: &String| super::mind::OWN_WORDS.iter().any(|(own, owner)| *own == said.as_str() && super::mind::YOU_WORDS.contains(owner));
            let to_you = super::mind::YOU_WORDS.contains(&word.as_str()) || (mark_word(&word) && mind.before.iter().any(yours));
            let name = if to_you { super::mind::ASSISTANT_NAME } else { super::mind::USER_NAME };
            let user = story_node(mind, name).unwrap_or_else(|| thing_made(mind, name));
            mind.first_mark = Some(user);
            landed_on(mind, step.act, Some(user));
        }
        WordMove::FindAskedStood => {
            let node = newest_for(mind, super::mind::PERSON_PRONOUNS.contains(&word.as_str()), &word);
            let of_things = node.filter(|&n| !mind.tree.node(mind.tree.node(n).parent).name.starts_with(BRACE_OPEN_TEXT));
            let node = if word == super::mind::THING_PRONOUNS[1] { of_things.or_else(|| newest_for(mind, true, &word)) } else { node };
            if node.is_none() {
                mind.at = 0;
            }
            landed_seen(mind, step.act, node);
        }
        WordMove::FindAsked => {
            let seeded = |name: &str| mind.tree.named(name).find(|&n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0);
            if let Some(claimed) = claims_asked(mind).then(|| claimed_named(mind, &word)).flatten() {
                landed_seen(mind, step.act, Some(claimed));
                return true;
            }
            let lettered = mind.before.iter().any(|b| b == super::mind::ORDER_RELATION) && story_node(mind, &word).is_some();
            let word = if super::mind::SELF_WORDS.contains(&word.as_str()) && !lettered { super::mind::USER_NAME.to_string() } else { word };
            let plural = |name: &str| mind.tree.named(name).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.story(n) && *singular(name) != *name && open_question(mind).is_none_or(|q| !inside(mind, n, q))).max();
            let activity = step_item(super::mind::ACTIVITY);
            let done = |name: &str| { let all: Vec<usize> = mind.tree.named(name).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.story(n) && open_question(mind).is_none_or(|q| !inside(mind, n, q))).collect(); all.iter().copied().filter(|&n| mind.tree.node(n).link.is_some_and(|l| *mind.tree.node(mind.tree.node(l).parent).name == *activity)).max().or_else(|| all.iter().copied().filter(|&n| *mind.tree.node(mind.tree.node(n).parent).name == *activity).max()) };
            let valued = |name: &str| mind.tree.named(name).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.story(n) && open_question(mind).is_none_or(|q| !inside(mind, n, q))).max();
            let described = |name: &str| mind.tree.named(name).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.story(n) && open_question(mind).is_none_or(|q| !inside(mind, n, q)) && told_of(mind, n)).max();
            let asked_words: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
            let matched = things_named(mind, &word).into_iter().find(|&n| question_match(mind, n, &word, &asked_words));
            let node = matched.or_else(|| story_node(mind, &word)).or_else(|| story_node(mind, &singular(&word))).or_else(|| described(&word)).or_else(|| done(&word)).or_else(|| done(&super::mind::verb_stem(mind, &word))).or_else(|| plural(&word)).or_else(|| seeded(&word)).or_else(|| seeded(&singular(&word))).or_else(|| valued(&word)).or_else(|| mind.tree.named(&word).find(|&n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n)));
            if node.is_none() {
                mind.at = 0;
            }
            landed_seen(mind, step.act, node);
        }
        WordMove::StepParent => {
            if braced && *mind.tree.node(at).name == *IS_FORM.trim() && own_child(mind, at, TIME_RELATION).is_some() {
                mind.flags.push((FLAG_TIME.to_string(), crate::cursor::bare_name(PAST_TIME)));
            }
            let node = (at != 0).then(|| mind.tree.node(at).parent).filter(|&p| p != 0);
            if node.is_none() {
                mind.at = 0;
            }
            landed_on(mind, step.act, node);
        }
        WordMove::StepNewest => {
            let node = newest_told(mind, false);
            let people: Vec<usize> = if group_pronoun(&heard_text(mind)) && mind.held.is_empty() {
                let named = |c: usize| mind.tree.node(c).link.is_none() && !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && (person(mind, c) || (own_child(mind, c, &step_item(FLAG_DEFINITE)).is_none() && own_child(mind, c, &step_item(FLAG_INDEFINITE)).is_none() && quantity_tag_of(mind, c).is_empty()));
                let placed_name = mind.first_mark.filter(|&t| t < mind.tree.len() && !mind.tree.node(t).gone && mind.tree.node(t).parent != 0 && !mind.tree.node(mind.tree.node(t).parent).name.starts_with(BRACE_OPEN_TEXT) && named(t) && number_of(&mind.tree.node(t).name).is_none());
                let told_person = |n: &usize| !mind.tree.node(*n).gone && mind.tree.node(*n).link.is_none() && person(mind, *n) && open_question(mind).is_none_or(|q| !inside(mind, *n, q));
                let fellows = |p: usize| -> Vec<usize> {
                    let holder = mind.tree.node(p).parent;
                    let siblings: Vec<usize> = if holder == 0 { Vec::new() } else { present_children(mind, holder).into_iter().filter(|&c| named(c)).collect() };
                    if siblings.len() > 1 {
                        return siblings;
                    }
                    let place_name = |n: usize| super::lookup::went_to(mind, n).map(|place| crate::cursor::bare_name(&mind.tree.node(place).name));
                    match place_name(p) {
                        Some(place) => (mind.tree.state..mind.tree.len()).filter(|&m| !mind.tree.node(m).gone && named(m) && place_name(m).as_deref() == Some(place.as_str())).collect(),
                        None => siblings,
                    }
                };
                let together = |n: &usize| fellows(*n).len() > 1;
                let newest = (mind.tree.state..mind.tree.len()).rev().filter(told_person).find(together).or_else(|| (mind.tree.state..mind.tree.len()).rev().find(told_person)).or(placed_name);
                let topic_fits = mind.first_mark.is_none_or(|t| t < mind.tree.len() && (person(mind, t) || Some(t) == placed_name || present_children(mind, t).into_iter().any(|c| person(mind, c))));
                newest.filter(|_| topic_fits).map(fellows).unwrap_or_default()
            } else {
                Vec::new()
            };
            let owners: Vec<usize> = if group_pronoun(&heard_text(mind)) && mind.held.is_empty() && people.len() < super::mind::LIST_HALVES { node.and_then(|n| own_child(mind, n, OWNER_TAG)).map(|tag| present_children(mind, tag).into_iter().filter_map(|m| mind.tree.node(m).link).collect()).unwrap_or_default() } else { Vec::new() };
            if owners.len() > 1 {
                mind.held = owners.clone();
                mind.flags.push((super::mind::FLAG_GROUP.to_string(), TRUE_TAG.to_string()));
                landed_on(mind, step.act, owners.last().copied());
                return true;
            }
            if people.len() > 1 && people.iter().all(|&p| mind.tree.node(p).parent != 0 || super::lookup::went_to(mind, p).is_some()) {
                mind.held = people.clone();
                mind.flags.push((super::mind::FLAG_GROUP.to_string(), TRUE_TAG.to_string()));
                landed_on(mind, step.act, people.last().copied());
                return true;
            }
            landed_on(mind, step.act, node);
        }
        WordMove::StepTop => {
            let node = newest_for(mind, true, &heard_text(mind));
            landed_on(mind, step.act, node);
        }
        _ => return false,
    }
    true
}
because!(carried, WordWorld, "the moves that take the cursor and what he holds: the activity move at the word to takes a past verb written as a deed off the one who did it, leaves the verb as a bare relation of theirs so a question that names it finds them, and opens their activity for what they loved to do; he grabs a thing, drops it where it now stands, hands it on, steps to the speaker or to the newest thing told, and finds the thing a question names");

fn dropped(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool) {
    let word = word.to_string();
    let held = mind.held.last().copied().filter(|&h| !mind.tree.node(h).gone);
    let stands = super::mind::THING_PRONOUNS.contains(&word.as_str()) || super::mind::PERSON_PRONOUNS.contains(&word.as_str()) || super::mind::OBJECT_PRONOUNS.iter().any(|(p, _)| *p == word) || super::mind::SELF_WORDS.contains(&word.as_str()) || super::mind::YOU_WORDS.contains(&word.as_str());
    let named = plain && { let name = &*mind.tree.node(at).name; *name == *word || *name == *singular(&word) || mark_word(&word) || flag_of(mind, FLAG_HAND) == Some(TRUE_TAG) || stands };
    let Some(held) = held.filter(|_| named) else { landed_on(mind, step.act, None); return; };
    if flag_of(mind, super::mind::FLAG_WITH).is_some() && super::lookup::went_to(mind, held).is_some() {
        if at != held && !inside(mind, at, held) {
            mind.tree.moved(at, None);
            mind.tree.moved(at, Some(held));
        }
        super::writing::went_along(mind, held, at);
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(held));
        return;
    }

    if flag_of(mind, FLAG_QUANTITY).is_some_and(|q| number_of(q) == Some(f32::default())) && flag_of(mind, super::mind::FLAG_PLACE).is_some() && held != at {
        let mention = mind.tree.linked(at, held, false);
        added_under(mind, mention, &quantity_tag_named(f32::default()), false);
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(held));
        return;
    }
    if let Some(deed) = moved_without_place(mind).filter(|_| held != at) {
        let relation = added_under(mind, held, &step_item(&deed), false);
        mind.tree.moved(at, None);
        mind.tree.moved(at, Some(relation));
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(held));
        return;
    }
    if let Some(role) = flag_of(mind, super::mind::FLAG_ROLE).map(str::to_string) {
        let relation = added_under(mind, at, &step_item(&role), false);
        mind.tree.linked(relation, held, false);
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(at));
        mind.second_mark = Some(at);
        return;
    }
    if flag_of(mind, super::mind::FLAG_LEAVE).is_some() {
        if inside(mind, held, at) && held != at {
            let outside = mind.tree.node(at).parent;
            mind.tree.moved(held, None);
            mind.tree.moved(held, Some(outside));
            traced(mind, held, at);
        }
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(held));
        return;
    }
    if flag_of(mind, super::mind::FLAG_WITH).is_some() {
        let subject_place = mind.tree.node(held).parent;
        let (moved, into) = if subject_place != 0 && subject_place != at { (at, subject_place) } else { (held, mind.tree.node(at).parent) };
        if into != 0 && moved != into && !inside(mind, into, moved) {
            let from = mind.tree.node(moved).parent;
            mind.tree.moved(moved, None);
            mind.tree.moved(moved, Some(into));
            traced(mind, moved, from);
        }
        if flag_of(mind, FLAG_GIVE).is_some() && at != held {
            let tag = added_under(mind, at, OWNER_TAG, false);
            mind.tree.linked(tag, held, false);
        }
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(at));
        return;
    }
    let giving = flag_of(mind, FLAG_GIVE).is_some();
    if flag_of(mind, FLAG_GIVE).is_some_and(|said| HAVING.contains(&said)) && activity_word(mind, &mind.tree.node(at).name.to_string()) && mind.tree.story(at) {
        let name = mind.tree.node(at).name.to_string();
        let holders: Vec<usize> = if flag_of(mind, super::mind::FLAG_GROUP).is_some() { mind.held.clone() } else { vec![held] };
        mind.tree.moved(at, None);
        let mut last = held;
        for holder in holders {
            let activity = added_under(mind, holder, &step_item(super::mind::ACTIVITY), false);
            last = added_under(mind, activity, &name, true);
        }
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(last));
        return;
    }
    let taking = flag_of(mind, super::mind::FLAG_TAKE).is_some();
    let releasing = flag_of(mind, super::mind::FLAG_RELEASE).is_some();
    let (moved, into) = if releasing { (at, super::lookup::place_of(mind, held).unwrap_or_else(|| mind.tree.node(held).parent)) } else if giving || taking || flag_of(mind, FLAG_CONTAIN).is_some() { (at, held) } else { (held, at) };
    if moved == into || inside(mind, into, moved) {
        landed_on(mind, step.act, None);
        return;
    }
    if flag_of(mind, super::mind::FLAG_GROUP).is_some() && !giving {
        let group: Vec<usize> = mind.held.iter().copied().filter(|&h| h != into && !inside(mind, into, h)).collect();
        let handed = flag_of(mind, FLAG_HAND).is_some();
        if super::mind::LINES.contains(&&*mind.tree.node(into).name) {
            let members: Vec<usize> = mind.held.clone();
            for (place, &member) in members.iter().enumerate() {
                let is = added_under(mind, member, IS_FORM.trim(), false);
                if let Some((ordinal, _)) = super::mind::LETTER_PLACES.iter().find(|(_, before)| *before == place) {
                    added_under(mind, is, ordinal, true);
                }
                if place + 1 == members.len() {
                    added_under(mind, is, super::mind::LAST_PLACE, true);
                }
            }
        }
        for member in group {
            let from = mind.tree.node(member).parent;
            mind.tree.moved(member, None);
            mind.tree.moved(member, Some(into));
            traced(mind, member, from);
            if handed {
                if let Some(old) = own_child(mind, member, OWNER_TAG) {
                    mind.tree.moved(old, None);
                }
                let tag = added_under(mind, member, OWNER_TAG, false);
                mind.tree.linked(tag, into, false);
            }
        }
        mind.first_mark = Some(held);
        mind.second_mark = Some(held);
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(held));
        return;
    }
    let doing = mind.tree.node(moved).parent != 0 && *mind.tree.node(mind.tree.node(moved).parent).name == *step_item(super::mind::ACTIVITY) || claim_node(mind, moved);
    if doing && !giving && !taking {
        let mention = mind.tree.linked(into, moved, false);
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(mention));
        return;
    }
    mind.flags.retain(|(k, _)| k != super::mind::FLAG_PLACE);
    mind.first_mark = Some(moved);
    mind.second_mark = Some(moved);
    let later = flag_of(mind, FLAG_TIME).is_some_and(|t| t == crate::cursor::bare_name(crate::cursor::LATER_TIME));
    if flag_of(mind, FLAG_TIME).is_some() && !later && !giving && !taking {
        traced(mind, moved, into);
        mind.held.clear();
        mind.flags.clear();
        landed_on(mind, step.act, Some(moved));
        return;
    }
    let placed = mind.tree.node(moved).parent != 0 && (giving || place_of(mind, moved).is_some());
    let handing = flag_of(mind, FLAG_HAND).is_some();
    if !((giving || handing) && placed) {
        let from = mind.tree.node(moved).parent;
        mind.tree.moved(moved, None);
        mind.tree.moved(moved, Some(into));
        traced(mind, moved, from);
        if let Some(when) = flag_of(mind, super::mind::FLAG_WHEN).map(str::to_string) {
            timed_at(mind, moved, &when);
        }
    }
    if later {
        timed_at(mind, moved, crate::cursor::LATER_TIME);
    }
    if releasing {
        if let Some(old) = own_child(mind, moved, OWNER_TAG) {
            mind.tree.moved(old, None);
        }
        let guess = added_under(mind, moved, ASSUMED_TAG, false);
        added_under(mind, guess, TRUE_TAG, true);
    }
    let giver = flag_of(mind, FLAG_HAND).filter(|g| *g != TRUE_TAG).map(str::to_string);
    let given = present_children(mind, moved).into_iter().find_map(|q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(q).name)).flatten());
    let stock = giver.as_deref().and_then(|g| story_node(mind, g)).zip(given).and_then(|(from, part)| held_or_owned(mind, from).into_iter().find(|&c| c != moved && *mind.tree.node(c).name == *mind.tree.node(moved).name && count_of(mind, c) as f32 >= part).map(|c| (c, part)));
    if let Some((rest, part)) = stock {
        let left = count_of(mind, rest) as f32 - part;
        for tag in present_children(mind, rest).into_iter().filter(|&q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG)).collect::<Vec<_>>() {
            mind.tree.moved(tag, None);
        }
        added_under(mind, rest, &quantity_tag_named(left), false);
    }
    if let Some(giver) = giver.filter(|_| own_child(mind, moved, OWNER_TAG).is_none()).and_then(|g| story_node(mind, &g)).filter(|&g| g != into && !inside(mind, moved, g)) {
        traced(mind, moved, giver);
    }
    if giving || taking || flag_of(mind, FLAG_HAND).is_some() {
        if let Some(old) = own_child(mind, moved, OWNER_TAG) {
            let before = present_children(mind, old).into_iter().find_map(|m| mind.tree.node(m).link);
            mind.tree.moved(old, None);
            if let Some(before) = before.filter(|&b| b != into && !present_children(mind, b).into_iter().any(|c| mind.tree.node(c).link == Some(moved))) {
                traced(mind, moved, before);
            }
        }
        let tag = added_under(mind, moved, OWNER_TAG, false);
        mind.tree.linked(tag, into, false);
        if flag_of(mind, FLAG_GIVE).is_some_and(|said| said != TRUE_TAG && super::mind::past_form(mind, said)) {
            dated(mind, tag);
        }
        let sharers: Vec<usize> = if giving && flag_of(mind, super::mind::FLAG_GROUP).is_some() { mind.held.iter().copied().filter(|&h| h != into && h != moved && !mind.tree.node(h).gone).collect() } else { Vec::new() };
        for sharer in sharers {
            mind.tree.linked(sharer, moved, false);
            mind.tree.linked(tag, sharer, false);
        }
    }
    mind.held.clear();
    mind.flags.clear();
    landed_on(mind, step.act, Some(moved));
}
because!(dropped, WordWorld, "the drop: the held thing goes inside the thing that appeared, or with a give or a hold the thing that appeared goes to the held one, a role that waits is written as a relation of the thing that appears, a leave takes the held thing out of the place, a thing told with no place keeps its deed, and what moved or changed hands leaves its trace where it was");

fn joined(mind: &mut CursorMind, step: &WordStep, at: usize, plain: bool) {
    if !plain {
        landed_on(mind, step.act, None);
        return;
    }
    let holder = mind.tree.node(at).parent;
    let contained_now = mind.held.is_empty() && mind.first_mark == Some(at) && holder != 0 && mind.before.iter().any(|said| super::mind::verb_base(mind, said).is_some_and(|base| super::mind::CONTAINING.contains(&base.as_str())));
    if contained_now {
        mind.held = vec![holder];
        mind.flags.push((FLAG_CONTAIN.to_string(), TRUE_TAG.to_string()));
        landed_on(mind, step.act, Some(at));
        return;
    }
    let placed_now = mind.held.is_empty() && mind.first_mark == Some(at) && owner_of(mind, at).is_none() && place_of(mind, at).is_some();
    if placed_now {
        mind.at = 0;
        mind.flags.clear();
        landed_on(mind, step.act, None);
        return;
    }
    match owner_of(mind, at).filter(|_| mind.held.is_empty() && mind.first_mark == Some(at)) {
        Some(owner) => {
            mind.held = vec![owner];
            mind.flags.push((FLAG_GIVE.to_string(), TRUE_TAG.to_string()));
        }
        None => {
            if !mind.held.contains(&at) {
                mind.held.push(at);
            }
            if flag_of(mind, super::mind::FLAG_GROUP).is_none() {
                mind.flags.push((super::mind::FLAG_GROUP.to_string(), TRUE_TAG.to_string()));
            }
        }
    }
    landed_on(mind, step.act, Some(at));
}
because!(joined, WordWorld, "the join at an and or a with: the thing the cursor stands on is held beside what is held already under the group flag, so the things named together land together; when it is the first thing said and nothing is held, its owner is held to be given the next thing, its holder when a word of containing was said, and a thing that only stands in a place is let go so the next thing starts from the world");
