use super::mind::CursorMind;
use super::tree::{child_named, quoted};
use super::{CursorTree, LATER_TIME, NODE_KINDS, PAST_TIME, PATH_MARK, STATEMENT_NODES, TIME_RELATION};
use crate::quiz::{BRACE_CLOSE, BRACE_OPEN, IN_FORM, IS_FORM, NOT_PREFIX, OWNS_FORM, WORTH_FORM};
use crate::words::number_of;
use patterns::{because, source};

pub fn statement_parts(statement: &str) -> Option<(String, String, String)> {
    let formed = [IN_FORM, OWNS_FORM, IS_FORM, WORTH_FORM].iter().find_map(|form| statement.split_once(form).map(|(older, newer)| (older.trim().to_string(), form.trim().to_string(), newer.trim().to_string())));
    formed.or_else(|| match statement.split_whitespace().collect::<Vec<_>>().as_slice() {
        [older, relation, newer] => Some((older.to_string(), relation.to_string(), newer.to_string())),
        _ => None,
    })
}
because!(statement_parts, CursorTree, "an expected statement of one relation split into the three names it takes in the tree: the older \
     name, the relation as its form writes it, and the newer name; a statement of three words in no form, as tom likes ann, takes its \
     middle word as the relation");

pub struct ConceptSeeds;
source!(ConceptSeeds, "the user's design of the seeds: they talk in concepts and hold no English, every node a braced concept, a number \
     written as the number concept, a conversion two bracketed amounts joined by equal, and the English side apart, each word to equal to \
     its concept");

pub const EQUAL_RELATION: &str = "{equal}";
because!(EQUAL_RELATION, ConceptSeeds, "the relation that says two nodes stand for one thing: an English word and its concept, a number \
     word and its number, one amount of a unit and the amount of another unit it equals");

pub const NUMBER_MARK: &str = "{number ";
because!(NUMBER_MARK, ConceptSeeds, "the opening of a number written as a concept in a statement, which stands for the node of that \
     number, since the seeds and the lessons talk in concepts and a number is one");

pub fn number_name(name: &str) -> String {
    if let Some(value) = name.strip_prefix(NUMBER_VALUE).and_then(|rest| rest.strip_suffix(PROPERTY_END)) {
        return value.trim().to_string();
    }
    name.strip_prefix(NUMBER_MARK).or_else(|| name.strip_prefix(super::QUANTITY_TAG)).and_then(|rest| rest.strip_suffix(BRACE_CLOSE)).map_or_else(|| name.to_string(), |n| n.trim().to_string())
}

pub const NUMBER_VALUE: &str = "{number:value(";
because!(NUMBER_VALUE, ConceptSeeds, "the opening of a number written with its value as a property, the user's form, which stands for the \
     node of that number");

pub const QUANTITY_PROPERTY: &str = ":quantity(";
because!(QUANTITY_PROPERTY, ConceptSeeds, "the quantity written as a property on the object counted, apple with quantity five, which the \
     tree holds as the object with its quantity beneath");

const PROPERTY_END: &str = ")}";
because!(PROPERTY_END, ConceptSeeds, "what closes a property inside a tag");

pub const TIME_PROPERTY: &str = ":time(";
because!(TIME_PROPERTY, ConceptSeeds, "the time written as a property on the value it is the time of, kitchen with time past, which the \
     tree holds as the value with its time beneath, so a fact's time is attached to the thing it is a fact of, as its count is");

pub fn property_names(name: &str) -> Vec<String> {
    if let Some((object, rest)) = name.split_once(TIME_PROPERTY) {
        let value = rest.trim_end_matches(BRACE_CLOSE).trim_end_matches(PROPERTY_END.chars().next().unwrap_or_default());
        let time = if value == PAST_TIME.trim_matches(|c| c == BRACE_OPEN || c == BRACE_CLOSE) || value == LATER_TIME.trim_matches(|c| c == BRACE_OPEN || c == BRACE_CLOSE) { format!("{BRACE_OPEN}{value}{BRACE_CLOSE}") } else { value.to_string() };
        return property_names(object).into_iter().chain([TIME_RELATION.to_string(), time]).collect();
    }
    let (object, rest) = match name.split_once(QUANTITY_PROPERTY) {
        Some(split) => split,
        None => return vec![number_name(name)],
    };
    let value = rest.trim_end_matches(BRACE_CLOSE).trim_end_matches(PROPERTY_END.chars().next().unwrap_or_default()).to_string();
    let object = if name.starts_with(BRACE_OPEN) { format!("{object}{BRACE_CLOSE}") } else { object.to_string() };
    vec![object, super::QUANTITY.to_string(), value]
}
because!(property_names, ConceptSeeds, "the names a written name takes in the tree: a value with a time property, kitchen with time past, \
     is the value, the time relation and the time, past and later braced as the tree writes them and a named time as itself; an object \
     with a quantity property, an apple with its count or a braced leg relation with its count, is the object, the quantity relation and \
     the number, and any other name is itself with a number written as a concept read as the number");
