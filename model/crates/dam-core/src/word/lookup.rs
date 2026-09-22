use super::mind::{mark_word, place_word, FLAG_HAND, HAVING, open_question, flag_of, is_number, kind_of, noun_word, quality_word, singular, verb_base, FLAG_INDEFINITE, FLAG_GIVE, FLAG_CONTAIN, FLAG_QUANTITY, FLAG_DEFINITE};
use super::WordReading;
use crate::cursor::{step_item, CursorMind, BRACE_OPEN_TEXT, TIME_RELATION};
use crate::quiz::IS_FORM;
use crate::words::number_of;
use patterns::because;
use super::physics::{WordWorld, ANSWER_TAG, OWNER_TAG, FOUND_SIGHTS, STEERED, GENDER_TAG, FIND_SEES_PLACE, FIND_SEES_OWNER, FIND_SEES_CONTENTS, FIND_SEES_ASKED, ORDER_SPAN, PLACE_DEPTH, CLASS_DEPTH};

pub(super) fn present_children(mind: &CursorMind, at: usize) -> Vec<usize> {
    mind.tree.node(at).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).collect()
}
because!(present_children, WordReading, "the children of a node that are not gone");

pub(super) fn tag_value(mind: &CursorMind, n: usize) -> bool {
    let parent = &*mind.tree.node(mind.tree.node(n).parent).name;
    parent == TIME_RELATION || parent == OWNER_TAG || parent == ANSWER_TAG || parent == step_item(FLAG_DEFINITE) || parent == step_item(FLAG_INDEFINITE)
}
because!(tag_value, WordReading, "whether a node is the value of a tag, a time, an owner, a last answer or an article, which is no thing \
     of its own");

fn standing(mind: &CursorMind, n: usize) -> bool {
    let parent = mind.tree.node(n).parent;
    parent == 0 || !mind.tree.node(parent).name.starts_with(BRACE_OPEN_TEXT)
}
because!(standing, WordReading, "whether a node stands in the world as a thing, under the world or inside another thing, and not as a \
     value under a relation, the orange under color");

pub(super) fn story_node(mind: &CursorMind, name: &str) -> Option<usize> {
    story_nodes(mind, name).into_iter().next()
}
because!(story_node, WordReading, "the story's newest thing of a name");

pub(super) fn story_nodes(mind: &CursorMind, name: &str) -> Vec<usize> {
    let asked = open_question(mind);
    let fit = |n: usize| n != 0 && !mind.tree.node(n).gone && mind.tree.story(n) && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && !tag_value(mind, n) && !mind.held.contains(&n) && mind.tree.node(n).link.is_none() && asked.is_none_or(|q| !inside(mind, n, q));
    let mut found: Vec<usize> = mind.tree.named(name).filter(|&n| fit(n) && standing(mind, n) && count_of(mind, n) > 0).collect();
    found.sort_unstable_by(|a, b| b.cmp(a));
    found
}
because!(story_nodes, WordReading, "the story's things of a name anywhere in the world, the newest first, never a relation, a tag's value, \
     what he holds, a mention such as the trace a thing left or a word of the open question, since the story makes its own things and a \
     thing inside a place is still that thing");

pub fn ranked_in_seeds(mind: &CursorMind, rank: &str, beside: &[String]) -> Vec<String> {
    if beside.is_empty() || !rank.ends_with(super::mind::SUPERLATIVE_END) && !super::mind::ORDINALS.contains(&rank) {
        return Vec::new();
    }
    let under_is = |v: usize| { let is = mind.tree.node(v).parent; (is != 0 && *mind.tree.node(is).name == *IS_FORM.trim()).then_some(is) };
    let sided = |is: usize, o: &str| present_children(mind, is).into_iter().any(|v| { let name = &*mind.tree.node(v).name; *name == *o || *name == *singular(o) });
    mind.tree.named(rank).filter(|&v| v != 0 && !mind.tree.node(v).gone && !mind.tree.story(v)).filter_map(under_is).filter(|&is| beside.iter().all(|o| sided(is, o))).map(|is| mind.tree.node(is).parent).filter(|&n| n != 0 && mind.tree.node(n).parent == 0).map(|n| mind.tree.node(n).name.to_string()).collect()
}
because!(ranked_in_seeds, WordWorld, "the things the seeds rank with a superlative or an ordinal beside the kind asked with it, the largest planet, \
     read when the story ranks nothing so");

fn told_rank(mind: &CursorMind, rank: &str) -> bool {
    story_nodes_all(mind).into_iter().any(|n| holds_value(mind, n, rank))
}
because!(told_rank, WordWorld, "whether the story told some thing with this place in an order, the second cup, so a question that names the place asks of that one");

pub fn question_match(mind: &CursorMind, n: usize, thing: &str, words: &[String]) -> bool {
    let others = || words.iter().filter(|w| *w != thing && **w != singular(thing));
    let owners: Vec<&String> = others().filter(|w| !super::mind::quality_word(mind, w) && told_thing(mind, w)).collect();
    let ranked = |w: &&String| super::mind::ORDINALS.contains(&w.as_str()) && !super::mind::ANOTHER.contains(&w.as_str()) && told_rank(mind, w);
    others().filter(|w| super::mind::quality_word(mind, w) || ranked(w)).all(|q| holds(mind, n, q)) && words.iter().filter(|w| !ranked(w)).filter_map(|w| is_number(mind, w)).all(|count| counted(mind, n) == count) && (owners.is_empty() || owners.iter().any(|o| holder_name(mind, n).as_deref() == Some(o.as_str()) || stands_in(mind, n, o)))
}
because!(question_match, WordReading, "whether a thing is the one a question asks of: it holds every quality the question names and every place in an order the story told, it counts \
     what the question counts, and one of the things the question names beside it holds it or has it standing in it");

pub fn found_sight(mind: &CursorMind, n: usize) -> [bool; FOUND_SIGHTS.len()] {
    let words: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let name = crate::cursor::bare_name(&mind.tree.node(n).name);
    let further = story_nodes(mind, &name).into_iter().any(|m| m < n);
    let relates = words.iter().any(|w| [w.clone(), super::mind::verb_stem(mind, w)].iter().any(|r| own_child(mind, n, &step_item(r)).is_some()));
    [place_of(mind, n).is_some(), who_has(mind, n).is_some(), !words.is_empty() && question_match(mind, n, &name, &words), further, relates]
}
because!(found_sight, WordReading, "what is seen of a node a step found: whether it stands in a place, whether someone has it, whether it \
     is the one the open question asks of, whether the story told an older thing of its name, and whether it has a relation the question \
     names, so a search knows when to look further and a get knows what the node can answer");

pub fn things_named(mind: &CursorMind, name: &str) -> Vec<usize> {
    let name = if super::mind::SELF_WORDS.contains(&name) { super::mind::USER_NAME } else { name };
    let found = story_nodes(mind, name);
    if found.is_empty() { story_nodes(mind, &singular(name)) } else { found }
}
because!(things_named, WordReading, "the story's things of a name or its singular, the newest first, the speaker's words standing for the \
     user");

pub fn claim_node(mind: &CursorMind, n: usize) -> bool {
    n != 0 && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && { let claim = mind.tree.node(n).parent; claim != 0 && mind.tree.node(claim).parent != 0 && super::mind::CLAIMING.contains(&crate::cursor::bare_name(&mind.tree.node(claim).name).as_str()) }
}
because!(claim_node, WordReading, "whether a node is the thing of a claim, a value right under a person's relation of saying, thinking or \
     believing, which holds what they claimed of it apart from the thing of the story");

pub fn claims_asked(mind: &CursorMind) -> bool {
    open_question(mind).is_some_and(|q| present_children(mind, q).into_iter().any(|c| verb_base(mind, &mind.tree.node(c).name).is_some_and(|b| super::mind::CLAIMING.contains(&b.as_str()))))
}
because!(claims_asked, WordReading, "whether the open question asks what someone said, thought or believed, so a find lands on the thing \
     of their claim");

pub(super) fn claimed_named(mind: &CursorMind, name: &str) -> Option<usize> {
    let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    (mind.tree.state..mind.tree.len()).rev().filter(|&n| !mind.tree.node(n).gone && claim_node(mind, n) && (*mind.tree.node(n).name == *name || *mind.tree.node(n).name == *singular(name))).find(|&n| { let claimer = mind.tree.node(mind.tree.node(mind.tree.node(n).parent).parent).name.to_string(); asked.iter().any(|a| *a == claimer) })
}
because!(claimed_named, WordReading, "the thing of a claim of a name, under a claimer the open question names");

pub(super) fn claim_right(mind: &CursorMind, claimer: usize) -> Option<bool> {
    let claims: Vec<usize> = present_children(mind, claimer).into_iter().filter(|&r| super::mind::CLAIMING.contains(&crate::cursor::bare_name(&mind.tree.node(r).name).as_str())).flat_map(|r| present_children(mind, r)).collect();
    if claims.is_empty() {
        return None;
    }
    Some(claims.into_iter().all(|claim| {
        let name = mind.tree.node(claim).name.to_string();
        let Some(real) = story_node(mind, &name) else { return false };
        let said: Vec<String> = own_child(mind, claim, IS_FORM.trim()).into_iter().flat_map(|is| present_children(mind, is)).flat_map(|k| if mind.tree.node(k).name.starts_with(BRACE_OPEN_TEXT) { present_children(mind, k) } else { vec![k] }).filter(|&v| !mind.tree.node(v).name.starts_with(BRACE_OPEN_TEXT)).map(|v| mind.tree.node(v).name.to_string()).collect();
        let placed: Vec<usize> = mind.tree.named(&name).filter(|&m| !mind.tree.node(m).gone && mind.tree.node(m).link == Some(claim)).map(|m| mind.tree.node(m).parent).filter(|&p| p != 0).collect();
        said.iter().all(|v| holds_value(mind, real, v)) && placed.iter().all(|&p| inside(mind, real, p))
    }))
}
because!(claim_right, WordReading, "whether what a person claimed is so: every quality they claimed of a thing is held by the thing of the \
     story of that name, and every place they claimed holds it, or none when they claimed nothing");

pub fn stands_in(mind: &CursorMind, thing: usize, place: &str) -> bool {
    let mut above = mind.tree.node(thing).parent;
    let mut steps = 0;
    while above != 0 && steps < PLACE_DEPTH {
        if *mind.tree.node(above).name == *place {
            return true;
        }
        above = mind.tree.node(above).parent;
        steps += 1;
    }
    false
}
because!(stands_in, WordReading, "whether a thing stands inside a thing of a name, however deep, so the search game takes the dog in the \
     house when a question names the house");

pub fn value_told(mind: &CursorMind, value: &str) -> bool {
    story_nodes_all(mind).into_iter().any(|n| mind.tree.node(n).name.as_ref() != value && mind.tree.story(n) && holds_value(mind, n, value))
}
because!(value_told, WordReading, "whether some thing of the story holds a word as a value, so what is orange asks the things that are \
     orange, while what is iron, which nothing is, asks the thing the story told");

