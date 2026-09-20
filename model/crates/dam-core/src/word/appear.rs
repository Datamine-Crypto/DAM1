use super::mind::{mark_word, place_word, FLAG_HAND, HAVING, open_question, flag_of, is_number, kind_of, noun_word, quality_word, singular, verb_base, COPULA, FLAG_INDEFINITE, FLAG_GIVE, FLAG_CONTAIN, FLAG_PROPERTY, FLAG_QUANTITY, FLAG_DEFINITE, FLAG_TIME};
use super::WordReading;
use crate::cursor::{step_item, CursorMind, BRACE_OPEN_TEXT, TIME_RELATION};
use crate::quiz::IS_FORM;
use crate::words::number_of;
use patterns::because;
use super::physics::{WordWorld, TRUE_TAG, OWNER_TAG, CAUSE_TAG, NEAR_DAYS, GENDER_TAG, PLACE_DEPTH};
use super::lookup::{present_children, tag_value, story_node, story_nodes, unseeded, holds, activity_word, has_relation, told_thing, own_child, kept_flag, newest_told, newest_for, inside, count_of, who_has, owner_of, held_or_owned, named_result};
use super::written::{thing_made, added_under, flagged, dated};

fn kin_alone(mind: &mut CursorMind, word: &str) -> Option<usize> {
    let at = mind.at;
    let named = crate::cursor::bare_name(&mind.tree.node(at).name);
    let under = mind.tree.node(at).parent;
    let kin_of = |n: usize| { let name = crate::cursor::bare_name(&mind.tree.node(n).name); mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && super::mind::KIN.contains(&name.as_str()) };
    let bare = at != 0 && mind.tree.story(at) && (kin_of(at) && present_children(mind, at).is_empty() || under != 0 && kin_of(under) && mind.tree.node(at).link.is_none());
    let named = if kin_of(at) { named } else { mind.tree.node(at).name.to_string() };
    let moving = place_word(word) || super::mind::verb_base(mind, word).is_some_and(|base| super::mind::MOVING.contains(&base.as_str()) || super::mind::POSING.contains(&base.as_str()));
    if !bare || !moving || !mind.held.is_empty() || open_question(mind).is_some() {
        return None;
    }
    let relation = if kin_of(at) { at } else { under };
    let value = !kin_of(at);
    let person = thing_made(mind, &named);
    if at != person && value {
        mind.tree.moved(at, None);
    }
    mind.tree.linked(relation, person, false);
    mind.first_mark = Some(person);
    mind.at = person;
    None
}
because!(kin_alone, WordWorld, "a person named only by what they are to someone, my sister, becomes a thing of the story when the sentence goes on to say where they are or where they went: a thing of that name, or of the name the relation was given, my sister anna, under the world which the relation names, so the sister can be put in a place and found there");

pub fn passive_deed(mind: &CursorMind) -> Option<(usize, Option<usize>, Option<usize>, String)> {
    let at = mind.at;
    if at == 0 || !mind.held.is_empty() || open_question(mind).is_some() || !mind.tree.story(at) {
        return None;
    }
    let said = mind.before.iter().rev().find(|b| *b != super::mind::PASSIVE_MARK)?.clone();
    let verb_said = |w: &str| !quality_word(mind, w) && (super::mind::past_form(mind, w) || w.ends_with(super::mind::PAST_END) || super::mind::verb_base(mind, w).is_some());
    let is_of_thing = |is: usize| is != 0 && mind.tree.node(is).parent != 0 && *mind.tree.node(is).name == *IS_FORM.trim();
    let above = mind.tree.node(at).parent;
    if is_of_thing(at) && verb_said(&said) {
        return Some((mind.tree.node(at).parent, None, Some(at), said));
    }
    if is_of_thing(above) && *mind.tree.node(at).name == *said && verb_said(&said) {
        return Some((mind.tree.node(above).parent, Some(at), Some(above), said));
    }
    let bare = *mind.tree.node(at).name == *step_item(&super::mind::verb_stem(mind, &said)) && present_children(mind, at).is_empty();
    let told_of = mind.first_mark.filter(|&t| t < mind.tree.len() && !mind.tree.node(t).gone && t != above).unwrap_or(above);
    (bare && above != 0 && !mind.tree.node(above).name.starts_with(BRACE_OPEN_TEXT) && verb_said(&said)).then_some((told_of, Some(at), None, said))
}
because!(passive_deed, WordReading, "the deed a thing is said to have had done to it, the telephone was invented, when by or the doer after it is heard: the thing it was done to, the is of that thing when the cursor stands on or under it, the node the deed was written as when it was, a value under the is or a bare verb under what the thing is said to be, and the verb form said last, which by after it turns into the deed of the doer named next");

fn done_by(mind: &mut CursorMind, word: &str) -> Option<usize> {
    if mind.before.last().is_none_or(|b| b != super::mind::PASSIVE_MARK) || !noun_word(mind, word) {
        return None;
    }
    let (subject, deed, is, said) = passive_deed(mind)?;
    let verb = super::mind::verb_stem(mind, &said);
    if let Some(deed) = deed {
        for kept in present_children(mind, deed) {
            mind.tree.moved(kept, None);
            if let Some(is) = is {
                mind.tree.moved(kept, Some(is));
            }
        }
        mind.tree.moved(deed, None);
    }
    if let Some(is) = is.filter(|&is| present_children(mind, is).into_iter().all(|c| *mind.tree.node(c).name == *TIME_RELATION)) {
        mind.tree.moved(is, None);
    }
    mind.flags.clear();
    let doer = story_node(mind, word).unwrap_or_else(|| thing_made(mind, word));
    let relation = added_under(mind, doer, &step_item(&verb), false);
    let done_to = mind.tree.linked(relation, subject, false);
    dated(mind, done_to);
    mind.first_mark = Some(doer);
    Some(doer)
}
because!(done_by, WordWorld, "a thing named right after by, when the cursor stands on a deed a thing had done to it, is the doer: the deed is written under the doer as the verb with the thing under it, in the past, as bell invented the telephone is, and what the thing was said to be beside the deed stays under its is, superman is a superhero created by siegel");

