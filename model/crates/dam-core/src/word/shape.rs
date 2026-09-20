use crate::cursor::{bare_name, quantity_tag_named, CursorMind, NodeTree, PAST_TIME, PATH_MARK, TIME_RELATION};
use crate::quiz::{BRACE_CLOSE, BRACE_OPEN, NOT_PREFIX};
use crate::words::number_of;
use patterns::{because, source};

pub struct WorldShape;
source!(
    WorldShape,
    "the user's world: the tree is a space, a thing inside a place is a child of it, bag -> corn, and what a thing has stands inside it \
     too; a tag on a node, the article, a count or a time, is written after its name with a colon, cat:the(true), and the tree holds it as \
     a relation named by the tag with the value under it, the count as the quantity tag; a lesson expects paths of such names from the \
     world down, whatever else the tree holds"
);

const TAG_MARK: char = ':';
because!(TAG_MARK, WorldShape, "what parts a name from each tag written on it");

const TAG_VALUE_OPEN: char = '(';
const TAG_VALUE_CLOSE: char = ')';
because!(TAG_VALUE_OPEN, WorldShape, "what opens the value of a tag after its name");
because!(TAG_VALUE_CLOSE, WorldShape, "the bracket after a tag's value, where the name before it or the next tag begins when the text is \
     read from its end");

const QUANTITY_TAG_NAME: &str = "quantity";
because!(QUANTITY_TAG_NAME, WorldShape, "the tag that holds a count, held in the tree as the quantity tag node rather than a relation with \
     the number under it");

const TIME_TAG_NAME: &str = "time";
because!(TIME_TAG_NAME, WorldShape, "the tag that holds a time, whose value the tree writes braced, past as {past}");

pub struct ShapeName {
    pub name: String,
    pub tags: Vec<(String, String)>,
}
because!(ShapeName, WorldShape, "one node of an expected path: its name and the tags written on it, each a name and a value");

pub fn shape_name(text: &str) -> ShapeName {
    let text = text.trim();
    let mut tags = Vec::new();
    let mut end = text.len();
    while let Some(open) = text[..end].rfind(TAG_VALUE_OPEN) {
        let Some(mark) = text[..open].rfind(TAG_MARK) else { break };
        if text[..end].ends_with(TAG_VALUE_CLOSE) {
            let key = text[mark + 1..open].trim().to_string();
            let value = text[open + 1..end - 1].trim().to_string();
            tags.push((key, value));
            end = mark;
        } else {
            break;
        }
    }
    tags.reverse();
    ShapeName { name: text[..end].trim().to_string(), tags }
}
because!(shape_name, WorldShape, "a node of a path read from its text: the name before the first tag, and every tag after it in the order \
     written, house:a(true):time(past) being the house with a and time");

pub fn shape_path(statement: &str) -> (bool, Vec<ShapeName>) {
    let mut rest = statement.trim();
    let mut denied = false;
    while let Some(after) = rest.strip_prefix(NOT_PREFIX.trim()) {
        rest = after.trim_start();
        denied = !denied;
    }
    (denied, rest.split(PATH_MARK.trim()).map(shape_name).filter(|n| !n.name.is_empty()).collect())
}
because!(shape_path, WorldShape, "an expected statement read: whether it is denied by a not before it, and its nodes from the world down");

pub fn shape_names(expect: &[String]) -> Vec<String> {
    expect.iter().flat_map(|s| shape_path(s).1).filter(|n| !n.name.starts_with(BRACE_OPEN)).map(|n| n.name).collect()
}
because!(shape_names, WorldShape, "the names a lesson states as things and values, whose text is taken off the twin rows so a name never \
     seen reads as they do");