pub fn unseeded(mind: &CursorMind, word: &str) -> bool {
    let only_opposed = has_relation(mind, word, super::mind::OPPOSITE_WORD) && !seeded_class(mind, word);
    let placing = word.ends_with(super::mind::SUPERLATIVE_END) || super::mind::ORDINALS.contains(&word) || super::mind::comparison_known(mind, word);
    !placing && (only_opposed || !mind.tree.named(word).any(|n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n))) && !quality_word(mind, word) && verb_base(mind, word).is_none() && is_number(mind, word).is_none() && !mark_word(word)
}
because!(unseeded, WordReading, "whether the seeds say nothing of a word but at most its opposite, useful and useless, no other node of \
     that name, no quality, no verb, no number and no superlative, ordinal or comparison, which place a thing among its kind, so a word \
     told under is right before a noun is read as describing it, a useful device");

pub fn seeded_class(mind: &CursorMind, name: &str) -> bool {
    mind.tree.named(name).any(|n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0 && own_child(mind, n, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT))))
}
because!(seeded_class, WordReading, "whether the seeds say what a thing of a name is, a dog a mammal, so a question of what it is reads \
     the seeds when the story told nothing of it");

pub fn told_with_article(mind: &CursorMind, name: &str) -> bool {
    things_named(mind, name).into_iter().any(|n| own_child(mind, n, &step_item(FLAG_DEFINITE)).is_some() || own_child(mind, n, &step_item(FLAG_INDEFINITE)).is_some() || own_child(mind, n, IS_FORM.trim()).is_some())
}
because!(told_with_article, WordReading, "whether the story told a thing of a name with an article or told what it is, so a word the seeds \
     also know as a quality is the thing in a question, the navy and no color, gold the metal");

pub fn holds(mind: &CursorMind, thing: usize, value: &str) -> bool {
    holds_value(mind, thing, value)
}
because!(holds, WordReading, "whether a thing holds a value, for the teacher's search");

pub fn holds_any(mind: &CursorMind, thing: usize) -> bool {
    !held_or_owned(mind, thing).is_empty()
}
because!(holds_any, WordReading, "whether a thing holds or owns anything, for the teacher's search");

pub fn person_named(mind: &CursorMind, name: &str) -> bool {
    mind.tree.named(name).any(|n| n != 0 && !mind.tree.node(n).gone && (person(mind, n) || (mind.tree.story(n) && mind.tree.node(n).link.is_none() && mind.tree.node(n).children.iter().any(|&c| !mind.tree.node(c).gone && !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)))))
}
because!(person_named, WordReading, "whether a name is a person's or a holder's, one the seeds give a gender or one of the story that has \
     or had a thing, for the teacher to tell the one given to from the thing given");

pub fn thing_number(mind: &CursorMind, word: &str, asked: &[String]) -> Option<f32> {
    if super::mind::SELF_WORDS.contains(&word) {
        let as_user: Vec<String> = asked.iter().map(|a| if super::mind::SELF_WORDS.contains(&a.as_str()) { super::mind::USER_NAME.to_string() } else { a.clone() }).collect();
        return thing_number(mind, super::mind::USER_NAME, &as_user);
    }
    if super::mind::AGE_WORDS.contains(&word) {
        return named_result(mind, super::mind::YEAR_NAME);
    }
    let part_told = |quality: &str| (mind.tree.state..mind.tree.len()).filter(|&r| !mind.tree.node(r).gone && *mind.tree.node(r).name == *step_item(quality) && asked.iter().any(|a| singular(a) == *mind.tree.node(mind.tree.node(r).parent).name)).find_map(|r| present_children(mind, r).into_iter().find_map(|v| number_of(&mind.tree.node(v).name)).map(|part| (mind.tree.node(r).parent, part)));
    if let Some((_, part)) = part_told(word) {
        return Some(part);
    }
    if let Some((group, part)) = opposites(mind, word).iter().find_map(|o| part_told(o)) {
        return Some(count_of(mind, group) as f32 - part);
    }
    if word == super::mind::CHANGE_ASKED {
        let payment = asked.iter().flat_map(|a| things_named(mind, a)).find_map(|payer| own_child(mind, payer, &step_item(super::mind::PAYING)))?;
        let priced = (mind.tree.state..payment).rev().find(|&n| !mind.tree.node(n).gone && *mind.tree.node(n).name == *step_item(super::mind::COSTING))?;
        return thing_number(mind, &mind.tree.node(mind.tree.node(priced).parent).name.to_string(), asked);
    }
    if let Some((_, by)) = super::mind::MULTIPLIERS.iter().find(|(m, _)| *m == word) {
        return Some(*by);
    }
    let named = |name: &str| asked.iter().any(|a| a == name || singular(a) == name);
    let things: Vec<usize> = things_named(mind, word).into_iter().chain(mind.tree.named(word).chain(mind.tree.named(&singular(word))).filter(|&n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0)).collect();
    let mut counted: Vec<f32> = (mind.tree.state..mind.tree.len()).filter(|&n| !mind.tree.node(n).gone && singular(&mind.tree.node(n).name) == singular(word) && open_question(mind).is_none_or(|q| !inside(mind, n, q))).filter_map(|n| present_children(mind, n).into_iter().find_map(|q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(q).name)).flatten())).collect();
    counted.dedup();
    for thing in things {
        let mut found: Vec<(bool, f32)> = Vec::new();
        for r in present_children(mind, thing).into_iter().filter(|&r| { let name = &*mind.tree.node(r).name; name.starts_with(BRACE_OPEN_TEXT) && *name != *TIME_RELATION && *name != *OWNER_TAG && *name != *GENDER_TAG && !name.starts_with(crate::cursor::QUANTITY_TAG) }) {
            let relation = crate::cursor::bare_name(&mind.tree.node(r).name);
            for v in present_children(mind, r) {
                let name = mind.tree.node(v).name.to_string();
                let counted = present_children(mind, v).into_iter().find_map(|q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(q).name)).flatten());
                let tagged = name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&name)).flatten();
                if let Some(value) = number_of(&name).filter(|_| !name.starts_with(BRACE_OPEN_TEXT)).or(counted).or(tagged) {
                    found.push((named(&relation) || named(&name), value));
                }
            }
        }
        for tag in present_children(mind, thing).into_iter().filter(|&q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG)) {
            let count = number_of(&mind.tree.node(tag).name).unwrap_or_default();
            let worths: Vec<usize> = present_children(mind, tag).into_iter().filter(|&e| *mind.tree.node(e).name == *crate::cursor::EQUAL_RELATION).flat_map(|e| present_children(mind, e)).collect();
            if worths.is_empty() {
                found.push((false, count));
            }
            for v in worths {
                let unit = mind.tree.node(v).link.map(|tagged| mind.tree.node(mind.tree.node(tagged).parent).name.to_string()).unwrap_or_default();
                if let Some(value) = number_of(&mind.tree.node(v).name).filter(|_| count.is_normal()) {
                    found.push((named(&unit), value / count));
                }
            }
        }
        if found.is_empty() {
            for had in held_or_owned(mind, thing) {
                let count = present_children(mind, had).into_iter().find_map(|q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(q).name)).flatten());
                if let Some(count) = count {
                    found.push((named(&mind.tree.node(had).name), count));
                }
            }
        }
        let chosen: Vec<f32> = found.iter().filter(|(named, _)| *named).map(|(_, v)| *v).collect();
        let of_seeds = chosen.is_empty() && !mind.tree.story(thing);
        let mut distinct = if chosen.is_empty() { found.iter().map(|(_, v)| *v).collect::<Vec<f32>>() } else { chosen };
        distinct.dedup();
        if let ([one], [told]) = (&distinct[..], &counted[..]) {
            return Some(if of_seeds { *told } else { *one });
        }
        if let [one] = distinct[..] {
            return Some(one);
        }
    }
    match counted[..] {
        [one] => Some(one),
        _ => None,
    }
}
because!(thing_number, WordReading, "a word the speaker says of themselves is worth what the user's node is worth, how old am i; the \
     number a word's thing holds, for a move on the number that points at a word that is no number, a quality being worth the part of the \
     asked group told to have it, or the rest of the group when the part told has its opposite, absent and present, the word change being \
     worth the price told just before the payer named paid, and a thing with no number of its own the count of what it has, tom his ten \
     dollars, or the one count the story gave a value of the name, three friends shared with, which also stands before a number of the \
     seeds the question does not name, the teeth of a child: the one number under its relations, or its own count, and among several the \
     one whose relation or unit the question names, the cents a dime is worth, or what one of a unit equals in the unit the question \
     names, read of the story's thing first and then of the seeds'");

pub fn operand_value(mind: &CursorMind, word: &str) -> Option<f32> {
    let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let spoken = super::mind::SELF_WORDS.contains(&word).then(|| thing_number(mind, word, &asked)).flatten();
    spoken.or_else(|| number_of(word)).or_else(|| is_number(mind, word).and_then(|n| number_of(&n))).or_else(|| thing_number(mind, word, &asked))
}
because!(operand_value, WordReading, "what a pointed word is worth to a move on the number: for a word the speaker says of themselves what \
     the user's node is worth, before i is read as the roman one; the number it writes or spells, or the number its thing holds");

pub fn left_trace(mind: &CursorMind, name: &str) -> bool {
    things_named(mind, name).into_iter().any(|thing| mind.tree.named(name).any(|n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(thing) && trace(mind, n)))
}
because!(left_trace, WordReading, "whether a thing of a name left a trace anywhere, so a question in the past asks where it was only of a \
     thing that moved or changed hands, and who was lincoln asks what he is");

pub fn activity_word(mind: &CursorMind, word: &str) -> bool {
    mind.tree.named(word).any(|n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0 && own_child(mind, n, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|c| *mind.tree.node(c).name == *super::mind::ACTIVITY)))
}
because!(activity_word, WordReading, "whether the seeds class a word as an activity, fun, a picnic or a nap, which one has for a while and \
     does not own");

pub fn activity_named(mind: &CursorMind, word: &str) -> bool {
    let activity = step_item(super::mind::ACTIVITY);
    let stem = super::mind::verb_stem(mind, word);
    [word, stem.as_str()].iter().any(|name| mind.tree.named(name).any(|n| n != 0 && !mind.tree.node(n).gone && mind.tree.story(n) && *mind.tree.node(mind.tree.node(n).parent).name == *activity))
}
because!(activity_named, WordReading, "whether a word, or the stem of its verb, names an activity the story told, so a where question \
     about it finds the activity and not the one who does it");

pub fn unit_worths(mind: &CursorMind, from: &str) -> Vec<(String, f32)> {
    let from = singular(from);
    let mut worths = Vec::new();
    for unit in mind.tree.named(&from).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0) {
        for tag in present_children(mind, unit).into_iter().filter(|&q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG)) {
            let Some(count) = number_of(&mind.tree.node(tag).name).filter(|c| c.is_normal()) else { continue };
            for v in present_children(mind, tag).into_iter().filter(|&e| *mind.tree.node(e).name == *crate::cursor::EQUAL_RELATION).flat_map(|e| present_children(mind, e)) {
                let unit_named = mind.tree.node(v).link.map(|tagged| mind.tree.node(mind.tree.node(tagged).parent).name.to_string());
                if let Some((named, worth)) = unit_named.zip(number_of(&mind.tree.node(v).name)) {
                    worths.push((named, worth / count));
                }
            }
        }
    }
    worths
}
because!(unit_worths, WordReading, "every unit the seeds give one of a unit a worth in, with that worth, ten cents and a tenth of a dollar \
     for a dime");

