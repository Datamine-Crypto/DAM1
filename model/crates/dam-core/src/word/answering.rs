use super::mind::{place_word, HAVING, open_question, flag_of, is_number, kind_of, landed_on, noun_word, quality_word, singular, verb_base, COPULA, FLAG_INDEFINITE, FLAG_PROPERTY, FLAG_DEFINITE, FLAG_TIME};
use super::moves::{WordMove, WordStep};
use crate::cursor::{step_item, CursorMind, BRACE_OPEN_TEXT, TIME_RELATION};
use crate::quiz::IS_FORM;
use crate::words::number_of;
use super::lookup::{present_children, tag_value, story_node, story_nodes, claim_right, operand_value, span_of, unit_worth, own_child, places_told, run_answer, person, story_nodes_all, quantity_tag_of, sides_of, count_of, place_value, speaker, trace, owner_of, to_come, counted_within, held_or_owned, denied_value, holds_value, order_step, ordered_told, grandparent, equation_solved, expression_value, asked_numbers, asked_number, relation_named, related_chain, measured_by, opposites, compared_through, class_things, induced, holds_through, chain_end, motives, asks_having, time_of_day};
use super::lookup::{ranked_in_seeds, things_named, claim_node, seeded_class, thing_number, activity_word, spans_told, converted, told_relation, told_thing, number_run, place_of, who_has, shifted_value, named_result, measure_asked, map_route};
use super::written::{thing_made, added_under, written_out};
use super::physics::{WordWorld, ANSWER_TAG, CAUSE_TAG, CLASS_DEPTH, KIND_STEM, ORDER_SPAN, OWNER_TAG, PLACE_DEPTH};
use patterns::because;