fn tag_holds(tree: &NodeTree, node: usize, (key, value): &(String, String)) -> bool {
    let children = || tree.node(node).children.iter().copied().filter(|&c| !tree.node(c).gone);
    if key == QUANTITY_TAG_NAME {
        return number_of(value).is_some_and(|count| children().any(|c| *tree.node(c).name == *quantity_tag_named(count)));
    }
    let relation = if key == TIME_TAG_NAME { TIME_RELATION.to_string() } else { format!("{BRACE_OPEN}{key}{BRACE_CLOSE}") };
    let wanted = if key == TIME_TAG_NAME && bare_name(PAST_TIME) == *value { PAST_TIME.to_string() } else { value.clone() };
    children().filter(|&c| *tree.node(c).name == *relation).any(|r| tree.node(r).children.iter().any(|&v| !tree.node(v).gone && (*tree.node(v).name == *wanted || bare_name(&tree.node(v).name) == *value)))
}
because!(tag_holds, WorldShape, "whether a node carries a tag: the count as its quantity tag node, and any other tag as a relation named \
     by the tag with the value under it, the time's past braced as the tree writes it");

fn fits(tree: &NodeTree, c: usize, first: &ShapeName) -> bool {
    let name = &tree.node(c).name;
    let timed = first.tags.iter().any(|(k, v)| k == TIME_TAG_NAME || (k == QUANTITY_TAG_NAME && number_of(v).is_some_and(|n| n.abs() < f32::EPSILON)));
    let traced = tree.node(c).link.is_some() && tree.node(c).children.iter().any(|&t| !tree.node(t).gone && *tree.node(t).name == *TIME_RELATION);
    !tree.node(c).gone && (timed || !traced) && (**name == *first.name || (!first.name.starts_with(BRACE_OPEN) && bare_name(name) == first.name && !name.starts_with(BRACE_OPEN))) && first.tags.iter().all(|tag| tag_holds(tree, c, tag))
}
because!(fits, WorldShape, "whether a node carries the name of a path's node with every tag written on it, the trace a thing left where it \
     was counting only for a name written with its time or with a count of none, john no longer has the milk");

fn owned_by(tree: &NodeTree, at: usize) -> Vec<usize> {
    let owner = super::physics::OWNER_TAG;
    (tree.state..tree.len()).filter(|&n| !tree.node(n).gone && tree.node(n).children.iter().any(|&t| !tree.node(t).gone && *tree.node(t).name == *owner && tree.node(t).children.iter().any(|&m| tree.node(m).link == Some(at)))).collect()
}
because!(owned_by, WorldShape, "the things a node owns, each carrying an owner tag with a mention of it, wherever they stand");

fn untimed(text: &str) -> String {
    let open = format!("{TAG_MARK}{TIME_TAG_NAME}{TAG_VALUE_OPEN}");
    match text.find(&open).and_then(|at| text[at..].find(TAG_VALUE_CLOSE).map(|end| (at, at + end + 1))) {
        Some((at, end)) => format!("{}{}", &text[..at], &text[end..]),
        None => text.to_string(),
    }
}
because!(untimed, WorldShape, "a written name with its time tag taken off, since the time of a place or of having belongs to the thing \
     that was there");

fn carried_by(tree: &NodeTree, at: usize) -> Vec<usize> {
    let gender = super::physics::GENDER_TAG;
    let person = |c: usize| tree.named(&tree.node(c).name).any(|n| tree.node(n).children.iter().any(|&g| *tree.node(g).name == *gender));
    let owner = super::physics::OWNER_TAG;
    let owned = |c: usize, by: usize| tree.node(c).children.iter().any(|&t| !tree.node(t).gone && *tree.node(t).name == *owner && tree.node(t).children.iter().any(|&m| tree.node(m).link == Some(by)));
    let standing: Vec<usize> = tree.node(at).children.iter().copied().filter(|&c| !tree.node(c).gone).collect();
    standing.into_iter().flat_map(|p| tree.node(p).children.iter().copied().filter(|&c| !tree.node(c).gone && (person(p) || owned(c, p))).collect::<Vec<_>>()).collect()
}
because!(carried_by, WorldShape, "the things carried by what stands in a place, all a person holds and what any other thing there owns, \
     which stand in that place with them, the football john carries in the hallway");

fn path_holds(tree: &NodeTree, at: usize, path: &[ShapeName]) -> bool {
    let Some(first) = path.first() else { return true };
    let inside = tree.node(at).children.iter().copied().chain(carried_by(tree, at)).filter(|&c| fits(tree, c, first)).any(|c| path_holds(tree, c, &path[1..]));
    inside || (at != 0 && !first.name.starts_with(BRACE_OPEN) && owned_by(tree, at).into_iter().filter(|&c| fits(tree, c, first)).any(|c| path_holds(tree, c, &path[1..])))
}
because!(path_holds, WorldShape, "whether a path stands below a node: some child, or some thing the node owns, carries the first name with \
     every tag written on it, and the rest of the path stands below that thing");