pub fn span_of(mind: &CursorMind, thing: usize) -> Option<f32> {
    let hour = |deed: &str| own_child(mind, thing, &step_item(deed)).and_then(|r| present_children(mind, r).into_iter().find_map(|v| number_of(&mind.tree.node(v).name)));
    super::mind::SPAN_ENDS.iter().find_map(|(first, last)| hour(first).zip(hour(last)).map(|(from, to)| to - from))
}
because!(span_of, WordReading, "how long a thing lasts, the hour it was told to end at less the hour it was told to start at");

pub fn spans_told(mind: &CursorMind) -> bool {
    story_nodes_all(mind).into_iter().any(|n| span_of(mind, n).is_some())
}
because!(spans_told, WordReading, "whether the story told a thing to start and to end, so longer and shorter may compare how long things \
     last");

pub fn unit_worth(mind: &CursorMind, from: &str, to: &str) -> Option<f32> {
    let to = singular(to);
    let direct = unit_worths(mind, from).into_iter().find(|(unit, _)| *unit == to).map(|(_, worth)| worth);
    direct.or_else(|| {
        let theirs = unit_worths(mind, &to);
        unit_worths(mind, from).into_iter().find_map(|(unit, worth)| theirs.iter().find(|(shared, other)| *shared == unit && other.is_normal()).map(|(_, other)| worth / other))
    })
}
because!(unit_worth, WordReading, "how many of one unit one of another is worth by the seeds, seven days for a week, read from the amount \
     of the first unit that equals an amount of the second, or through a unit both are worth in, a dime in pennies through cents");

pub fn converted(mind: &CursorMind, wanted: &str, asked: &[String]) -> Option<f32> {
    let worth_in = |given: &String| unit_worth(mind, given, wanted).or_else(|| unit_worth(mind, wanted, given).filter(|w| w.is_normal()).map(|worth| worth.recip()));
    let part_of = |a: &String| crate::cursor::FRACTION_WORDS.iter().find(|(f, _)| *f == a.as_str()).map(|(_, parts)| parts.recip());
    let parted = |a: &String| asked.iter().position(|b| b == a).and_then(|at| asked.get(at + 1)).is_some_and(|unit| worth_in(unit).is_some() || singular(unit) == singular(wanted));
    let amount_of = |a: &String| number_of(a).or_else(|| part_of(a).filter(|_| parted(a)));
    if !asked.iter().any(|a| amount_of(a).is_some()) {
        return asked.iter().rev().filter(|a| *a != wanted && singular(a) != singular(wanted)).find_map(worth_in);
    }
    let worded = |a: &String| number_of(a).or_else(|| is_number(mind, a).and_then(|n| number_of(&n)));
    let parts_counted = |at: usize| at.checked_sub(1).and_then(|before| asked.get(before)).and_then(worded).filter(|_| part_of(&asked[at]).is_some());
    let counts_parts = |at: usize| asked.get(at + 1).is_some_and(|next| part_of(next).is_some() && parted(next));
    let said: Vec<usize> = asked.iter().enumerate().filter(|(at, a)| amount_of(a).is_some() && !counts_parts(*at)).map(|(at, _)| at).collect();
    let amounts: Vec<Option<f32>> = said.iter().map(|&at| {
        let amount = amount_of(&asked[at])? * parts_counted(at).unwrap_or(f32::from(u8::from(true)));
        let given = asked.get(at + 1)?;
        if said.len() > 1 && singular(given) == singular(wanted) {
            return Some(amount);
        }
        worth_in(given).map(|worth| amount * worth)
    }).collect();
    amounts.iter().all(Option::is_some).then(|| amounts.into_iter().flatten().sum())
}
because!(converted, WordReading, "an amount of one unit counted in another, the one number of a question and the unit after it turned into \
     the unit asked, fourteen days as two weeks, or with no number one of the last unit named, how many pennies make a dime, and with \
     several numbers the sum of each in the unit asked, four dollars and five cents in cents, a word of a part before a unit counting as \
     its number and a number before the part as that many of it, three quarters of an hour and half an hour, half a day, while two \
     quarters are coins");

pub fn says_what(mind: &CursorMind, n: usize) -> bool {
    own_child(mind, n, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|c| *mind.tree.node(c).name != *TIME_RELATION && count_of(mind, c) > 0))
}
because!(says_what, WordReading, "whether a thing holds under is a value the story did not deny, so the teacher's search walks past a \
     thing of the name that only says what it is not, a whale is not a fish");

pub fn told_relation(mind: &CursorMind, relation: &str) -> bool {
    let braced = step_item(relation);
    mind.tree.named(&braced).any(|r| !mind.tree.node(r).gone && mind.tree.story(r) && open_question(mind).is_none_or(|q| !inside(mind, r, q)))
}
because!(told_relation, WordReading, "whether the story told a relation of a word of any thing, so a question asking it of a thing that \
     lacks it is answered nothing and not by the thing's class, what does a bee make");

pub fn node_relates(mind: &CursorMind, n: usize, word: &str) -> bool {
    own_child(mind, n, &relation_named(mind, word)).is_some()
}
because!(node_relates, WordReading, "whether one node holds the relation a word of a question names, so the teacher's search walks past a \
     thing of the name that lacks it, the duck in the lake for the duck that says quack");

pub fn owns_relation(mind: &CursorMind, thing: &str, relation: &str) -> bool {
    let braced = step_item(relation);
    let one = singular(thing);
    mind.tree.named(thing).chain(mind.tree.named(&one)).any(|n| n != 0 && !mind.tree.node(n).gone && own_child(mind, n, &braced).is_some())
}
because!(owns_relation, WordReading, "whether a thing of a name holds a relation of a word as its own, so the teacher reads it forward, \
     and backward when the thing only stands under it");

pub fn has_relation(mind: &CursorMind, thing: &str, relation: &str) -> bool {
    let braced = step_item(relation);
    let one = singular(thing);
    let things: Vec<usize> = mind.tree.named(thing).chain(mind.tree.named(&one)).filter(|&n| n != 0 && !mind.tree.node(n).gone).collect();
    things.iter().any(|&n| own_child(mind, n, &braced).is_some()) || mind.tree.named(thing).any(|v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *braced)
}
because!(has_relation, WordReading, "whether a thing of a name holds a relation of a word, or stands under one, france under capital, for \
     the teacher's walk");

pub fn holder_name(mind: &CursorMind, thing: usize) -> Option<String> {
    who_has(mind, thing).or_else(|| { let p = mind.tree.node(thing).parent; (p != 0 && !mind.tree.node(p).name.starts_with(BRACE_OPEN_TEXT)).then_some(p) }).map(|h| mind.tree.node(h).name.to_string())
}
because!(holder_name, WordReading, "the name of who has a thing or what it stands in, for the teacher's search");

pub fn counted(mind: &CursorMind, thing: usize) -> String {
    count_of(mind, thing).to_string()
}
because!(counted, WordReading, "how many a thing stands for, written as a count is written, for the teacher's search");

pub fn told_thing(mind: &CursorMind, name: &str) -> bool {
    !things_named(mind, name).is_empty()
}
because!(told_thing, WordReading, "whether the story made a thing of a name, or of its singular, outside the open question, the speaker's \
     words standing for the user");

pub(super) fn own_child(mind: &CursorMind, under: usize, name: &str) -> Option<usize> {
    present_children(mind, under).into_iter().find(|&c| *mind.tree.node(c).name == *name)
}
because!(own_child, WordReading, "the child of a node with a name among the node's own children, never through the thing a mention links \
     to");

pub(super) fn kept_flag(key: &str) -> bool {
    [FLAG_GIVE, FLAG_CONTAIN, FLAG_HAND, super::mind::FLAG_TAKE, super::mind::FLAG_RELEASE, super::mind::FLAG_GROUP, super::mind::FLAG_STATE, super::mind::FLAG_PLACE, super::mind::FLAG_WITH, super::mind::FLAG_LEAVE, super::mind::FLAG_FROM, super::mind::FLAG_WHEN, super::mind::FLAG_ROLE, super::mind::FLAG_KIN].contains(&key)
}
because!(kept_flag, WordReading, "whether a flag says what happens to the thing appearing next rather than what it is, or when it happens, \
     so it stays until the drop");

pub(super) fn mention_thing(mind: &CursorMind, at: usize) -> usize {
    let name = mind.tree.node(at).name.to_string();
    let of_story = |n: usize| n != at && !mind.tree.node(n).gone && mind.tree.node(n).link.is_none() && *mind.tree.node(n).name == *name && !mind.tree.node(mind.tree.node(n).parent).name.starts_with(BRACE_OPEN_TEXT) && open_question(mind).is_none_or(|q| !inside(mind, n, q));
    mind.tree.node(at).link.filter(|&t| mind.tree.story(t)).or_else(|| (mind.tree.state..mind.tree.len()).rev().find(|&n| of_story(n))).or(mind.tree.node(at).link).unwrap_or(at)
}
because!(mention_thing, WordReading, "the thing a mention under a deed stands for: the thing of the story it links to, else the newest \
     thing of the story with its name, since a name the seeds know links to the seed thing, else what it links to or itself");

pub fn gained_by_word(mind: &CursorMind, word: &str) -> bool {
    let relation = mind.at;
    if relation == 0 || !mind.held.is_empty() || !super::mind::GAINING.contains(&crate::cursor::bare_name(&mind.tree.node(relation).name).as_str()) {
        return false;
    }
    let doer = mind.tree.node(relation).parent;
    let left = |thing: usize| mind.tree.named(&mind.tree.node(thing).name.to_string()).any(|n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(thing) && trace(mind, n));
    doer != 0 && story_nodes(mind, word).into_iter().chain(story_nodes(mind, &singular(word))).any(|thing| mind.tree.node(thing).parent != doer && (mind.tree.node(thing).parent != 0 || left(thing)))
}
because!(gained_by_word, WordReading, "whether a word said on a deed of finding or buying names a thing the story already placed or lost \
     and the doer does not have, so the teacher hands it to the doer at that word, zoe finds the coin");

pub fn traded(mind: &CursorMind, deeds: &[&str]) -> bool {
    let at = mind.at;
    if at == 0 || mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) || !mind.held.is_empty() {
        return false;
    }
    let relation = mind.tree.node(at).parent;
    let doer = mind.tree.node(relation).parent;
    let deed = crate::cursor::bare_name(&mind.tree.node(relation).name);
    let thing = mention_thing(mind, at);
    if thing == at {
        return false;
    }
    let gains = deeds.iter().any(|d| super::mind::GAINING.contains(d));
    let known = mind.tree.node(thing).parent != 0 || mind.tree.named(&mind.tree.node(thing).name.to_string()).any(|n| !mind.tree.node(n).gone && mind.tree.node(n).link == Some(thing) && trace(mind, n));
    relation != 0 && doer != 0 && mind.tree.node(relation).name.starts_with(BRACE_OPEN_TEXT) && deeds.contains(&deed.as_str()) && known && (mind.tree.node(thing).parent != doer) == gains
}
because!(traded, WordReading, "whether the cursor stands on the thing a deed of trade names, under the deed of its doer, a thing the story \
     already placed or lost and the doer does not have, so the teacher hands it on");