pub(super) fn told_back(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool, _braced: bool) -> bool {
    let word = word.to_string();
    match step.act {
        WordMove::GetChoice => {
            let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
            let split = asked.iter().position(|a| a == super::mind::CHOICE_WORD);
            let options: Vec<String> = split.map(|at| asked[..at].iter().rev().find(|a| noun_word(mind, a) || quality_word(mind, a)).into_iter().chain(asked[at + 1..].iter().find(|a| noun_word(mind, a) || quality_word(mind, a))).cloned().collect()).unwrap_or_default();
            let compared = asked.iter().find(|a| super::mind::compared_relation(a).is_some() && !options.contains(a)).cloned();
            if let (Some(comparison), [one, other]) = (compared, &options[..]) {
                let ahead = |by: &str, a: &str, b: &str| super::mind::compared_relation(by).is_some_and(|r| story_nodes(mind, a).into_iter().any(|n| related_chain(mind, n, &step_item(&r), b, CLASS_DEPTH)));
                let reversed = opposites(mind, &comparison);
                let first_wins = ahead(&comparison, one, other) || reversed.iter().any(|o| ahead(o, other, one));
                let other_wins = ahead(&comparison, other, one) || reversed.iter().any(|o| ahead(o, one, other));
                written_out(mind, step.act, if first_wins { Some(one.clone()) } else if other_wins { Some(other.clone()) } else { None });
                return true;
            }
            let chosen = options.into_iter().find(|o| plain && holds_value(mind, at, o));
            written_out(mind, step.act, chosen);
        }
        WordMove::GetShifted => {
            let said = shifted_value(mind, plain.then_some(at));
            written_out(mind, step.act, said);
        }
        WordMove::GetReply => {
            let relation = step_item(super::mind::REPLY_RELATION);
            let said = mind.before.iter().find_map(|b| mind.tree.named(b).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0).find_map(|n| own_child(mind, n, &relation)).and_then(|r| present_children(mind, r).into_iter().next())).map(|v| mind.tree.node(v).name.to_string());
            written_out(mind, step.act, said);
        }
        WordMove::GetDistanceRoute => {
            for way in map_route(mind, &crate::cursor::bare_name(&mind.tree.node(at).name), &word).unwrap_or_default() {
                written_out(mind, step.act, Some(way));
            }
        }
        WordMove::GetDistance => {
            let said: Vec<String> = asked_numbers(mind).into_iter().map(|n| crate::cursor::worked_text(n, None)).collect();
            let (from, word) = match &said[..] {
                [first, second] if number_of(&word).is_none() || said.len() > 1 => (Some(first.clone()), second.clone()),
                _ => (ordered_told(mind).map(|(_, value)| value), word),
            };
            let steps = from.and_then(|from| (0..=ORDER_SPAN).scan(Some(from), |now, _| { let here = now.clone(); *now = now.as_ref().and_then(|name| order_step(mind, name, true)); here }).position(|name| name == word || name == singular(&word)));
            written_out(mind, step.act, steps.map(|n| n.to_string()));
        }
        WordMove::GetPast => {
            let traces: Vec<usize> = if plain { mind.tree.named(&mind.tree.node(at).name.to_string()).filter(|&n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(at) && trace(mind, n)).collect() } else { Vec::new() };
            let asks_who = open_question(mind).is_some_and(|q| *mind.tree.node(q).name == *super::mind::WHO_ASKED);
            let mut owners: Vec<usize> = traces.iter().map(|&t| mind.tree.node(t).parent).filter(|&p| p != 0 && asks_who).collect();
            owners.dedup();
            if owners.len() > 1 && owners.len() == traces.len() {
                for owner in owners {
                    written_out(mind, step.act, Some(crate::cursor::bare_name(&mind.tree.node(owner).name)));
                }
                return true;
            }
            let earlier = || {
                let places = super::lookup::goings_in_order(mind, at);
                let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| crate::cursor::bare_name(&mind.tree.node(c).name)).collect()).unwrap_or_default();
                let named = places.iter().rposition(|&p| asked.iter().any(|one| *one == crate::cursor::bare_name(&mind.tree.node(p).name)));
                let onward = asked.iter().any(|one| *one == super::mind::LATER);
                match named {
                    Some(at) if onward => places.get(at + 1).copied(),
                    Some(at) => at.checked_sub(1).and_then(|before| places.get(before).copied()),
                    None if places.len() >= super::physics::PAIR_LEAST => Some(places[places.len() - super::physics::PAIR_LEAST]),
                    None => (!places.is_empty()).then(|| mind.tree.node(at).parent).filter(|&p| p != 0 && !mind.tree.node(p).name.starts_with(BRACE_OPEN_TEXT) && !person(mind, p)),
                }
            };
            let was = traces.into_iter().max().map(|t| mind.tree.node(t).parent).filter(|&p| p != 0).or_else(|| plain.then(earlier).flatten()).or_else(|| plain.then(|| who_has(mind, at)).flatten()).or_else(|| plain.then(|| place_of(mind, at)).flatten().filter(|_| !asks_who));
            written_out(mind, step.act, was.map(|p| crate::cursor::bare_name(&mind.tree.node(p).name)));
        }
        WordMove::GetAbout => {
            let mut said: Vec<String> = Vec::new();
            if plain && mind.tree.story(at) {
                if let Some(near) = owner_of(mind, at).or_else(|| place_of(mind, at)) {
                    said.push(mind.tree.node(near).name.to_string());
                }
                said.extend(held_or_owned(mind, at).into_iter().map(|c| mind.tree.node(c).name.to_string()));
                for r in present_children(mind, at).into_iter().filter(|&r| { let name = &*mind.tree.node(r).name; name.starts_with(BRACE_OPEN_TEXT) && *name != *TIME_RELATION && *name != *OWNER_TAG && !name.starts_with(crate::cursor::QUANTITY_TAG) && !tag_value(mind, r) && *name != step_item(FLAG_DEFINITE) && *name != step_item(FLAG_INDEFINITE) }) {
                    for v in present_children(mind, r) {
                        let values = if mind.tree.node(v).name.starts_with(BRACE_OPEN_TEXT) { present_children(mind, v) } else { vec![v] };
                        said.extend(values.into_iter().filter(|&x| !mind.tree.node(x).name.starts_with(BRACE_OPEN_TEXT)).map(|x| mind.tree.node(x).name.to_string()));
                    }
                }
            }
            let mut kept: Vec<String> = Vec::new();
            for name in said {
                if !kept.contains(&name) {
                    kept.push(name);
                }
            }
            let said = kept;
            if said.is_empty() {
                written_out(mind, step.act, None);
            }
            for name in said {
                written_out(mind, step.act, Some(name));
            }
        }
        WordMove::GetGiven => {
            let had: Vec<usize> = if plain { present_children(mind, at).into_iter().filter(|&t| trace(mind, t)).filter_map(|t| mind.tree.node(t).link).collect() } else { Vec::new() };
            let own = plain && { let name = &*mind.tree.node(at).name; *name == *word };
            let given: Vec<String> = had.into_iter().filter(|&thing| !mind.tree.node(thing).gone && (own || who_has(mind, thing).is_some_and(|o| *mind.tree.node(o).name == *word))).map(|thing| mind.tree.node(thing).name.to_string()).collect();
            if given.is_empty() {
                written_out(mind, step.act, None);
            }
            for name in given {
                written_out(mind, step.act, Some(name));
            }
        }
        WordMove::GetBefore | WordMove::GetAfter => {
            let mut places: Vec<usize> = if plain { mind.tree.named(&mind.tree.node(at).name.to_string()).filter(|&n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(at) && trace(mind, n)).collect() } else { Vec::new() };
            places.sort_unstable();
            let now = if plain { place_of(mind, at).or_else(|| who_has(mind, at)) } else { None };
            if now.is_some() {
                places.push(at);
            }
            let rank = |n: usize| time_of_day(mind, n).and_then(|t| super::mind::TIMES_OF_DAY.iter().position(|d| *d == t));
            if places.iter().all(|&n| rank(n).is_some()) {
                places.sort_by_key(|&n| rank(n));
            }
            let mut sequence: Vec<usize> = places.into_iter().map(|n| if n == at { now.unwrap_or(at) } else { mind.tree.node(n).parent }).collect();
            if sequence.len() < super::physics::PAIR_LEAST && plain {
                let walked = super::lookup::goings_in_order(mind, at);
                if walked.len() >= super::physics::PAIR_LEAST {
                    sequence = walked;
                }
            }
            let named = sequence.iter().rposition(|&p| *mind.tree.node(p).name == *word || crate::cursor::bare_name(&mind.tree.node(p).name) == word);
            let found = named.and_then(|i| if step.act == WordMove::GetBefore { i.checked_sub(1) } else { Some(i + 1) }).and_then(|i| sequence.get(i).copied());
            written_out(mind, step.act, found.map(|p| mind.tree.node(p).name.to_string()));
        }
        WordMove::GetAmount => {
            let amount = if plain { present_children(mind, at).into_iter().filter(|&r| mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT)).flat_map(|r| present_children(mind, r)).find(|&v| !quantity_tag_of(mind, v).is_empty()).map(|v| count_of(mind, v)) } else { None };
            written_out(mind, step.act, amount.map(|a| a.to_string()));
        }
        WordMove::GetSubject | WordMove::GetRelation if (word == super::mind::LATER || word == super::mind::EARLIER) && (plain && number_of(&mind.tree.node(at).name).is_some() || asked_number(mind).is_some()) => {
            let forward = step.act == WordMove::GetRelation;
            let from = plain.then(|| number_of(&mind.tree.node(at).name)).flatten().or_else(|| asked_number(mind));
            let next = from.map(|n| crate::cursor::worked_text(if forward { n + f32::from(u8::from(true)) } else { n - f32::from(u8::from(true)) }, None));
            written_out(mind, step.act, next);
        }
        WordMove::GetSubject => {
            let relation = relation_named(mind, &word);
            let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
            let told = !super::mind::role_word(mind, &word) && mind.tree.named(&relation).any(|r| !mind.tree.node(r).gone && mind.tree.story(r));
            let subjects: Vec<usize> = mind.tree.named(&name).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *relation).map(|v| mind.tree.node(mind.tree.node(v).parent).parent).filter(|&t| t != 0 && t != at).collect();
            let back = subjects.iter().copied().find(|&t| mind.tree.story(t)).or(subjects.first().copied().filter(|_| !told));
            let turned = || opposites(mind, &word).into_iter().flat_map(|o| [super::mind::compared_relation(&o), Some(o)]).flatten().find_map(|o| own_child(mind, at, &step_item(&o))).and_then(|r| present_children(mind, r).into_iter().next());
            let found = back.or_else(|| plain.then(turned).flatten());
            let mut every: Vec<String> = subjects.iter().copied().filter(|&t| mind.tree.story(t)).map(|t| crate::cursor::bare_name(&mind.tree.node(t).name)).collect();
            every.sort();
            every.dedup();
            if every.len() > 1 {
                for name in every {
                    written_out(mind, step.act, Some(name));
                }
                return true;
            }
            written_out(mind, step.act, found.map(|n| crate::cursor::bare_name(&mind.tree.node(n).name)));
        }
        WordMove::GetRelationSaid => {
            let said: Vec<String> = own_child(mind, 0, ANSWER_TAG).map(|last| present_children(mind, last).into_iter().map(|v| mind.tree.node(v).name.to_string()).collect()).unwrap_or_default();
            if said.is_empty() {
                written_out(mind, step.act, None);
            }
            for text in said {
                written_out(mind, step.act, Some(text));
            }
        }
        WordMove::GetRelationToward => {
            let of: Vec<String> = own_child(mind, at, &step_item(super::mind::TOWARD)).map(|r| present_children(mind, r).into_iter().map(|v| mind.tree.node(v).name.to_string()).collect()).unwrap_or_default();
            if of.is_empty() {
                written_out(mind, step.act, None);
            }
            for text in of {
                written_out(mind, step.act, Some(text));
            }
        }
        WordMove::GetRelation => relation_got(mind, step, &word, at, plain),
        WordMove::GetRelationDoing => {
            let relation = relation_named(mind, &word);
            let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
            let holder = relation_holder(mind, &name, &relation, plain);
            let doing = holder.and_then(|n| own_child(mind, n, &relation).and_then(|r| own_child(mind, r, &step_item(super::mind::ACTIVITY))).or_else(|| own_child(mind, n, &step_item(super::mind::ACTIVITY)))).and_then(|r| present_children(mind, r).into_iter().last());
            written_out(mind, step.act, doing.map(|n| crate::cursor::bare_name(&mind.tree.node(n).name)));
        }
        WordMove::GetRelationThrough => {
            let through = plain.then(|| grandparent(mind, at, &word)).flatten();
            written_out(mind, step.act, through.map(|n| crate::cursor::bare_name(&mind.tree.node(n).name)));
        }
        WordMove::GetRelationRanked => {
            let relation = relation_named(mind, &word);
            let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
            let ranked = plain.then(|| ranked_value(mind, &name, &relation)).flatten();
            written_out(mind, step.act, ranked.map(|n| crate::cursor::bare_name(&mind.tree.node(n).name)));
        }
        WordMove::GetRelationBackward => {
            let relation = relation_named(mind, &word);
            let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
            let back = backward_holder(mind, &name, &relation, at);
            written_out(mind, step.act, back.map(|n| crate::cursor::bare_name(&mind.tree.node(n).name)));
        }
        WordMove::Compute => computed(mind, step, &word, at, plain),
        WordMove::GetAllWith => all_with(mind, step, &word),
        WordMove::GetName => {
            let goes_by = plain.then(|| own_child(mind, at, &step_item(super::mind::NAME_ROLE)).and_then(|r| present_children(mind, r).into_iter().next())).flatten();
            let said_to_be = || own_child(mind, at, IS_FORM.trim()).and_then(|is| present_children(mind, is).into_iter().find(|&v| { let name = &*mind.tree.node(v).name; !name.starts_with(BRACE_OPEN_TEXT) && !seeded_class(mind, name) && !quality_word(mind, name) }));
            let named = goes_by.or_else(|| (plain && speaker(mind, at)).then(said_to_be).flatten()).map(|v| mind.tree.node(v).name.to_string());
            let seeded_name = || plain.then(|| mind.tree.named(&mind.tree.node(at).name.to_string()).filter(|&n| n != at && n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0).find_map(|n| own_child(mind, n, &step_item(super::mind::NAME_ROLE))).and_then(|r| present_children(mind, r).into_iter().next()).map(|v| mind.tree.node(v).name.to_string())).flatten();
            let named = named.or_else(seeded_name);
            let name = named.or_else(|| (plain && !speaker(mind, at)).then(|| crate::cursor::bare_name(&mind.tree.node(at).name)));
            written_out(mind, step.act, name);
        }
        WordMove::GetMost | WordMove::GetLeast => {
            let fits = |c: usize| { let name = &*mind.tree.node(c).name; *name == *word || *name == *singular(&word) };
            let measure = measure_asked(mind, &word).map_or_else(|| singular(&word), |(m, _)| m);
            let related = |n: usize| own_child(mind, n, &step_item(&measure)).into_iter().flat_map(|r| present_children(mind, r)).filter_map(|q| number_of(&mind.tree.node(q).name).or_else(|| present_children(mind, q).into_iter().find_map(|t| mind.tree.node(t).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(t).name)).flatten()))).map(|q| q as usize).sum::<usize>() + span_of(mind, n).filter(|_| measure == super::mind::SPAN_ASKED).map_or(0, |span| span as usize);
            let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
            let named: Vec<usize> = asked.iter().filter(|a| **a != word).filter_map(|a| story_node(mind, a).or_else(|| mind.tree.named(a).find(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0))).collect();
            let counted = |over: Vec<usize>| -> Vec<(usize, usize)> { over.into_iter().map(|n| (held_or_owned(mind, n).into_iter().filter(|&c| fits(c)).map(|c| count_of(mind, c)).sum::<usize>() + related(n), n)).filter(|(count, _)| *count > 0).collect() };
            let of_named = counted(named);
            let mut counts = if of_named.len() > 1 { of_named } else { counted(story_nodes_all(mind)) };
            counts.sort_unstable();
            let found = if step.act == WordMove::GetMost { counts.last() } else { counts.first() };
            written_out(mind, step.act, found.map(|(_, n)| mind.tree.node(*n).name.to_string()));
        }
        WordMove::NumberSay => {
            let said = mind.number.map(|n| crate::cursor::worked_text(n, mind.places));
            written_out(mind, step.act, said);
        }
        act if super::moves::NUMBER_KINDS.iter().any(|(m, _)| *m == act) => number_worked(mind, step, &word),
        WordMove::GetTotal | WordMove::GetDifference => {
            let all = super::mind::GENERAL_THINGS.contains(&word.as_str());
            let fits = |c: usize| { let name = &*mind.tree.node(c).name; all || *name == *word || *name == *singular(&word) };
            let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
            let amount = |c: usize| if fits(c) { Some(count_of(mind, c) as f32) } else if all { None } else { unit_worth(mind, &mind.tree.node(c).name, &word).map(|worth| worth * count_of(mind, c) as f32) };
            let holders: Vec<usize> = story_nodes_all(mind).into_iter().filter(|&n| held_or_owned(mind, n).into_iter().any(|c| amount(c).is_some())).collect();
            let named: Vec<usize> = holders.iter().copied().filter(|&n| asked.iter().any(|a| **a == *mind.tree.node(n).name)).collect();
            let over = if named.is_empty() { holders } else { named };
            let counts: Vec<f32> = over.iter().map(|&n| held_or_owned(mind, n).into_iter().filter_map(|c| amount(c)).sum()).collect();
            let most = counts.iter().copied().reduce(f32::max).unwrap_or_default();
            let least = counts.iter().copied().reduce(f32::min).unwrap_or_default();
            let total: f32 = if step.act == WordMove::GetDifference { most - least } else { counts.iter().sum() };
            written_out(mind, step.act, (!over.is_empty()).then(|| crate::cursor::worked_text(total, None)));
        }
        WordMove::GetCount => {
            let all = super::mind::GENERAL_THINGS.contains(&word.as_str());
            let fits = |c: usize, by_name: bool| { let name = &*mind.tree.node(c).name; all || *name == *word || *name == *singular(&word) || !by_name && holds_value(mind, c, &singular(&word)) };
            let counted = if plain { counted_within(mind, at, &fits, CLASS_DEPTH) } else { 0 };
            let parts_seeded = |n: usize| [word.clone(), singular(&word)].iter().find_map(|part| own_child(mind, n, &step_item(part))).filter(|&r| present_children(mind, r).into_iter().any(|t| mind.tree.node(t).name.starts_with(crate::cursor::QUANTITY_TAG))).map(|r| count_of(mind, r));
            let counted = if counted == 0 && plain && !all { parts_seeded(at).or_else(|| mind.tree.named(&mind.tree.node(at).name.to_string()).filter(|&n| n != at && n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0).find_map(parts_seeded)).unwrap_or(counted) } else { counted };
            let worths: Vec<f32> = if plain && !all { held_or_owned(mind, at).into_iter().filter(|&c| !fits(c, false)).filter_map(|c| unit_worth(mind, &mind.tree.node(c).name, &word).map(|worth| worth * count_of(mind, c) as f32)).collect() } else { Vec::new() };
            if !worths.is_empty() {
                let total = worths.iter().sum::<f32>() + counted as f32;
                written_out(mind, step.act, Some(crate::cursor::worked_text(total, None)));
                return true;
            }
            let denied = plain && present_children(mind, at).into_iter().any(|c| count_of(mind, c) == 0 && { let name = &*mind.tree.node(c).name; *name == *word || *name == *singular(&word) });
            written_out(mind, step.act, (counted > 0 || denied || (all && plain && mind.tree.story(at))).then(|| counted.to_string()));
        }
        WordMove::GetLocationClaimed => {
            let name = mind.tree.node(at).name.to_string();
            let place = mind.tree.named(&name).filter(|&n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(at)).map(|n| mind.tree.node(n).parent).find(|&p| p != 0 && !mind.tree.node(p).name.starts_with(BRACE_OPEN_TEXT));
            written_out(mind, step.act, place.map(|p| crate::cursor::bare_name(&mind.tree.node(p).name)));
        }
        WordMove::GetLocationMotive => {
            let place = motives(mind, at).into_iter().next().map(|(_, place)| place);
            written_out(mind, step.act, place);
        }
        WordMove::GetLocation => {
            if plain && to_come(mind, at) {
                written_out(mind, step.act, None);
                return true;
            }
            let places = places_of(mind, at, plain);
            if places.len() > 1 {
                for place in places {
                    written_out(mind, step.act, Some(place));
                }
                return true;
            }
            written_out(mind, step.act, places.into_iter().next());
        }
        WordMove::GetOwnerEvery => {
            let thing = mind.tree.node(at).link.unwrap_or(at);
            let mut owners: Vec<String> = own_child(mind, thing, OWNER_TAG).map(|tag| present_children(mind, tag).into_iter().filter_map(|m| mind.tree.node(m).link).map(|o| mind.tree.node(o).name.to_string()).collect()).unwrap_or_default();
            owners.sort();
            owners.dedup();
            for owner in owners {
                written_out(mind, step.act, Some(owner));
            }
        }
        WordMove::GetOwner => {
            let owner = plain.then(|| owner_told(mind, at)).flatten();
            written_out(mind, step.act, owner.map(|p| crate::cursor::bare_name(&mind.tree.node(p).name)));
        }
        WordMove::GetChildren => {
            let asks_having = asks_having(mind);
            let asks_who = open_question(mind).is_some_and(|q| *mind.tree.node(q).name == *super::mind::WHO_ASKED);
            let had = |c: usize| (!asks_having || person(mind, at) || speaker(mind, at) || owner_of(mind, c) == Some(at)) && (!asks_who || person(mind, c) || speaker(mind, c) || owner_of(mind, c).is_none());
            let inside: Vec<String> = if plain { held_or_owned(mind, at).into_iter().filter(|&c| had(c)).map(|c| mind.tree.node(c).name.to_string()).collect() } else { Vec::new() };
            if inside.is_empty() {
                written_out(mind, step.act, None);
            }
            for name in inside {
                written_out(mind, step.act, Some(name));
            }
        }
        WordMove::GetKindWhy => {
            let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
            if let Some(why) = own_child(mind, at, CAUSE_TAG) {
                for cause in present_children(mind, why) {
                    let name = mind.tree.node(cause).name.to_string();
                    written_out(mind, step.act, Some(name));
                }
                return true;
            }
            let sent = motives(mind, at);
            let state = sent.iter().find(|(_, place)| asked.iter().any(|a| a == place)).or(sent.first()).map(|(state, _)| state.clone());
            written_out(mind, step.act, state);
        }
        WordMove::GetKindMeasure => {
            written_out(mind, step.act, None);
        }
        WordMove::GetKind => kind_got(mind, step, &word, at, plain),
        WordMove::CheckRelation => property_checked(mind, step, &word, at, plain),
        WordMove::CheckRight => {
            let right = claim_right(mind, at).unwrap_or_default();
            written_out(mind, step.act, Some(if right { crate::quiz::YES } else { crate::quiz::NO }.to_string()));
        }
        WordMove::Check => {
            let was = flag_of(mind, FLAG_TIME).is_some() && plain && mind.tree.named(&mind.tree.node(at).name.to_string()).any(|n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(at) && trace(mind, n) && { let place = &*mind.tree.node(mind.tree.node(n).parent).name; *place == *word || *place == *singular(&word) });
            let own_kind = plain && kind_of(mind, &word).is_some_and(|k| own_child(mind, at, IS_FORM.trim()).and_then(|i| own_child(mind, i, &step_item(&k))).is_some());
            let borrowed = plain && !own_kind && quality_word(mind, &word) && !denied_value(mind, at, &word) && induced(mind, at, |n| holds_value(mind, n, &word)).is_some();
            let shifted = plain && !mind.tree.story(at) && shifted_value(mind, Some(at)).is_some_and(|member| member == word);
            let said_digits = open_question(mind).and_then(|q| present_children(mind, q).into_iter().find_map(|c| number_of(&mind.tree.node(c).name)));
            let counted_right = said_digits.filter(|_| plain).and_then(|said| held_or_owned(mind, at).into_iter().find(|&c| { let name = &*mind.tree.node(c).name; *name == *word || *name == *singular(&word) }).map(|c| count_of(mind, c) as f32).or_else(|| measured_by(mind, at, &singular(&word))).map(|count| (count - said).abs() < f32::EPSILON));
            let asks_having = asks_having(mind);
            let only_deed = asks_having && plain && mind.tree.story(at) && !held_or_owned(mind, at).into_iter().any(|c| { let name = &*mind.tree.node(c).name; *name == *word || *name == *singular(&word) }) && present_children(mind, at).into_iter().filter(|&r| mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT) && verb_base(mind, &crate::cursor::bare_name(&mind.tree.node(r).name)).is_some_and(|b| super::mind::PARTING.contains(&b.as_str()) || super::mind::GAINING.contains(&b.as_str()))).flat_map(|r| present_children(mind, r)).any(|v| { let name = &*mind.tree.node(v).name; *name == *word || *name == *singular(&word) });
            let at_end = plain && chain_end(mind, &word).is_some_and(|end| *mind.tree.node(at).name == *end);
            let held = was || borrowed || shifted || at_end || (plain && !only_deed && holds_value(mind, at, &word));
            let held = counted_right.unwrap_or(held);
            let kinds_beside: Vec<String> = if word.ends_with(super::mind::SUPERLATIVE_END) { open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).skip_while(|a| *a != word).skip(1).filter(|a| noun_word(mind, a)).collect()).unwrap_or_default() } else { Vec::new() };
            let held = held && kinds_beside.iter().all(|k| plain && own_child(mind, at, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|v| *mind.tree.node(v).name == **k)));
            written_out(mind, step.act, Some(if held { crate::quiz::YES } else { crate::quiz::NO }.to_string()));
        }
        _ => return false,
    }
    true
}
because!(told_back, WordWorld, "the moves that answer a question: each get reads one thing off the world and writes it out, the compute works the numbers of the question, and the check says yes or no");