pub fn world_holds(tree: &NodeTree, expect: &[String]) -> bool {
    expect.iter().all(|statement| {
        let (denied, path) = shape_path(statement);
        let Some(first) = path.first() else { return !denied };
        let story = tree.named(&first.name).any(|n| n != 0 && tree.story(n) && !tree.node(n).gone);
        let held = tree.named(&first.name).filter(|&n| n != 0 && (tree.story(n) || !story) && fits(tree, n, first)).any(|n| path_holds(tree, n, &path[1..]));
        held != denied
    })
}
because!(world_holds, WorldShape, "whether the world holds every path a lesson expects, starting at any node of the story that carries its \
     first name, or at the seeds' node when the story never named it, seven is a number, since a box that holds the cat may itself stand \
     in the garden, and none it denies, whatever else it holds");

pub fn world_form(mind: &CursorMind, statement: &str) -> Vec<String> {
    if !statement.contains(PATH_MARK.trim()) && statement.contains(BRACE_OPEN) {
        let spaced: Vec<&str> = statement.split_whitespace().collect();
        return world_form(mind, &spaced.join(PATH_MARK));
    }
    let (denied, path) = shape_path(statement);
    if statement.contains(NUMBER_OPEN) {
        return Vec::new();
    }
    let mut names: Vec<String> = statement.trim().trim_start_matches(NOT_PREFIX.trim()).trim().split(PATH_MARK.trim()).map(|n| n.trim().to_string()).collect();
    let time = TIME_RELATION;
    if let Some(at) = names.iter().position(|n| n == time).filter(|&at| at > 1 && at + 1 < names.len()) {
        let value = bare_name(&names[at + 1]);
        let dated = format!("{}{TAG_MARK}{TIME_TAG_NAME}{TAG_VALUE_OPEN}{value}{TAG_VALUE_CLOSE}", names[at - 1]);
        names.splice(at - 1..at + PATH_STRIDE, [dated]);
        return world_form(mind, &format!("{}{}", if denied { NOT_PREFIX } else { "" }, names.join(PATH_MARK)));
    }
    let braced = |i: usize| path.get(i).is_some_and(|n| n.name.starts_with(BRACE_OPEN));
    let is = crate::quiz::IS_FORM.trim();
    let has = crate::quiz::OWNS_FORM.trim();
    let world = (0..path.len()).any(|i| path[i].name == is && braced(i + 1)) || path.iter().any(|n| n.name == super::physics::OWNER_TAG);
    let old = (1..path.len()).step_by(PATH_STRIDE).any(|i| braced(i));
    if world || !old || names.len() != path.len() {
        return vec![statement.to_string()];
    }
    let quality = |a: &str, q: &str| match super::mind::kind_of(mind, q) {
        Some(kind) => format!("{a}{PATH_MARK}{is}{PATH_MARK}{BRACE_OPEN}{kind}{BRACE_CLOSE}{PATH_MARK}{q}"),
        None => format!("{a}{PATH_MARK}{is}{PATH_MARK}{q}"),
    };
    let prefix = if denied { NOT_PREFIX } else { "" };
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < path.len() {
        let (a, relation) = (&names[i], &path[i + 1].name);
        let bare_relation = bare_name(relation);
        if i + PATH_STRIDE >= path.len() || braced(i + PATH_STRIDE) {
            out.push(format!("{prefix}{}", quality(a, &bare_relation)));
            break;
        }
        let b_said = &names[i + PATH_STRIDE];
        let b_named = shape_name(b_said);
        let past = b_named.tags.iter().find(|(k, _)| k == TIME_TAG_NAME).map(|(_, v)| format!("{TAG_MARK}{TIME_TAG_NAME}{TAG_VALUE_OPEN}{v}{TAG_VALUE_CLOSE}"));
        let b = &untimed(b_said);
        let a_past = format!("{a}{}", past.clone().unwrap_or_default());
        let moving = super::mind::MOVING.contains(&bare_relation.as_str());
        let written = if moving && past.is_some() {
            i += PATH_STRIDE;
            continue;
        } else if moving {
            format!("{b}{PATH_MARK}{a}")
        } else if super::mind::STATE_WORDS.contains(&bare_relation.as_str()) && crate::words::number_of(b).is_some() {
            format!("{a}{PATH_MARK}{relation}{PATH_MARK}{b_said}")
        } else if super::mind::place_word(&bare_relation) && !super::mind::DIRECTIONS.contains(&bare_relation.as_str()) {
            format!("{b}{PATH_MARK}{a_past}")
        } else if super::mind::CONTAINING.contains(&bare_relation.as_str()) || *relation == *has {
            match &past {
                Some(tag) => format!("{a}{PATH_MARK}{b}{tag}"),
                None => format!("{a}{PATH_MARK}{b}"),
            }
        } else if super::mind::NEGATIONS.contains(&bare_relation.as_str()) {
            format!("{a}{PATH_MARK}{is}{PATH_MARK}{b}{TAG_MARK}{QUANTITY_TAG_NAME}{TAG_VALUE_OPEN}0{TAG_VALUE_CLOSE}")
        } else if *relation == *is {
            quality(a, &path[i + PATH_STRIDE].name)
        } else if super::moves::KINDS.iter().any(|(_, kind)| *kind == bare_relation) {
            format!("{a}{PATH_MARK}{is}{PATH_MARK}{relation}{PATH_MARK}{b_said}")
        } else {
            format!("{a}{PATH_MARK}{relation}{PATH_MARK}{b_said}")
        };
        out.push(format!("{prefix}{written}"));
        i += PATH_STRIDE;
    }
    out
}
because!(world_form, WorldShape, "a statement in the older notation of relations read as the world holds it, a state word with a count \
     staying a relation, three fans on: a relation named for a kind of quality stands under is, the ring's material, and a denial is the \
     value under is counted none, a bat is not a bird; each thing, relation and value along the path in turn, a time under a value moved \
     onto the thing as its tag, while a time right under the first thing stays its relation, it is three o'clock, the cat was in the \
     garden read as the garden holding the trace of the cat, a moving verb read as the place the thing went to, and a going in the past \
     left out since where the thing is now is said beside it, a place relation turned into the place holding the thing, a relation of \
     holding or having into the holder holding the thing, which counts too when the thing stands elsewhere and is only owned, a quality \
     under is or said bare set under the kind the seeds give it, and any other relation kept as it is with the time of its value, tom met \
     sally in the past; a statement already in the world's form stays as written, a statement that names a number as a concept is no shape \
     of the world, and a turn's statement written with spaces for its arrows, tom {has} cat, is read as the path it spells");