pub(super) fn places_told(mind: &CursorMind) -> Option<usize> {
    let round = story_node(mind, super::mind::ROUNDING)?;
    let holder = mind.tree.node(round).parent;
    (holder != 0).then(|| is_number(mind, &mind.tree.node(holder).name)).flatten().and_then(|n| number_of(&n)).map(|n| n as usize).or(mind.places)
}
because!(places_told, WordReading, "how many decimals an answer is written to: the number the story dropped the rounding in, else the \
     places the mind was set");

pub fn number_run(mind: &CursorMind) -> Vec<f32> {
    (mind.tree.state..mind.tree.len()).filter(|&n| !mind.tree.node(n).gone && mind.tree.node(n).parent == 0 && mind.tree.node(n).link.is_none() && present_children(mind, n).is_empty()).filter_map(|n| number_of(&mind.tree.node(n).name)).collect()
}
because!(number_run, WordReading, "the run of numbers the story told, each a bare thing of the world, in the order said");

pub(super) fn run_answer(run: &[f32], word: &str) -> Option<f32> {
    let steps: Vec<f32> = run.windows(super::mind::LIST_HALVES).map(|pair| pair[1] - pair[0]).collect();
    let ratios: Vec<f32> = run.windows(super::mind::LIST_HALVES).filter(|pair| pair[0].is_normal()).map(|pair| pair[1] / pair[0]).collect();
    let even = |all: &[f32]| all.first().copied().filter(|first| all.iter().all(|s| (s - first).abs() < f32::EPSILON));
    let asked = super::mind::RUN_ASKED.iter().position(|a| *a == word)?;
    let smallest = run.iter().copied().reduce(f32::min);
    let largest = run.iter().copied().reduce(f32::max);
    let next = || even(&steps).map(|step| run[run.len() - 1] + step).or_else(|| even(&ratios).filter(|_| ratios.len() == steps.len()).map(|ratio| run[run.len() - 1] * ratio));
    let missing = || {
        let step = steps.iter().copied().reduce(|a, b| if b.abs() < a.abs() { b } else { a })?;
        let wide = if steps.len() == 1 { Some(0) } else { steps.iter().position(|s| (s - step * super::mind::HALVING).abs() < f32::EPSILON) }?;
        Some(if steps.len() == 1 { run[wide] + step / super::mind::HALVING } else { run[wide] + step })
    };
    let answers: [Option<f32>; super::mind::RUN_ASKED.len()] = [next(), missing(), run.first().copied(), run.last().copied(), largest, smallest];
    answers[asked]
}
because!(run_answer, WordReading, "the number of a run a word asks: next is the last and the same step again, or the same factor again, \
     missing is the one halfway between two, or the one that fills the step twice as wide as the others, and first, last, largest and \
     smallest are read off the run");

pub(super) fn moved_without_place(mind: &CursorMind) -> Option<String> {
    let plain_carry = [FLAG_GIVE, FLAG_HAND, super::mind::FLAG_GROUP, FLAG_QUANTITY].iter().all(|flag| flag_of(mind, flag).is_none());
    let object_like = mind.before.iter().any(|said| super::mind::ARTICLE_FLAGS.contains(&said.as_str())) || mind.before.last().is_some_and(|last| *singular(last) != **last);
    let placed = mind.before.iter().any(|said| super::mind::place_word(said) || said == super::mind::INFINITIVE);
    mind.before.iter().filter_map(|said| super::mind::verb_base(mind, said)).find(|base| STEERED.contains(&base.as_str())).filter(|_| plain_carry && !placed && object_like)
}
because!(moved_without_place, WordWorld, "the verb of steering a sentence said with no place word after it and with an article or a plural after it, she drives a bus, he flies planes, whose thing named next is what the verb is done to and no place the doer goes into");

pub fn person(mind: &CursorMind, n: usize) -> bool {
    let name = &mind.tree.node(n).name;
    mind.tree.named(name).any(|m| !mind.tree.node(m).gone && own_child(mind, m, GENDER_TAG).is_some())
}
because!(person, WordReading, "whether a thing is a person: a node of its name carries a gender, as the seeds give tom and ann");

fn gendered(mind: &CursorMind, n: usize, pronoun: &str) -> bool {
    let Some((_, wanted)) = super::mind::PRONOUN_GENDERS.iter().find(|(p, _)| *p == pronoun) else { return true };
    let name = mind.tree.node(n).name.to_string();
    let genders: Vec<String> = mind.tree.named(&name).filter(|&m| !mind.tree.node(m).gone).filter_map(|m| own_child(mind, m, GENDER_TAG)).flat_map(|g| present_children(mind, g)).map(|v| mind.tree.node(v).name.to_string()).collect();
    genders.is_empty() || genders.iter().any(|g| g == wanted)
}
because!(gendered, WordReading, "whether a thing may be what a pronoun stands for by its gender: one the seeds give the pronoun's gender, \
     or none at all");

pub(super) fn newest_told(mind: &CursorMind, people: bool) -> Option<usize> {
    newest_for(mind, people, "")
}
because!(newest_told, WordReading, "what a pronoun stands for when its gender does not matter");

pub(super) fn newest_for(mind: &CursorMind, people: bool, pronoun: &str) -> Option<usize> {
    let asked = open_question(mind);
    let fit = |n: usize| !mind.tree.node(n).gone && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && !tag_value(mind, n) && number_of(&mind.tree.node(n).name).is_none() && !super::mind::MEASURES.contains(&crate::cursor::bare_name(&mind.tree.node(mind.tree.node(n).parent).name).as_str()) && !mind.held.contains(&n) && mind.tree.node(n).link.is_none() && asked.is_none_or(|q| q != n && !inside(mind, n, q)) && person(mind, n) == people && gendered(mind, n, pronoun);
    let placing = !mind.held.is_empty() && flag_of(mind, super::mind::FLAG_PLACE).is_some();
    let before = mind.last_topic.filter(|&t| placing && !people && t < mind.tree.len() && fit(t));
    let topic = before.or(mind.first_mark.filter(|&t| t < mind.tree.len() && fit(t)));
    let gone_to = |n: usize| !people && !mind.tree.node(n).gone && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && !tag_value(mind, n) && !person(mind, n) && super::writing::going_relation(mind, mind.tree.node(n).parent) && asked.is_none_or(|q| q != n && !inside(mind, n, q));
    let placed = |n: usize| gone_to(n) || !people && fit(n) && (went_to(mind, n).is_some() || mind.tree.node(n).parent != 0 && !mind.tree.node(mind.tree.node(n).parent).name.starts_with(BRACE_OPEN_TEXT));
    let topic = topic.or_else(|| (mind.tree.state..mind.tree.len()).rev().find(|&n| placed(n)));
    let top = |n: usize| people && mind.tree.node(n).parent == 0 && !mind.tree.node(n).gone && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && asked.is_none_or(|q| !inside(mind, n, q));
    let moved = mind.first_mark.filter(|&t| people && t < mind.tree.len() && !mind.tree.node(t).gone && mind.tree.node(t).link.is_none() && !mind.tree.node(t).name.starts_with(BRACE_OPEN_TEXT) && (mind.tree.node(t).parent != 0 || went_to(mind, t).is_some()) && owner_of(mind, t).is_none() && own_child(mind, t, &step_item(FLAG_DEFINITE)).is_none() && own_child(mind, t, &step_item(FLAG_INDEFINITE)).is_none() && gendered(mind, t, pronoun) && number_of(&mind.tree.node(t).name).is_none());
    topic.or_else(|| (mind.tree.state..mind.tree.len()).rev().find(|&n| fit(n))).or(moved).or_else(|| (mind.tree.state..mind.tree.len()).rev().find(|&n| top(n)))
}
because!(newest_for, WordReading, "what a pronoun stands for, for it while a thing is held to be placed the thing the sentence before was \
     about, the key is in it after the box is on the bed, never a number or the unit of a measure, and for he or she with no person of the \
     seeds the thing the last sentence placed before any thing of the world, sandra whom the seeds do not know and not the bedroom: one of \
     its gender when it has one, she the newest woman; the topic, the thing the last sentence was about, when it is a person for he or she \
     and no person for it or they, else the newest thing that was placed somewhere before one that only holds others, the car and not the \
     street, the newest thing the story told of that sort, no relation and no tag's value, and for he or she with no person the seeds \
     know, the newest thing standing right in the world, as lena who has a bag");

pub(super) fn inside(mind: &CursorMind, node: usize, of: usize) -> bool {
    let mut at = node;
    while at != 0 {
        if at == of {
            return true;
        }
        at = mind.tree.node(at).parent;
    }
    false
}
because!(inside, WordReading, "whether a node stands at or below another, so a thing is never dropped into itself");

pub(super) fn story_nodes_all(mind: &CursorMind) -> Vec<usize> {
    let asked = open_question(mind);
    (mind.tree.state..mind.tree.len()).rev().filter(|&n| !mind.tree.node(n).gone && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && !tag_value(mind, n) && standing(mind, n) && asked.is_none_or(|q| !inside(mind, n, q))).collect()
}
because!(story_nodes_all, WordReading, "every thing the story made, the newest first, outside the open question");

pub(super) fn quantity_tag_of(mind: &CursorMind, n: usize) -> String {
    present_children(mind, n).into_iter().map(|c| mind.tree.node(c).name.to_string()).find(|name| name.starts_with(crate::cursor::QUANTITY_TAG)).unwrap_or_default()
}
because!(quantity_tag_of, WordReading, "the name of the quantity tag a node carries, or nothing");

pub(super) fn sides_of(mind: &CursorMind, shape: &str) -> Option<usize> {
    let part = step_item(super::mind::SIDE_NAMED);
    mind.tree.named(shape).filter(|&n| n != 0 && !mind.tree.node(n).gone).find_map(|n| own_child(mind, n, &part)).map(|sides| count_of(mind, sides))
}
because!(sides_of, WordReading, "how many sides a shape has: the count under the side relation of a thing of that name, a rectangle four");

pub(super) fn count_of(mind: &CursorMind, n: usize) -> usize {
    present_children(mind, n).into_iter().find(|&c| mind.tree.node(c).name.starts_with(crate::cursor::QUANTITY_TAG)).and_then(|c| number_of(&mind.tree.node(c).name)).map(|q| q as usize).unwrap_or(1)
}
because!(count_of, WordReading, "how many a thing stands for: the count of its quantity tag, or one");

pub(super) fn place_value(mind: &CursorMind, at: usize) -> Option<usize> {
    present_children(mind, at).into_iter().filter(|&r| { let name = crate::cursor::bare_name(&mind.tree.node(r).name); mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT) && place_word(&name) }).flat_map(|r| present_children(mind, r)).last()
}
because!(place_value, WordReading, "the place a thing of the seeds stands at, written in their older way as a place relation with the \
     place under it, france in europe");

pub(super) fn speaker(mind: &CursorMind, n: usize) -> bool {
    let name = &*mind.tree.node(n).name;
    *name == *super::mind::USER_NAME || *name == *super::mind::ASSISTANT_NAME
}
because!(speaker, WordReading, "whether a thing is one of the two who speak, the user or the assistant, who have what stands inside them \
     as a person does");