fn computed(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool) {
    let word = word.to_string();
    let numbers: Vec<f32> = open_question(mind).map(|q| present_children(mind, q).into_iter().filter_map(|c| { let said = mind.tree.node(c).name.to_string(); named_result(mind, &said).or_else(|| is_number(mind, &said).and_then(|n| number_of(&n))) }).collect()).unwrap_or_default();
    let asked_now: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    if let Some(value) = converted(mind, &word, &asked_now).filter(|_| number_of(&word).is_none() && super::mind::number_operation(&word).is_none()) {
        written_out(mind, step.act, Some(crate::cursor::worked_text(value, None)));
        return;
    }
    let asks_span = spans_told(mind) && (word == super::mind::SPAN_ASKED || measure_asked(mind, &word).is_some_and(|(m, _)| m == super::mind::SPAN_ASKED));
    let spans: Vec<f32> = if asks_span { asked_now.iter().flat_map(|a| things_named(mind, a)).filter_map(|n| span_of(mind, n)).collect() } else { Vec::new() };
    if spans.len() > 1 {
        let apart = spans.iter().copied().reduce(f32::max).unwrap_or_default() - spans.iter().copied().reduce(f32::min).unwrap_or_default();
        let compared = asked_now.iter().any(|a| a == super::mind::COMPARED);
        written_out(mind, step.act, Some(crate::cursor::worked_text(if compared { apart } else { spans.iter().sum() }, None)));
        return;
    }
    if asks_span && plain {
        written_out(mind, step.act, span_of(mind, at).map(|span| crate::cursor::worked_text(span, None)));
        return;
    }
    let run = number_run(mind);
    if run.len() > 1 && numbers.is_empty() {
        let told = if super::mind::RUN_ASKED.contains(&word.as_str()) { run_answer(&run, &word) } else if super::mind::COUNTING.contains(&word.as_str()) { Some(run.len() as f32) } else if word == super::mind::RUN_SUM { Some(run.iter().sum()) } else { None };
        if told.is_some() || super::mind::RUN_ASKED.contains(&word.as_str()) {
            written_out(mind, step.act, told.map(|t| crate::cursor::worked_text(t, None)));
            return;
        }
    }
    if let Some((letter, value)) = equation_solved(mind) {
        if let Some(question) = open_question(mind) {
            mind.tree.moved(question, None);
        }
        mind.question_start = None;
        let named = story_node(mind, &letter).unwrap_or_else(|| thing_made(mind, &letter));
        let is = added_under(mind, named, IS_FORM.trim(), false);
        added_under(mind, is, &crate::cursor::worked_text(value, None), true);
        landed_on(mind, step.act, None);
        mind.at = 0;
        return;
    }
    if let Some(value) = expression_value(mind) {
        let value = if value.is_finite() && places_told(mind).is_none() { (value * super::mind::SHOWN_THOUSANDTHS).round() / super::mind::SHOWN_THOUSANDTHS } else { value };
        written_out(mind, step.act, Some(crate::cursor::worked_text(value, places_told(mind))));
        return;
    }
    let op = super::mind::number_operation(&word);
    let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let holds = |w: &str| asked.iter().any(|a| a == w);
    let odd = |n: f32| (n % super::mind::HALVING).abs() > f32::EPSILON;
    let said = |text: String| Some(text);
    if word == super::mind::AROUND_ASKED {
        let lengths: Vec<f32> = asked.iter().filter_map(|a| number_of(a)).collect();
        let sides = asked.iter().filter(|a| number_of(a).is_none()).find_map(|a| sides_of(mind, a));
        let around = sides.filter(|_| !lengths.is_empty()).map(|sides| lengths.iter().sum::<f32>() * sides as f32 / lengths.len() as f32);
        written_out(mind, step.act, around.map(|n| crate::cursor::worked_text(n, None)));
        mind.number = None;
        return;
    }
    if word == super::mind::PLURAL_ASKED || word == super::mind::SINGULAR_ASKED {
        let target = asked.iter().rev().find(|a| **a != word && a.chars().all(char::is_alphabetic)).cloned();
        let formed = target.map(|t| if word == super::mind::PLURAL_ASKED { super::mind::plural_of(&t) } else { singular(&t) });
        written_out(mind, step.act, formed);
        return;
    }
    if word == super::mind::IN_WORDS && !asked.iter().any(|a| super::mind::COUNTING.contains(&a.as_str())) {
        let spelled = |n: f32| mind.tree.named(&crate::cursor::worked_text(n, None)).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *crate::cursor::EQUAL_RELATION).map(|v| mind.tree.node(mind.tree.node(mind.tree.node(v).parent).parent).name.to_string()).find(|w| w.chars().all(char::is_alphabetic) && w.len() > 1 && !w.chars().all(|c| super::mind::ROMAN_LETTERS.contains(c)));
        let Some(n) = asked.iter().find_map(|a| number_of(a)) else { written_out(mind, step.act, None); return; };
        match spelled(n) {
            Some(one) => written_out(mind, step.act, Some(one)),
            None => {
                let ones = n % super::mind::DECIMAL_BASE;
                let parts = spelled(n - ones).zip(spelled(ones));
                match parts {
                    Some((tens, ones)) => { written_out(mind, step.act, Some(tens)); written_out(mind, step.act, Some(ones)); }
                    None => written_out(mind, step.act, None),
                }
            }
        }
        mind.number = None;
        return;
    }
    if word == super::mind::NUMBER_ASKED {
        let total: f32 = asked.iter().filter(|a| number_of(a).is_none()).filter_map(|a| is_number(mind, a).and_then(|n| number_of(&n))).sum();
        written_out(mind, step.act, (total > f32::default()).then(|| crate::cursor::worked_text(total, None)));
        return;
    }
    if word == super::mind::LETTER_ASKED {
        let target = asked.iter().rev().find(|a| **a != word && a.chars().all(char::is_alphabetic)).cloned().unwrap_or_default();
        let letters: Vec<char> = target.chars().collect();
        let place = super::mind::LETTER_PLACES.iter().find(|(p, _)| asked.iter().any(|a| a == p)).map(|(_, before)| *before);
        let picked = if asked.iter().any(|a| a == super::mind::LAST_PLACE) { letters.last().copied() } else { place.and_then(|p| letters.get(p).copied()) };
        written_out(mind, step.act, picked.map(|c| c.to_string()));
        return;
    }
    if super::mind::SPELLING.contains(&word.as_str()) && asked.iter().any(|a| super::mind::COUNTING.contains(&a.as_str())) {
        let quoted: Vec<usize> = asked.iter().enumerate().filter(|(_, a)| *a == super::mind::QUOTE).map(|(i, _)| i).collect();
        let target = if word == super::mind::DIGITS_ASKED { asked.iter().find(|a| number_of(a).is_some()).cloned().unwrap_or_default() } else { asked.iter().rev().find(|a| **a != word && *a != super::mind::QUOTE && !HAVING.contains(&a.as_str()) && !COPULA.contains(&a.as_str())).cloned().unwrap_or_default() };
        let vowel = |c: char| super::mind::VOWEL_LETTERS.contains(c);
        let count = match (word.as_str(), &quoted[..]) {
            ("words", [open, close, ..]) => close - open - 1,
            ("words", _) => 0,
            ("vowels", _) => target.chars().filter(|c| c.is_alphabetic() && vowel(*c)).count(),
            ("consonants", _) => target.chars().filter(|c| c.is_alphabetic() && !vowel(*c)).count(),
            ("digits", _) => target.chars().filter(char::is_ascii_digit).count(),
            _ => target.chars().filter(|c| c.is_alphabetic()).count(),
        };
        written_out(mind, step.act, Some(count.to_string()));
        return;
    }
    if let Some((_, parts)) = crate::cursor::FRACTION_WORDS.iter().find(|(f, _)| *f == word.as_str()) {
        let said_at = asked.iter().position(|a| *a == word).unwrap_or_default();
        let worded = |a: &String| number_of(a).or_else(|| is_number(mind, a).and_then(|n| number_of(&n)));
        let many = said_at.checked_sub(1).and_then(|b| asked.get(b)).and_then(worded).unwrap_or(f32::from(u8::from(true)));
        let whole = asked.iter().skip(said_at + 1).find_map(|a| number_of(a).or_else(|| thing_number(mind, a, &asked)));
        written_out(mind, step.act, whole.map(|w| crate::cursor::worked_text(w * many / parts, None)));
        mind.number = None;
        return;
    }
    let listed: Option<String> = match super::mind::list_function(&word) {
        Some(super::mind::MEDIAN_SIGN) if !numbers.is_empty() => {
            let mut sorted = numbers.clone();
            sorted.sort_by(f32::total_cmp);
            let upper = sorted.len() / super::mind::LIST_HALVES;
            let middle = if sorted.len() % super::mind::LIST_HALVES == 1 { sorted[upper] } else { (sorted[upper - 1] + sorted[upper]) / super::mind::HALVING };
            Some(crate::cursor::worked_text(middle, None))
        }
        Some(super::mind::LEVEL_SIGN) if numbers.len() > 1 => Some(crate::cursor::worked_text(numbers.iter().sum::<f32>() / numbers.len() as f32, None)),
        Some(super::mind::MODE_SIGN) if !numbers.is_empty() => {
            let count = |n: &f32| numbers.iter().filter(|m| (**m - *n).abs() < f32::EPSILON).count();
            numbers.iter().max_by_key(|n| count(n)).map(|n| crate::cursor::worked_text(*n, None))
        }
        Some(super::mind::SPREAD_SIGN) if !numbers.is_empty() => {
            let most = numbers.iter().copied().fold(f32::MIN, f32::max);
            let least = numbers.iter().copied().fold(f32::MAX, f32::min);
            Some(crate::cursor::worked_text(most - least, None))
        }
        Some(super::mind::DIGITS_SIGN) => asked.iter().find(|a| number_of(a).is_some()).map(|a| a.chars().filter_map(|c| c.to_digit(super::mind::DECIMAL_BASE as u32)).sum::<u32>().to_string()),
        _ => None,
    };
    if listed.is_some() {
        written_out(mind, step.act, listed);
        return;
    }
    let special: Option<String> = if holds(super::mind::ALPHABET_WORD) {
        let mut options: Vec<&String> = asked.iter().skip_while(|a| *a != super::mind::ALPHABET_WORD).skip(1).filter(|a| noun_word(mind, a) && *a != super::mind::CHOICE_WORD).collect();
        options.sort();
        if holds(super::mind::LAST_PLACE) { options.last().map(|o| o.to_string()) } else { options.first().map(|o| o.to_string()) }
    } else if holds(super::mind::RANGE_WORD) && numbers.len() > 1 {
        let (from, to) = (numbers[0].min(numbers[1]) as i64, numbers[0].max(numbers[1]) as i64);
        let fits = |n: &i64| if holds(super::mind::ODD_SIGN) { odd(*n as f32) } else if holds(super::mind::EVEN_SIGN) { !odd(*n as f32) } else { true };
        said((from..=to).filter(fits).count().to_string())
    } else if let (Some(unit), Some(n)) = (super::mind::PLACE_UNITS.iter().find(|(u, _)| *u == word).map(|(_, worth)| *worth), numbers.first().copied()) {
        let made = if holds(super::mind::MAKING) { n / unit } else { ((n / unit).floor()) % super::mind::DECIMAL_BASE };
        said(crate::cursor::worked_text(made, None))
    } else if let (true, Some(n)) = (word == super::mind::ODD_SIGN || word == super::mind::EVEN_SIGN, numbers.first().copied()) {
        if holds(super::mind::ODD_SIGN) && holds(super::mind::EVEN_SIGN) {
            said(if odd(n) { super::mind::ODD_SIGN } else { super::mind::EVEN_SIGN }.to_string())
        } else {
            said(if odd(n) == (word == super::mind::ODD_SIGN) { crate::quiz::YES } else { crate::quiz::NO }.to_string())
        }
    } else {
        None
    };
    if special.is_some() {
        written_out(mind, step.act, special);
        return;
    }
    let asks = open_question(mind).is_some_and(|q| COPULA.contains(&&*mind.tree.node(q).name));
    let pair = numbers.first().copied().zip(numbers.get(1).copied());
    let compared = match (op, pair) {
        (Some(super::mind::MORE_SIGN), Some((a, b))) if asks => Some(a > b),
        (Some(super::mind::LESS_SIGN), Some((a, b))) if asks => Some(a < b),
        (Some(super::mind::MULTIPLE_SIGN), Some((a, b))) if asks && b.is_normal() => Some((a % b).abs() < f32::EPSILON),
        (Some(super::mind::FACTOR_SIGN), Some((a, b))) if asks && a.is_normal() => Some((b % a).abs() < f32::EPSILON),
        (Some(super::mind::EQUAL_SIGN), Some((a, b))) if asks => Some((a - b).abs() < f32::EPSILON),
        _ => None,
    };
    if let Some(holds) = compared {
        written_out(mind, step.act, Some(if holds { crate::quiz::YES } else { crate::quiz::NO }.to_string()));
        return;
    }
    let choosing = open_question(mind).is_some_and(|q| *mind.tree.node(q).name == *super::mind::CHOICE_ASKED) && matches!(op, Some(super::mind::MORE_SIGN | super::mind::MOST_SIGN | super::mind::LESS_SIGN | super::mind::LEAST_SIGN));
    if choosing {
        let mut worded: Vec<(f32, String)> = asked.iter().filter(|a| number_of(a).is_none()).filter_map(|a| is_number(mind, a).and_then(|n| number_of(&n)).map(|n| (n, a.clone()))).collect();
        worded.sort_by(|a, b| a.0.total_cmp(&b.0));
        let most = matches!(op, Some(super::mind::MORE_SIGN | super::mind::MOST_SIGN));
        let chosen = if most { worded.last() } else { worded.first() };
        if worded.len() > 1 {
            written_out(mind, step.act, chosen.map(|(_, word)| word.clone()));
            return;
        }
    }
    let extreme = match op {
        Some(super::mind::MOST_SIGN) => numbers.iter().copied().reduce(f32::max),
        Some(super::mind::LEAST_SIGN) => numbers.iter().copied().reduce(f32::min),
        Some(super::mind::MORE_SIGN) => pair.map(|(a, b)| a + b),
        Some(super::mind::LESS_SIGN) => pair.map(|(a, b)| b - a),
        _ => None,
    };
    if extreme.is_some() {
        written_out(mind, step.act, extreme.map(|r| crate::cursor::worked_text(r, None)));
        return;
    }
    let middle = (op == Some(super::mind::MIDDLE_SIGN) && !numbers.is_empty()).then(|| numbers.iter().sum::<f32>() / numbers.len() as f32);
    let result = middle.or_else(|| numbers.split_first().and_then(|(first, rest)| rest.iter().try_fold(*first, |acc, &n| match op {
        Some("+") => Some(acc + n),
        Some("-") => Some(acc - n),
        Some("*") => Some(acc * n),
        Some("/") if n.abs() > f32::EPSILON || acc.is_normal() => Some(acc / n),
        _ => None,
    })));
    written_out(mind, step.act, result.filter(|_| numbers.len() > 1).map(|r| crate::cursor::worked_text(r, places_told(mind))));
}
because!(computed, WordWorld, "the compute: the numbers the question says, the spans and the runs the story told, an equation or an expression, a part of a whole, a list or the letters of a word are worked by the operation the pointed word names, and the result is kept as the number of the mind for the say that follows");

