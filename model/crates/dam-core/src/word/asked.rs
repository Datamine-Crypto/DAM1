use super::mind::{GIVING, kind_word, noun_word, open_question, is_number, kind_of, past_form, place_word, quality_word, singular, verb_base, ARTICLE_FLAGS, ASKING, CONTAINING, COPULA, HAVING, HELPERS, PERSON_PRONOUNS, SKIPPED, THING_PRONOUNS};
use super::moves::{WordMove, KINDS};
use crate::cursor::CursorMind;
use crate::quiz::{APOSTROPHE, IS_FORM};
use patterns::because;
use super::teacher::{WordGame, punctuation, MUCH_SPAN, opens_question, REQUESTS, QUESTION_OPENERS, WHERE_FORM, WHEN_FORM, WHOSE_FORM, WHAT_FORM, CHOICE_FORM, HOW_FORM, WHO_FORM, CURIOUS_WORDS};

pub(super) fn asked_word(mind: &CursorMind, w: &str, start: bool) -> Vec<(WordMove, Option<String>)> {
    if opens_question(w, start) && open_question(mind).is_none() {
        return vec![(WordMove::AddQuestion, None)];
    }
    let Some(question) = open_question(mind) else { return Vec::new() };
    if super::mind::PARTING_MARKS.contains(&w) {
        return vec![(WordMove::PointNothing, None)];
    }
    let unknown_asked = std::iter::once(question).chain(mind.tree.node(question).children.iter().copied()).filter(|&c| !mind.tree.node(c).gone).any(|c| { let name = &*mind.tree.node(c).name; super::mind::unknown_letter(name) && super::physics::named_result(mind, name).is_none() });
    let equated = mind.tree.node(question).children.iter().any(|&c| !mind.tree.node(c).gone && *mind.tree.node(c).name == *super::mind::EQUAL_SIGN);
    if equated && unknown_asked && ASKING.contains(&w) {
        return vec![(WordMove::Compute, None), (WordMove::AddQuestion, None)];
    }
    if !punctuation(w) && !mind.output.is_empty() {
        return vec![(WordMove::NameResult, None)];
    }
    if !punctuation(w) {
        let newest = mind.tree.node(question).children.iter().rev().find(|&&c| !mind.tree.node(c).gone).map(|&c| mind.tree.node(c).name.to_string()).unwrap_or_default();
        let after_role = super::mind::role_word(mind, &newest);
        let after_order = newest == super::mind::LATER || newest == super::mind::EARLIER;
        let quoting = mind.tree.node(question).children.iter().filter(|&&c| !mind.tree.node(c).gone && *mind.tree.node(c).name == *super::mind::QUOTE).count() % super::mind::LIST_HALVES == 1;
        if quoting {
            return vec![(WordMove::AddValue, None)];
        }
        let skipped = w == APOSTROPHE || super::mind::OWN_WORDS.iter().any(|(o, _)| *o == w) || (super::mind::FILLERS.contains(&w) && w != super::mind::LATER) || ARTICLE_FLAGS.contains(&w) || SKIPPED.contains(&w) || (HELPERS.contains(&w) && !super::mind::COUNTING.contains(&w)) || (COPULA.contains(&w) && !past_form(mind, w)) || QUESTION_OPENERS.contains(&w);
        let asked_deed = w == super::mind::DONE_ASKED && mind.tree.node(question).children.iter().any(|&c| !mind.tree.node(c).gone && super::physics::told_thing(mind, &mind.tree.node(c).name));
        let said_before = mind.before.iter().rev().nth(1);
        let ordered_letter = w.len() == 1 && (said_before.is_some_and(|b| b == super::mind::TOWARD) && mind.before.iter().any(|b| b == super::mind::ORDER_RELATION) || w == super::mind::POSSESSIVE_S && said_before.is_some_and(|b| b != APOSTROPHE));
        let state_asked = super::mind::STATE_WORDS.contains(&w) && super::mind::STATE_WORDS.iter().any(|state| super::physics::told_relation(mind, state));
        let doubled = ARTICLE_FLAGS.contains(&w) && said_before.is_some_and(|b| b == w);
        let skipped = skipped && !doubled && !state_asked && !ordered_letter && !asked_deed && !(super::mind::FILLERS.contains(&w) && super::physics::has_relation(mind, w, super::mind::ORDER_RELATION)) && !super::mind::COMPARING.iter().any(|(c, _)| *c == w) && !(after_role && super::mind::FILLERS.contains(&w)) && !(after_order && (super::mind::FILLERS.contains(&w) || w == super::mind::ARTICLE_FLAGS[1]));
        return vec![(if skipped { WordMove::PointNothing } else { WordMove::AddValue }, None)];
    }
    if w == super::mind::EQUAL_SIGN && unknown_asked && !equated {
        return vec![(WordMove::AddValue, None)];
    }
    if equated && unknown_asked {
        return vec![(WordMove::Compute, Some(super::mind::EQUAL_SIGN.to_string()))];
    }
    let form = mind.tree.node(question).name.to_string();
    let inner = (COPULA.contains(&form.as_str()) || QUESTION_OPENERS.contains(&form.as_str()) || REQUESTS.contains(&form.as_str())).then(|| mind.tree.node(question).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.to_string()).find(|w| ASKING.contains(&w.as_str()))).flatten();
    let inside = inner.is_some();
    let form = inner.unwrap_or(form);
    let words: Vec<String> = mind.tree.node(question).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.to_string()).collect();
    let words: Vec<String> = match words.iter().position(|w| *w == form).filter(|_| inside) {
        Some(at) => words[at + 1..].to_vec(),
        None => words,
    };
    let after_verb = words.iter().position(|w| w == super::mind::ABOUT).filter(|&at| at > 0 && verb_base(mind, &words[at - 1]).is_some_and(|base| mind.tree.holders_of(&crate::cursor::step_item(&base)).any(|holder| mind.tree.story(holder))));
    let words: Vec<String> = words.iter().enumerate().filter(|(at, _)| Some(*at) != after_verb).map(|(_, w)| w.clone()).collect();
    let clock_told = form == WHAT_FORM && words.len() == super::mind::LIST_HALVES && words[0] == super::mind::TIME_ASKED && words[1] == THING_PRONOUNS[0] && super::physics::told_thing(mind, &words[1]) && super::physics::has_relation(mind, &words[1], &crate::cursor::bare_name(IS_FORM.trim()));
    if clock_told {
        return vec![(WordMove::FindAsked, Some(words[1].clone())), (WordMove::GetKind, Some(form.clone()))];
    }
    let own_place = form == WHERE_FORM && words.iter().any(|w| super::mind::YOU_WORDS.contains(&w.as_str())) && !words.iter().any(|w| (noun_word(mind, w) && !super::mind::YOU_WORDS.contains(&w.as_str())) || (super::mind::YOU_WORDS.contains(&w.as_str()) && super::physics::told_thing(mind, w)));
    if own_place {
        return vec![(WordMove::StepTop, None), (WordMove::GetName, None)];
    }
    let curious = words.iter().any(|w| CURIOUS_WORDS.contains(&w.as_str())) && words.iter().any(|w| super::mind::YOU_WORDS.contains(&w.as_str()));
    if curious && form == WHAT_FORM {
        return vec![(WordMove::StepNewest, None), (WordMove::GetName, None)];
    }
    let reply = words.iter().chain(std::iter::once(&form)).find(|w| super::mind::reply_word(mind, w));
    if let Some(reply) = reply.filter(|_| form != WHO_FORM && (words.iter().any(|w| super::mind::YOU_WORDS.contains(&w.as_str())) || super::mind::REPLY_WORDS.contains(&form.as_str()))) {
        return vec![(WordMove::GetRelation, Some(reply.clone()))];
    }
    if let Some(defined) = words.iter().find(|w| super::physics::defined_name(mind, w)).filter(|_| form == WHAT_FORM && words.iter().any(|w| crate::words::number_of(w).is_some())) {
        return vec![(WordMove::Compute, Some(defined.clone()))];
    }
    if form == HOW_FORM && words.iter().any(|w| verb_base(mind, w).is_some_and(|b| b == super::mind::GOING)) {
        let places: Vec<&String> = words.iter().filter(|w| noun_word(mind, w) && verb_base(mind, w).is_none() && !super::mind::YOU_WORDS.contains(&w.as_str())).collect();
        if let [.., from, to] = places[..] {
            if super::physics::map_route(mind, from, to).is_some() {
                return vec![(WordMove::FindAsked, Some(from.clone())), (WordMove::GetDistance, Some(to.clone()))];
            }
        }
    }
    if form == super::mind::WHY_ASKED {
        if let Some(person) = words.iter().find(|w| super::physics::told_thing(mind, w)) {
            return vec![(WordMove::FindAsked, Some(person.clone())), (WordMove::GetKind, Some(form.clone()))];
        }
    }
    if let [sort, after] = &words[..] {
        if form == WHAT_FORM && after == super::mind::RUN_ASKED[0] && super::physics::told_thing(mind, sort) && super::physics::number_run(mind).is_empty() && super::physics::shifted_value(mind, None).is_some() {
            return vec![(WordMove::GetShifted, None)];
        }
    }
    if let [placed, place] = &words[..] {
        if form == WHAT_FORM && place_word(place) && super::physics::told_thing(mind, placed) {
            return vec![(WordMove::FindAsked, Some(placed.clone())), (WordMove::GetLocation, None)];
        }
    }
    if let [role, of] = &words[..] {
        if (form == WHAT_FORM || form == WHO_FORM) && super::mind::role_word(mind, role) && !super::mind::role_word(mind, of) && mind.before.iter().any(|b| b == super::mind::TOWARD) && !super::physics::has_relation(mind, of, role) && !super::physics::told_relation(mind, role) {
            return vec![(WordMove::FindAsked, Some(of.clone())), (WordMove::GetRelation, Some(role.clone()))];
        }
    }
    if let [owner, sort] = &words[..] {
        if form == WHAT_FORM && mind.before.iter().any(|b| b == APOSTROPHE) && super::physics::told_thing(mind, owner) && !super::physics::told_thing(mind, sort) && super::physics::value_told(mind, sort) {
            return vec![(WordMove::FindWith, Some(sort.clone())), (WordMove::GetName, None)];
        }
    }
    if let [place, sort] = &words[..] {
        if (form == WHAT_FORM || form == WHO_FORM || form == CHOICE_FORM) && super::mind::LETTER_PLACES.iter().any(|(ordinal, _)| ordinal == place) && noun_word(mind, sort) && super::physics::value_told(mind, place) {
            return vec![(WordMove::GetAllWith, Some(place.clone()))];
        }
    }
    let claimed = words.iter().position(|w| verb_base(mind, w).is_some_and(|b| super::mind::CLAIMING.contains(&b.as_str())));
    if let Some(thing) = claimed.and_then(|at| words[at + 1..].iter().find(|w| noun_word(mind, w) && !COPULA.contains(&w.as_str()))).filter(|_| (form == WHAT_FORM || form == WHERE_FORM) && mind.before.iter().rev().nth(1).is_some_and(|w| COPULA.contains(&w.as_str()))) {
        let kind = words.first().filter(|w| KINDS.iter().any(|(_, k)| *k == w.as_str())).cloned().unwrap_or_else(|| form.clone());
        let get = if form == WHERE_FORM { (WordMove::GetLocation, None) } else { (WordMove::GetKind, Some(kind)) };
        return vec![(WordMove::FindAsked, Some(thing.clone())), get];
    }
    let mine = mind.before.iter().any(|b| super::mind::OWN_WORDS.iter().any(|(own, owner)| own == b && super::mind::SELF_WORDS.contains(owner)));
    if let [named, role] = &words[..] {
        if mine && COPULA.contains(&form.as_str()) && (super::mind::KIN.contains(&role.as_str()) || role == super::mind::NAME_ROLE) {
            return vec![(WordMove::StepUser, None), (WordMove::FlagProperty, Some(role.clone())), (WordMove::Check, Some(named.clone()))];
        }
    }
    if let [first, second] = &words[..] {
        let kin = [first, second].into_iter().find(|w| super::mind::KIN.contains(&w.as_str()));
        let naming = [first, second].into_iter().any(|w| super::mind::NAMED_BY.contains(&w.as_str()) || w == super::mind::NAME_ROLE);
        if let Some(role) = kin.filter(|_| mine && form == WHAT_FORM && naming) {
            return vec![(WordMove::StepUser, None), (WordMove::GetRelation, Some(role.clone()))];
        }
    }
    let yours = mind.before.iter().find(|b| super::mind::OWN_WORDS.iter().any(|(own, owner)| own == *b && super::mind::YOU_WORDS.contains(owner))).cloned();
    if let ([role], Some(_)) = (&words[..], yours) {
        if (form == WHAT_FORM || form == WHO_FORM) && role == super::mind::NAME_ROLE {
            return vec![(WordMove::StepUser, None), (WordMove::GetName, None)];
        }
    }
    if let [role] = &words[..] {
        if mine && (form == WHAT_FORM || form == WHO_FORM) && (super::mind::KIN.contains(&role.as_str()) || role == super::mind::NAME_ROLE) {
            return vec![(WordMove::StepUser, None), (WordMove::GetRelation, Some(role.clone()))];
        }
    }
    if let Some(target) = words.iter().position(|w| w == super::mind::UNTIL).and_then(|at| words[at + 1..].iter().find(|w| noun_word(mind, w))) {
        return vec![(WordMove::GetDistance, Some(target.clone()))];
    }
    let clock_said = words.iter().any(|w| w == super::mind::CLOCK_WORD);
    if clock_said && !words.iter().any(|w| w == super::mind::CHOICE_WORD) && words.iter().filter(|w| is_number(mind, w).is_some()).count() > 1 {
        if let Some(target) = words.iter().rev().find(|w| is_number(mind, w).is_some()) {
            return vec![(WordMove::GetDistance, Some(target.clone()))];
        }
    }
    if let Some(alphabet) = words.iter().find(|w| *w == super::mind::ALPHABET_WORD) {
        return vec![(WordMove::Compute, Some(alphabet.clone()))];
    }
    if let Some(unit) = words.iter().position(|w| super::mind::COUNTING.contains(&w.as_str())).and_then(|at| words.get(at + 1)).filter(|unit| super::physics::converted(mind, unit, &words).is_some()) {
        return vec![(WordMove::Compute, Some(unit.clone()))];
    }
    if super::physics::number_run(mind).len() > 1 && !words.iter().any(|w| crate::words::number_of(w).is_some()) {
        let of_numbers = words.iter().any(|w| singular(w) == super::mind::NUMBER_ASKED);
        let key = words.iter().find(|w| super::mind::RUN_ASKED.contains(&w.as_str())).or_else(|| words.iter().find(|w| of_numbers && (super::mind::COUNTING.contains(&w.as_str()) || *w == super::mind::RUN_SUM)));
        if let Some(key) = key {
            return vec![(WordMove::Compute, Some(key.clone()))];
        }
    }
    let whole_said = |from: usize| words.iter().skip(from + 1).any(|w| crate::words::number_of(w).is_some() || super::physics::thing_number(mind, w, &words).is_some());
    if let Some(at) = words.iter().position(|w| crate::cursor::FRACTION_WORDS.iter().any(|(f, _)| *f == w.as_str())).filter(|&at| form == WHAT_FORM && at <= usize::from(true) && whole_said(at) && mind.before.windows(super::mind::LIST_HALVES).any(|pair| pair[0] == words[at] && pair[1] == super::mind::TOWARD)) {
        return vec![(WordMove::Compute, Some(words[at].clone()))];
    }
    if let Some(around) = words.iter().find(|w| *w == super::mind::AROUND_ASKED).filter(|_| words.iter().any(|w| crate::words::number_of(w).is_some())) {
        return vec![(WordMove::Compute, Some(around.clone()))];
    }
    if let Some(form_asked) = words.iter().find(|w| *w == super::mind::PLURAL_ASKED || *w == super::mind::SINGULAR_ASKED) {
        return vec![(WordMove::Compute, Some(form_asked.clone()))];
    }
    if let Some(spelt) = words.iter().find(|w| *w == super::mind::IN_WORDS).filter(|_| form == WHAT_FORM && words.iter().any(|w| crate::words::number_of(w).is_some()) && !words.iter().any(|w| super::mind::COUNTING.contains(&w.as_str()))) {
        return vec![(WordMove::Compute, Some(spelt.clone()))];
    }
    let worded = words.iter().filter(|w| crate::words::number_of(w).is_none() && is_number(mind, w).is_some()).count();
    if let Some(asked_number) = words.iter().find(|w| *w == super::mind::NUMBER_ASKED).filter(|_| form == WHAT_FORM && worded > 0 && words.len() == worded + 1) {
        return vec![(WordMove::Compute, Some(asked_number.clone()))];
    }
    if let Some(letter) = words.iter().find(|w| *w == super::mind::LETTER_ASKED).filter(|_| words.iter().any(|w| *w == super::mind::LAST_PLACE || super::mind::LETTER_PLACES.iter().any(|(p, _)| *p == w.as_str()))) {
        return vec![(WordMove::Compute, Some(letter.clone()))];
    }
    if let Some(spelt) = words.iter().find(|w| super::mind::SPELLING.contains(&w.as_str())).filter(|_| words.iter().any(|w| super::mind::COUNTING.contains(&w.as_str()))) {
        return vec![(WordMove::Compute, Some(spelt.clone()))];
    }
    let numbered = words.iter().filter(|w| is_number(mind, w).is_some() || super::physics::named_result(mind, w).is_some()).count();
    if let Some(op) = words.iter().find(|w| super::mind::operation_of(w).is_some()).filter(|_| words.iter().any(|w| super::physics::named_result(mind, w).is_some())) {
        return vec![(WordMove::Compute, Some(op.clone()))];
    }
    if let Some(sign) = std::iter::once(&form).chain(words.iter()).find(|w| super::mind::expression_word(w)) {
        return vec![(WordMove::Compute, Some(sign.clone()))];
    }
    if let Some(sign) = words.iter().find(|w| is_number(mind, w).is_some() || super::mind::operation_of(w).is_some()).filter(|w| super::mind::operation_of(w).is_some() && is_number(mind, w).is_none() && super::physics::named_result(mind, w).is_none() && mind.number.is_some() && numbered == 1) {
        return vec![(WordMove::Compute, Some(sign.clone()))];
    }
    if let Some(function) = words.iter().find(|w| super::mind::list_function(w).is_some()).filter(|_| numbered > 0) {
        return vec![(WordMove::Compute, Some(function.clone()))];
    }
    if let Some(op) = words.iter().find(|w| super::mind::number_operation(w).is_some()).filter(|op| numbered > 1 || (numbered == 1 && super::mind::unary_operation(op))).or_else(|| super::mind::operation_of(&form).is_some().then_some(&form)) {
        return vec![(WordMove::Compute, Some(op.clone()))];
    }
    if form == WHO_FORM || form == WHAT_FORM || form == CHOICE_FORM {
        let measured = words.iter().find_map(|w| super::physics::measure_asked(mind, w).filter(|(m, _)| super::physics::told_relation(mind, m) || m == super::mind::SPAN_ASKED && super::physics::spans_told(mind)).map(|(_, most)| (w.clone(), most)));
        let timed = words.iter().find(|w| super::mind::TIMED_VERBS.contains(&super::mind::verb_stem(mind, w).as_str()) && super::physics::told_relation(mind, &super::mind::verb_stem(mind, w)));
        let end = words.iter().find(|w| **w == super::mind::ORDINALS[0] || **w == super::mind::LAST_PLACE);
        if let (Some(timed), Some(end)) = (timed, end) {
            return vec![(if end == super::mind::LAST_PLACE { WordMove::GetMost } else { WordMove::GetLeast }, Some(timed.clone()))];
        }
        if let Some((word, most)) = measured.filter(|_| !words.iter().any(|w| w == super::mind::COMPARED)) {
            return vec![(if most { WordMove::GetMost } else { WordMove::GetLeast }, Some(word))];
        }
    }
    if form == WHAT_FORM || form == WHO_FORM || form == CHOICE_FORM {
        let more = words.iter().find(|w| matches!(super::mind::number_operation(w), Some(super::mind::MORE_SIGN | super::mind::MOST_SIGN | super::mind::LESS_SIGN | super::mind::LEAST_SIGN)));
        let told = words.iter().find(|w| !COPULA.contains(&w.as_str()) && !HAVING.contains(&w.as_str()) && super::physics::told_relation(mind, &super::mind::verb_stem(mind, w)));
        if let (Some(more), Some(told)) = (more, told) {
            if !words.iter().any(|w| is_number(mind, w).is_some() || w == super::mind::COMPARED) {
                let most = matches!(super::mind::number_operation(more), Some(super::mind::MORE_SIGN | super::mind::MOST_SIGN));
                return vec![(if most { WordMove::GetMost } else { WordMove::GetLeast }, Some(told.clone()))];
            }
        }
    }
    let extreme = words.iter().position(|w| super::mind::COMPARING.iter().any(|(c, _)| *c == w.as_str())).filter(|_| (form == WHO_FORM || form == CHOICE_FORM || form == WHAT_FORM) && words.iter().any(|w| HAVING.contains(&w.as_str())) && !words.iter().any(|w| w == super::mind::COMPARED) && !words.iter().any(|w| super::mind::COUNTING.contains(&w.as_str())));
    if let Some(at) = extreme {
        if let Some(counted) = words[at + 1..].iter().find(|w| noun_word(mind, w)) {
            let most = matches!(super::mind::number_operation(&words[at]), Some(super::mind::MORE_SIGN | super::mind::MOST_SIGN));
            return vec![(if most { WordMove::GetMost } else { WordMove::GetLeast }, Some(counted.clone()))];
        }
    }
    let every = words.iter().any(|w| super::mind::TOGETHER.contains(&w.as_str()) || *w == THING_PRONOUNS[1]);
    if let Some(span) = words.iter().find(|w| *w == super::mind::SPAN_ASKED || super::physics::measure_asked(mind, w).is_some_and(|(m, _)| m == super::mind::SPAN_ASKED)).filter(|_| form == HOW_FORM && super::physics::spans_told(mind)) {
        let Some(timed_thing) = words.iter().rev().find(|w| noun_word(mind, w) && **w != *span) else { return Vec::new() };
        return vec![(WordMove::FindAsked, Some(timed_thing.clone())), (WordMove::Compute, Some(span.clone()))];
    }
    let much = words.iter().position(|w| super::mind::COUNTING.contains(&w.as_str())).filter(|_| !every).filter(|&at| words.get(at + 1).is_some_and(|w| super::physics::told_thing(mind, w)) && words.iter().skip(at + MUCH_SPAN).all(|w| verb_base(mind, w).is_some() || !super::physics::told_thing(mind, w)));
    if let Some(at) = much {
        return vec![(WordMove::FindAsked, Some(words[at + 1].clone())), (WordMove::GetAmount, None)];
    }
    if let Some(at) = words.iter().position(|w| super::mind::COUNTING.contains(&w.as_str())) {
        let counted = words.iter().skip(at + 1).find(|w| !super::mind::COMPARING.iter().any(|(c, _)| *c == w.as_str())).cloned();
        let holder = words.iter().skip(at + 1).filter(|w| Some(*w) != counted.as_ref()).rev().find(|w| super::physics::told_thing(mind, w) && Some(*w) != counted.as_ref()).cloned();
        let holder = holder.or_else(|| counted.as_ref().and_then(|part| words.iter().skip(at + 1).filter(|w| *w != part).find(|w| super::physics::has_relation(mind, w, &singular(part))).cloned()));
        let holders = words.iter().skip(at + 1).filter(|w| Some(*w) != counted.as_ref() && super::physics::told_thing(mind, w)).count();
        let held = counted.as_ref().is_some_and(|c| !super::physics::things_named(mind, c).is_empty() || !super::physics::unit_worths(mind, c).is_empty());
        if let Some(counted) = counted.clone().filter(|_| (holders > 1 || every) && held) {
            let get = if words.iter().any(|w| w == super::mind::COMPARED) { WordMove::GetDifference } else { WordMove::GetTotal };
            return vec![(get, Some(counted))];
        }
        return match (counted, holder) {
            (Some(counted), Some(holder)) => search_walk(mind, &holder, &words, |_| true, (WordMove::GetCount, Some(counted))),
            _ => Vec::new(),
        };
    }
    if (form == WHAT_FORM || form == WHO_FORM || form == CHOICE_FORM) && words.iter().any(|w| super::mind::NEGATIONS.contains(&w.as_str())) {
        if let Some(value) = words.iter().rev().find(|w| noun_word(mind, w) && !super::mind::NEGATIONS.contains(&w.as_str())) {
            return vec![(WordMove::GetAllWith, Some(value.clone()))];
        }
    }
    let content: Vec<String> = words.iter().filter(|w| !COPULA.contains(&w.as_str()) && !(form == WHAT_FORM && *w == super::mind::MADE)).cloned().collect();
    if (form == WHAT_FORM || form == WHO_FORM) && content.len() == 1 && words.len() > 1 && !super::physics::told_thing(mind, &content[0]) && noun_word(mind, &content[0]) {
        return vec![(WordMove::GetAllWith, Some(content[0].clone()))];
    }
    if form == WHAT_FORM && words.iter().any(|w| *w == THING_PRONOUNS[1]) {
        let role = words.first().filter(|w| super::mind::role_word(mind, w));
        let thing = role.and_then(|r| words.iter().rev().find(|x| *x != r && super::physics::has_relation(mind, x, r)));
        if let (Some(role), Some(thing)) = (role, thing) {
            return vec![(WordMove::FindAsked, Some(thing.clone())), (WordMove::GetRelation, Some(role.clone()))];
        }
    }
    let compares = words.iter().any(|w| super::mind::compared_relation(w).is_some() && !super::physics::told_thing(mind, w));
    let compared_things: Vec<&String> = words.iter().filter(|w| *w != super::mind::CHOICE_WORD && super::mind::compared_relation(w).is_none() && (super::physics::told_thing(mind, w) || super::physics::value_told(mind, w))).collect();
    if form == CHOICE_FORM && compares && words.iter().any(|w| w == super::mind::CHOICE_WORD) && compared_things.len() > 1 {
        return vec![(WordMove::FindAsked, Some(compared_things[0].clone())), (WordMove::GetChoice, None)];
    }
    if let [end, kind_named] = &words[..] {
        let ends = end == super::mind::ORDINALS[0] || end == super::mind::LAST_PLACE;
        if form == WHAT_FORM && ends && !super::physics::told_thing(mind, kind_named) && !super::physics::told_relation(mind, end) && noun_word(mind, kind_named) {
            return vec![(WordMove::FindAsked, Some(kind_named.clone())), (WordMove::GetRelation, Some(end.clone()))];
        }
    }
    if let [me] = &words[..] {
        if form == WHO_FORM && super::mind::SELF_WORDS.contains(&me.as_str()) {
            return vec![(WordMove::StepUser, None), (WordMove::GetName, None)];
        }
        if form == WHO_FORM && super::mind::YOU_WORDS.contains(&me.as_str()) {
            return vec![(WordMove::StepUser, Some(me.clone())), (WordMove::GetName, None)];
        }
    }
    if let [able] = &words[..] {
        if (form == WHAT_FORM || form == WHO_FORM) && mind.before.iter().any(|b| b == super::mind::ABLE) {
            return vec![(WordMove::GetAllWith, Some(able.clone()))];
        }
    }
    let kind_said = form == WHAT_FORM && words.len() == 1 && (mind.before.iter().rev().nth(super::mind::LIST_HALVES).is_some_and(|b| ARTICLE_FLAGS[1..].contains(&b.as_str())) || !quality_word(mind, &words[0]) && !super::physics::told_thing(mind, &words[0])) && super::physics::seeded_class(mind, &words[0]) && !super::physics::value_told(mind, &words[0]);
    if (form == WHAT_FORM || form == WHO_FORM || form == CHOICE_FORM) && words.len() == 1 && !kind_said && (!super::physics::told_thing(mind, &words[0]) || super::mind::quality_word(mind, &words[0]) && !mind.before.iter().rev().nth(super::mind::LIST_HALVES).is_some_and(|b| ARTICLE_FLAGS[1..].contains(&b.as_str())) && (super::physics::value_told(mind, &words[0]) || kind_of(mind, &words[0]).as_deref() != WordMove::SetMaterial.kind())) && (noun_word(mind, &words[0]) || super::mind::LETTER_PLACES.iter().any(|(ordinal, _)| *ordinal == words[0]) || super::mind::compared_relation(&words[0]).is_some() || verb_base(mind, &words[0]).is_some_and(|b| super::mind::STATES.iter().any(|(v, _, _)| *v == b))) {
        return vec![(WordMove::GetAllWith, Some(words[0].clone()))];
    }
    if let Some(at) = words.iter().position(|w| super::mind::DIRECTIONS.contains(&w.as_str())) {
        let before = words[..at].iter().find(|w| noun_word(mind, w)).cloned();
        let after = words[at + 1..].iter().find(|w| noun_word(mind, w)).cloned();
        match (COPULA.contains(&form.as_str()), before, after) {
            (true, Some(subject), Some(other)) => return vec![(WordMove::FindAsked, Some(subject)), (WordMove::FlagProperty, Some(words[at].clone())), (WordMove::Check, Some(other))],
            (false, _, Some(other)) if form == WHAT_FORM || form == WHO_FORM => return vec![(WordMove::FindAsked, Some(other)), (WordMove::GetSubject, Some(words[at].clone()))],
            _ => {}
        }
    }
    if form == WHO_FORM {
        if let Some(kin) = words.iter().find(|w| super::mind::KIN.contains(&w.as_str())) {
            if let Some(person) = words.iter().find(|w| *w != kin && noun_word(mind, w)) {
                return vec![(WordMove::FindAsked, Some(person.clone())), (WordMove::GetRelation, Some(kin.clone()))];
            }
        }
    }
    if form == WHOSE_FORM {
        let kin = words.iter().find(|w| (super::mind::KIN.contains(&w.as_str()) || super::mind::role_word(mind, w)) && words.iter().any(|x| x != *w && super::physics::has_relation(mind, x, w) && !super::physics::owns_relation(mind, x, w)));
        if let Some(relation) = kin {
            if let Some(thing) = words.iter().find(|x| *x != relation && super::physics::has_relation(mind, x, relation)) {
                return vec![(WordMove::FindAsked, Some(thing.clone())), (WordMove::GetSubject, Some(relation.clone()))];
            }
        }
    }
    if form == WHO_FORM && !words.iter().any(|w| w == super::mind::COMPANION) || form == WHAT_FORM && words.len() == 1 {
        if let Some(done) = words.iter().find(|w| super::physics::activity_named(mind, w) && !super::physics::told_thing(mind, w)) {
            return vec![(WordMove::GetAllWith, Some(done.clone()))];
        }
    }
    let extreme_value = words.iter().find(|w| w.ends_with(super::mind::SUPERLATIVE_END) && super::physics::things_named(mind, w).is_empty()).filter(|_| !words.iter().any(|w| is_number(mind, w).is_some()));
    if let Some(value) = extreme_value.filter(|_| (form == WHAT_FORM || form == WHO_FORM || form == CHOICE_FORM) && !words.iter().any(|w| !super::physics::things_named(mind, w).is_empty()) && !words.iter().any(|w| HAVING.contains(&w.as_str()))) {
        return vec![(WordMove::GetAllWith, Some(value.clone()))];
    }
    let placed_rank = words.iter().find(|w| super::mind::ORDINALS.contains(&w.as_str())).filter(|_| (form == WHAT_FORM || form == CHOICE_FORM) && !words.iter().any(|w| super::physics::told_thing(mind, w)));
    if let Some(rank) = placed_rank {
        let beside: Vec<String> = words.iter().filter(|w| *w != rank && noun_word(mind, w)).cloned().collect();
        if !super::physics::ranked_in_seeds(mind, rank, &beside).is_empty() {
            return vec![(WordMove::GetAllWith, Some(rank.clone()))];
        }
    }
    let named: Vec<&String> = words.iter().filter(|w| noun_word(mind, w) && !kind_word(mind, w)).collect();
    let nouns: Vec<&String> = if named.is_empty() { words.iter().filter(|w| noun_word(mind, w) || w.len() == 1 && super::physics::has_relation(mind, w, super::mind::ORDER_RELATION)).collect() } else { named };
    let told: Vec<&String> = words.iter().filter(|w| super::physics::told_thing(mind, w) || THING_PRONOUNS.contains(&w.as_str()) || PERSON_PRONOUNS.contains(&w.as_str())).collect();
    let solid: Vec<&String> = told.iter().copied().filter(|w| !super::mind::quality_word(mind, w) || super::physics::told_with_article(mind, w)).collect();
    let things = if solid.is_empty() { told } else { solid };
    let (Some(first), Some(last)) = (things.first().or(nouns.first()), things.last().or(nouns.last())) else { return Vec::new() };
    let find = (WordMove::FindAsked, Some((*last).clone()));
    let _ = &find;
    if (COPULA.contains(&form.as_str()) || QUESTION_OPENERS.contains(&form.as_str())) && words.iter().any(|w| w == super::mind::CHOICE_WORD) {
        return vec![(WordMove::FindAsked, Some((*first).clone())), (WordMove::GetChoice, None)];
    }
    if COPULA.contains(&form.as_str()) || QUESTION_OPENERS.contains(&form.as_str()) {
        let value = words.last().filter(|w| **w != **first).cloned().unwrap_or_else(|| (*last).clone());
        if let Some(at) = words.iter().position(|w| w == super::mind::COMPARED).filter(|&at| at > 0) {
            let compared = words[at - 1].clone();
            let value = words[at + 1..].iter().find(|w| noun_word(mind, w)).cloned().unwrap_or(value);
            let subject = words[..at - 1].iter().find(|w| noun_word(mind, w)).cloned().unwrap_or_else(|| (*first).clone());
            return vec![(WordMove::FindAsked, Some(subject)), (WordMove::FlagProperty, Some(compared)), (WordMove::Check, Some(value))];
        }
        if let Some(at) = words.iter().position(|w| super::mind::KIN.contains(&w.as_str())).filter(|_| !words.iter().any(|w| w == super::mind::SAMENESS)) {
            let people: Vec<String> = words[..at].iter().filter(|w| noun_word(mind, w)).cloned().collect();
            if let [subject, .., holder] = &people[..] {
                return vec![(WordMove::FindAsked, Some(holder.clone())), (WordMove::FlagProperty, Some(words[at].clone())), (WordMove::Check, Some(subject.clone()))];
            }
        }
        let placing_asked = QUESTION_OPENERS.contains(&form.as_str()) && words.iter().any(|w| verb_base(mind, w).is_some_and(|b| super::mind::MOVING.contains(&b.as_str())));
        if let Some(at) = words.iter().position(|w| place_word(w)).filter(|&at| (COPULA.contains(&form.as_str()) || placing_asked) && at > 0 && !past_form(mind, &form) && !words.iter().any(|w| super::mind::DIRECTIONS.contains(&w.as_str()) || w == super::mind::FRONT)) {
            let owned = mind.before.iter().any(|w| w == APOSTROPHE);
            let subject = if owned { words[..at].iter().rev().find(|w| noun_word(mind, w)).cloned() } else { words[..at].iter().find(|w| noun_word(mind, w)).cloned() };
            let place = words[at + 1..].iter().find(|w| noun_word(mind, w)).cloned();
            if let (Some(subject), Some(place)) = (subject, place) {
                let sought = place.clone();
                let mut walk = search_walk(mind, &subject, &[], |n| super::physics::holds(mind, n, &sought), (WordMove::FlagProperty, Some(words[at].clone())));
                walk.push((WordMove::Check, Some(place)));
                return walk;
            }
        }
        let top = words.iter().find(|w| w.ends_with(super::mind::SUPERLATIVE_END) && !super::physics::told_thing(mind, w) && **w != **first);
        if let Some(top) = top.filter(|_| COPULA.contains(&form.as_str())) {
            return vec![(WordMove::FindAsked, Some((*first).clone())), (WordMove::Check, Some(top.clone()))];
        }
        let nameable = |w: &String| noun_word(mind, w) || is_number(mind, w).is_some() || super::physics::has_relation(mind, w, super::mind::ORDER_RELATION);
        let role = words.iter().position(|w| super::mind::role_word(mind, w) && **w != **first).filter(|&at| COPULA.contains(&form.as_str()) && (words[at + 1..].iter().any(|w| nameable(w) && *w != super::mind::LIKENESS) || words[..at].iter().filter(|w| nameable(w)).count() > 1));
        if let Some(at) = role {
            let after = words[at + 1..].iter().find(|w| nameable(w) && **w != super::mind::LIKENESS && **w != super::mind::COMPARED).cloned();
            let nouns_before: Vec<String> = words[..at].iter().filter(|w| nameable(w)).cloned().collect();
            let holder = after.clone().or_else(|| nouns_before.last().cloned()).unwrap_or(value);
            let subject = nouns_before.first().cloned().unwrap_or_else(|| (*first).clone());
            let (found, checked) = if words[at] == super::mind::SAMENESS { (subject, holder) } else { (holder, subject) };
            return vec![(WordMove::FindAsked, Some(found)), (WordMove::FlagProperty, Some(words[at].clone())), (WordMove::Check, Some(checked))];
        }
        let deed = QUESTION_OPENERS.contains(&form.as_str()).then(|| words.iter().find(|w| **w != **first && **w != value && !place_word(w) && !HAVING.contains(&w.as_str()) && !super::physics::told_thing(mind, w) && !super::mind::known_base(&verb_base(mind, w).unwrap_or_else(|| (*w).clone())) && (super::physics::has_relation(mind, first, &super::mind::verb_stem(mind, w)) || super::physics::has_relation(mind, &value, &super::mind::verb_stem(mind, w))))).flatten();
        let felt = COPULA.contains(&form.as_str()).then(|| words.iter().find(|w| quality_word(mind, w) && super::physics::told_relation(mind, w) && words.last().is_some_and(|l| l != *w))).flatten();
        let deed = deed.or(felt);
        if let Some(deed) = deed {
            let at = words.iter().position(|w| *w == *deed).unwrap_or_default();
            let subject = words[..at].iter().find(|w| noun_word(mind, w)).cloned().unwrap_or_else(|| (*first).clone());
            let value = words[at + 1..].iter().find(|w| noun_word(mind, w)).cloned().unwrap_or(value);
            let mut walk = search_walk(mind, &subject, &words, |n| super::physics::node_relates(mind, n, deed), (WordMove::FlagProperty, Some(deed.clone())));
            walk.push((WordMove::Check, Some(value)));
            return walk;
        }
        if let Some(by) = words.iter().position(|w| w == super::mind::PASSIVE_MARK) {
            let subject = words[..by].iter().find(|w| noun_word(mind, w) && verb_base(mind, w).is_none() && !super::mind::verb_like(mind, w)).cloned();
            let doer = words[by + 1..].iter().find(|w| noun_word(mind, w)).cloned();
            if let (Some(subject), Some(doer)) = (subject, doer) {
                return vec![(WordMove::FindAsked, Some(doer)), (WordMove::Check, Some(subject))];
            }
        }
        if past_form(mind, &form) && words.iter().any(|w| place_word(w)) {
            return vec![(WordMove::FindAsked, Some((*first).clone())), (WordMove::FlagTime, None), (WordMove::Check, Some(value))];
        }
        let sought = value.clone();
        return search_walk(mind, first, &[], |n| super::physics::holds(mind, n, &sought), (WordMove::Check, Some(value)));
    }
    if form == WHO_FORM || form == WHAT_FORM {
        let than = words.iter().position(|w| w == super::mind::COMPARED);
        let compared = than.and_then(|at| at.checked_sub(1).map(|b| (words[b].clone(), words[at + 1..].iter().find(|w| noun_word(mind, w)).cloned())));
        if let Some((word, Some(value))) = compared {
            return vec![(WordMove::FindAsked, Some(value)), (WordMove::GetSubject, Some(word))];
        }
        let by = words.iter().position(|w| w == super::mind::PASSIVE_MARK);
        let verb = by.and_then(|at| words[..at].iter().rev().find(|w| !super::physics::told_thing(mind, w)).cloned());
        let thing = by.and_then(|at| words[..at].iter().find(|w| noun_word(mind, w) && Some(*w) != verb.as_ref()).cloned());
        if let (Some(verb), Some(thing)) = (verb, thing) {
            return vec![(WordMove::FindAsked, Some(thing)), (WordMove::GetSubject, Some(verb))];
        }
    }
    if form == WHAT_FORM && words.iter().any(|w| w == super::mind::TIME_ASKED) && super::physics::shifted_value(mind, None).is_some() {
        if words.iter().any(|w| is_number(mind, w).is_some()) {
            return vec![(WordMove::GetShifted, None)];
        }
        if let Some(thing) = words.iter().find(|w| super::physics::told_thing(mind, w)) {
            return vec![(WordMove::FindAsked, Some(thing.clone())), (WordMove::GetShifted, None)];
        }
    }
    let order = step_relation();
    let ordered_kind = words.iter().any(|w| kind_word(mind, w)) && super::physics::shifted_value(mind, None).is_some();
    if form == WHAT_FORM && ordered_kind && words.iter().any(|w| is_number(mind, w).is_some() || *w == super::mind::RUN_ASKED[0]) && super::physics::number_run(mind).is_empty() {
        return vec![(WordMove::GetShifted, None)];
    }
    if form == WHAT_FORM {
        let untold = words.iter().find(|w| super::physics::has_relation(mind, w, &order) && super::physics::things_named(mind, w).is_empty());
        if let Some(word) = untold.filter(|w| mind.tree.named(w).find(|&n| n != 0 && !mind.tree.node(n).gone).is_some_and(|n| super::physics::shifted_value(mind, Some(n)).is_some())) {
            return vec![(WordMove::FindAsked, Some(word.clone())), (WordMove::GetShifted, None)];
        }
    }
    let ordered = words.iter().position(|w| w == super::mind::EARLIER || w == super::mind::LATER).filter(|_| form != WHERE_FORM && form != WHO_FORM);
    if let Some(at) = ordered {
        let get = if words[at] == super::mind::LATER { WordMove::GetRelation } else { WordMove::GetSubject };
        if let Some(target) = words[at + 1..].iter().find(|w| noun_word(mind, w) || is_number(mind, w).is_some() || w.len() == 1) {
            if is_number(mind, target).is_some() {
                return vec![(get, Some(words[at].clone()))];
            }
            return vec![(WordMove::FindAsked, Some(target.clone())), (get, Some(words[at].clone()))];
        }
    }
    if form == WHAT_FORM && words.last().is_some_and(|w| w == super::mind::ABOUT) && words.len() > 1 {
        return vec![(WordMove::FindAsked, Some((*last).clone())), (WordMove::GetRelation, Some(super::mind::ABOUT.to_string()))];
    }
    if words.iter().any(|w| w == super::mind::ABOUT) {
        return vec![(WordMove::FindAsked, Some((*last).clone())), (WordMove::GetAbout, None)];
    }
    if form == WHERE_FORM {
        let around = words.iter().position(|w| w == super::mind::EARLIER || w == super::mind::LATER);
        if let Some(at) = around {
            let thing = words[..at].iter().rev().find(|w| super::physics::told_thing(mind, w)).cloned();
            let place = words[at + 1..].iter().find(|w| super::physics::told_thing(mind, w)).cloned();
            if let (Some(thing), Some(place)) = (thing, place) {
                let get = if words[at] == super::mind::EARLIER { WordMove::GetBefore } else { WordMove::GetAfter };
                return vec![(WordMove::FindAsked, Some(thing)), (get, Some(place))];
            }
        }
    }
    if let Some(at) = words.iter().position(|w| w == super::mind::EARLIER).filter(|_| form == WHO_FORM && words.iter().any(|w| HAVING.contains(&w.as_str()))) {
        let thing = words[..at].iter().rev().find(|w| super::physics::told_thing(mind, w) && !super::physics::person_named(mind, w)).cloned();
        let holder = words[at + 1..].iter().find(|w| super::physics::person_named(mind, w)).cloned();
        if let (Some(thing), Some(holder)) = (thing, holder) {
            return vec![(WordMove::FindAsked, Some(thing)), (WordMove::GetBefore, Some(holder))];
        }
    }
    let past = words.iter().any(|w| past_form(mind, w) && (COPULA.contains(&w.as_str()) || HAVING.contains(&w.as_str()) || verb_base(mind, w).is_some_and(|b| GIVING.contains(&b.as_str())))) || words.iter().any(|w| w == "before");
    let given = words.iter().position(|w| verb_base(mind, w).is_some_and(|b| GIVING.contains(&b.as_str())));
    if let Some(at) = given.filter(|_| form == WHO_FORM || form == WHAT_FORM) {
        let giver = words[..at].iter().find(|w| noun_word(mind, w)).cloned();
        let thing = words[at + 1..].iter().find(|w| noun_word(mind, w) && !super::physics::person_named(mind, w)).cloned();
        let taker = words[at + 1..].iter().find(|w| super::physics::person_named(mind, w)).cloned();
        let plan = match (form.as_str(), giver, thing, taker) {
            (WHO_FORM, None, Some(thing), Some(taker)) => vec![(WordMove::FindAsked, Some(thing)), (WordMove::GetBefore, Some(taker))],
            (WHO_FORM, None, Some(thing), None) => vec![(WordMove::FindAsked, Some(thing)), (WordMove::GetPast, None)],
            (WHO_FORM, Some(giver), Some(thing), _) => vec![(WordMove::FindAsked, Some(thing)), (WordMove::GetAfter, Some(giver))],
            (_, Some(giver), None, Some(taker)) => vec![(WordMove::FindAsked, Some(giver)), (WordMove::GetGiven, Some(taker))],
            (_, Some(giver), None, None) => vec![(WordMove::FindAsked, Some(giver.clone())), (WordMove::GetGiven, Some(giver))],
            (_, None, None, Some(taker)) => vec![(WordMove::FindAsked, Some(taker)), (WordMove::GetChildren, None)],
            _ => Vec::new(),
        };
        if !plan.is_empty() && !words.iter().any(|w| HAVING.contains(&w.as_str())) {
            return plan;
        }
    }
    if form == WHO_FORM && words.iter().any(|w| (verb_base(mind, w).is_some_and(|b| super::mind::RECEIVING.contains(&b.as_str())) || super::mind::RECEIVING.contains(&super::mind::verb_stem(mind, w).as_str())) && !super::physics::told_relation(mind, &super::mind::verb_stem(mind, w))) {
        return vec![(WordMove::FindAsked, Some((*last).clone())), (WordMove::GetOwner, None)];
    }
    let has_now = form == WHO_FORM && words.iter().any(|w| HAVING.contains(&w.as_str())) && !words.iter().any(|w| w == super::mind::EARLIER);
    if past && !has_now && (form == WHERE_FORM || form == WHO_FORM) && (form == WHERE_FORM || super::physics::left_trace(mind, last) || words.iter().any(|w| !COPULA.contains(&w.as_str()) && past_form(mind, w))) {
        return vec![(WordMove::FindAsked, Some((*last).clone())), (WordMove::GetPast, None)];
    }
    if (form == WHAT_FORM || form == WHO_FORM) && words.first().is_some_and(|w| HAVING.contains(&w.as_str()) || verb_base(mind, w).is_some_and(|b| super::mind::OWNING.contains(&b.as_str()))) {
        return search_walk(mind, last, &words, |n| super::physics::who_has(mind, n).is_some(), (WordMove::GetOwner, None));
    }
    if form == WHAT_FORM && words.len() > 1 {
        if let Some(done) = words.last().filter(|w| super::physics::activity_named(mind, w) && **w != super::mind::DONE_ASKED) {
            return vec![(WordMove::FindAsked, Some(done.clone())), (WordMove::GetRelation, Some(done.clone()))];
        }
    }
    if form == WHO_FORM && words.iter().any(|w| w == super::mind::COMPANION) {
        if let Some(done) = words.iter().find(|w| super::physics::activity_named(mind, w)) {
            return vec![(WordMove::FindAsked, Some(done.clone())), (WordMove::GetRelation, Some(super::mind::COMPANION.to_string()))];
        }
    }
    if form == WHAT_FORM && words.last().is_some_and(|w| w == super::mind::DONE_ASKED) && words.len() > 1 {
        if let Some(person) = words.iter().find(|w| super::physics::told_thing(mind, w)) {
            return vec![(WordMove::FindAsked, Some(person.clone())), (WordMove::GetRelation, Some(super::mind::DONE_ASKED.to_string()))];
        }
    }
    if form == WHERE_FORM {
        if let Some(done) = words.iter().find(|w| super::physics::activity_named(mind, w)) {
            return vec![(WordMove::FindAsked, Some(done.clone())), (WordMove::GetLocation, None)];
        }
    }
    if form == WHERE_FORM || form == WHEN_FORM {
        return search_walk(mind, last, &words, |n| super::physics::place_of(mind, n).is_some(), (WordMove::GetLocation, None));
    }
    if form == WHO_FORM && words.iter().any(|w| place_word(w)) {
        return search_walk(mind, last, &words, |n| super::physics::holds_any(mind, n), (WordMove::GetChildren, None));
    }
    let owning = words.iter().any(|w| HAVING.contains(&w.as_str()) || verb_base(mind, w).is_some_and(|b| super::mind::OWNING.contains(&b.as_str()) || GIVING.contains(&b.as_str()) || super::mind::TAKING.contains(&b.as_str())));
    if (form == WHO_FORM || form == WHAT_FORM) && !owning {
        let object = |verb: &String| words.iter().position(|w| w == verb) < words.iter().position(|w| w == *last);
        for (at, verb) in words.iter().enumerate().filter(|(_, w)| !place_word(w) && !COPULA.contains(&w.as_str()) && !HAVING.contains(&w.as_str()) && !kind_word(mind, w) && !KINDS.iter().any(|(_, k)| *k == w.as_str())) {
            let stem = super::mind::verb_stem(mind, verb);
            if let Some((was, thing)) = words.iter().enumerate().filter(|(i, w)| *i != at && !place_word(w)).rev().find(|(_, w)| super::physics::has_relation(mind, w, &stem)) {
                let get = if at < was && !super::mind::role_word(mind, verb) && !super::mind::ORDINALS.contains(&verb.as_str()) { WordMove::GetSubject } else { WordMove::GetRelation };
                if get == WordMove::GetRelation && !super::physics::things_named(mind, thing).is_empty() {
                    return search_walk(mind, thing, &[], |n| super::physics::node_relates(mind, n, verb), (get, Some(verb.clone())));
                }
                return vec![(WordMove::FindAsked, Some(thing.clone())), (get, Some(verb.clone()))];
            }
        }
        if let Some(verb) = words.iter().find(|w| super::mind::verb_like(mind, w) && !COPULA.contains(&w.as_str()) && (!super::physics::told_thing(mind, w) || past_form(mind, w))).filter(|_| form == WHO_FORM) {
            let get = if object(verb) { WordMove::GetSubject } else { WordMove::GetRelation };
            return vec![(WordMove::FindAsked, Some((*last).clone())), (get, Some(verb.clone()))];
        }
        if form == WHO_FORM {
            return search_walk(mind, last, &words, |_| true, (WordMove::GetKind, Some(form.clone())));
        }
    }
    if form == WHOSE_FORM {
        return search_walk(mind, first, &words, |_| true, (WordMove::GetOwner, None));
    }
    if form == WHO_FORM {
        return search_walk(mind, last, &words, |_| true, (WordMove::GetOwner, None));
    }
    let told_deed = words.iter().find(|w| **w != **last && verb_base(mind, w).is_some() && super::physics::told_relation(mind, &super::mind::verb_stem(mind, w)));
    if let Some(deed) = told_deed.filter(|_| form == HOW_FORM) {
        return vec![(WordMove::FindAsked, Some((*last).clone())), (WordMove::GetRelation, Some(deed.clone()))];
    }
    if form == HOW_FORM {
        let asked = words.iter().rev().find(|w| !things.contains(w) && **w != **last).cloned().or_else(|| Some(form.clone()));
        return match asked {
            Some(verb) => vec![find, (WordMove::GetKind, Some(verb))],
            None => Vec::new(),
        };
    }
    if let Some(relation) = words.iter().find(|w| **w != **last && !place_word(w) && !HAVING.contains(&w.as_str()) && !KINDS.iter().any(|(_, k)| *k == w.as_str()) && super::physics::has_relation(mind, &**last, w)) {
        let get = if super::physics::owns_relation(mind, last, relation) { WordMove::GetRelation } else { WordMove::GetSubject };
        return vec![(WordMove::FindAsked, Some((*last).clone())), (get, Some(relation.clone()))];
    }
    let deed = words.iter().find(|w| **w != **last && !place_word(w) && !HAVING.contains(&w.as_str()) && !COPULA.contains(&w.as_str()) && !kind_word(mind, w) && verb_base(mind, w).is_none_or(|b| !super::mind::known_base(&b)) && super::physics::told_relation(mind, &super::mind::verb_stem(mind, w)));
    if let Some(deed) = deed.filter(|_| form == WHAT_FORM || form == WHO_FORM) {
        return vec![(WordMove::FindAsked, Some((*last).clone())), (WordMove::GetRelation, Some(deed.clone()))];
    }
    if (form == WHAT_FORM || form == WHO_FORM) && first != last && quality_word(mind, last) && super::physics::told_relation(mind, last) {
        return vec![(WordMove::FindAsked, Some((*first).clone())), (WordMove::GetRelation, Some((*last).clone()))];
    }
    let holding = words.iter().any(|w| place_word(w) || HAVING.contains(&w.as_str()) || verb_base(mind, w).is_some_and(|b| CONTAINING.contains(&b.as_str()) || super::mind::OWNING.contains(&b.as_str()) || super::mind::TAKING.contains(&b.as_str())));
    match words.iter().find(|w| kind_word(mind, w)) {
        Some(kind) => search_walk(mind, last, &words, |_| true, (WordMove::GetKind, Some(kind.clone()))),
        None if !holding => search_walk(mind, last, &words, |n| super::physics::says_what(mind, n), (WordMove::GetKind, Some(words.iter().find(|w| *w == super::mind::MADE).cloned().unwrap_or_else(|| form.clone())))),
        None => search_walk(mind, last, &words, |n| super::physics::holds_any(mind, n), (WordMove::GetChildren, None)),
    }
}