fn cause_tied(mind: &mut CursorMind) {
    let Some(told) = mind.cause_of.take().filter(|&t| t < mind.tree.len() && !mind.tree.node(t).gone) else { return };
    let cause = mind.first_mark.or((mind.at != 0).then_some(mind.at)).filter(|&c| c != told && c < mind.tree.len() && !mind.tree.node(c).gone && !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT));
    if let Some(cause) = cause {
        let why = added_under(mind, told, CAUSE_TAG, false);
        mind.tree.linked(why, cause, false);
    }
}
because!(cause_tied, WordWorld, "at the end of a sentence that said because, the thing the first clause told of holds the thing the clause after because told of under because, so why it is so can be asked and a person sees the cause as a line");

fn smaller_group(mind: &mut CursorMind, group: usize) -> Option<usize> {
    let part = flag_of(mind, FLAG_QUANTITY).and_then(number_of)?;
    if part <= f32::default() || (count_of(mind, group) as f32) <= part || open_question(mind).is_some() {
        return None;
    }
    let name = mind.tree.node(group).name.to_string();
    mind.flags.retain(|(k, _)| k == FLAG_QUANTITY);
    let some = mind.tree.added(group, &name);
    flagged(mind, some);
    mind.first_mark = Some(some);
    Some(some)
}
because!(smaller_group, WordWorld, "a count of a counted group, three of the cars or two of them, is a smaller group of the same name inside the group with the count said, which what is said next is told of, so five cars hold three red cars and stay five");

fn joined_noun(mind: &mut CursorMind, word: &str) -> Option<Option<usize>> {
    let said = mind.before.len();
    let first = mind.before.last().cloned()?;
    let article_at = |back: usize| said > back && super::mind::ARTICLE_FLAGS.contains(&mind.before[said - 1 - back].as_str());
    let counted = said > 1 && super::mind::is_number(mind, &mind.before[said - 1 - 1]).is_some() && article_at(super::mind::LIST_HALVES);
    let articled = article_at(1) || counted;
    if !articled || first == word || open_question(mind).is_some() || flag_of(mind, FLAG_QUANTITY).is_some() || super::mind::role_word(mind, &first) || super::mind::KIN.contains(&first.as_str()) {
        return None;
    }
    let plain_thing = |w: &str| { let classes = super::mind::word_classes(mind, w); classes.contains(&super::mind::THING_CLASS) && !quality_word(mind, w) && super::mind::verb_base(mind, w).is_none() && !super::mind::past_form(mind, w) && !mind.tree.named(w).any(|m| !mind.tree.node(m).gone && own_child(mind, m, GENDER_TAG).is_some()) };
    if !plain_thing(&first) || !plain_thing(word) || first.ends_with(super::mind::SUPERLATIVE_END) {
        return None;
    }
    let made = (mind.sentence_from..mind.tree.len()).rev().find(|&n| !mind.tree.node(n).gone && *mind.tree.node(n).name == *first && !tag_value(mind, n) && mind.tree.node(n).link.is_none());
    let Some(made) = made else {
        let placed = story_nodes(mind, &first).into_iter().flat_map(|old| present_children(mind, old)).find(|&m| mind.tree.node(m).link.is_none() && !mind.tree.node(m).name.starts_with(BRACE_OPEN_TEXT) && mind.before.iter().any(|said| **said == *mind.tree.node(m).name))?;
        let whole = thing_made(mind, word);
        flagged(mind, whole);
        mind.tree.moved(placed, None);
        mind.tree.moved(placed, Some(whole));
        let of = added_under(mind, whole, &step_item(super::mind::TOWARD), false);
        added_under(mind, of, &first, true);
        return Some(None);
    };
    let holder = mind.tree.node(made).parent;
    let whole = mind.tree.added(holder, word);
    if holder == 0 {
        mind.tree.adopted(whole);
    }
    for child in present_children(mind, made) {
        mind.tree.moved(child, None);
        mind.tree.moved(child, Some(whole));
    }
    mind.tree.moved(made, None);
    let of = added_under(mind, whole, &step_item(super::mind::TOWARD), false);
    let part = added_under(mind, of, &first, true);
    if counted {
        let tags: Vec<usize> = present_children(mind, whole).into_iter().filter(|&c| mind.tree.node(c).name.starts_with(crate::cursor::QUANTITY_TAG)).collect();
        for tag in tags {
            mind.tree.moved(tag, None);
            mind.tree.moved(tag, Some(part));
        }
    }
    for mark in [&mut mind.first_mark, &mut mind.second_mark, &mut mind.last_topic] {
        if *mark == Some(made) {
            *mark = Some(whole);
        }
    }
    for held in mind.held.iter_mut().filter(|held| **held == made) {
        *held = whole;
    }
    if mind.at == made {
        return Some(Some(whole));
    }
    Some(None)
}
because!(joined_noun, WordWorld, "a noun said right after a noun the sentence made after an article, the car garage, the tool box, names one thing by the last noun, and when the first noun is a thing the story already had, what the sentence named and put into it goes into a new thing of the last noun and the old thing stays as it was: the thing the first noun made takes the new name, keeps what was put into it and what it has, and holds the first noun under of, a garage of car, with the count said before it when there is one, a five car garage; the answer is the thing to step onto when the cursor stood on the old one, or none when the cursor stays");

fn switched(mind: &mut CursorMind) {
    let Some(&thing) = mind.held.last() else { return };
    let said = mind.before.last().cloned().unwrap_or_default();
    let Some((state, undone)) = super::mind::SWITCHED.iter().find(|(word, _)| **word == *said) else { return };
    if thing >= mind.tree.len() || mind.tree.node(thing).gone || mind.tree.node(thing).name.starts_with(BRACE_OPEN_TEXT) {
        return;
    }
    let is = added_under(mind, thing, IS_FORM.trim(), false);
    let others: Vec<usize> = present_children(mind, is).into_iter().filter(|&value| *mind.tree.node(value).name == **undone).collect();
    for other in others {
        mind.tree.moved(other, None);
    }
    added_under(mind, is, state, true);
    mind.held.clear();
    mind.flags.clear();
    mind.first_mark = Some(thing);
}
because!(switched, WordWorld, "a word of working said last with a thing held and the sentence over, the lamp is on, is a state of that thing: it is written under its is, the state it undoes is taken away and the thing is let go");