fn all_with(mind: &mut CursorMind, step: &WordStep, word: &str) {
    let word = word.to_string();
    let stem = super::mind::verb_stem(mind, &word);
    let word = if stem != word && activity_word(mind, &stem) { stem } else { word };
    let compared = super::mind::compared_relation(&word).filter(|_| !quality_word(mind, &word)).map(|r| step_item(&r));
    let others: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).filter(|w| *w != word && noun_word(mind, w) && !told_thing(mind, w)).collect()).unwrap_or_default();
    let denying = open_question(mind).is_some_and(|q| present_children(mind, q).into_iter().any(|c| super::mind::NEGATIONS.contains(&&*mind.tree.node(c).name)));
    if denying {
        let mut denied: Vec<String> = story_nodes_all(mind).into_iter().filter(|&n| denied_value(mind, n, &word)).map(|n| mind.tree.node(n).name.to_string()).collect();
        denied.dedup();
        if denied.is_empty() {
            written_out(mind, step.act, None);
        }
        for name in denied {
            written_out(mind, step.act, Some(name));
        }
        return;
    }
    let ends = chain_end(mind, &word);
    let told_so = story_nodes_all(mind).into_iter().any(|n| holds_value(mind, n, &word) && mind.tree.node(n).name.as_ref() != word);
    if let Some(end) = ends.filter(|_| !told_so) {
        written_out(mind, step.act, Some(end));
        return;
    }
    let beside = |n: usize, o: &str| own_child(mind, n, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|v| { let name = &*mind.tree.node(v).name; *name == *o || *name == *singular(o) }));
    let holders: Vec<usize> = story_nodes_all(mind).into_iter().filter(|&n| match &compared { Some(r) => own_child(mind, n, r).is_some() || holds_value(mind, n, &word), None => holds_value(mind, n, &word) && others.iter().all(|o| if word.ends_with(super::mind::SUPERLATIVE_END) { beside(n, o) } else { holds_value(mind, n, o) }) } && mind.tree.node(n).name.as_ref() != word).collect();
    let named_one = |n: usize| own_child(mind, n, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|c| holders.iter().any(|&m| m != n && *mind.tree.node(m).name == *mind.tree.node(c).name && own_child(mind, m, &step_item(FLAG_DEFINITE)).is_some())));
    let mut all: Vec<String> = holders.iter().copied().filter(|&n| !named_one(n)).map(|n| mind.tree.node(n).name.to_string()).collect();
    if all.is_empty() && compared.is_some() {
        let turned: Vec<String> = opposites(mind, &word).into_iter().filter_map(|o| super::mind::compared_relation(&o)).map(|o| step_item(&o)).collect();
        all = story_nodes_all(mind).into_iter().filter_map(|n| turned.iter().find_map(|r| own_child(mind, n, r))).flat_map(|r| present_children(mind, r)).map(|v| crate::cursor::bare_name(&mind.tree.node(v).name)).collect();
        all.dedup();
    }
    if all.is_empty() && compared.is_none() {
        all = ranked_in_seeds(mind, &word, &others);
    }
    if all.is_empty() {
        written_out(mind, step.act, None);
    }
    let mut seen_names = Vec::new();
    for name in all {
        if !seen_names.contains(&name) {
            seen_names.push(name.clone());
            written_out(mind, step.act, Some(name));
        }
    }
}
because!(all_with, WordWorld, "the get of every thing that holds what the pointed word says, a quality, an activity or a compared relation, or with a denial in the question every thing told not to hold it, each name written once");