pub fn goings(mind: &CursorMind, at: usize) -> Vec<usize> {
    if at == 0 {
        return Vec::new();
    }
    present_children(mind, at)
        .into_iter()
        .filter(|&c| mind.tree.story(c) && super::mind::MOVING.contains(&crate::cursor::bare_name(&mind.tree.node(c).name).as_str()))
        .flat_map(|c| present_children(mind, c).into_iter().filter(|&v| !mind.tree.node(v).name.starts_with(BRACE_OPEN_TEXT)))
        .collect()
}
because!(goings, WordReading, "the places a thing went to, oldest first, each one what a relation of moving carries, so where it is now is the last of them and where it was is the one before");

pub fn goings_in_order(mind: &CursorMind, at: usize) -> Vec<usize> {
    let mut walked = goings(mind, at);
    let day = |n: usize| time_of_day(mind, mind.tree.node(n).parent).and_then(|t| super::mind::TIMES_OF_DAY.iter().position(|one| *one == t));
    if walked.iter().all(|&n| day(n).is_some()) {
        walked.sort_by_key(|&n| day(n));
    }
    walked
}
because!(goings_in_order, WordReading, "the places a thing went to in the order they were made: by the time of day each going carries when every one of them carries one, else the order the text told them in");

pub fn went_to(mind: &CursorMind, at: usize) -> Option<usize> {
    if at == 0 {
        return None;
    }
    goings(mind, at).pop()
}
because!(went_to, WordReading, "the place a thing went to: what the newest relation of moving it carries holds, which says where it is when it stands inside nothing, since a going writes what was done and not a thing put into a place");

pub fn place_of(mind: &CursorMind, at: usize) -> Option<usize> {
    let parent = (at != 0).then(|| mind.tree.node(at).parent).filter(|&p| p != 0 && Some(p) != open_question(mind) && !mind.tree.node(p).name.starts_with(BRACE_OPEN_TEXT));
    if let Some(parent) = parent {
        let bare_named = own_child(mind, parent, &step_item(FLAG_DEFINITE)).is_none() && own_child(mind, parent, &step_item(FLAG_INDEFINITE)).is_none();
        if person(mind, parent) || speaker(mind, parent) || (owner_of(mind, at) == Some(parent) && bare_named && mind.tree.node(parent).parent != 0) {
            return place_of(mind, parent);
        }
    }
    if let Some(gone) = went_to(mind, at) {
        return Some(gone);
    }
    parent.filter(|&p| owner_of(mind, at) != Some(p))
}
because!(place_of, WordReading, "where a thing stands: the thing it stands inside, unless that is its owner, who has it rather than being \
     its place, or a person, a speaker or an owner told by a bare name that stands somewhere, who carries it where they stand, daniel and \
     not the senator");

pub(super) fn seen(mind: &CursorMind, n: usize) -> Vec<crate::network::FeatureId> {
    let words: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let name = mind.tree.node(n).name.to_string();
    let qualities: Vec<&String> = words.iter().filter(|w| **w != name && **w != singular(&name) && super::mind::quality_word(mind, w)).collect();
    let mut ids = Vec::new();
    if place_of(mind, n).is_some() {
        ids.push(crate::events::feature(FIND_SEES_PLACE, true));
    }
    if owner_of(mind, n).is_some() || mind.tree.node(n).parent != 0 && person(mind, mind.tree.node(n).parent) {
        ids.push(crate::events::feature(FIND_SEES_OWNER, true));
    }
    if !held_or_owned(mind, n).is_empty() {
        ids.push(crate::events::feature(FIND_SEES_CONTENTS, true));
    }
    if qualities.iter().all(|q| holds_value(mind, n, q)) {
        ids.push(crate::events::feature(FIND_SEES_ASKED, true));
    }
    ids
}
because!(seen, WordReading, "what a find sees of the thing it landed on, added to its event: whether it stands in a place, has an owner, \
     holds things and holds the qualities the question names, since a name can stand on many nodes and only one of them answers");

pub(super) fn trace(mind: &CursorMind, n: usize) -> bool {
    mind.tree.node(n).link.is_some() && own_child(mind, n, TIME_RELATION).is_some()
}
because!(trace, WordReading, "whether a node is the trace a thing left where it was, a mention of it with the time past, which is no thing \
     standing there now");

pub fn said_to_have(mind: &CursorMind, thing: usize, owner: usize) -> bool {
    owner_of(mind, thing) == Some(owner)
}
because!(said_to_have, WordReading, "whether a thing was said to be had by another, read from its owner tag, for the teacher to tell the \
     thing of a clause of having from a subject");


pub fn who_has(mind: &CursorMind, thing: usize) -> Option<usize> {
    let mut at = thing;
    for _ in 0..CLASS_DEPTH {
        if let Some(owner) = owner_of(mind, at) {
            return Some(owner);
        }
        let parent = mind.tree.node(at).parent;
        if parent == 0 || mind.tree.node(parent).name.starts_with(BRACE_OPEN_TEXT) {
            return None;
        }
        if person(mind, parent) {
            return Some(parent);
        }
        at = parent;
    }
    None
}
because!(who_has, WordReading, "who has a thing: its owner, or the person it stands inside, or the owner or person of what holds it, tom \
     who has the bag the coin is in");

pub(super) fn owner_of(mind: &CursorMind, thing: usize) -> Option<usize> {
    let tag = own_child(mind, thing, OWNER_TAG)?;
    present_children(mind, tag).into_iter().find_map(|m| mind.tree.node(m).link)
}
because!(owner_of, WordReading, "the one a thing was given to, read from the mention under its owner tag, or none for a thing never given");

pub(super) fn to_come(mind: &CursorMind, n: usize) -> bool {
    own_child(mind, n, TIME_RELATION).is_some_and(|t| present_children(mind, t).into_iter().any(|v| *mind.tree.node(v).name == *crate::cursor::LATER_TIME)) && !mind.before.iter().any(|b| b == super::mind::WILL)
}
because!(to_come, WordReading, "whether a thing stands where it is only for a time to come, the cat will be in the garden, while the \
     sentence read does not say will, so it is not there now");

pub(super) fn counted_within(mind: &CursorMind, at: usize, fits: &dyn Fn(usize, bool) -> bool, deeper: usize) -> usize {
    held_or_owned(mind, at).into_iter().map(|c| {
        let each = if deeper > 0 && !fits(c, true) { counted_within(mind, c, fits, deeper - 1) } else { 0 };
        if each > 0 { count_of(mind, c) * each } else if fits(c, false) { count_of(mind, c) } else { 0 }
    }).sum()
}
because!(counted_within, WordReading, "how many of a kind a thing holds: the counts of the ones right in it, and for every other group in \
     it the count of a group not of that name times how many each one of the group holds, so a shop of five shelves with four boxes each \
     holds twenty boxes");

pub(super) fn came_to(mind: &CursorMind, at: usize) -> Vec<usize> {
    let named = |n: usize| crate::cursor::bare_name(&mind.tree.node(n).name);
    (mind.tree.state..mind.tree.len())
        .filter(|&n| !mind.tree.node(n).gone && went_to(mind, n).is_some_and(|place| place == at || named(place) == named(at)))
        .collect()
}
because!(came_to, WordReading, "the things that went to a place: those whose newest relation of moving carries it, so who is in the park is answered from what was done and not from what stands inside the park");

pub(super) fn held_or_owned(mind: &CursorMind, at: usize) -> Vec<usize> {
    let here = |c: usize| went_to(mind, c).is_none_or(|place| place == at || crate::cursor::bare_name(&mind.tree.node(place).name) == crate::cursor::bare_name(&mind.tree.node(at).name));
    let mut things: Vec<usize> = present_children(mind, at).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && !trace(mind, c) && count_of(mind, c) > 0 && !to_come(mind, c) && here(c)).collect();
    let owned = (mind.tree.state..mind.tree.len()).filter(|&n| !mind.tree.node(n).gone && !things.contains(&n) && !things.iter().any(|&t| mind.tree.node(t).link == Some(n)) && owner_of(mind, n) == Some(at) && count_of(mind, n) > 0);
    things.extend(owned.collect::<Vec<_>>());
    for one in came_to(mind, at) {
        if !things.contains(&one) && !things.iter().any(|&t| mind.tree.node(t).link == Some(one)) {
            things.push(one);
        }
    }
    things
}
because!(held_or_owned, WordReading, "the things a thing holds, standing inside it, and the things it owns that stand elsewhere, so what \
     ann has lists the dog she was given that is now in the garden");

fn related_value(mind: &CursorMind, thing: usize, value: &str) -> bool {
    let named = |n: usize| { let name = &*mind.tree.node(n).name; *name == *value || *name == *singular(value) || singular(name) == value };
    present_children(mind, thing).into_iter().filter(|&r| { let name = &*mind.tree.node(r).name; name.starts_with(BRACE_OPEN_TEXT) && *name != *IS_FORM.trim() && *name != *TIME_RELATION && *name != *OWNER_TAG && *name != *GENDER_TAG }).flat_map(|r| present_children(mind, r)).any(named)
}
because!(related_value, WordReading, "whether a thing holds a value under one of its own relations but is, so what a class of the story is \
     afraid of is found for one of its members");

pub(super) fn denied_value(mind: &CursorMind, thing: usize, value: &str) -> bool {
    let named = |n: usize| { let name = &*mind.tree.node(n).name; (*name == *value || *name == *singular(value)) && count_of(mind, n) == 0 };
    present_children(mind, thing).into_iter().filter(|&r| mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT)).flat_map(|r| present_children(mind, r)).any(|v| named(v) || (mind.tree.node(v).name.starts_with(BRACE_OPEN_TEXT) && present_children(mind, v).into_iter().any(named)))
}
because!(denied_value, WordReading, "whether the story denied a value of a thing, written under its relation with a count of none, a \
     square is not round, so a check of it says no whatever the seeds hold");

fn denied_through(mind: &CursorMind, thing: usize, value: &str, depth: usize) -> bool {
    if denied_value(mind, thing, value) {
        return true;
    }
    let Some(is) = own_child(mind, thing, IS_FORM.trim()).filter(|_| depth > 0) else { return false };
    present_children(mind, is).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0).any(|c| class_things(mind, c).into_iter().filter(|&t| mind.tree.story(t) && t != thing).any(|t| denied_through(mind, t, value, depth - 1)))
}
because!(denied_through, WordReading, "whether the story denied a value of a thing or of a class of the story the thing is, a square is \
     not round and the box is a square, as far as the class depth");