fn step_relation() -> String {
    super::mind::ORDER_RELATION.to_string()
}
because!(step_relation, WordGame, "the name of the order relation, for the teacher's test of a word that stands in an order");

fn search_walk(mind: &CursorMind, thing: &str, words: &[String], answers: impl Fn(usize) -> bool, get: (WordMove, Option<String>)) -> Vec<(WordMove, Option<String>)> {
    let candidates = super::physics::things_named(mind, thing);
    let asked = |n: usize| super::physics::question_match(mind, n, thing, words);
    let qualified = words.iter().any(|w| *w != thing && **w != singular(thing) && super::mind::quality_word(mind, w));
    let past_all = if qualified && get.0 == WordMove::GetOwner { candidates.len() } else { 0 };
    let pick = candidates.iter().position(|&n| asked(n) && answers(n)).or_else(|| candidates.iter().position(|&n| asked(n))).unwrap_or(past_all);
    let landed = candidates.iter().position(|&n| asked(n)).unwrap_or_default();
    let mut moves = vec![(WordMove::FindAsked, Some(thing.to_string()))];
    moves.extend(std::iter::repeat_n((WordMove::FindNext, None), pick.saturating_sub(landed)));
    moves.push(get);
    moves
}
because!(search_walk, WordGame, "the search game, which for an owner asked of a thing with a quality none of the name holds walks past the \
     last one, so who has the blue hat finds nothing: the find lands on the newest thing of the name, and while that one does not hold the \
     qualities or the count the question names, or is not had by the one it names, rose's car, or cannot answer it, a find next looks at \
     the next older one, as a person looks in the next box, then the get answers");