const NUMBER_OPEN: &str = "{number";
because!(NUMBER_OPEN, WorldShape, "the opening of a number written as a concept, a statement the older lessons add beside a count, which \
     says nothing of the world");

const PATH_STRIDE: usize = 2;
because!(PATH_STRIDE, WorldShape, "how far apart the relations of an older path stand: a name and a relation take turns along it");


fn child_of(mind: &mut CursorMind, under: usize, name: &str) -> usize {
    let had = mind.tree.node(under).children.iter().copied().find(|&c| !mind.tree.node(c).gone && mind.tree.story(c) && *mind.tree.node(c).name == *name);
    had.unwrap_or_else(|| mind.tree.added(under, name))
}
because!(child_of, WorldShape, "the story's child of a node with a name, added when the node holds none, so paths that share a head build \
     one node and never write into the seeds");

fn story_named(mind: &CursorMind, name: &str) -> Option<usize> {
    mind.tree.named(name).filter(|&n| n != 0 && mind.tree.story(n) && !mind.tree.node(n).gone && { let p = mind.tree.node(n).parent; p == 0 || !mind.tree.node(p).name.starts_with(BRACE_OPEN) }).max()
}
because!(story_named, WorldShape, "the story's newest thing of a name standing in the world, never a value under a relation");

fn thing_moved(mind: &mut CursorMind, node: usize, under: usize) {
    mind.tree.moved(node, None);
    mind.tree.moved(node, Some(under));
}
because!(thing_moved, WorldShape, "a thing put inside another, taken out of where it stood");