pub(super) fn holds_value(mind: &CursorMind, thing: usize, value: &str) -> bool {
    if denied_through(mind, thing, value, CLASS_DEPTH) {
        return false;
    }
    let named = |n: usize| { let name = &*mind.tree.node(n).name; *name == *value || *name == *singular(value) };
    let related = present_children(mind, thing).into_iter().filter(|&r| { let name = &*mind.tree.node(r).name; name.starts_with(BRACE_OPEN_TEXT) && !super::mind::DIRECTIONS.contains(&crate::cursor::bare_name(name).as_str()) && !super::mind::SYMMETRIC_ROLES.contains(&crate::cursor::bare_name(name).as_str()) && *name != *IS_FORM.trim() && *name != *TIME_RELATION && *name != *OWNER_TAG && *name != *GENDER_TAG && !name.starts_with(crate::cursor::QUANTITY_TAG) }).flat_map(|r| present_children(mind, r)).any(named);
    let of_kind = own_child(mind, thing, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().filter(|&k| mind.tree.node(k).name.starts_with(BRACE_OPEN_TEXT) && *mind.tree.node(k).name != *TIME_RELATION).flat_map(|k| present_children(mind, k)).any(|v| named(v) && count_of(mind, v) > 0));
    let describes = own_child(mind, thing, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)).flat_map(|c| present_children(mind, c)).any(|q| named(q) && mind.tree.story(q)));
    let tagged = describes || own_child(mind, thing, &step_item(value)).is_some_and(|tag| present_children(mind, tag).is_empty());
    let seeds_tell = !(mind.tree.story(thing) && kind_of(mind, value).is_some_and(|kind| super::moves::KINDS.contains(&&*kind)));
    related || of_kind || tagged || holds_through(mind, thing, value, CLASS_DEPTH, seeds_tell) || compared_through(mind, thing, value, CLASS_DEPTH, None)
}

pub(super) fn told_of(mind: &CursorMind, n: usize) -> bool {
    present_children(mind, n).into_iter().any(|r| { let child = &*mind.tree.node(r).name; child.starts_with(BRACE_OPEN_TEXT) && *child != *TIME_RELATION && *child != *OWNER_TAG && *child != *GENDER_TAG && !child.starts_with(crate::cursor::QUANTITY_TAG) && *child != *step_item(FLAG_DEFINITE) && *child != *step_item(FLAG_INDEFINITE) })
}
because!(told_of, WordReading, "whether the story told a fact of a node, a relation of its own beyond its time, count, owner, gender and \
     article, the cat that sees the dog");

pub(super) fn order_step(mind: &CursorMind, name: &str, forward: bool) -> Option<String> {
    let relation = step_item(super::mind::ORDER_RELATION);
    if forward {
        let holder = mind.tree.named(name).find(|&n| n != 0 && !mind.tree.node(n).gone && own_child(mind, n, &relation).is_some())?;
        return own_child(mind, holder, &relation).and_then(|r| present_children(mind, r).into_iter().next()).map(|v| crate::cursor::bare_name(&mind.tree.node(v).name));
    }
    mind.tree.named(name).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *relation).map(|v| mind.tree.node(mind.tree.node(v).parent).parent).find(|&t| t != 0).map(|t| crate::cursor::bare_name(&mind.tree.node(t).name))
}
because!(order_step, WordReading, "the member one step after a member of an order the seeds chain, or one step before it, monday to \
     tuesday or back to sunday");

pub(super) fn ordered_told(mind: &CursorMind) -> Option<(usize, String)> {
    let relation = step_item(super::mind::ORDER_RELATION);
    let clocked = story_nodes_all(mind).into_iter().find_map(|n| {
        let time = own_child(mind, n, TIME_RELATION)?;
        let hour = present_children(mind, time).into_iter().find_map(|v| present_children(mind, v).into_iter().find_map(|q| mind.tree.node(q).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(q).name)).flatten()))?;
        Some((n, crate::cursor::worked_text(hour, None)))
    });
    if clocked.is_some() {
        return clocked;
    }
    story_nodes_all(mind).into_iter().find_map(|n| {
        let is = own_child(mind, n, IS_FORM.trim())?;
        let value = present_children(mind, is).into_iter().map(|v| crate::cursor::bare_name(&mind.tree.node(v).name)).find(|name| mind.tree.named(name).any(|m| m != 0 && !mind.tree.node(m).gone && own_child(mind, m, &relation).is_some()))?;
        Some((n, value))
    })
}
because!(ordered_told, WordReading, "the newest thing of the story told to be a member of an order, today is monday, with that member, or \
     the thing whose time is an hour of the clock with that hour, since the seeds ring the hours from twelve round to twelve");

pub fn shifted_value(mind: &CursorMind, found: Option<usize>) -> Option<String> {
    let (told, value) = ordered_told(mind)?;
    let asked: Vec<String> = open_question(mind).map(|q| present_children(mind, q).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect()).unwrap_or_default();
    let counted = asked_number(mind).map(|n| n as usize);
    let (steps, forward) = match counted {
        Some(n) => (n, !asked.iter().any(|a| a == super::mind::AGO || a == super::mind::EARLIER)),
        None if asked.iter().any(|a| *a == super::mind::RUN_ASKED[0]) => (1, true),
        None => {
            let from = mind.tree.node(told).name.to_string();
            let Some(to) = found.map(|f| mind.tree.node(f).name.to_string()) else { return Some(value) };
            let ahead = (0..=CLASS_DEPTH).scan(Some(from.clone()), |now, _| { let here = now.clone(); *now = now.as_ref().and_then(|name| order_step(mind, name, true)); here }).position(|name| name == to);
            let behind = (0..=CLASS_DEPTH).scan(Some(from), |now, _| { let here = now.clone(); *now = now.as_ref().and_then(|name| order_step(mind, name, false)); here }).position(|name| name == to);
            match (ahead, behind) {
                (Some(a), _) => (a, true),
                (None, Some(b)) => (b, false),
                _ => return None,
            }
        }
    };
    (0..steps).try_fold(value, |now, _| order_step(mind, &now, forward))
}
because!(shifted_value, WordReading, "the member of an order a question asks for from the one the story told, the one after it when the \
     question says next: as many steps on as the question counts, or back with ago, what day will it be in three days, or as far as the \
     word found stands from the word the story told it of, tomorrow from today, today from yesterday");

pub(super) fn grandparent(mind: &CursorMind, at: usize, word: &str) -> Option<usize> {
    let wanted = step_item(word.strip_prefix(super::mind::GRAND)?);
    super::mind::PARENTS.iter().filter_map(|p| own_child(mind, at, &step_item(p))).flat_map(|r| present_children(mind, r)).filter_map(|m| mind.tree.node(m).link.or_else(|| story_node(mind, &mind.tree.node(m).name.to_string()))).find_map(|parent| own_child(mind, parent, &wanted).and_then(|r| present_children(mind, r).into_iter().next()))
}
because!(grandparent, WordReading, "the grandmother or the grandfather of a thing, the mother or the father of one of its parents, read \
     through the parent's own relation");

struct Expression<'a> {
    mind: &'a CursorMind,
    tokens: Vec<String>,
    at: usize,
    unknown: Option<(String, f32)>,
}
because!(Expression, WordReading, "an expression being worked out: the mind its names are read from, its tokens in the order said and the \
     place reached");

fn function_value(name: &str, inner: f32) -> Option<f32> {
    match name {
        "sqrt" if inner >= f32::default() => Some(inner.sqrt()),
        "abs" => Some(inner.abs()),
        "sin" => Some(inner.sin()),
        "cos" => Some(inner.cos()),
        "ln" if inner > f32::default() => Some(inner.ln()),
        _ => None,
    }
}
because!(function_value, WordReading, "what a function of arithmetic makes of a number, or none outside where it is defined, the root of a \
     number below nothing");

pub fn defined_name(mind: &CursorMind, name: &str) -> bool {
    story_node(mind, name).is_some_and(|n| own_child(mind, n, &step_item(super::mind::MEANS)).is_some())
}
because!(defined_name, WordReading, "whether the story defined a name by what it means, so an expression runs it as a function");

fn defined_value(mind: &CursorMind, name: &str, inner: f32, depth: usize) -> Option<f32> {
    let node = story_node(mind, name)?;
    let meaning = own_child(mind, node, &step_item(super::mind::MEANS)).and_then(|m| present_children(mind, m).into_iter().next()).map(|v| mind.tree.node(v).name.to_string())?;
    let base = if super::mind::FUNCTIONS.contains(&meaning.as_str()) { function_value(&meaning, inner)? } else if depth > 0 { defined_value(mind, &meaning, inner, depth - 1)? } else { return None };
    let worked = present_children(mind, node).into_iter().filter_map(|r| super::mind::operation_of(&crate::cursor::bare_name(&mind.tree.node(r).name)).zip(present_children(mind, r).into_iter().find_map(|v| number_of(&mind.tree.node(v).name)))).fold(Some(base), |value, (op, by)| match (value, op) {
        (Some(v), "+") => Some(v + by),
        (Some(v), "-") => Some(v - by),
        (Some(v), "*") => Some(v * by),
        (Some(v), "/") if by.is_normal() => Some(v / by),
        _ => None,
    });
    worked
}
because!(defined_value, WordReading, "what a defined name makes of a number: the function it means, or the defined name it means run \
     first, then each operation the definition carries with its number, tk means hj times three");

impl Expression<'_> {
    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.at).map(String::as_str)
    }

    fn sum(&mut self) -> Option<f32> {
        let mut value = self.product()?;
        while let Some(op) = self.peek().and_then(super::mind::operation_of).filter(|op| *op == "+" || *op == "-") {
            self.at += 1;
            let next = self.product()?;
            value = if op == "+" { value + next } else { value - next };
        }
        Some(value)
    }

    fn product(&mut self) -> Option<f32> {
        let mut value = self.power()?;
        while let Some(op) = self.peek().and_then(super::mind::operation_of).filter(|op| *op == "*" || *op == "/") {
            self.at += 1;
            let next = self.power()?;
            value = if op == "*" { value * next } else if next.is_normal() || value.is_normal() { value / next } else { return None };
        }
        Some(value)
    }

    fn power(&mut self) -> Option<f32> {
        let base = self.operand()?;
        if self.peek() == Some(super::mind::POWER_SIGN) {
            self.at += 1;
            return self.power().map(|exponent| base.powf(exponent));
        }
        Some(base)
    }

    fn operand(&mut self) -> Option<f32> {
        let token = self.peek()?.to_string();
        self.at += 1;
        if token == super::mind::BRACKET_OPENS {
            let inner = self.sum()?;
            if self.peek() == Some(super::mind::BRACKET_CLOSES) {
                self.at += 1;
            }
            return Some(inner);
        }
        if super::mind::FUNCTIONS.contains(&token.as_str()) {
            let inner = self.operand()?;
            return function_value(&token, inner);
        }
        if defined_name(self.mind, &token) {
            let inner = self.operand()?;
            return defined_value(self.mind, &token, inner, CLASS_DEPTH);
        }
        if super::mind::operation_of(&token) == Some("-") {
            return self.operand().map(|v| -v);
        }
        number_of(&token).or_else(|| named_result(self.mind, &token)).or_else(|| self.unknown.as_ref().filter(|(name, _)| *name == token).map(|(_, value)| *value))
    }
}

pub fn named_result(mind: &CursorMind, name: &str) -> Option<f32> {
    let thing = story_node(mind, name)?;
    let is = own_child(mind, thing, IS_FORM.trim())?;
    present_children(mind, is).into_iter().find_map(|v| number_of(&mind.tree.node(v).name))
}
because!(named_result, WordReading, "the number a name was given as the result of a sum, two plus three equals result");