because!(number_name, CursorTree, "the name a statement's name takes in the tree: the number itself for a number written as a concept, and \
     the name as written otherwise");

pub fn bracket_parts(statement: &str) -> Option<(Vec<String>, String, Vec<String>)> {
    if !statement.starts_with(BRACKET_OPEN) && statement.contains(QUANTITY_PROPERTY) && !statement.contains(PATH_MARK) {
        let (older, relation, newer) = statement_parts(statement)?;
        return (relation.starts_with(BRACE_OPEN) && older.contains(QUANTITY_PROPERTY) && newer.contains(QUANTITY_PROPERTY)).then(|| (property_names(&older), relation, property_names(&newer)));
    }
    let inner = statement.strip_prefix(BRACKET_OPEN)?.strip_suffix(BRACKET_CLOSE)?;
    let (older, rest) = inner.split_once(BRACKET_CLOSE)?;
    let (relation, newer) = rest.trim().split_once(BRACKET_OPEN)?;
    let relation = relation.trim();
    let path = |text: &str| statement_path(text.trim()).filter(|names| names.len() >= STATEMENT_NODES);
    (relation.starts_with(BRACE_OPEN) && relation.ends_with(BRACE_CLOSE)).then(|| Some((path(older)?, relation.to_string(), path(newer)?))).flatten()
}
because!(bracket_parts, ConceptSeeds, "a statement of two amounts joined by a relation, as one minute equals sixty seconds, each amount an \
     object with its quantity as a property, or the older form of two paths in brackets: the older path, the relation and the newer path, \
     which the tree holds as the end of each path linked to the end of the other by that relation, so a concept is explained by what it \
     equals and never by nesting one unit under another");

use super::plan::{BRACKET_CLOSE, BRACKET_OPEN};

pub fn statement_path(statement: &str) -> Option<Vec<String>> {
    if let Some(number) = statement.strip_prefix(NUMBER_MARK).and_then(|rest| rest.strip_suffix(BRACE_CLOSE)).filter(|_| !statement.contains(PATH_MARK)) {
        let number = number.trim().to_string();
        return Some(vec![number.clone(), WORTH_FORM.trim().to_string(), number]);
    }
    if statement.contains(PATH_MARK) {
        let names: Vec<String> = statement.split(PATH_MARK).flat_map(|name| property_names(name.trim())).flat_map(|name| if super::is_quantity_tag(&name) { vec![super::QUANTITY.to_string(), number_name(&name)] } else { vec![name] }).collect();
        return (names.len() >= super::BARE_STATEMENT_NODES && names.iter().all(|name| !name.is_empty())).then_some(names);
    }
    if statement.contains(QUANTITY_PROPERTY) && !statement.contains(PATH_MARK) {
        return None;
    }
    statement_parts(statement).map(|(older, relation, newer)| vec![number_name(&older), relation, number_name(&newer)])
}
because!(statement_path, CursorTree, "the names an expected statement takes in the tree, one under another: a path written with the path \
     mark names every node along it, a name and a relation in turn, a path of a thing and a bare relation counting too, a number written \
     as a concept taking its number's name; a statement of a number alone the number's worth of itself; any other statement its three \
     names");

pub(crate) fn value_place(place: usize) -> bool {
    place >= NODE_KINDS && place % NODE_KINDS == 0
}
because!(value_place, CursorTree, "whether a place along a path counts from its head is a value's: the head is a thing, then a relation \
     and a value take turns, so every even place after the head is a value that may name a thing under the world");

pub fn cursor_told(mind: &mut CursorMind, statement: &str, hidden: bool) -> bool {
    if let Some(banned) = statement.strip_prefix(NOT_PREFIX) {
        let Some(names) = statement_path(banned) else {
            return false;
        };
        let mut at = 0;
        for name in &names {
            let Some(next) = child_named(&mind.tree, at, name) else {
                return true;
            };
            at = next;
        }
        mind.tree.moved(at, None);
        return true;
    }
    if let Some((older, relation, newer)) = bracket_parts(statement) {
        let older_end = told_path(mind, &older, hidden);
        let newer_end = told_path(mind, &newer, hidden);
        for (from, to) in [(older_end, newer_end), (newer_end, older_end)] {
            let holder = child_named(&mind.tree, from, &relation).unwrap_or_else(|| mind.tree.added_as(from, &relation, hidden));
            if !mind.tree.node(holder).children.iter().any(|&c| mind.tree.node(c).link == Some(to)) {
                mind.tree.linked(holder, to, hidden);
            }
        }
        return true;
    }
    let Some(names) = statement_path(statement) else {
        return false;
    };
    let end = told_path(mind, &names, hidden);
    if names.len() == STATEMENT_NODES && names[1] == EQUAL_RELATION {
        let head = child_named(&mind.tree, 0, &names[0]).filter(|&h| mind.tree.node(h).link.is_none() && !mind.tree.story(h));
        let target = mind.tree.node(end).link.or_else(|| (mind.tree.node(end).parent != 0 && number_of(&names.last().map_or("", String::as_str)).is_some()).then(|| child_named(&mind.tree, 0, &names.last().map_or("", String::as_str))).flatten());
        if let Some(thing) = target.filter(|&thing| thing != end && mind.tree.node(end).link.is_none() && mind.tree.node(end).parent != 0) {
            mind.tree.changed(end, |node| node.link = Some(thing));
        }
        if let Some((word, thing)) = head.zip(target).filter(|&(word, thing)| word != thing) {
            mind.tree.changed(word, |node| node.link = Some(thing));
        }
    }
    true
}