fn went_off(mind: &mut CursorMind) {
    let Some(&doer) = mind.held.last().filter(|_| !mind.before.iter().any(|said| place_word(said) || super::mind::TOWARD == said)) else { return };
    let said = mind.before.iter().rev().find_map(|word| super::mind::verb_base(mind, word).map(|base| (word.clone(), base)));
    let Some((said, base)) = said.filter(|(_, base)| super::mind::MOVING.contains(&base.as_str())) else { return };
    if doer >= mind.tree.len() || mind.tree.node(doer).gone || mind.tree.node(doer).name.starts_with(BRACE_OPEN_TEXT) {
        return;
    }
    let activity = added_under(mind, doer, &step_item(super::mind::ACTIVITY), false);
    let plain = crate::cursor::told_past(&mind.tree, &said).map(|n| mind.tree.node(n).name.to_string()).unwrap_or_else(|| super::mind::verb_stem(mind, &base));
    let value = added_under(mind, activity, &plain, true);
    if super::mind::past_form(mind, &said) || said.ends_with(super::mind::PAST_END) {
        dated(mind, value);
    }
    mind.held.clear();
    mind.flags.clear();
    mind.first_mark = Some(doer);
}
because!(went_off, WordWorld, "a verb of moving that took hold of its doer and said no place word by the end of the sentence, tom walked away, is what the doer did: it is written under their activity, in the past when the verb was said so, and they are let go");

fn done_alone(mind: &mut CursorMind) {
    let at = mind.at;
    let doer = mind.tree.node(at).parent;
    let name = crate::cursor::bare_name(&mind.tree.node(at).name);
    let bare_verb = at != 0 && doer != 0 && mind.tree.story(at) && mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) && present_children(mind, at).is_empty();
    if !bare_verb || !activity_word(mind, &name) {
        return;
    }
    let said = mind.before.last().cloned().unwrap_or_default();
    mind.tree.moved(at, None);
    let activity = added_under(mind, doer, &step_item(super::mind::ACTIVITY), false);
    let value = added_under(mind, activity, &name, true);
    if super::mind::past_form(mind, &said) || said.ends_with(super::mind::PAST_END) {
        dated(mind, value);
    }
}
because!(done_alone, WordWorld, "a verb the sentence ends right after, with nothing it is done to and no place, the bucket fell, the dog barked, is what its doer did when the seeds class it as an activity: it is written under the doer's activity, in the past when the verb was said so, as a swim in the lake is");

pub fn clause_done(mind: &CursorMind) -> bool {
    let at = mind.at;
    if at == 0 || !mind.held.is_empty() || mind.flags.iter().any(|(k, _)| k != super::mind::FLAG_DEFINITE && k != super::mind::FLAG_INDEFINITE) || open_question(mind).is_some() || mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) {
        return false;
    }
    let parent = mind.tree.node(at).parent;
    let value = parent != 0 && mind.tree.node(parent).name.starts_with(BRACE_OPEN_TEXT);
    let placed = mind.second_mark == Some(at) && !value;
    value || placed
}
because!(clause_done, WordReading, "whether the clause said so far is complete: nothing held, no flag set but an article's, and the cursor \
     on a value just written, on the thing the last drop placed or on the thing a role was just given to, so a thing named next starts a \
     new clause and a copula next asks, i have a dog tom has a cat do i have a dog");

fn pending_thing(mind: &mut CursorMind) -> Option<usize> {
    let said = mind.flags.iter().rev().find(|(k, _)| k == FLAG_PROPERTY).map(|(_, v)| v.clone())?;
    let thing = if flag_of(mind, FLAG_QUANTITY).is_some() { singular(&said) } else { said.clone() };
    if let Some(at) = mind.flags.iter().rposition(|(k, v)| k == FLAG_PROPERTY && *v == said) {
        mind.flags.remove(at);
    }
    let node = story_node(mind, &thing).or_else(|| story_node(mind, &singular(&thing)).filter(|_| flag_of(mind, FLAG_DEFINITE).is_some())).unwrap_or_else(|| { let made = thing_made(mind, &thing); flagged(mind, made); made });
    Some(node)
}
because!(pending_thing, WordReading, "the quality still waiting for its noun, made the thing itself when no noun came: the orange is \
     orange, the ring holds a stone");

fn fresh_thing(mind: &CursorMind) -> bool {
    let at = mind.at;
    at != 0 && open_question(mind).is_none() && !mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) && mind.tree.node(at).parent == 0 && (mind.first_mark == Some(at) || mind.held.contains(&at)) && present_children(mind, at).into_iter().all(|c| { let name = &*mind.tree.node(c).name; *name == *step_item(FLAG_DEFINITE) || *name == *step_item(FLAG_INDEFINITE) || name.starts_with(crate::cursor::QUANTITY_TAG) })
}
because!(fresh_thing, WordReading, "whether the cursor stands on a thing just named at the world of which nothing was told yet, so a comma \
     after it lists it and ends no clause");

pub fn listing(mind: &CursorMind, word: &str) -> bool {
    word == super::mind::LIST_MARK && fresh_thing(mind)
}
because!(listing, WordReading, "whether a comma lists the thing the cursor stands on, the key, the pen and the cup");

pub fn during_time(mind: &CursorMind, word: &str) -> bool {
    flag_of(mind, super::mind::FLAG_WHEN) == Some(super::mind::DURING) && mind.at == 0 && mind.held.is_empty() && noun_word(mind, word) && !super::mind::ARTICLE_FLAGS.contains(&word)
}
because!(during_time, WordReading, "whether a noun is the stretch of time said after during, which is when the sentence holds and no thing \
     of the world");

pub fn class_after_quality(mind: &CursorMind, heard: usize) -> bool {
    let at = mind.at;
    if at == 0 || !mind.held.is_empty() || mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) {
        return false;
    }
    let kind = mind.tree.node(at).parent;
    let is = mind.tree.node(kind).parent;
    let under_kind = kind != 0 && is != 0 && mind.tree.node(kind).name.starts_with(BRACE_OPEN_TEXT) && *mind.tree.node(is).name == *IS_FORM.trim();
    let said = mind.before.len().saturating_sub(heard);
    under_kind && said > 1 && super::mind::ARTICLE_FLAGS[1..].contains(&mind.before[said - super::mind::LIST_HALVES].as_str()) && *mind.before[said - 1] == *mind.tree.node(at).name
}
because!(class_after_quality, WordReading, "whether the cursor stands on a quality just said after an article, counted back past the words \
     heard since, so the noun that follows is the class the thing is and no thing of its own, a kitten is a young cat");