pub(super) fn equation_solved(mind: &CursorMind) -> Option<(String, f32)> {
    let question = open_question(mind)?;
    let form = mind.tree.node(question).name.to_string();
    let mut said: Vec<String> = present_children(mind, question).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect();
    if super::mind::operation_of(&form).is_some() && number_of(&form).is_none() && !super::mind::unknown_letter(&form) || form == super::mind::POWER_SIGN {
        said.insert(said.len().min(1), form);
    } else {
        said.insert(0, form);
    }
    let equals = said.iter().position(|t| t == super::mind::EQUAL_SIGN)?;
    let letter = said.iter().enumerate().find(|(i, t)| super::mind::unknown_letter(t) && named_result(mind, t).is_none() && !(super::mind::operation_of(t).is_some() && *i > 0 && *i + 1 < said.len() && number_of(&said[*i - 1]).is_some() && number_of(&said[*i + 1]).is_some())).map(|(_, t)| t.clone())?;
    let side = |tokens: &[String], guess: f32| -> Option<f32> {
        let mut expression = Expression { mind, tokens: tokens.to_vec(), at: 0, unknown: Some((letter.clone(), guess)) };
        let value = expression.sum()?;
        (expression.at == expression.tokens.len()).then_some(value)
    };
    let gap = |guess: f32| Some(side(&said[..equals], guess)? - side(&said[equals + 1..], guess)?);
    let straight = gap(f32::default()).zip(gap(f32::from(u8::from(true)))).filter(|(zero, one)| (one - zero).is_normal()).map(|(zero, one)| -zero / (one - zero)).filter(|&u| gap(u).is_some_and(|g| g.abs() < super::mind::SOLVE_NEAR));
    let whole = || (0..=super::mind::SOLVE_SPAN).map(|n| n as f32).find(|&u| gap(u).is_some_and(|g| g.abs() < super::mind::SOLVE_NEAR));
    straight.or_else(whole).map(|value| (letter, value))
}
because!(equation_solved, WordReading, "the unknown of an equation and what it is worth: the one letter of the open question no sum has \
     named, found from the two sides of the equals sign as a straight line through two guesses, or among the whole numbers when the line \
     does not fit, x squared equals nine");

pub(super) fn expression_value(mind: &CursorMind) -> Option<f32> {
    let question = open_question(mind)?;
    let form = mind.tree.node(question).name.to_string();
    let mut said: Vec<String> = present_children(mind, question).into_iter().map(|c| mind.tree.node(c).name.to_string()).collect();
    if (super::mind::operation_of(&form).is_some() || form == super::mind::POWER_SIGN) && said.len() > 1 {
        said.insert(1, form);
    } else {
        said.insert(0, form);
    }
    let carried = said.iter().find(|t| number_of(t).is_some() || super::mind::operation_of(t).is_some()).is_some_and(|t| super::mind::operation_of(t).is_some() && number_of(t).is_none() && named_result(mind, t).is_none()) && said.iter().filter(|t| number_of(t).is_some()).count() == 1;
    let kept = |t: &String| number_of(t).is_some() || super::mind::operation_of(t).is_some() || super::mind::expression_word(t) || named_result(mind, t).is_some() || defined_name(mind, t);
    let mut tokens: Vec<String> = said.into_iter().filter(kept).collect();
    if let Some(last) = mind.number.filter(|_| carried) {
        tokens.insert(0, crate::cursor::worked_text(last, None));
    } else if !tokens.iter().any(|t| super::mind::expression_word(t)) && !tokens.iter().any(|t| named_result(mind, t).is_some() || defined_name(mind, t)) && tokens.iter().filter(|t| super::mind::operation_of(t).is_some()).count() <= 1 {
        return None;
    }
    let opened = tokens.iter().filter(|t| *t == super::mind::BRACKET_OPENS).count();
    let closed = tokens.iter().filter(|t| *t == super::mind::BRACKET_CLOSES).count();
    for _ in opened..closed {
        tokens.insert(0, super::mind::BRACKET_OPENS.to_string());
    }
    let mut expression = Expression { mind, tokens, at: 0, unknown: None };
    let value = expression.sum()?;
    (expression.at == expression.tokens.len()).then_some(value)
}
because!(expression_value, WordReading, "what the open question works out to as an expression, when it holds a bracket, a power, a \
     function, a named result or several signs: its numbers, signs and names in the order said, the sign that opened it put back after its \
     first number and a bracket said before it opened put back in front, worked by the rules of arithmetic, a product before a sum and a \
     bracket first; one that opens with a sign carries on from the number last said, divided by two");

pub(super) fn asked_numbers(mind: &CursorMind) -> Vec<f32> {
    open_question(mind).map(|q| present_children(mind, q).into_iter().filter_map(|c| is_number(mind, &mind.tree.node(c).name).and_then(|n| number_of(&n))).collect()).unwrap_or_default()
}
because!(asked_numbers, WordReading, "every number the open question holds, in the order said, for a mean over how many numbers were said");

pub(super) fn asked_number(mind: &CursorMind) -> Option<f32> {
    let question = open_question(mind)?;
    present_children(mind, question).into_iter().find_map(|c| is_number(mind, &mind.tree.node(c).name).and_then(|n| number_of(&n)))
}
because!(asked_number, WordReading, "the first number the open question holds, for a question about a number the world has no node of, \
     what number comes after ninety-nine");

pub(super) fn relation_named(mind: &CursorMind, word: &str) -> String {
    if word == super::mind::DONE_ASKED {
        return step_item(super::mind::ACTIVITY);
    }
    if word == super::mind::LATER || word == super::mind::EARLIER {
        return step_item(super::mind::ORDER_RELATION);
    }
    step_item(&super::mind::compared_relation(word).filter(|_| verb_base(mind, word).is_none() && !quality_word(mind, word) && !super::mind::role_word(mind, word)).unwrap_or_else(|| super::mind::verb_stem(mind, word)))
}
because!(relation_named, WordReading, "the relation a word of a question names: a comparison with than after it, taller as tallerthan, or \
     else the stem of the verb, liked as like, the order relation for after and before, and the activity for do");

pub(super) fn related_chain(mind: &CursorMind, thing: usize, relation: &str, value: &str, depth: usize) -> bool {
    let Some(r) = own_child(mind, thing, relation) else { return false };
    present_children(mind, r).into_iter().any(|v| {
        let name = mind.tree.node(v).name.to_string();
        name == value || (depth > 0 && mind.tree.node(v).link.into_iter().chain(story_nodes(mind, &name)).filter(|&t| t != thing).any(|t| related_chain(mind, t, relation, value, depth - 1)))
    })
}
because!(related_chain, WordReading, "whether a thing reaches a value through a chain of one relation, the box left of the chest and the \
     chest left of the bed, followed as far as the class depth");

pub(super) fn measured_by(mind: &CursorMind, n: usize, measure: &str) -> Option<f32> {
    own_child(mind, n, &step_item(measure)).into_iter().flat_map(|r| present_children(mind, r)).find_map(|q| number_of(&mind.tree.node(q).name).or_else(|| present_children(mind, q).into_iter().find_map(|t| mind.tree.node(t).name.starts_with(crate::cursor::QUANTITY_TAG).then(|| number_of(&mind.tree.node(t).name)).flatten())))
}
because!(measured_by, WordReading, "how much of a measure a thing has, the count under the measure's relation, tom's five years under old");

pub fn measure_asked(mind: &CursorMind, word: &str) -> Option<(String, bool)> {
    let top = step_item(super::mind::SUPERLATIVE_RELATION);
    let comparison = mind.tree.named(word).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *top).map(|v| mind.tree.node(mind.tree.node(mind.tree.node(v).parent).parent).name.to_string()).next().unwrap_or_else(|| word.to_string());
    if let Some((_, most)) = super::mind::WEIGHT_COMPARISONS.iter().find(|(c, _)| *c == word) {
        return Some((super::mind::WEIGHING.to_string(), *most));
    }
    let measure = |c: &str| c.strip_suffix(super::mind::COMPARISON_END).filter(|m| super::mind::MEASURES.contains(m)).map(str::to_string);
    let told = |m: &String| (mind.tree.state..mind.tree.len()).any(|n| !mind.tree.node(n).gone && *mind.tree.node(n).name == *step_item(m));
    let least: Vec<String> = opposites(mind, &comparison).iter().filter_map(|o| measure(o)).collect();
    measure(&comparison).map(|m| (m, true)).or_else(|| least.iter().find(|m| told(m)).or(least.first()).cloned().map(|m| (m, false)))
}
because!(measure_asked, WordReading, "the measure a comparison or a superlative asks by and whether it asks the most of it, a comparison \
     of weight by what a thing weighs, and of several opposites the one whose measure the story told, shorter the least of long after a \
     rope five meters long: older and oldest ask the most of old, younger and youngest, through the seeds' opposite, the least");

pub(super) fn opposites(mind: &CursorMind, word: &str) -> Vec<String> {
    let opposite = step_item(super::mind::OPPOSITE_WORD);
    let forward = mind.tree.named(word).filter(|&n| !mind.tree.node(n).gone).filter_map(|n| own_child(mind, n, &opposite)).flat_map(|r| present_children(mind, r)).map(|v| mind.tree.node(v).name.to_string());
    let backward = mind.tree.named(word).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *opposite).map(|v| mind.tree.node(mind.tree.node(mind.tree.node(v).parent).parent).name.to_string());
    let mut all: Vec<String> = forward.chain(backward).filter(|o| o != word).collect();
    all.dedup();
    all
}
because!(opposites, WordReading, "the words the seeds give as the opposite of a word, either way round, taller and shorter, so a question \
     asked by one comparison is read on the other");

pub(super) fn compared_through(mind: &CursorMind, thing: usize, value: &str, depth: usize, only: Option<&str>) -> bool {
    let comparison = format!("{}{}", super::mind::COMPARISON_END, super::mind::COMPARED);
    let relations: Vec<usize> = present_children(mind, thing).into_iter().filter(|&r| crate::cursor::bare_name(&mind.tree.node(r).name).ends_with(&comparison) && mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT) && only.is_none_or(|o| *mind.tree.node(r).name == *o)).collect();
    relations.into_iter().any(|r| {
        let relation = mind.tree.node(r).name.to_string();
        present_children(mind, r).into_iter().any(|v| {
            let name = mind.tree.node(v).name.to_string();
            if name == value || name == singular(value) {
                return true;
            }
            depth > 0 && mind.tree.node(v).link.into_iter().chain(story_nodes(mind, &name)).filter(|&t| t != thing).any(|t| own_child(mind, t, &relation).is_some() && compared_through(mind, t, value, depth - 1, Some(&relation)))
        })
    })
}
because!(compared_through, WordReading, "whether a thing holds a value through a chain of the same comparison, ann is taller than tom and \
     tom is taller than sam, so ann is taller than sam, followed as far as the class depth");
because!(holds_value, WordReading, "whether a thing holds a value, under one of its own relations but a direction or a relation between \
     words, the opposite of hot, the box is bigger than the bag, under a kind of what it is and not denied, made of atoms, or directly or \
     through what it is");