fn tagged(mind: &mut CursorMind, node: usize, tags: &[(String, String)]) {
    for (key, value) in tags {
        if key == QUANTITY_TAG_NAME {
            if let Some(count) = number_of(value) {
                child_of(mind, node, &quantity_tag_named(count));
            }
            continue;
        }
        let relation = if key == TIME_TAG_NAME { TIME_RELATION.to_string() } else { format!("{BRACE_OPEN}{key}{BRACE_CLOSE}") };
        let wanted = if key == TIME_TAG_NAME && bare_name(PAST_TIME) == *value { PAST_TIME.to_string() } else { value.clone() };
        let tag = child_of(mind, node, &relation);
        child_of(mind, tag, &wanted);
    }
}
because!(tagged, WorldShape, "a node given the tags written on its name, the count as its quantity tag and any other tag as its relation \
     with the value under it");

fn older_taken(mind: &mut CursorMind, path: &[ShapeName]) {
    if let ([first, quality], Some(holder)) = (path, path.first().and_then(|f| story_named(mind, &f.name))) {
        let _ = first;
        let is = crate::quiz::IS_FORM.trim();
        let denied = bare_name(&quality.name);
        let under: Vec<usize> = mind.tree.node(holder).children.iter().copied().filter(|&c| !mind.tree.node(c).gone && *mind.tree.node(c).name == *is).collect();
        let values: Vec<usize> = under.iter().flat_map(|&i| mind.tree.node(i).children.iter().copied().collect::<Vec<usize>>()).flat_map(|c| if mind.tree.node(c).name.starts_with(BRACE_OPEN) { mind.tree.node(c).children.iter().copied().collect::<Vec<usize>>() } else { vec![c] }).filter(|&v| !mind.tree.node(v).gone && *mind.tree.node(v).name == *denied).collect();
        for value in values {
            mind.tree.moved(value, None);
        }
        return;
    }
    let (Some(first), Some(relation), Some(last)) = (path.first(), path.get(1), path.get(PATH_STRIDE)) else { return };
    let Some(holder) = story_named(mind, &first.name) else { return };
    let owner = super::physics::OWNER_TAG;
    let has = crate::quiz::OWNS_FORM.trim();
    let place = super::mind::place_word(&bare_name(&relation.name));
    let gone: Vec<usize> = if place {
        story_named(mind, &last.name).filter(|&p| mind.tree.node(holder).parent == p).map(|_| holder).into_iter().collect()
    } else if relation.name == has || super::mind::CONTAINING.contains(&bare_name(&relation.name).as_str()) {
        (mind.tree.state..mind.tree.len()).filter(|&n| !mind.tree.node(n).gone && *mind.tree.node(n).name == *last.name && last.tags.iter().all(|t| tag_holds(&mind.tree, n, t)) && (mind.tree.node(n).parent == holder || mind.tree.node(n).children.iter().any(|&t| *mind.tree.node(t).name == *owner && mind.tree.node(t).children.iter().any(|&m| mind.tree.node(m).link == Some(holder))))).collect()
    } else {
        Vec::new()
    };
    for n in gone {
        mind.tree.moved(n, None);
        if place {
            mind.tree.moved(n, Some(0));
        }
    }
}
because!(older_taken, WorldShape, "a seeded statement denied with not, taken out of the world, a quality of a thing from under is or its \
     kind, the ball no longer red: the things of that name and tags the holder has or holds are gone, john no longer has the three \
     marbles, and a thing no longer in a place stands in the world itself");