because!(
    asked_word,
    WordGame,
    "the moves for a word of a question: how with a verb the story told a value of gets that value, how does the bathroom smell, what time is it, after the story said it is twelve, finds the it the story told and gets what it is, about right after a verb the story told a deed of is left out of the words asked, who cares about tom, as the statement leaves it out, the first word adds the question node, what is a part of a whole, the part said first and of right after it, computes it by the word of the part, what is two fifths of ten, which of two told things a comparison puts ahead finds the first and gets the choice, which is faster a car or a bike, what is the first or the last of a kind the story told nothing of finds the kind and gets that end, what is the first month, what role they have somewhere, with a pronoun that names nobody, finds the thing that has the role and gets it, what language do they speak in spain, who am i steps to the user and gets the name they go by, \
     what or who can do a thing gets all that are able to, what can fly, what is a thing the seeds class, said with a or an or being no quality and told nothing of in the story, what is rain, asks what it \
     is and never which things are it, what is a dog, what my kin is called, or the name of my kin, asks the kin as who is my sister does, \
     who had a thing before a person finds the thing and gets who came before that person among its holders, who had the car before bob, \
     who had a thing without before asks who has it now and never its past, what is next of a thing the story gave a member of an order \
     gets the member after it, what month is next, a sentence to fill in that ends on a place word after its thing asks where the thing \
     is, the key is in the, a role asked of a thing that has none gets that relation and finds nothing, the capital of camelot, why a \
     person did a thing finds the person and gets the kind pointed at why, what is an owner's thing of a kind finds the thing with that \
     kind and gets its name, what is tom's dog, an ordinal and a kind the story told ask every thing that holds the ordinal and is of the \
     kind, what is the third element, a defined name with a number computes the name run on the number, how you go from one place to \
     another finds the first and gets the route to the second, what or where someone said, thought or believed a thing is finds the thing \
     of their claim and gets its kind or its place, what a thing is made of gets its kind pointed at made, which is the material, what is \
     my name, or who is my mother, steps to the user and gets that relation, and is ann my sister steps there, flags the kin and checks \
     the name, what a person plays finds the activity and gets what it is of, what did you say or answer, or say it again, gets what the \
     assistant last answered, which starts first or ends last gets the least or the most of the hours told under that verb, longer and \
     shorter compare how long things told to start and end last, how long a thing told to start and end is finds it and computes the hours \
     between, an article said twice is kept the second time as the letter it is, is a a vowel, on or off is kept when the story told one \
     of them of a counted part, how many fans are off, a single letter after the successor of, or the letter s after any word but the \
     apostrophe, is kept as the letter it is, what a thing is about reads its relation of that name, after a run of numbers next, missing, \
     first, last, largest, smallest, how many numbers and the sum of the numbers compute over the run, an equals sign after an unknown \
     letter is kept and the mark after the other side computes the unknown, as does an asking word, which then opens its own question, \
     who, what or which with a negation asks every thing denied the value named last, which is not a bird, a perimeter with lengths told \
     computes how far it is around, plural or singular computes that form of the last word, a number in words computes its spelling, and \
     what number with number words alone computes what they spell, every word between two quotes is kept, what with a past copula and one \
     value asks every thing holding it, what was sunday, how many of a unit with one amount of another computes the amount in that unit, \
     how many weeks are fourteen days, a yes or no check searches past a thing of the name that lacks the value, does a human have a \
     heart, letter with an ordinal computes that letter, how many letters, vowels, consonants, digits or words computes the count, a sum \
     that opens with a sign computes on from the number last said, who with an activity and with asks the activity's companion, who did \
     ann run with, what did a person do asks their activity, where with an activity the story told finds the activity and asks its place, \
     where does tom swim, and who with one asks everyone who has it, who had fun, a bracket, a power sign or a function name makes it an \
     expression to compute, a yes or no question that offers two options asks which the thing holds, is the ball red or blue, a direction \
     checks or reads backward from the thing after it, what is east of the office, who with a word of kin reads that relation of the \
     person named, who is ben's grandmother, whose with a relation reads it backward from the one named, whose mother is ann, a role or a \
     word of kin after its holder checks from that holder, is ann tom's mother, a superlative with no thing named asks every thing holding \
     it and the kind beside it, what is the largest planet, a mark that only parts the sentence is passed over, until asks the distance to \
     the member named, how many days until friday, as two hours of the clock ask the distance between them, and what time asks the hour \
     shifted by the hours counted, or the hour itself, a number with a kind of an order asks the member shifted, what day will it be in \
     three days, and a word of an order the story told nothing of asks it shifted from the one it told, what day is tomorrow, what comes \
     after or before a thing reads the order relation forward or backward from it, what season comes after spring, the letter a being kept \
     after it, and a number needs no find; who has more or fewer of a thing asks the holder with the most or the fewest, more or less with \
     a relation the story counts asks the one with the most or the least under it, what costs more, as a comparison of a measure the story \
     told asks the one with the most or the least of it, who is older; a word that compares with two numbers computes, as does odd, even \
     or a unit of place value with one, and alphabet with the words after it; how many over several holders or together asks the total, \
     and with than the difference; whose searches from the thing named after it, whose party is in spring; and an ordinal reads forward, \
     the first day of the week, a question on giving finds the thing given and asks who had it, who gave the football to jeff, or who had \
     it next, who did jeff give the apple to, or finds the taker and asks what they hold, what did mary give to bill; who received or is \
     holding asks the owner; a yes or no question with a place word flags it before its check, of the thing owned when an owner is said \
     before it, is tom s dog in the garden, is oxford in the vault, as does one asked with does and a verb that places, does the box fit \
     in the pen; made counts for nothing beside the one value of a what question, what is made of stone; a yes or no question with a \
     superlative checks the superlative of the thing, is mars the largest planet; an ordinal with a kind the seeds rank so asks the seeds for it, what is the third planet; \
     a filler that stands in an order of the seeds is kept, \
     may; a yes or no question on a role flags the role and checks it from the thing or the number after of, is blick the author of zog, \
     or from the first thing for same, and who with a role reads it forward from the thing after of, who is the author of zog, a yes or no \
     question that asks the network who it is or what its name is steps to the network and names it, who are you, a question that asks the network where it is, when the story put it nowhere, steps to the thing the talk is about and names it, where are you, a question that asks the network what it is curious about steps to the newest thing told and names it, a question on a deed the story tells, or on a feeling it tells as a relation, flags it before its check, does ann see tom, is emily \
     afraid of sheep, and what with such a feeling last reads it of the one named first, what is winona afraid of, searching past a thing \
     of the name that lacks the deed, the duck in the lake, who or what with a word naming a relation the thing holds reads that relation, \
     what cuts wood, and one naming a relation the story told of another thing reads it too, which finds nothing, what does a bee make, \
     forward when the thing comes before the verb, who does ann hear, and backward when the verb comes first, who sees the cat, a yes or \
     no question with a comparison flags the comparison before its check, is ann shorter than tom, and one with by checks the doer, is ann \
     liked by tom; who with by reads the verb backward, who is ann liked by; who or what with a comparison and than reads the comparison \
     backward from the thing after than, who is taller than tom, a question of a place asked with a past copula flags the past before its \
     check, was mary in the school, about asks everything of a thing, tell me about tom; where before or after a place asks the place next \
     to it in the thing's past, where was mary before the cinema; and as does a copula, a helper or a word that asks after a complete \
     clause, i have a dog do i have a dog; a question opened by a helper or a request that holds a word that asks is asked by that word \
     and the words after it, tell me what is black, do you know where tom is; a word after a question already answered names its result, \
     one plus one equals sum; at the mark what with a word alone writes every thing holding it, what is white, and what or who with having \
     first finds the thing had and gets who has it, what has water; at the mark a question with a word of arithmetic and numbers is worked \
     out, what is two plus three; a question with a word naming a relation the thing asked holds, or is held by, gets the value under it, \
     the capital of france; an article, a link word, a helper or a copula points at nothing, and any other word is written under the \
     question node; at the question mark the question is on the stack and the walk is two moves, the thing asked being a word that names a \
     thing the story told, a quality only when nothing else is named, else a word that can name one: a question opened by a copula or a \
     helper finds the first thing it names and checks the last word on it, yes or no; where was and who gave find the thing and get where \
     it was or who had it, from the trace it left; where and when find the thing and get its location, who with having finds it and gets \
     its owner, who without it gets what it is, who is ben's sister, how much with a thing alone gets the amount a relation of it holds, \
     how finds it and gets the property its last other word names, how does kim feel; a what with a kind word finds the thing and gets its \
     property of that kind; and any other what finds the thing and gets what stands inside it when it names a place, having or holding, \
     else the class under its is, what is felix"
);