fn number_worked(mind: &mut CursorMind, step: &WordStep, word: &str) {
    let word = word.to_string();
    let act = step.act;
    let fraction = crate::cursor::FRACTION_WORDS.iter().find(|(f, _)| *f == word.as_str()).map(|(_, by)| *by);
    let operand = if act.points() { if act == WordMove::NumberDivide { fraction.or_else(|| operand_value(mind, &word)) } else { operand_value(mind, &word) } } else { None };
    let said = asked_numbers(mind).len() as f32;
    let whole = crate::cursor::PERCENT_WHOLE;
    let worked = match (act, mind.number, operand) {
        (WordMove::NumberSet, _, Some(b)) => Some(b),
        (WordMove::NumberAdd, Some(a), Some(b)) => Some(a + b),
        (WordMove::NumberSubtract, Some(a), Some(b)) => Some(a - b),
        (WordMove::NumberMultiply, Some(a), Some(b)) => Some(a * b),
        (WordMove::NumberDivide, Some(a), Some(b)) if b.is_normal() => Some(a / b),
        (WordMove::NumberLarger, Some(a), Some(b)) => Some(a.max(b)),
        (WordMove::NumberSmaller, Some(a), Some(b)) => Some(a.min(b)),
        (WordMove::NumberRemainder, Some(a), Some(b)) if b.is_normal() => Some(a.rem_euclid(b)),
        (WordMove::NumberPower, Some(a), Some(b)) => Some(a.powf(b)),
        (WordMove::NumberPercent, Some(a), Some(b)) => Some(a * b / whole),
        (WordMove::NumberAddPercent, Some(a), Some(b)) => Some(a + a * b / whole),
        (WordMove::NumberLessPercent, Some(a), Some(b)) => Some(a - a * b / whole),
        (WordMove::NumberNegate, Some(a), _) => Some(-a),
        (WordMove::NumberMean, Some(a), _) if said.is_normal() => Some(a / said),
        (WordMove::NumberWhole, Some(a), _) => Some(a.floor()),
        (WordMove::NumberRoot, Some(a), _) if a >= f32::default() => Some(a.sqrt()),
        _ => None,
    };
    if act == WordMove::NumberRound {
        if let (Some(a), Some(places)) = (mind.number, operand) {
            mind.places = Some(places as usize);
            mind.number = Some(a);
            let shown = crate::cursor::worked_text(a, mind.places);
            let item = super::mind::found_word_item(mind, &shown, act.family(), None);
            mind.stack.push(item);
            return;
        }
    }
    mind.number = worked.filter(|v| !v.is_nan());
    let shown = mind.number.map_or_else(|| crate::cursor::CURSOR_NOTHING.to_string(), |n| crate::cursor::worked_text(n, mind.places));
    let item = super::mind::found_word_item(mind, &shown, act.family(), None);
    mind.stack.push(item);
}
because!(number_worked, WordWorld, "a move of number work: the number of the mind and the operand the pointed word gives, a number, a fraction word or what a thing named is worth, are worked by the kind of the move, a round keeps the places to show, and the result is the new number of the mind, shown on the stack");