pub fn rounding_held(mind: &CursorMind) -> bool {
    mind.held.last().is_some_and(|&h| !mind.tree.node(h).gone && *mind.tree.node(h).name == *super::mind::ROUNDING)
}
because!(rounding_held, WordReading, "whether the thing held is the rounding, so the number said next is the place it is dropped in");

pub fn rounding_here(mind: &CursorMind) -> bool {
    let at = mind.at;
    at != 0 && (*mind.tree.node(at).name == *super::mind::ROUNDING || present_children(mind, at).into_iter().any(|c| *mind.tree.node(c).name == *super::mind::ROUNDING))
}
because!(rounding_here, WordReading, "whether the cursor stands on the rounding or on the number that holds it, where decimal and places \
     name no thing");

fn givers_group(mind: &CursorMind, least: usize) -> Option<usize> {
    let giver = flag_of(mind, FLAG_HAND).and_then(|g| story_node(mind, g))?;
    held_or_owned(mind, giver).into_iter().rev().find(|&c| count_of(mind, c) >= least && present_children(mind, c).into_iter().any(|t| mind.tree.node(t).name.starts_with(crate::cursor::QUANTITY_TAG)))
}
because!(givers_group, WordReading, "the counted group of the one who gives that a count given alone is of: the newest group the giver \
     holds with at least that many");