fn older_built(mind: &mut CursorMind, statement: &str) {
    let (denied, path) = shape_path(statement);
    if path.first().is_some_and(|n| n.name.starts_with(NUMBER_OPEN)) {
        return;
    }
    if denied {
        older_taken(mind, &path);
        return;
    }
    let is = crate::quiz::IS_FORM.trim();
    let has = crate::quiz::OWNS_FORM.trim();
    let braced = |i: usize| path.get(i).is_some_and(|n| n.name.starts_with(BRACE_OPEN));
    let thing = |mind: &mut CursorMind, name: &str| story_named(mind, name).unwrap_or_else(|| mind.tree.added(0, name));
    let mut at = thing(mind, &path[0].name);
    tagged(mind, at, &path[0].tags);
    let mut i = 0;
    while i + 1 < path.len() {
        let relation = path[i + 1].name.clone();
        let bare_relation = bare_name(&relation);
        if i + PATH_STRIDE >= path.len() || braced(i + PATH_STRIDE) {
            let quality = bare_relation.clone();
            let kind = super::mind::kind_of(mind, &quality);
            let under = child_of(mind, at, is);
            let under = match kind { Some(k) => child_of(mind, under, &format!("{BRACE_OPEN}{k}{BRACE_CLOSE}")), None => under };
            child_of(mind, under, &quality);
            break;
        }
        let next = &path[i + PATH_STRIDE];
        let fresh = relation == *has || super::mind::CONTAINING.contains(&bare_relation.as_str());
        let other = if fresh { mind.tree.added(0, &next.name) } else if relation == *is || !(super::mind::place_word(&bare_relation)) { 0 } else { thing(mind, &next.name) };
        if super::mind::place_word(&bare_relation) {
            thing_moved(mind, at, other);
            tagged(mind, other, &next.tags);
            at = other;
        } else if fresh {
            tagged(mind, other, &next.tags);
            thing_moved(mind, other, at);
            if relation == *has {
                let tag = child_of(mind, other, super::physics::OWNER_TAG);
                mind.tree.linked(tag, at, false);
            }
            at = other;
        } else if relation == *is {
            let quality = next.name.clone();
            let kind = super::mind::kind_of(mind, &quality);
            let under = child_of(mind, at, is);
            let under = match kind { Some(k) => child_of(mind, under, &format!("{BRACE_OPEN}{k}{BRACE_CLOSE}")), None => under };
            let value = child_of(mind, under, &quality);
            tagged(mind, value, &next.tags);
            at = value;
        } else {
            let under = child_of(mind, at, &relation);
            let value = child_of(mind, under, &next.name);
            tagged(mind, value, &next.tags);
            at = value;
        }
        i += PATH_STRIDE;
    }
}
because!(older_built, WorldShape, "a seeded statement in the older notation of relations built by what it means, each name the story's \
     thing of that name or a new one: a place relation moves the thing inside the place, which the path then goes on from, the tom has a \
     kite that is in the garden; having and holding make a new thing of the name inside the holder, the owner tagged on it, since two \
     people who each have a ball have two balls; a quality under is or said bare goes under the kind the seeds give it; and any other \
     relation keeps the value under it");

pub fn world_built(mind: &mut CursorMind, statements: &[String]) {
    if let Some(question) = super::mind::open_question(mind) {
        mind.tree.moved(question, None);
    }
    mind.question_start = None;
    let mut modern = Vec::new();
    for statement in statements {
        if world_form(mind, statement).first().is_some_and(|s| s == statement) {
            modern.push(statement.clone());
        } else {
            older_built(mind, statement);
        }
    }
    let statements = modern;
    for statement in &statements {
        let (denied, path) = shape_path(statement);
        if denied {
            continue;
        }
        let mut at = 0;
        for node in path {
            at = child_of(mind, at, &node.name);
            for (key, value) in &node.tags {
                if key == QUANTITY_TAG_NAME {
                    if let Some(count) = number_of(value) {
                        child_of(mind, at, &quantity_tag_named(count));
                    }
                    continue;
                }
                let relation = if key == TIME_TAG_NAME { TIME_RELATION.to_string() } else { format!("{BRACE_OPEN}{key}{BRACE_CLOSE}") };
                let wanted = if key == TIME_TAG_NAME && bare_name(PAST_TIME) == *value { PAST_TIME.to_string() } else { value.clone() };
                let tag = child_of(mind, at, &relation);
                child_of(mind, tag, &wanted);
            }
        }
    }
}
because!(world_built, WorldShape, "a world seeded before a line is read, the user's world line, the question of the line before taken out \
     first so nothing is written under its words: every path, an older one read as the world holds it first, written from the world down \
     in the same form a reading leaves, a thing inside a place as its child and each tag as the relation named by it with the value under \
     it, the count as the quantity tag, and a denied path left out, so the network is asked about a world it never read");