fn property_checked(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool) {
    let word = word.to_string();
    let compared = flag_of(mind, FLAG_PROPERTY).map(str::to_string).unwrap_or_default();
    mind.flags.retain(|(k, _)| k != FLAG_PROPERTY);
    let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
    let relation = relation_named(mind, &compared);
    let named = |v: usize| { let name = &*mind.tree.node(v).name; *name == *word || *name == *singular(&word) };
    let classed = plain && own_child(mind, at, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)).any(|c| class_things(mind, c).into_iter().any(|t| own_child(mind, t, &relation).is_some_and(|r| present_children(mind, r).into_iter().any(|v| { let name = &*mind.tree.node(v).name; *name == *word || *name == *singular(&word) || singular(name) == word })))));
    let anywhere = plain && mind.tree.named(&name).any(|n| n != at && !mind.tree.node(n).gone && own_child(mind, n, &relation).is_some_and(|r| present_children(mind, r).into_iter().any(named)));
    let measured = measure_asked(mind, &compared).filter(|_| plain).and_then(|(measure, most)| {
        let other = story_node(mind, &word)?;
        let (mine, theirs) = (measured_by(mind, at, &measure)?, measured_by(mind, other, &measure)?);
        Some(if most { mine > theirs } else { mine < theirs })
    });
    if let Some(holds) = measured {
        written_out(mind, step.act, Some(if holds { crate::quiz::YES } else { crate::quiz::NO }.to_string()));
        return;
    }
    if place_word(&compared) && plain && !super::mind::DIRECTIONS.contains(&compared.as_str()) {
        let mut above = Vec::new();
        let mut frontier: Vec<usize> = std::iter::once(at).chain(mind.tree.named(&name).filter(|&n| n != at && n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0 && !mind.tree.story(n))).collect();
        for _ in 0..PLACE_DEPTH {
            let next: Vec<usize> = frontier.iter().flat_map(|&n| place_of(mind, n).into_iter().chain(present_children(mind, n).into_iter().filter(|&r| mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT) && place_word(&crate::cursor::bare_name(&mind.tree.node(r).name))).flat_map(|r| present_children(mind, r)).flat_map(|v| { let named = mind.tree.node(v).name.to_string(); let roots: Vec<usize> = mind.tree.named(&named).filter(|&t| t != 0 && !mind.tree.node(t).gone && mind.tree.node(t).parent == 0).collect(); mind.tree.node(v).link.into_iter().chain(roots).chain(std::iter::once(v)).collect::<Vec<usize>>() }))).collect();
            if next.is_empty() {
                break;
            }
            above.extend(next.iter().copied());
            frontier = next;
        }
        let within = count_of(mind, at) > 0 && above.iter().any(|&p| { let place = &*mind.tree.node(p).name; *place == *word || *place == *singular(&word) });
        written_out(mind, step.act, Some(if within { crate::quiz::YES } else { crate::quiz::NO }.to_string()));
        return;
    }
    let chained = plain && super::mind::DIRECTIONS.contains(&compared.as_str()) && related_chain(mind, at, &relation, &word, CLASS_DEPTH);
    let forward = chained || anywhere || classed || plain && (grandparent(mind, at, &compared).is_some_and(named) || own_child(mind, at, &relation).is_some_and(|r| present_children(mind, r).into_iter().any(named)) || compared_through(mind, at, &word, CLASS_DEPTH, Some(&relation)));
    let turned: Vec<String> = opposites(mind, &compared).into_iter().filter_map(|o| super::mind::compared_relation(&o)).map(|o| step_item(&o)).collect();
    let facing: Vec<String> = opposites(mind, &compared).into_iter().map(|o| step_item(&o)).collect();
    let faced = plain && mind.tree.named(&word).any(|v| !mind.tree.node(v).gone && facing.iter().any(|r| related_chain(mind, v, r, &name, CLASS_DEPTH)));
    let backward = faced || plain && story_nodes(mind, &word).into_iter().any(|v| turned.iter().any(|r| compared_through(mind, v, &name, CLASS_DEPTH, Some(r))));
    let reverse = plain && super::mind::SYMMETRIC_ROLES.contains(&compared.as_str()) && mind.tree.named(&word).any(|v| !mind.tree.node(v).gone && own_child(mind, v, &relation).is_some_and(|r| present_children(mind, r).into_iter().any(|x| crate::cursor::bare_name(&mind.tree.node(x).name) == name)));
    let backward = backward || reverse;
    written_out(mind, step.act, Some(if forward || backward { crate::quiz::YES } else { crate::quiz::NO }.to_string()));
}
because!(property_checked, WordWorld, "the check of a relation flagged before it, is tom taller than ann, is the cat in the house: yes when the thing the cursor stands on, a thing of its name or a class it is of holds the pointed word under the flagged relation, by a measure both hold, by the places it stands inside, by a chain of the relation, or by the opposite relation told the other way, and no in every other case");

fn kind_asked(mind: &CursorMind, word: &str) -> String {
    let word = if word == super::mind::MADE { super::moves::MATERIAL_KIND.to_string() } else { word.to_string() };
    super::mind::kind_of(mind, &word).map(|k| k.to_string()).filter(|_| !super::moves::KINDS.contains(&word.as_str())).unwrap_or(word)
}
because!(kind_asked, WordWorld, "the kind a word asks for: the kind the seeds class the word by where the word is a quality of one, \
     red asking for a colour, else the word itself");

fn kind_under(mind: &CursorMind, is: usize, word: &str) -> Option<usize> {
    let stem: String = word.chars().take(KIND_STEM).collect();
    own_child(mind, is, &step_item(word)).or_else(|| present_children(mind, is).into_iter().find(|&k| crate::cursor::bare_name(&mind.tree.node(k).name).starts_with(&stem)))
}
because!(kind_under, WordWorld, "what a thing holds under the kind a word asks for, by the word or by the letters it shares with a \
     kind, feel with feeling");

fn kind_held(mind: &CursorMind, k: usize) -> Option<usize> {
    if mind.tree.node(k).name.starts_with(BRACE_OPEN_TEXT) { present_children(mind, k).into_iter().next() } else { Some(k) }
}
because!(kind_held, WordWorld, "the value under a kind a thing holds, or the kind itself where it is no relation");

fn kind_strict(mind: &CursorMind, word: &str) -> bool {
    let class_known = !super::mind::ASKING.contains(&word) && mind.tree.named(word).any(|c| !mind.tree.node(c).gone && !mind.tree.story(c) && *mind.tree.node(mind.tree.node(c).parent).name == *IS_FORM.trim());
    super::moves::KINDS.contains(&word) || class_known
}
because!(kind_strict, WordWorld, "whether a word names a kind outright, one of the kinds a quality is classed by or one the seeds \
     say a thing is, so a value is taken for it only where it is of that kind");

fn kind_only(mind: &CursorMind, is: usize, word: &str) -> Option<usize> {
    let stem: String = word.chars().take(KIND_STEM).collect();
    let asked_kind = super::moves::KINDS.iter().copied().find(|k| k.starts_with(&stem)).unwrap_or(word).to_string();
    let strict = kind_strict(mind, word);
    let counts = |v: usize| { let q = mind.tree.node(v).name.to_string(); !strict || mind.tree.named(&q).any(|s| s != v && !mind.tree.story(s) && !mind.tree.node(s).gone && own_child(mind, s, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|k| *mind.tree.node(k).name == *asked_kind))) };
    let kinds: Vec<usize> = present_children(mind, is).into_iter().filter(|&k| *mind.tree.node(k).name != *TIME_RELATION && count_of(mind, k) > 0).collect();
    (kinds.len() == 1).then(|| kinds[0]).filter(|&k| kind_held(mind, k).is_some_and(counts))
}
because!(kind_only, WordWorld, "the one kind a thing holds, where it holds one only and a thing of the kind asked is known to be of \
     it, so what a thing is is answered from the one thing the story says it is");