pub(super) fn class_things(mind: &CursorMind, class: usize) -> Vec<usize> {
    let name = mind.tree.node(class).name.to_string();
    let plurals = story_nodes_all(mind).into_iter().filter(|&n| n != class && *mind.tree.node(n).name != *name && singular(&mind.tree.node(n).name) == name);
    let one = |n: &usize| mind.tree.story(*n) && own_child(mind, *n, &step_item(FLAG_DEFINITE)).is_some();
    mind.tree.node(class).link.into_iter().chain(mind.tree.named(&name).filter(|&n| n != class && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0)).chain(plurals).filter(|n| !one(n)).collect()
}
because!(class_things, WordReading, "the things a class value stands for: the thing it links to, the things of its name under the world, \
     and the story's things named by its plural, mice for a mouse, but no thing the story told with the, which is one of its kind and no \
     kind");

pub(super) fn induced(mind: &CursorMind, thing: usize, holds: impl Fn(usize) -> bool) -> Option<usize> {
    let is = own_child(mind, thing, IS_FORM.trim())?;
    let classes: Vec<String> = present_children(mind, is).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)).map(|c| mind.tree.node(c).name.to_string()).collect();
    story_nodes_all(mind).into_iter().filter(|&n| n != thing).filter(|&n| own_child(mind, n, IS_FORM.trim()).is_some_and(|other| present_children(mind, other).into_iter().any(|c| classes.iter().any(|k| **k == *mind.tree.node(c).name)))).find(|&n| holds(n))
}
because!(induced, WordReading, "another thing of the story that is what the thing is and holds what is asked, so what is told of one lion \
     is taken for the next, brian is white as bernhard is");

pub(super) fn holds_through(mind: &CursorMind, thing: usize, value: &str, depth: usize, seeds_tell: bool) -> bool {
    if !seeds_tell && !mind.tree.story(thing) {
        return false;
    }
    if holds_directly(mind, thing, value) {
        return true;
    }
    let told_class = own_child(mind, thing, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().any(|c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)));
    if depth > 0 && mind.tree.story(thing) && !told_class && mind.tree.named(&mind.tree.node(thing).name.to_string()).filter(|&n| n != thing && n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0).any(|seeded| holds_through(mind, seeded, value, depth - 1, seeds_tell)) {
        return true;
    }
    if depth > 0 && own_child(mind, thing, IS_FORM.trim()).is_some_and(|is| present_children(mind, is).into_iter().filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0).any(|c| class_things(mind, c).into_iter().any(|t| mind.tree.story(t) && related_value(mind, t, value)))) {
        return true;
    }
    let Some(is) = own_child(mind, thing, IS_FORM.trim()).filter(|_| depth > 0) else { return false };
    let of_kinds: Vec<usize> = present_children(mind, is).into_iter().filter(|&k| mind.tree.node(k).name.starts_with(BRACE_OPEN_TEXT) && *mind.tree.node(k).name != *TIME_RELATION).flat_map(|k| present_children(mind, k)).collect();
    present_children(mind, is).into_iter().chain(of_kinds).filter(|&c| !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, c) > 0).any(|c| {
        let name = mind.tree.node(c).name.to_string();
        mind.tree.node(c).link.into_iter().chain(mind.tree.named(&name).filter(|&n| n != c && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0)).any(|t| holds_through(mind, t, value, depth - 1, seeds_tell))
    })
}
because!(holds_through, WordReading, "whether a thing holds a value, a node of the seeds never telling it when the caller says the seeds \
     do not tell, as for a quality of a kind asked of a thing of the story, so an apple the story made green is not red by the seeds; or \
     what it is holds it, a thing the story told no class of being what the seeds say a thing of its name is, kim's dog a pet, a class \
     written under a kind counting as one, gold the metal, followed as far as the class depth: a kitten is a cat and a cat is an animal, \
     so a kitten is an animal");

pub fn map_route(mind: &CursorMind, from: &str, to: &str) -> Option<Vec<String>> {
    let mut edges: Vec<(String, String, String)> = Vec::new();
    for n in (mind.tree.state..mind.tree.len()).filter(|&n| !mind.tree.node(n).gone && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT)) {
        for r in present_children(mind, n).into_iter().filter(|&r| super::mind::DIRECTIONS.contains(&crate::cursor::bare_name(&mind.tree.node(r).name).as_str())) {
            let way = crate::cursor::bare_name(&mind.tree.node(r).name);
            for v in present_children(mind, r) {
                let (here, there) = (mind.tree.node(n).name.to_string(), mind.tree.node(v).name.to_string());
                for back in opposites(mind, &way) {
                    edges.push((here.clone(), back, there.clone()));
                }
                edges.push((there, way.clone(), here));
            }
        }
    }
    let mut reached: Vec<(String, Vec<String>)> = vec![(from.to_string(), Vec::new())];
    let mut next = 0;
    while next < reached.len() && reached.len() <= ORDER_SPAN {
        let (place, steps) = reached[next].clone();
        if place == to {
            return (!steps.is_empty()).then_some(steps);
        }
        for (start, way, end) in edges.iter().filter(|(start, _, _)| *start == place) {
            let _ = start;
            if !reached.iter().any(|(seen, _)| seen == end) {
                let mut longer = steps.clone();
                longer.push(way.clone());
                reached.push((end.clone(), longer));
            }
        }
        next += 1;
    }
    None
}
because!(map_route, WordReading, "the steps from one place to another over the map the story drew with directions: a place told north of \
     another is a step north from it and a step south back, searched breadth first so the shortest route is found, as far as the order \
     span");

pub(super) fn chain_end(mind: &CursorMind, word: &str) -> Option<String> {
    let top = step_item(super::mind::SUPERLATIVE_RELATION);
    let comparison = mind.tree.named(word).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == *top).map(|v| mind.tree.node(mind.tree.node(mind.tree.node(v).parent).parent).name.to_string()).next()?;
    let own = super::mind::compared_relation(&comparison).map(|r| step_item(&r))?;
    let turned: Vec<String> = opposites(mind, &comparison).into_iter().filter_map(|o| super::mind::compared_relation(&o)).map(|o| step_item(&o)).collect();
    let under = |n: usize, relation: &str| mind.tree.named(&mind.tree.node(n).name.to_string()).any(|v| !mind.tree.node(v).gone && mind.tree.story(v) && *mind.tree.node(mind.tree.node(v).parent).name == *relation);
    let first = story_nodes_all(mind).into_iter().find(|&n| own_child(mind, n, &own).is_some() && !under(n, &own));
    let ends_chain = |v: usize, r: &str| !mind.tree.named(&mind.tree.node(v).name.to_string()).any(|n| !mind.tree.node(n).gone && own_child(mind, n, r).is_some());
    let last = || turned.iter().find_map(|r| (mind.tree.state..mind.tree.len()).find(|&v| !mind.tree.node(v).gone && *mind.tree.node(mind.tree.node(v).parent).name == **r && ends_chain(v, r)));
    first.or_else(last).map(|n| mind.tree.node(n).name.to_string())
}
because!(chain_end, WordReading, "the thing at the end of a chain a superlative asks: the comparison the seeds give the superlative as the \
     top of, the first thing of the story that compares so and is compared to by none, or, told by the opposite comparison, the value \
     under it that no thing of its name goes on from");

pub(super) fn motives(mind: &CursorMind, person: usize) -> Vec<(String, String)> {
    let states: Vec<String> = own_child(mind, person, IS_FORM.trim()).into_iter().flat_map(|is| present_children(mind, is)).flat_map(|k| if mind.tree.node(k).name.starts_with(BRACE_OPEN_TEXT) { present_children(mind, k) } else { vec![k] }).filter(|&v| !mind.tree.node(v).name.starts_with(BRACE_OPEN_TEXT) && count_of(mind, v) > 0).map(|v| mind.tree.node(v).name.to_string()).collect();
    states.into_iter().filter_map(|state| mind.tree.named(&state).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0).find_map(|n| own_child(mind, n, &step_item(super::mind::GOAL))).and_then(|goal| present_children(mind, goal).into_iter().next()).map(|place| (state, crate::cursor::bare_name(&mind.tree.node(place).name)))).collect()
}
because!(motives, WordReading, "the states of a person the seeds give a goal, each with the place it sends them, thirsty to the kitchen");

pub(super) fn group_pronoun(word: &str) -> bool {
    word == super::mind::THING_PRONOUNS[1] || super::mind::OWN_WORDS.iter().any(|(own, owner)| *own == word && *owner == super::mind::THING_PRONOUNS[1])
}
because!(group_pronoun, WordReading, "whether a word stands for a group, they or their");

pub(super) fn asks_having(mind: &CursorMind) -> bool {
    open_question(mind).is_some_and(|q| present_children(mind, q).into_iter().any(|c| HAVING.contains(&&*mind.tree.node(c).name)))
}
because!(asks_having, WordReading, "whether the open question asks with a word of having, which counts only what a thing holds or owns and \
     never the place around it");

fn holds_directly(mind: &CursorMind, thing: usize, value: &str) -> bool {
    let named = |n: usize| { let name = &*mind.tree.node(n).name; *name == *value || *name == *singular(value) };
    let mut above = mind.tree.node(thing).parent;
    let mut around = false;
    while above != 0 && !mind.tree.node(above).name.starts_with(BRACE_OPEN_TEXT) {
        around |= named(above);
        above = mind.tree.node(above).parent;
    }
    let mut inner = held_or_owned(mind, thing);
    let mut depth = 0;
    let mut found = false;
    while !inner.is_empty() && depth < CLASS_DEPTH && !found {
        found = inner.iter().any(|&c| named(c));
        inner = inner.into_iter().flat_map(|c| held_or_owned(mind, c)).collect();
        depth += 1;
    }
    let placed = present_children(mind, thing).into_iter().filter(|&r| mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT) && place_word(&crate::cursor::bare_name(&mind.tree.node(r).name))).flat_map(|r| present_children(mind, r)).any(|p| named(p));
    if found || (around && !asks_having(mind)) || placed {
        return true;
    }
    if own_child(mind, thing, super::mind::DEED_TAG).is_some_and(|d| present_children(mind, d).into_iter().any(|c| named(c))) {
        return true;
    }
    let Some(is) = own_child(mind, thing, IS_FORM.trim()) else { return false };
    let under = match kind_of(mind, value) {
        Some(kind) => own_child(mind, is, &step_item(&kind)),
        None => Some(is),
    };
    under.is_some_and(|u| present_children(mind, u).into_iter().any(|c| named(c))) || present_children(mind, is).into_iter().any(|c| named(c) && count_of(mind, c) > 0)
}
because!(holds_directly, WordReading, "whether a thing holds a value itself: a thing of that name, or its singular, stands inside it or \
     inside what it holds, as the coin in the bag tom has, or is a place it stands in, or is under its place relation as the seeds write \
     it, berlin in germany, or is a deed it did, the shed the box of the ball stands in too, or the value stands under its is, under the \
     kind the seeds class it by when they class it");

pub(super) fn time_of_day(mind: &CursorMind, n: usize) -> Option<String> {
    let time = own_child(mind, n, TIME_RELATION)?;
    present_children(mind, time).into_iter().map(|v| mind.tree.node(v).name.to_string()).find(|v| super::mind::TIMES_OF_DAY.contains(&v.as_str()))
}
because!(time_of_day, WordReading, "the time of day a thing was placed at, read under its time relation, or none for a move told with no \
     time");