fn told_path(mind: &mut CursorMind, names: &[String], hidden: bool) -> usize {
    let story = mind.tree.state > 0;
    let mut at = 0;
    let mut skip = false;
    for (place, name) in names.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        if *name == *super::QUANTITY {
            if let Some(count) = names.get(place + 1).and_then(|n| number_of(n)) {
                let tag = super::quantity_tag_named(count);
                at = child_named(&mind.tree, at, &tag).filter(|&c| !mind.tree.node(c).gone).unwrap_or_else(|| mind.tree.added_as(at, &tag, hidden));
                skip = true;
                continue;
            }
        }
        let target = (value_place(place) && !quoted(&mind.tree, at)).then(|| mind.tree.referred(name)).flatten();
        let child = child_named(&mind.tree, at, name).filter(|&c| mind.tree.node(c).link == target && (at != 0 || !story || mind.tree.story(c)));
        at = match (child, target) {
            (Some(c), Some(thing)) if mind.tree.node(c).link == Some(thing) => if story { c } else { thing },
            (Some(c), _) => c,
            (None, Some(thing)) => {
                let mention = mind.tree.linked(at, thing, hidden);
                if story { mention } else { thing }
            }
            (None, None) => {
                let node = mind.tree.added_as(at, name, hidden);
                if at == 0 {
                    mind.tree.adopted(node);
                    if let Some(kind) = super::tree::tag_kind(name).map(str::to_string) {
                        let holder = mind.tree.added_as(node, IS_FORM.trim(), hidden);
                        match super::tree::concept_named(&mind.tree, &kind) {
                            Some(concept) => mind.tree.linked(holder, concept, hidden),
                            None => mind.tree.added_as(holder, &kind, hidden),
                        };
                    }
                }
                node
            }
        };
    }
    at
}
because!(told_path, CursorTree, "one path of a statement written into the tree, each name under the one before it, a concept added under \
     the world with a kind in its tag holding that kind under is, as {mammal dog} holds mammal, linked to the kind's concept when the tree \
     holds it, so the tag is read as the concept's kind; and the node the path ends at");
because!(
    cursor_told,
    CursorTree,
    "a statement of a seeds file or a world written into the tree before a line is read: a word of the English side that equals a concept \
     or a number, cat to {cat} and zero to its number, is linked to what it equals, so the word is a mention of its concept and every read \
     through the word reaches the concept in one step; a bracket statement as its two paths with the end of each linked under the relation \
     to the end of the other, so what one minute equals is read from the minute and from the seconds alike; any other as its path, each \
     name under the one before it, found when the tree holds it and added when not, hidden when asked, as a world's nodes are, since the \
     network has not seen them, and seen otherwise, as a seeds file's are; a value that names a thing under the world is added as a link \
     to that thing and the path goes on from the thing itself, so a chain of successors is a cycle of the things, each holding its \
     successor, except under what a sayer said, where every value stays a plain value of the claim, and a thing added takes over the plain \
     values of its name and the story's mentions that linked to a thing the state held, since the dog tom and ann have is the story's dog \
     once the story states it; a world told beside a state, once the state is marked as what the story begins from, starts at a top node \
     the story told and never at one the state held, and its path goes on from the mention and never from the thing, since the kim who \
     owns a sock in a hidden world is not the novel the seeds call kim and the sock's basket is the story's fact and not the fact of every \
     sock; a statement that says not takes the newest node of its path out with everything below it, when the tree holds the path, so a \
     world can lose a fact as well as gain one; none for a statement of no shape the tree takes"
);

pub struct SeedsFiles;
source!(SeedsFiles, "the seeds a line may name: files of statements in the forms the tree holds, one a line, which people annotate with \
     comment lines the cursor never reads");

pub const SEED_COMMENT: &str = "#";
because!(SEED_COMMENT, SeedsFiles, "what starts a comment line in a seeds file");

pub fn seed_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with(SEED_COMMENT))
}
because!(seed_lines, SeedsFiles, "the statements of a seeds text, comment lines and blank lines left out");

pub fn told_seeds(mind: &mut CursorMind, text: &str) -> Result<usize, String> {
    let mut told = 0;
    for line in seed_lines(text) {
        if !cursor_told(mind, line, false) {
            return Err(format!("{line:?} is no statement the cursor can hold"));
        }
        told += 1;
    }
    Ok(told)
}
because!(told_seeds, SeedsFiles, "every statement of a seeds text written into a mind as a path, refused at the first statement the cursor \
     cannot hold, and how many it wrote");