fn kind_value(mind: &CursorMind, word: &str, at: usize, plain: bool) -> Option<usize> {
    let is = plain.then(|| own_child(mind, at, IS_FORM.trim())).flatten();
    let value = is.and_then(|is| kind_under(mind, is, word).or_else(|| kind_only(mind, is, word))).and_then(|k| kind_held(mind, k));
    let value = value.filter(|&v| count_of(mind, v) > 0).or_else(|| is.filter(|_| value.is_some()).and_then(|is| present_children(mind, is).into_iter().find(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0)));
    value.or_else(|| is.filter(|_| word == super::mind::ASKING[0] && plain && !mind.tree.story(at)).and_then(|is| present_children(mind, is).into_iter().find(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0)))
}
because!(kind_value, WordWorld, "the value the plain get of a kind writes: what the thing holds under the kind asked, or the one kind \
     it holds, and where that value was taken away, any kind it still holds, and for what asked of a thing the story never told, the \
     first kind the seeds give it");

fn kind_many(mind: &CursorMind, word: &str, at: usize, plain: bool) -> Vec<String> {
    let is = plain.then(|| own_child(mind, at, IS_FORM.trim())).flatten();
    let of_kind: Vec<String> = is.and_then(|is| kind_under(mind, is, word)).filter(|&k| mind.tree.node(k).name.starts_with(BRACE_OPEN_TEXT)).map(|k| present_children(mind, k).into_iter().filter(|&v| count_of(mind, v) > 0).map(|v| mind.tree.node(v).name.to_string()).collect()).unwrap_or_default();
    if of_kind.len() > 1 {
        return of_kind;
    }
    let every: Vec<String> = if word == super::mind::ASKING[0] && plain && mind.tree.story(at) { is.map(|is| present_children(mind, is).into_iter().filter(|&c| *mind.tree.node(c).name != *TIME_RELATION).flat_map(|c| if mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) { present_children(mind, c) } else { vec![c] }).filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0).map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default() } else { Vec::new() };
    if every.len() > 1 { every } else { Vec::new() }
}
because!(kind_many, WordWorld, "every value a thing holds under the kind asked where it holds several, and every kind it is where \
     what is asked of a thing the story told");

fn kind_borrowed(mind: &CursorMind, word: &str, at: usize, plain: bool) -> Option<usize> {
    (kind_strict(mind, word) && plain && kind_value(mind, word, at, plain).is_none()).then(|| induced(mind, at, |n| own_child(mind, n, IS_FORM.trim()).and_then(|i| own_child(mind, i, &step_item(word))).is_some()).and_then(|n| own_child(mind, n, IS_FORM.trim())).and_then(|i| own_child(mind, i, &step_item(word))).and_then(|k| kind_held(mind, k))).flatten()
}
because!(kind_borrowed, WordWorld, "a value of the kind asked borrowed from a thing like the one asked of, where the thing itself \
     holds none and the word names a kind outright");

fn kind_kin(mind: &CursorMind, word: &str, at: usize, plain: bool) -> Option<String> {
    (plain && (word == super::mind::WHO_ASKED || word == super::mind::ASKING[0])).then(|| (mind.tree.state..mind.tree.len()).filter(|&v| !mind.tree.node(v).gone && (mind.tree.node(v).link == Some(at) || *mind.tree.node(v).name == *mind.tree.node(at).name && v != at)).map(|v| crate::cursor::bare_name(&mind.tree.node(mind.tree.node(v).parent).name)).find(|r| super::mind::KIN.contains(&r.as_str()))).flatten()
}
because!(kind_kin, WordWorld, "the kin a thing is to another, written where what or who is asked of one the story told nothing else \
     of, ann being tom's mother and nothing more");

fn kind_got(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool) {
    let asked = kind_asked(mind, word);
    let many = kind_many(mind, &asked, at, plain);
    if !many.is_empty() {
        for name in many {
            written_out(mind, step.act, Some(name));
        }
        return;
    }
    let value = kind_value(mind, &asked, at, plain).or_else(|| kind_borrowed(mind, &asked, at, plain));
    if value.is_none() {
        if let Some(kin) = kind_kin(mind, word, at, plain) {
            written_out(mind, step.act, Some(kin));
            return;
        }
    }
    written_out(mind, step.act, value.map(|v| mind.tree.node(v).name.to_string()));
}
because!(kind_got, WordWorld, "the get of a kind: the value the thing holds under the kind the pointed word names, by the word or by \
     the letters it shares with a kind, feel with feeling, or one borrowed from a thing like it");

fn owner_told(mind: &CursorMind, at: usize) -> Option<usize> {
    owner_of(mind, at)
}
because!(owner_told, WordWorld, "the owner a giving wrote on the thing itself, which is the owner asked for wherever the thing stands");

pub(super) fn stood_way(mind: &CursorMind, word: &str) -> WordMove {
    let _ = mind;
    if super::mind::THING_PRONOUNS.contains(&word) || super::mind::PERSON_PRONOUNS.contains(&word) {
        return WordMove::FindAskedStood;
    }
    WordMove::FindAsked
}
because!(stood_way, WordWorld, "which find answers the word pointed at: the one that walks to what a pronoun stands for, else the one \
     that walks to a thing the word names");

pub(super) fn distance_way(mind: &CursorMind, word: &str, at: usize) -> WordMove {
    if at != 0 && map_route(mind, &crate::cursor::bare_name(&mind.tree.node(at).name), word).is_some() {
        return WordMove::GetDistanceRoute;
    }
    WordMove::GetDistance
}
because!(distance_way, WordWorld, "which get of a distance answers: the one that adds the steps of a route the map knows between the \
     thing and the word, else the plain one");

pub(super) fn doing_way(mind: &CursorMind, at: usize, plain: bool) -> WordMove {
    let carries = plain && own_child(mind, at, super::mind::DEED_TAG).is_some_and(|deed| !present_children(mind, deed).is_empty());
    if carries && super::mind::heard_text(mind) == super::mind::INFINITIVE {
        return WordMove::ActivityNamed;
    }
    if carries {
        return WordMove::ActivityDone;
    }
    if plain && mind.tree.node(at).parent != 0 {
        let deed = mind.tree.node(at).parent;
        let name = &*mind.tree.node(deed).name;
        if name.starts_with(BRACE_OPEN_TEXT) && *name != *step_item(super::mind::ACTIVITY) && mind.tree.node(deed).parent != 0 {
            return WordMove::ActivityUnder;
        }
    }
    WordMove::Activity
}
because!(doing_way, WordWorld, "which nesting of a doing answers the thing the cursor stands on: the one that turns a deed already \
     written into a relation and opens a doing under it, for a verb said plain after to, the one that writes the deed it carries as \
     the doing itself, the one that writes the doing on the doer above where a deed holds the thing, else the plain one, so the \
     network chooses the way of nesting where tests on the words said and on what the thing carries chose it");

pub(super) fn check_way(mind: &CursorMind, word: &str, at: usize, plain: bool) -> WordMove {
    if flag_of(mind, FLAG_PROPERTY).is_some() {
        return WordMove::CheckRelation;
    }
    if word == super::mind::RIGHT_ASKED && plain && claim_right(mind, at).is_some() {
        return WordMove::CheckRight;
    }
    WordMove::Check
}
because!(check_way, WordWorld, "which check answers the word asked of the thing the cursor stands on: the check of a relation a word \
     before it flagged, is tom taller than ann, the check of whether a claim the story made was right, else the plain check of a \
     value the thing holds, so the network chooses the way of reading where tests on the flags and on the word chose it");

pub(super) fn kind_way(mind: &CursorMind, word: &str, at: usize, plain: bool) -> WordMove {
    if word == super::mind::WHY_ASKED && plain {
        return WordMove::GetKindWhy;
    }
    if super::mind::MEASURES.contains(&word) && plain && own_child(mind, at, &step_item(word)).is_none() && !mind.tree.story(at) {
        return WordMove::GetKindMeasure;
    }
    WordMove::GetKind
}
because!(kind_way, WordWorld, "which get of a kind answers the word asked of the thing the cursor stands on: why it is as it is, \
     nothing for a measure the story never gave it, else the plain one, so the network chooses the way of reading where a test on \
     the word chose it before the move ever ran");

fn places_of(mind: &CursorMind, at: usize, plain: bool) -> Vec<String> {
    let bare_named = plain && own_child(mind, at, &step_item(FLAG_DEFINITE)).is_none() && own_child(mind, at, &step_item(FLAG_INDEFINITE)).is_none();
    let seeded_place = || bare_named.then(|| mind.tree.named(&mind.tree.node(at).name.to_string()).filter(|&n| n != at && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0).find_map(|n| place_value(mind, n))).flatten();
    let parent = place_of(mind, at).or_else(|| plain.then(|| place_value(mind, at)).flatten()).or_else(seeded_place);
    let asked_words = open_question(mind).map(|q| present_children(mind, q).len()).unwrap_or_default();
    let alike: Vec<usize> = if plain && mind.tree.story(at) && asked_words == 1 { story_nodes(mind, &mind.tree.node(at).name.to_string()).into_iter().filter(|&n| n != at).filter_map(|n| place_of(mind, n)).collect() } else { Vec::new() };
    let mut places: Vec<String> = parent.into_iter().chain(alike).map(|p| crate::cursor::bare_name(&mind.tree.node(p).name)).collect();
    places.sort();
    places.dedup();
    places
}
because!(places_of, WordWorld, "the places a thing stands in: where it stands, or the place written as a value of it, or the place \
     the seeds give a thing of its name where the story named it bare, and beside those the places its other mentions stand in, \
     where the story told of it and one word was asked");