pub fn appeared(mind: &mut CursorMind, word: &str) -> Option<usize> {
    if word == super::mind::PASSIVE_MARK && passive_deed(mind).is_some() {
        return None;
    }
    if mind.at == 0 && mind.held.is_empty() && open_question(mind).is_none() && super::mind::opens_story(&mind.before, word) {
        return None;
    }
    if NEAR_DAYS.contains(&word) && mind.at != 0 && mind.held.is_empty() && open_question(mind).is_none() && clause_done(mind) {
        return None;
    }
    let given_count = flag_of(mind, FLAG_QUANTITY).and_then(number_of).filter(|_| word == super::mind::INFINITIVE && mind.held.is_empty() && open_question(mind).is_none());
    if let Some(name) = given_count.and_then(|count| givers_group(mind, count as usize)).map(|group| mind.tree.node(group).name.to_string()) {
        return appeared(mind, &name);
    }
    let lone_number = mind.at == 0 && mind.held.is_empty() && open_question(mind).is_none() && mind.flags.len() == 1 && (word == super::mind::LIST_MARK || super::mind::closing_mark(word));
    if let Some(count) = flag_of(mind, FLAG_QUANTITY).map(str::to_string).filter(|_| lone_number) {
        mind.tree.added(0, &count);
        mind.flags.clear();
        return None;
    }
    let part_had = super::mind::closing_mark(word) && open_question(mind).is_none() && !mind.held.is_empty() && flag_of(mind, FLAG_GIVE).is_some() && flag_of(mind, FLAG_QUANTITY).is_some() && flag_of(mind, FLAG_INDEFINITE).is_some() && mind.before.last().is_some_and(|said| number_of(said).is_none() && is_number(mind, said).is_some());
    if part_had {
        let said = mind.before.last().cloned().unwrap_or_default();
        mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
        let made = thing_made(mind, &said);
        flagged(mind, made);
        return Some(made);
    }
    let lost_count = super::mind::closing_mark(word) && open_question(mind).is_none() && !mind.held.is_empty() && flag_of(mind, super::mind::FLAG_RELEASE).is_some_and(|verb| verb != TRUE_TAG) && flag_of(mind, FLAG_QUANTITY).is_some();
    if lost_count {
        let verb = flag_of(mind, super::mind::FLAG_RELEASE).map(str::to_string).unwrap_or_default();
        let count = flag_of(mind, FLAG_QUANTITY).map(str::to_string).unwrap_or_default();
        if let Some(&loser) = mind.held.last() {
            let deed = added_under(mind, loser, &step_item(&verb), false);
            added_under(mind, deed, &count, true);
        }
        mind.held.clear();
        mind.flags.clear();
        mind.at = 0;
        return None;
    }
    let number_told = mind.at == 0 && mind.held.is_empty() && open_question(mind).is_none() && mind.flags.len() == 1 && COPULA.contains(&word) && mind.before.len() == 1;
    if let Some(count) = flag_of(mind, FLAG_QUANTITY).map(str::to_string).filter(|_| number_told) {
        let count = mind.before.first().cloned().unwrap_or(count);
        mind.flags.clear();
        let told = (mind.tree.state..mind.tree.len()).find(|&n| !mind.tree.node(n).gone && mind.tree.node(n).parent == 0 && *mind.tree.node(n).name == *count);
        return Some(match told { Some(n) => n, None => mind.tree.added(0, &count) });
    }
    if listing(mind, word) {
        return None;
    }
    if (super::mind::CLAUSE_BREAKS.contains(&word) || super::mind::closing_mark(word)) && open_question(mind).is_none() && mind.held.is_empty() {
        done_alone(mind);
    }
    if super::mind::closing_mark(word) && open_question(mind).is_none() {
        switched(mind);
        went_off(mind);
        cause_tied(mind);
        let bare: Vec<usize> = (mind.sentence_from..mind.tree.len()).filter(|&n| !mind.tree.node(n).gone && mind.at != n && *mind.tree.node(n).name == *IS_FORM.trim() && present_children(mind, n).is_empty()).collect();
        for is in bare {
            mind.tree.moved(is, None);
        }
    }
    if word == super::mind::CAUSE_WORD && mind.at != 0 && mind.held.is_empty() && open_question(mind).is_none() {
        let mut told = mind.at;
        while told != 0 && (mind.tree.node(told).name.starts_with(BRACE_OPEN_TEXT) || mind.tree.node(mind.tree.node(told).parent).name.starts_with(BRACE_OPEN_TEXT) && mind.tree.node(told).parent != 0) {
            told = mind.tree.node(told).parent;
        }
        mind.cause_of = (told != 0).then_some(told);
        mind.at = 0;
        mind.first_mark = None;
        mind.flags.clear();
        return None;
    }
    let corrects = word == super::mind::CLAUSE_BREAKS[0] && mind.at != 0 && mind.before.iter().any(|b| super::mind::NEGATIONS.contains(&b.as_str()));
    let strengthens = mind.at != 0 && mind.tree.node(mind.at).name.starts_with(BRACE_OPEN_TEXT) && present_children(mind, mind.at).is_empty();
    if super::mind::CLAUSE_BREAKS.contains(&word) && !corrects && !strengthens && open_question(mind).is_none() && mind.held.is_empty() {
        mind.at = 0;
        mind.flags.clear();
        return None;
    }
    if let Some(kin) = kin_alone(mind, word) {
        return Some(kin);
    }
    if flag_of(mind, super::mind::FLAG_KIN).is_some() && !mark_word(word) {
        return None;
    }
    if word == super::mind::SAMENESS && mind.at == 0 {
        return None;
    }
    let of_them = word == super::mind::GROUP_OBJECT && mind.held.is_empty() && mind.at == 0 && flag_of(mind, FLAG_QUANTITY).is_some() && mind.before.last().is_some_and(|b| b == super::mind::TOWARD);
    if let Some(some) = mind.last_topic.filter(|&topic| of_them && topic < mind.tree.len() && !mind.tree.node(topic).gone).and_then(|topic| smaller_group(mind, topic)) {
        return Some(some);
    }
    if let Some(doer) = done_by(mind, word) {
        return Some(doer);
    }
    let weakening = super::mind::WEAK_DEGREES.contains(&word) && flag_of(mind, FLAG_INDEFINITE).is_none();
    if super::mind::DEGREE_WORDS.contains(&word) && !weakening && open_question(mind).is_none() {
        mind.flags.retain(|(k, _)| k != super::mind::FLAG_DEGREE && k != FLAG_INDEFINITE);
        mind.flags.push((super::mind::FLAG_DEGREE.to_string(), word.to_string()));
        return None;
    }
    if super::mind::REFLEXIVES.contains(&word) {
        return None;
    }
    if word == super::mind::LABEL_OPENER && mind.before.is_empty() && mind.at == 0 {
        return None;
    }
    if super::mind::ANOTHER.contains(&word) && !mind.held.is_empty() && open_question(mind).is_none() {
        return None;
    }
    let twin = mind.last_topic.filter(|_| super::mind::ANOTHER.contains(&word) && mind.at == 0 && mind.held.is_empty() && open_question(mind).is_none() && flag_of(mind, FLAG_DEFINITE).is_some()).and_then(|told| {
        let name = mind.tree.node(told).name.to_string();
        story_nodes(mind, &name).into_iter().find(|&n| n != told && mind.tree.node(n).parent == mind.tree.node(told).parent)
    });
    if let Some(other) = twin {
        mind.flags.clear();
        mind.first_mark = Some(other);
        return Some(other);
    }
    if (word == super::mind::THERE_OPENER || super::mind::SEQUENCE_WORDS.contains(&word)) && mind.held.is_empty() && open_question(mind).is_none() && clause_done(mind) {
        mind.at = 0;
        mind.flags.clear();
        return None;
    }
    if super::mind::expression_word(word) {
        return None;
    }
    let on_activity = mind.at != 0 && mind.held.is_empty() && open_question(mind).is_none() && *mind.tree.node(mind.tree.node(mind.at).parent).name == *step_item(super::mind::ACTIVITY);
    if let Some(stood_for) = (word == super::mind::THING_PRONOUNS[0] && on_activity).then(|| newest_for(mind, false, word)).flatten() {
        let of = added_under(mind, mind.at, &step_item(super::mind::TOWARD), false);
        mind.tree.linked(of, stood_for, false);
        return None;
    }
    if class_after_quality(mind, 0) && noun_word(mind, word) && open_question(mind).is_none() {
        return None;
    }
    if during_time(mind, word) {
        mind.flags.retain(|(k, _)| k != super::mind::FLAG_WHEN);
        mind.flags.push((super::mind::FLAG_WHEN.to_string(), word.to_string()));
        return None;
    }
    if word == super::mind::ROUNDING && mind.at == 0 && mind.held.is_empty() && mind.flags.is_empty() && open_question(mind).is_none() {
        let round = story_node(mind, word).filter(|&n| mind.tree.node(n).parent == 0 || mind.tree.story(mind.tree.node(n).parent)).unwrap_or_else(|| thing_made(mind, word));
        return Some(round);
    }
    if rounding_held(mind) && is_number(mind, word).is_some() {
        return Some(thing_made(mind, word));
    }
    if !mind.held.is_empty() && flag_of(mind, super::mind::FLAG_PLACE) == Some(super::mind::HOUR_OPENER) && number_of(word).is_some() && open_question(mind).is_none() {
        let hour = story_node(mind, word).filter(|&n| mind.tree.node(n).parent == 0).unwrap_or_else(|| thing_made(mind, word));
        return Some(hour);
    }
    if rounding_here(mind) && super::mind::ROUNDING_WORDS.contains(&word) {
        return None;
    }
    if super::mind::unknown_letter(word) && mind.at == 0 && mind.held.is_empty() && mind.flags.is_empty() && open_question(mind).is_none() && named_result(mind, word).is_none() {
        return None;
    }
    if super::mind::OBJECT_PRONOUNS.iter().any(|(p, _)| *p == word) {
        return None;
    }
    if word == super::mind::THING_PRONOUNS[0] && mind.at == 0 && mind.held.is_empty() && open_question(mind).is_none() && newest_told(mind, false).is_none() {
        let it = story_node(mind, word).unwrap_or_else(|| thing_made(mind, word));
        mind.first_mark = Some(it);
        return Some(it);
    }
    if (super::mind::KIN.contains(&word) || word == super::mind::NAME_ROLE) && !mind.held.is_empty() && mind.flags.last().is_some_and(|(k, _)| k == FLAG_GIVE) && open_question(mind).is_none() {
        return None;
    }
    if super::mind::TIMES_OF_DAY.contains(&word) && mind.at == 0 && mind.held.is_empty() && open_question(mind).is_none() {
        return None;
    }
    let held = mind.held.last().copied().filter(|&h| !mind.tree.node(h).gone);
    if COPULA.contains(&word) && open_question(mind).is_none() && mind.at == 0 && held.is_none() {
        if let Some(at) = mind.flags.iter().rposition(|(k, _)| k == super::mind::FLAG_WHEN) {
            let (_, when) = mind.flags.remove(at);
            mind.flags.push((FLAG_PROPERTY.to_string(), when));
        }
    }
    let closing = super::mind::BELONGS_IN.contains(&word) || COPULA.contains(&word) || HAVING.contains(&word) || place_word(word) || mark_word(word) || word == super::mind::JOINER || word == crate::quiz::APOSTROPHE || (!noun_word(mind, word) && verb_base(mind, word).is_some());
    let waits = mind.flags.iter().rev().find(|(k, _)| k == FLAG_PROPERTY).is_some_and(|(_, v)| !quality_word(mind, v) && !super::mind::ORDINALS.contains(&v.as_str()));
    let deed = waits && flag_of(mind, FLAG_QUANTITY).is_none() && super::mind::verb_like(mind, word) && !told_thing(mind, word) && {
        let named = |w: &str| { let classes = super::mind::word_classes(mind, w); classes.contains(&super::mind::THING_CLASS) || classes.contains(&super::mind::VALUE_CLASS) };
        let waiting = mind.flags.iter().rposition(|(k, _)| k == FLAG_PROPERTY);
        let articled = mind.flags.iter().position(|(k, _)| k == FLAG_DEFINITE || k == FLAG_INDEFINITE).zip(waiting).is_some_and(|(article, word)| article < word);
        super::mind::word_classes(mind, &singular(word)).contains(&super::mind::VERB_CLASS) || (!named(word) && (!named(&singular(word)) || articled))
    };
    let closing = closing || deed || word == super::mind::LABEL_AS && super::mind::labelling(&mind.before);
    if closing && open_question(mind).is_none() && mind.at == 0 && held.is_none() {
        if let Some(node) = pending_thing(mind) {
            mind.first_mark = Some(node);
            return Some(node);
        }
    }
    if mark_word(word) && open_question(mind).is_none() && held.is_none() && mind.at != 0 && mind.tree.node(mind.at).name.starts_with(BRACE_OPEN_TEXT) {
        let year = flag_of(mind, FLAG_QUANTITY).and_then(number_of).filter(|n| *n >= super::mind::YEAR_LEAST && n.fract().abs() < f32::EPSILON && *mind.tree.node(mind.at).name == *IS_FORM.trim()).map(|n| crate::cursor::worked_text(n, None));
        if let Some(year) = year {
            let is = mind.at;
            if present_children(mind, is).is_empty() {
                mind.tree.moved(is, None);
            }
            let thing = story_node(mind, super::mind::YEAR_NAME).unwrap_or_else(|| thing_made(mind, super::mind::YEAR_NAME));
            let under = added_under(mind, thing, IS_FORM.trim(), false);
            for old in present_children(mind, under) {
                mind.tree.moved(old, None);
            }
            added_under(mind, under, &year, true);
            mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
        }
        if let Some(count) = flag_of(mind, FLAG_QUANTITY).map(str::to_string) {
            let at = mind.at;
            added_under(mind, at, &count, true);
        }
    }
    if mark_word(word) && open_question(mind).is_none() && held.is_none() {
        mind.at = 0;
    }
    if mark_word(word) {
        let placing = [FLAG_GIVE, FLAG_CONTAIN, super::mind::FLAG_PLACE].iter().any(|k| flag_of(mind, k).is_some());
        let pending = mind.flags.iter().rev().find(|(k, _)| k == FLAG_PROPERTY).map(|(_, v)| v.clone());
        if let (Some(thing), true, Some(_)) = (pending, placing, held) {
            if let Some(at) = mind.flags.iter().rposition(|(k, v)| k == FLAG_PROPERTY && *v == thing) {
                mind.flags.remove(at);
            }
            let node = story_node(mind, &thing).or_else(|| story_node(mind, &singular(&thing)).filter(|_| flag_of(mind, FLAG_DEFINITE).is_some())).unwrap_or_else(|| { let made = thing_made(mind, &thing); flagged(mind, made); made });
            return Some(node);
        }
        mind.flags.clear();
        mind.held.clear();
        return None;
    }
    if open_question(mind).is_some() {
        return None;
    }
    let handing = flag_of(mind, FLAG_HAND).is_some() || flag_of(mind, super::mind::FLAG_STATE).is_some();
    let kind_after = mind.at != 0 && (mind.tree.node(mind.at).name.ends_with(super::mind::SUPERLATIVE_END) || super::mind::ORDINALS.contains(&&*mind.tree.node(mind.at).name)) && mind.tree.node(mind.at).parent != 0 && *mind.tree.node(mind.tree.node(mind.at).parent).name == *IS_FORM.trim();
    let measured = kind_after || super::mind::MEASURES.contains(&word) && mind.at != 0 && present_children(mind, mind.at).into_iter().any(|c| mind.tree.node(c).name.starts_with(crate::cursor::QUANTITY_TAG));
    let on_part = mind.at != 0 && mind.first_mark == Some(mind.at) && { let whole = mind.tree.node(mind.at).parent; whole != 0 && !mind.tree.node(whole).name.starts_with(BRACE_OPEN_TEXT) };
    let part_named = on_part && mind.before.iter().rev().take(PLACE_DEPTH).any(|b| b == super::mind::TOWARD) && super::mind::unknown_word(mind, word) && !told_thing(mind, word);
    let describing = mind.at != 0 && mind.tree.node(mind.at).parent != 0 && *mind.tree.node(mind.tree.node(mind.at).parent).name == *IS_FORM.trim() && mind.before.last().is_some_and(|b| *mind.tree.node(mind.at).name == **b && unseeded(mind, b)) && mind.before.iter().rev().nth(1).is_some_and(|b| super::mind::ARTICLE_FLAGS.contains(&b.as_str()));
    if describing && noun_word(mind, word) {
        return None;
    }
    let named_new = super::mind::unknown_word(mind, word) && mind.flags.last().is_some_and(|(k, _)| k == super::mind::FLAG_DEFINITE || k == super::mind::FLAG_INDEFINITE);
    let new_subject = super::mind::PERSON_PRONOUNS.contains(&word) && mind.held.is_empty() && mind.flags.is_empty();
    if (noun_word(mind, word) || named_new || new_subject) && !measured && !part_named && !(super::mind::verb_like(mind, word) && !told_thing(mind, word)) && clause_done(mind) {
        mind.at = 0;
        mind.first_mark = None;
    }
    if !super::mind::ARTICLE_FLAGS.contains(&word) {
        mind.second_mark = None;
    }
    let on_relation = mind.at != 0 && mind.tree.node(mind.at).name.starts_with(BRACE_OPEN_TEXT);
    let month_place = held.is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_some() && super::mind::FILLERS.contains(&word) && has_relation(mind, word, super::mind::ORDER_RELATION);
    if month_place {
        let node = story_node(mind, word).unwrap_or_else(|| thing_made(mind, word));
        return Some(node);
    }
    if noun_word(mind, word) && !on_relation {
        if let Some(joined) = joined_noun(mind, word) {
            return joined;
        }
    }
    if !noun_word(mind, word) || on_relation || (held.is_none() && mind.at != 0 && !handing) {
        return None;
    }
    let waiting = super::mind::unknown_word(mind, word) && !super::mind::role_word(mind, word) && held.is_none() && mind.at == 0 && flag_of(mind, FLAG_QUANTITY).is_none() && flag_of(mind, FLAG_PROPERTY).is_none();
    if (quality_word(mind, word) && (held.is_some() || mind.at == 0) || waiting) && !handing {
        return None;
    }
    let left_place = flag_of(mind, FLAG_QUANTITY).is_some_and(|q| number_of(q) == Some(f32::default())) && flag_of(mind, super::mind::FLAG_PLACE).is_some() && held.is_some_and(|h| story_node(mind, word).is_some_and(|p| p != h && inside(mind, h, p)));
    if left_place {
        mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
        mind.flags.push((super::mind::FLAG_LEAVE.to_string(), TRUE_TAG.to_string()));
    }
    let not_there = !left_place && held.is_some() && flag_of(mind, FLAG_QUANTITY).is_some_and(|q| number_of(q) == Some(f32::default())) && flag_of(mind, super::mind::FLAG_PLACE).is_some();
    if not_there {
        mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
    }
    let placed_when = flag_of(mind, FLAG_TIME).map(str::to_string).filter(|when| *when == crate::cursor::bare_name(crate::cursor::LATER_TIME) && held.is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_some());
    if placed_when.is_some() {
        mind.flags.retain(|(k, _)| k != FLAG_TIME);
    }
    let alike: Vec<usize> = story_nodes(mind, &singular(word)).into_iter().filter(|&n| held.is_none_or(|owner| mind.tree.node(n).parent == owner)).collect();
    let one_of = alike.len() > 1 && open_question(mind).is_none() && flag_of(mind, FLAG_QUANTITY).and_then(number_of).is_some_and(|n| n == f32::from(1u8)) && mind.before.iter().rev().take(super::mind::LIST_HALVES).any(|b| b == super::mind::TOWARD);
    if let Some(&oldest) = alike.last().filter(|_| one_of) {
        mind.flags.clear();
        mind.held.clear();
        mind.first_mark = Some(oldest);
        return Some(oldest);
    }
    let of_group = flag_of(mind, FLAG_QUANTITY).is_some() && held.is_none() && mind.before.iter().rev().take(super::mind::LIST_HALVES).any(|b| b == super::mind::TOWARD) && story_node(mind, &singular(word)).is_some_and(|group| count_of(mind, group) > 1);
    if of_group {
        if let Some(some) = story_node(mind, &singular(word)).and_then(|group| smaller_group(mind, group)) {
            return Some(some);
        }
        mind.flags.retain(|(k, _)| k != FLAG_QUANTITY);
    }
    let part_count = flag_of(mind, FLAG_QUANTITY).and_then(number_of).filter(|_| !of_group && held.is_none() && mind.at == 0 && mind.before.len() == 1);
    let part_of = part_count.and_then(|part| story_node(mind, &singular(word)).filter(|&group| count_of(mind, group) as f32 > part && part > f32::default()));
    if let Some(group) = part_of {
        mind.first_mark = Some(group);
        return Some(group);
    }
    let counted = flag_of(mind, FLAG_QUANTITY).is_some();
    let name = if counted || of_group { singular(word) } else { word.to_string() };
    let another = mind.flags.iter().any(|(k, v)| k == FLAG_PROPERTY && super::mind::ANOTHER.contains(&v.as_str()));
    if another {
        mind.flags.retain(|(k, v)| !(k == FLAG_PROPERTY && super::mind::ANOTHER.contains(&v.as_str())) && k != FLAG_DEFINITE);
    }
    let new_one = another || counted || flag_of(mind, FLAG_INDEFINITE).is_some();
    let class_told = (!another && !counted && held.is_none() && mind.at == 0 && flag_of(mind, FLAG_INDEFINITE).is_some()).then(|| story_node(mind, &name)).flatten().filter(|&n| mind.tree.node(n).parent == 0 && owner_of(mind, n).is_none() && own_child(mind, n, IS_FORM.trim()).is_some() && own_child(mind, n, &step_item(FLAG_INDEFINITE)).is_some() && !present_children(mind, n).into_iter().any(|c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)));
    let possessor = held.filter(|_| flag_of(mind, FLAG_GIVE) == Some(TRUE_TAG));
    let owners_of = |n: usize| -> Vec<usize> { own_child(mind, n, OWNER_TAG).map(|tag| present_children(mind, tag).into_iter().filter_map(|m| mind.tree.node(m).link).collect()).unwrap_or_default() };
    let theirs = |n: usize| possessor.is_none_or(|owner| who_has(mind, n).is_none_or(|has| has == owner) || owners_of(n).iter().any(|o| mind.held.contains(o)));
    let waiting_qualities: Vec<String> = mind.flags.iter().filter(|(k, v)| k == FLAG_PROPERTY && quality_word(mind, v)).map(|(_, v)| v.clone()).collect();
    let clashes = |n: usize| waiting_qualities.iter().any(|q| !holds(mind, n, q) && kind_of(mind, q).is_some_and(|kind| own_child(mind, n, IS_FORM.trim()).and_then(|is| own_child(mind, is, &step_item(&kind))).is_some_and(|told| !present_children(mind, told).is_empty())));
    let waiting_ranks: Vec<String> = mind.flags.iter().filter(|(k, v)| k == FLAG_PROPERTY && super::mind::ORDINALS.contains(&v.as_str()) && !super::mind::ANOTHER.contains(&v.as_str())).map(|(_, v)| v.clone()).collect();
    let ranked_apart = |n: usize| waiting_ranks.iter().any(|rank| { let told: Vec<String> = own_child(mind, n, IS_FORM.trim()).map(|is| present_children(mind, is).into_iter().map(|v| mind.tree.node(v).name.to_string()).filter(|v| super::mind::ORDINALS.contains(&v.as_str())).collect()).unwrap_or_default(); count_of(mind, n) > 1 || (!told.is_empty() && !told.contains(rank)) });
    let known = if class_told.is_some() { class_told } else if new_one { None } else { story_nodes(mind, &name).into_iter().chain(story_nodes(mind, &singular(&name))).find(|&n| theirs(n) && !clashes(n) && !ranked_apart(n)) };
    let node = match known {
        Some(known) => {
            let placed_as: Vec<String> = mind.flags.iter().filter(|(k, v)| k == FLAG_PROPERTY && super::mind::ORDINALS.contains(&v.as_str())).map(|(_, v)| v.clone()).collect();
            for ordinal in placed_as {
                let is = added_under(mind, known, IS_FORM.trim(), false);
                added_under(mind, is, &ordinal, false);
            }
            mind.flags.retain(|(k, _)| kept_flag(k));
            known
        }
        None => { let made = thing_made(mind, &name); flagged(mind, made); made }
    };
    if held.is_none() && !handing {
        mind.first_mark = Some(node);
    }
    if not_there {
        mind.flags.push((FLAG_QUANTITY.to_string(), 0.to_string()));
    }
    if let Some(when) = placed_when {
        mind.flags.push((FLAG_TIME.to_string(), when));
    }
    Some(node)
}
because!(
    appeared,
    WordWorld,
    "what a word does to the world by itself: one of several things of a name, one of my cars, is the oldest of them, and the other with the before it is the twin of the thing the sentence before told of, of the same name under the same holder; a count of a counted group, three of the cars, is a smaller group of that name inside the group, which what is said next is told of; a thing named after by is the doer of the deed the cursor stands on; an is the sentence made and left with nothing under it is taken away at its end, anna is reading a book; another after having names no thing, and label opening an order to label names none, as a word that names the doer again names none; a noun said right after an article and a word the seeds never state, under is, makes no \
     thing, since it is what the subject is, a useful device, to said after a count given alone names the group for it after the giver's \
     newest counted group, she gave two to max, a count alone after a verb of losing is written under the verb on the one who lost, she \
     loses two, a filler the seeds hold in an order, said while a thing is held to be placed, is a place in time, the race is in may, an \
     ordinal said before a thing the story knows is written on that thing, the second bag, an unknown word right after a part named with \
     of is a deed of the part and sends the cursor nowhere, the wheel of the bike broke, a noun after an article and a quality of a copula \
     is the class and no thing, a young cat, a number word said last after a word of having and an article is a thing the owner has, tom \
     has a half, a thing named after a possessive is one its owner has or one nobody has, never another owner's, max's dog beside tom's \
     dog, a count said first before a group the story counted higher is a part of that group, which the cursor goes to with the count \
     still flagged, five students are absent, but after a not of the sentence keeps the cursor on the thing, whose right place follows, \
     not in the house but in the garden, a number said first and alone before a copula is a thing of the world the sentence tells of, \
     named as it was said, seven is a number, a thing said with a again at the start of a sentence, when the story told one of that name \
     only what it is, is that same thing, a fern is alive and a fern is a plant, it said on an activity is what the activity is of, the \
     thing it stands for mentioned under of, he loves to ride it, there, then, after or following said after a finished clause sends the \
     cursor back to the world, as a word that parts two clauses does, a count and of before a group the story counted is that group and no \
     new thing, so what is said of one of the balls is written on the balls, the time to come flagged while a thing is held to be placed \
     is the time of the placing and is left for the drop, never taken by the place that appears, the cat will be in the garden, a number \
     in digits said after at while a thing is held is an hour, a thing of the world to drop it in, a noun after during is the time the \
     sentence holds at, flagged in place of during, and no thing, a place said after not that the thing held does not stand in is the \
     place itself, never one counted none, and the count is left for the drop, a place the thing held stands in, said after not, is that \
     place and the thing leaves it, the dog is not in the garden, a number said alone before a comma or a closing mark is a thing of the \
     world, one of a run of numbers, round said first is the rounding, a thing, the number said while it is held is a thing of its own to \
     drop it in, and decimal and places after it are none, a bracket, the power sign or a function name is no thing, a lone letter said \
     first at the world is the unknown of an equation and no thing appears for it, a comma after a thing just named lists it and changes \
     nothing, a word that parts two clauses sends the cursor back to the world, other before a noun makes a new thing of the name, of or \
     for after a waiting word makes it the thing, the lid of the box; same before a noun at the world makes nothing appear, the same key, \
     and neither does a pronoun that stands after a verb, him or them; it with nothing told to stand for is a thing of its own, it is \
     three o'clock; a word after a possessive under is makes nothing appear, since it names a relation or a kind, and neither does a word \
     of kin after a possessive, tom's brother; a mark after a bare year said after is writes it as what the year is, it is twenty twenty; \
     a mark after a number said on a relation with no unit makes the number the relation's value, he eats four, the film starts at three; \
     a word with a verb's ending whose stem the seeds hold as a verb, or that names no thing and no value, or whose singular does while an \
     article stands before the waiting word, a tailor fixes clothes, after a waiting unknown word, makes that word the thing, a tailor \
     fixes clothes; a time of day opening a sentence makes nothing appear, since it tells when, unless a copula follows, which makes it \
     the thing, yesterday was sunday; a mark in a statement sends the cursor back to the world, since a comma or a full stop ends a \
     clause; a mark drops what he holds and the flags, except that a quality still waiting for its noun while he places something was the \
     thing itself, the ring holds a stone, which then appears for him to drop; a thing heard while he holds nothing is what the sentence \
     is about, the topic it and they stand for; a word of an open question makes nothing appear, since a question asks about the world and \
     never changes it; a word that can name a thing, heard at the world or while he holds something, makes the thing appear, the story's \
     newest node of its name anywhere, or a new one under the world that takes the flags, always new when a count was said, since a \
     counted plural is a group of its own, ann has apples beside the apples of tom, the article a making a new thing even when one of the \
     name is known, a car is in the garage and a car is in the street; a quality at the world or while he holds something, and a word in \
     no class said first in a clause, is a flag for him to set, not a thing, the best book, and becomes the thing when a verb, a copula or \
     a place word comes before any noun, the orange is orange; a verb of handing on lets the next thing appear for him to grab; and any \
     other word, a word heard while the cursor stands on a relation, which is its value, and a thing named after a complete clause starts \
     a new clause at the world, but a verb there is a deed of the value just named, the cat sees the dog, a word of measure stays with the \
     counted unit, five years old, and a noun after a superlative under is names the kind, the largest planet, or a thing word while the \
     cursor stands elsewhere, leaves the world to his moves"
);