pub(super) fn place_way(mind: &CursorMind, at: usize, plain: bool) -> WordMove {
    if plain && mind.tree.node(at).parent != 0 && (*mind.tree.node(mind.tree.node(at).parent).name == *step_item(super::mind::ACTIVITY) || claim_node(mind, at)) {
        return WordMove::GetLocationClaimed;
    }
    if plain && mind.before.iter().any(|said| said == super::mind::WILL) && !motives(mind, at).is_empty() && own_child(mind, at, TIME_RELATION).is_none() {
        return WordMove::GetLocationMotive;
    }
    WordMove::GetLocation
}
because!(place_way, WordWorld, "which get of a place answers the thing the cursor stands on: where a mention of it stands, for a thing \
     a claim or a doing holds rather than a place, the place a motive names, for a move still to come, else the plain one, so the \
     network chooses the way of reading where a test on the thing chose it before the move ever ran");

pub(super) fn owner_way(mind: &CursorMind, at: usize, plain: bool) -> WordMove {
    let thing = mind.tree.node(at).link.unwrap_or(at);
    if plain && own_child(mind, thing, OWNER_TAG).is_some_and(|tag| present_children(mind, tag).len() > 1) {
        return WordMove::GetOwnerEvery;
    }
    WordMove::GetOwner
}
because!(owner_way, WordWorld, "which get of an owner answers the thing the cursor stands on: every owner where it was handed on \
     more than once, else the owner a giving wrote on it, so the teacher shows one way of reading for one shape and the network is \
     asked to tell the two shapes apart where a count of the owners standing there used to tell them");

pub(super) fn relation_way(mind: &CursorMind, word: &str, at: usize, plain: bool) -> WordMove {
    if super::mind::reply_word(mind, word) && !told_relation(mind, &super::mind::verb_stem(mind, word)) && !(plain && own_child(mind, at, &step_item(&super::mind::verb_stem(mind, word))).is_some()) {
        return WordMove::GetRelationSaid;
    }
    if plain && mind.tree.node(at).parent != 0 && *mind.tree.node(mind.tree.node(at).parent).name == *step_item(super::mind::ACTIVITY) && *mind.tree.node(at).name == *super::mind::verb_stem(mind, word) {
        return WordMove::GetRelationToward;
    }
    let relation = relation_named(mind, word);
    let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
    let holder = relation_holder(mind, &name, &relation, plain);
    if holder.and_then(|n| own_child(mind, n, &relation)).and_then(|r| present_children(mind, r).into_iter().find(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) || mind.tree.node(c).name.starts_with(crate::cursor::QUANTITY_TAG))).is_some() {
        return WordMove::GetRelation;
    }
    if holder.and_then(|n| own_child(mind, n, &relation).and_then(|r| own_child(mind, r, &step_item(super::mind::ACTIVITY))).or_else(|| own_child(mind, n, &step_item(super::mind::ACTIVITY)))).and_then(|r| present_children(mind, r).into_iter().last()).is_some() {
        return WordMove::GetRelationDoing;
    }
    if plain && grandparent(mind, at, word).is_some() {
        return WordMove::GetRelationThrough;
    }
    if plain && (word == super::mind::ORDINALS[0] || word == super::mind::LAST_PLACE) && ranked_value(mind, &name, &relation).is_some() {
        return WordMove::GetRelationRanked;
    }
    if super::mind::SYMMETRIC_ROLES.contains(&word) && backward_holder(mind, &name, &relation, at).is_some() {
        return WordMove::GetRelationBackward;
    }
    WordMove::GetRelation
}
because!(relation_way, WordWorld, "which get of a relation answers the word asked of the thing the cursor stands on: the plain one when it \
     holds the relation with a value, else the doing when what it holds is an activity, else the one two steps up, else the first or last \
     of a kind the seeds order, else the backward read of a relation that goes both ways; the teacher shows the one that answers and the \
     network learns to pick it, since a reader that tried them in a fixed order could never be taught to prefer another");

fn relation_holder(mind: &CursorMind, name: &str, relation: &str, plain: bool) -> Option<usize> {
    let at = mind.at;
    let classes: Vec<usize> = if plain { own_child(mind, at, IS_FORM.trim()).into_iter().flat_map(|is| present_children(mind, is)).flat_map(|c| { let class = mind.tree.node(c).name.to_string(); mind.tree.named(&class).filter(|&n| n != 0 && n != c && !mind.tree.node(n).gone && mind.tree.node(n).link.is_none()).chain(class_things(mind, c)).collect::<Vec<_>>() }).collect() } else { Vec::new() };
    let mut classes = classes;
    classes.sort_by_key(|&c| !mind.tree.story(c));
    plain.then(|| std::iter::once(at).chain(mind.tree.named(name).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).link.is_none())).chain(classes).find(|&n| own_child(mind, n, relation).is_some())).flatten()
}
because!(relation_holder, WordWorld, "the thing a relation is read off: the thing the cursor stands on, else another node of its name, else \
     a thing of a class it is of, the first of them that holds the relation");

pub(super) fn ranked_value(mind: &CursorMind, name: &str, relation: &str) -> Option<usize> {
    let is_a = |v: usize| mind.tree.named(&mind.tree.node(v).name.to_string()).any(|t| !mind.tree.node(t).gone && own_child(mind, t, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|k| *mind.tree.node(k).name == *name)));
    mind.tree.named(relation).filter(|&r| !mind.tree.node(r).gone && !mind.tree.story(r)).flat_map(|r| present_children(mind, r)).find(|&v| is_a(v))
}
because!(ranked_value, WordWorld, "the first or the last of a kind the seeds order, found under the relation the word names among the \
     nodes the story never wrote, the first day of the week");

fn backward_holder(mind: &CursorMind, name: &str, relation: &str, at: usize) -> Option<usize> {
    mind.tree.named(name).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *relation).map(|v| mind.tree.node(mind.tree.node(v).parent).parent).find(|&t| t != 0 && t != at)
}
because!(backward_holder, WordWorld, "the thing that holds the cursor's thing under a relation that goes both ways, read from the value \
     back to its holder");

fn relation_got(mind: &mut CursorMind, step: &WordStep, word: &str, at: usize, plain: bool) {
    let word = word.to_string();
    let relation = relation_named(mind, &word);
    let name = if plain { mind.tree.node(at).name.to_string() } else { String::new() };
    let classes: Vec<usize> = if plain { own_child(mind, at, IS_FORM.trim()).into_iter().flat_map(|is| present_children(mind, is)).flat_map(|c| { let class = mind.tree.node(c).name.to_string(); mind.tree.named(&class).filter(|&n| n != 0 && n != c && !mind.tree.node(n).gone && mind.tree.node(n).link.is_none()).chain(class_things(mind, c)).collect::<Vec<_>>() }).collect() } else { Vec::new() };
    let mut classes = classes;
    classes.sort_by_key(|&c| !mind.tree.story(c));
    let holder = plain.then(|| std::iter::once(at).chain(mind.tree.named(&name).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).link.is_none())).chain(classes).find(|&n| own_child(mind, n, &relation).is_some())).flatten();
    let forward = holder.and_then(|n| own_child(mind, n, &relation)).and_then(|r| present_children(mind, r).into_iter().find(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) || mind.tree.node(c).name.starts_with(crate::cursor::QUANTITY_TAG)));
    let found = forward;
    let several: Vec<usize> = holder.and_then(|n| own_child(mind, n, &relation)).map(|r| present_children(mind, r).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0).collect()).unwrap_or_default();
    if several.len() > 1 {
        let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
        let classed = |v: usize, a: &str| { let value = mind.tree.node(v).name.to_string(); mind.tree.named(&value).any(|t| t != 0 && !mind.tree.node(t).gone && holds_through(mind, t, a, CLASS_DEPTH, true)) };
        let kind = asked.iter().find(|a| **a != word && **a != name && singular(a) != name && !several.iter().all(|&v| classed(v, a)) && several.iter().any(|&v| { let value = mind.tree.node(v).name.to_string(); mind.tree.named(&value).any(|t| t != 0 && !mind.tree.node(t).gone && holds_through(mind, t, a, CLASS_DEPTH, true)) }));
        let mut names: Vec<String> = several.iter().filter(|&&v| kind.is_none_or(|k| { let value = mind.tree.node(v).name.to_string(); mind.tree.named(&value).any(|t| t != 0 && !mind.tree.node(t).gone && holds_through(mind, t, k, CLASS_DEPTH, true)) })).map(|&v| crate::cursor::bare_name(&mind.tree.node(v).name)).collect();
        names.dedup();
        for value in names {
            written_out(mind, step.act, Some(value));
        }
        return;
    }
    written_out(mind, step.act, found.map(|n| crate::cursor::bare_name(&mind.tree.node(n).name)));
}
because!(relation_got, WordWorld, "the plain read of a relation: what the thing the cursor stands on, a thing of its name or a class \
     it is of holds under the relation the pointed word names, and every value when it holds several, kept to the kind the question \
     names; the other ways that relation may be read are moves of their own, so a reader that finds nothing here says nothing rather \
     than trying them in an order nobody chose");
