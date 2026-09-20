use super::mind::{CursorMind, seen_nodes};
use super::{ASKS_FORM, BRACE_OPEN_TEXT, CURSOR_WORLD, CursorTree, FUTURE_FORM, LATER_TIME, PAST_FORM, PAST_TIME, PATH_MARK, QUANTITY, QUESTION_NODE, QUOTING, TIME_RELATION};
use crate::numbers::zero;
use super::statements::EQUAL_RELATION;
use crate::quiz::{IS_FORM, WORTH_FORM};
use crate::words::number_of;
use imbl::{HashMap, OrdMap, Vector};
use patterns::{because, source};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub name: Arc<str>,
    pub parent: usize,
    pub children: Vector<usize>,
    pub gone: bool,
    pub hidden: bool,
    pub link: Option<usize>,
}
because!(TreeNode, CursorTree, "one node of the tree: its name, the node it is a child of, the world being its own parent, its children in \
     the order they were added, whether it was removed, so a removed node keeps its place in the list and is never found again, whether it \
     is hidden, a node the network has not seen, which no find reaches until a walk lands on it, as a thing in a box is there before \
     anyone opens the box, and the thing it links to when it is a value that names a thing under the world, so one thing is one node \
     however many facts name it; the children are a persistent list, so a child added under the world copies a few of its blocks and never \
     the whole list");

#[derive(Clone, Debug)]
pub struct NodeTree {
    base: Arc<Vec<TreeNode>>,
    over: Vec<(usize, TreeNode)>,
    added: Vec<TreeNode>,
    pub by_name: HashMap<Arc<str>, Vector<usize>>,
    pub linkers: HashMap<usize, Vector<usize>>,
    pub stems: HashMap<String, Vector<Arc<str>>>,
    pub things: HashMap<Arc<str>, Vector<usize>>,
    pub holders: HashMap<Arc<str>, OrdMap<usize, usize>>,
    pub hidden: usize,
    pub mark: u64,
    pub version: u64,
    pub state: usize,
}
because!(NodeTree, CursorTree, "the permanent state as a tree whose first node is the world; a name refers to the root thing of that name \
     unless the name is a number or the thing is a unit, a thing whose every relation is a worth in another thing's name and holds a \
     number, as a cent with its worth in dollars, while a thing that only has a count or a number word that only equals its number is no \
     unit, since the three under pens must still be the three that equals three, and since a counted mention of a unit is a group of its \
     own and the cents of rose are not the cents of fred; things stand right under the world, each thing's relations under it and the \
     values of each relation under that, a value that names a thing being a link to it, so the tree is the nested circles the user draws, \
     and no two nodes are one thing; how a tree changes and is read: a node is added under another by name, seen or hidden; a link is a \
     node named as the thing it links to; a thing named is the child of the world with that name, and the thing referred to by a name is \
     that thing unless the name is a number, a unit or a word form; a node moved under another takes everything below it along, and a node \
     moved nowhere is gone with everything below it, kept in its place so no id changes; and the paths a person reads are the names from \
     the world down to every leaf that is not gone; how it is held, for a state of millions of nodes: the nodes stand in a base list every \
     reading of a search shares by pointer and never copies, a reading that changes a node keeps its own copy of that node over the base, \
     and a reading that adds nodes keeps them after the base, so a step that writes copies one node and a reading cloned copies only what \
     it changed, and once an input is done the changes settle into the base when nothing else shares it, while a base another mind shares, \
     as the seeded start every line clones, is left alone until the overlay grows past the width of a wide node, since copying a base of a \
     million nodes for every line costs more than reading through a small overlay; beside the nodes the tree keeps what a search asks of \
     it a million times: the nodes of every name in the order they were added, the things of every name, the children of the world, the \
     nodes that link to every node, the names by their stem, the holders of every relation in the order they were added, how many nodes \
     are hidden, a mark that changes with every node's name, parent, gone and hidden, kept as a sum of one hash a node, so two readings \
     that made the same tree by different steps carry the same mark and a step never hashes the tree again, and a version that every \
     change raises, so a step that changed nothing is known without comparing; and how many nodes the state held when the story began, so \
     the story's own nodes, which come after, are told from the state's, since a name the story says means the story's node first and a \
     seed's only when the story has none");

fn stem_of(name: &str) -> Option<String> {
    let word = name.strip_prefix(BRACE_OPEN_TEXT).unwrap_or(name);
    let stem: String = word.chars().take(super::SHARED_STEM).collect();
    (stem.chars().count() == super::SHARED_STEM).then_some(stem)
}
because!(stem_of, CursorTree, "the first letters of a name, its brace dropped when it is a relation, as many as the stem a built word \
     shares, or none for a name too short to share one; the key the names are kept under for the words that share their stem, so a \
     relation is found from the word it is written from");

fn node_mark(at: usize, node: &TreeNode) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    at.hash(&mut hasher);
    node.name.hash(&mut hasher);
    node.parent.hash(&mut hasher);
    node.gone.hash(&mut hasher);
    node.hidden.hash(&mut hasher);
    hasher.finish()
}
because!(node_mark, CursorTree, "the hash of one node's place, name, parent and whether it is gone or hidden, the part of the tree's mark \
     that node contributes");

impl Default for NodeTree {
    fn default() -> Self {
        NodeTree::from_nodes(vec![TreeNode { name: CURSOR_WORLD.into(), parent: 0, children: Vector::new(), gone: false, hidden: false, link: None }])
    }
}

impl NodeTree {
    pub fn from_nodes(nodes: Vec<TreeNode>) -> NodeTree {
        let mut tree = NodeTree { base: Arc::new(Vec::new()), over: Vec::new(), added: Vec::with_capacity(nodes.len()), by_name: HashMap::new(), linkers: HashMap::new(), stems: HashMap::new(), things: HashMap::new(), holders: HashMap::new(), hidden: 0, mark: 0, version: 0, state: 0 };
        for node in nodes {
            tree.pushed(TreeNode { children: Vector::new(), ..node });
        }
        for at in 1..tree.len() {
            if !tree.node(at).gone {
                let parent = tree.node(at).parent;
                tree.added[parent].children.push_back(at);
            }
        }
        tree.settled();
        tree
    }

    pub fn len(&self) -> usize {
        self.base.len() + self.added.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn node(&self, at: usize) -> &TreeNode {
        let n = self.base.len();
        if at >= n {
            &self.added[at - n]
        } else if self.over.is_empty() {
            &self.base[at]
        } else {
            self.over.iter().find(|(o, _)| *o == at).map_or(&self.base[at], |(_, node)| node)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &TreeNode> + '_ {
        (0..self.len()).map(move |at| self.node(at))
    }

    fn node_mut(&mut self, at: usize) -> &mut TreeNode {
        let n = self.base.len();
        self.version += 1;
        if at >= n {
            &mut self.added[at - n]
        } else {
            let place = match self.over.iter().position(|(o, _)| *o == at) {
                Some(place) => place,
                None => {
                    self.over.push((at, self.base[at].clone()));
                    self.over.len() - 1
                }
            };
            &mut self.over[place].1
        }
    }

    pub fn settled(&mut self) {
        if (self.over.is_empty() && self.added.is_empty()) || (Arc::strong_count(&self.base) > 1 && self.over.len() <= WIDE_NODE) {
            return;
        }
        if self.base.is_empty() && self.over.is_empty() {
            self.added.shrink_to_fit();
            self.base = Arc::new(std::mem::take(&mut self.added));
            return;
        }
        let base = Arc::make_mut(&mut self.base);
        for (at, node) in self.over.drain(..) {
            base[at] = node;
        }
        base.append(&mut self.added);
        base.shrink_to_fit();
    }

    fn pushed(&mut self, node: TreeNode) -> usize {
        let at = self.len();
        self.version += 1;
        self.mark = self.mark.wrapping_add(node_mark(at, &node));
        if let Some(stem) = stem_of(&node.name).filter(|_| !self.by_name.contains_key(&node.name)) {
            self.stems.entry(stem).or_default().push_back(node.name.clone());
        }
        self.by_name.entry(node.name.clone()).or_default().push_back(at);
        if node.parent == 0 && at != 0 {
            self.things.entry(node.name.clone()).or_default().push_back(at);
        }
        if node.hidden {
            self.hidden += 1;
        }
        if let Some(target) = node.link {
            self.linkers.entry(target).or_default().push_back(at);
        }
        if !node.gone && node.name.starts_with(BRACE_OPEN_TEXT) && at != 0 {
            *self.holders.entry(node.name.clone()).or_default().entry(node.parent).or_insert(0) += 1;
        }
        self.added.push(node);
        at
    }

    pub(crate) fn changed(&mut self, at: usize, change: impl FnOnce(&mut TreeNode)) {
        let node = self.node_mut(at);
        let before = node_mark(at, node);
        let (was_hidden, was_link, was_parent, was_gone, name) = (node.hidden, node.link, node.parent, node.gone, node.name.clone());
        change(node);
        let after = node_mark(at, node);
        let (hidden, link, parent, gone) = (node.hidden, node.link, node.parent, node.gone);
        if at != 0 && name.starts_with(BRACE_OPEN_TEXT) && (was_parent != parent || was_gone != gone) {
            if !was_gone {
                if let Some(held) = self.holders.get_mut(&name) {
                    let left = held.get(&was_parent).copied().unwrap_or(0).saturating_sub(1);
                    if left == 0 {
                        held.remove(&was_parent);
                    } else {
                        held.insert(was_parent, left);
                    }
                }
            }
            if !gone {
                *self.holders.entry(name.clone()).or_default().entry(parent).or_insert(0) += 1;
            }
        }
        if was_parent != parent && at != 0 {
            if was_parent == 0 {
                if let Some(list) = self.things.get_mut(&name) {
                    list.retain(|&t| t != at);
                }
            }
            if parent == 0 {
                self.things.entry(name).or_default().push_back(at);
            }
        }
        self.mark = self.mark.wrapping_sub(before).wrapping_add(after);
        if was_hidden && !hidden {
            self.hidden -= 1;
        } else if !was_hidden && hidden {
            self.hidden += 1;
        }
        if was_link != link {
            if let Some(old) = was_link {
                if let Some(list) = self.linkers.get_mut(&old) {
                    list.retain(|&l| l != at);
                }
            }
            if let Some(target) = link {
                self.linkers.entry(target).or_default().push_back(at);
            }
        }
    }

    pub fn named(&self, name: &str) -> impl Iterator<Item = usize> + '_ {
        self.by_name.get(name).into_iter().flat_map(|list| list.iter().copied())
    }

    pub fn linkers_of(&self, target: usize) -> impl Iterator<Item = usize> + '_ {
        self.linkers.get(&target).into_iter().flat_map(|list| list.iter().copied())
    }

    pub fn names_sharing_stem<'a>(&'a self, word: &str) -> impl Iterator<Item = &'a Arc<str>> + 'a {
        stem_of(word).and_then(|stem| self.stems.get(&stem)).into_iter().flat_map(|list| list.iter())
    }

    pub fn holders_of<'a>(&'a self, relation: &str) -> impl Iterator<Item = usize> + 'a {
        self.holders.get(relation).into_iter().flat_map(|held| held.keys().copied())
    }

    pub fn has_hidden(&self) -> bool {
        self.hidden > 0
    }

    pub fn ptr_eq(&self, other: &NodeTree) -> bool {
        Arc::ptr_eq(&self.base, &other.base) && self.version == other.version
    }

    pub(crate) fn added(&mut self, under: usize, name: &str) -> usize {
        self.added_as(under, name, false)
    }

    pub(crate) fn added_as(&mut self, under: usize, name: &str, hidden: bool) -> usize {
        let at = self.pushed(TreeNode { name: name.into(), parent: under, children: Vector::new(), gone: false, hidden, link: None });
        self.node_mut(under).children.push_back(at);
        at
    }

    pub(crate) fn linked(&mut self, under: usize, target: usize, hidden: bool) -> usize {
        let name: Arc<str> = word_name(&self.node(target).name).into();
        let at = self.added_as(under, &name, hidden);
        self.changed(at, |node| node.link = Some(target));
        at
    }

    pub(crate) fn adopted(&mut self, thing: usize) {
        let name = self.node(thing).name.clone();
        if number_of(&name).is_some() {
            return;
        }
        let of_state = |n: usize| self.story(n) && self.node(n).link.is_some_and(|t| !self.story(t));
        let plain: Vec<usize> = self.named(&name).filter(|&n| n != thing && !self.node(n).gone && (self.node(n).link.is_none() || of_state(n)) && self.node(n).parent != 0 && self.node(self.node(n).parent).name.starts_with(BRACE_OPEN_TEXT) && !quoted(self, n) && !asked_node(self, n)).collect();
        for n in plain {
            let held: Vec<usize> = self.node(n).children.iter().copied().filter(|&c| !mention_relation(&self.node(c).name)).collect();
            for c in held {
                self.moved(c, Some(thing));
            }
            let kept: Vector<usize> = self.node(n).children.iter().copied().filter(|&c| mention_relation(&self.node(c).name)).collect();
            self.changed(n, |node| {
                node.children = kept;
                node.link = Some(thing);
            });
        }
    }

    pub fn story(&self, at: usize) -> bool {
        at >= self.state
    }

    pub(crate) fn thing_named(&self, name: &str) -> Option<usize> {
        let things = self.things.get(name);
        things.into_iter().flat_map(|list| list.iter().rev().copied()).find(|&c| !self.node(c).gone && self.story(c)).or_else(|| things.into_iter().flat_map(|list| list.iter().copied()).find(|&c| !self.node(c).gone))
    }

    pub(crate) fn referred(&self, name: &str) -> Option<usize> {
        self.thing_named(name).filter(|&thing| number_of(name).is_none() && !self.unit(thing) && !word_form(self, thing)).map(|thing| self.node(thing).link.filter(|&c| self.node(c).parent == 0 && !self.node(c).gone).unwrap_or(thing))
    }

    pub(crate) fn unit(&self, thing: usize) -> bool {
        amount_of(self, thing).is_some_and(|amount| child_named(self, amount, EQUAL_RELATION).is_some())
    }

    pub(crate) fn unhidden(&mut self, nodes: impl Iterator<Item = usize>) {
        let hidden: Vec<usize> = nodes.filter(|&n| self.node(n).hidden).collect();
        for n in hidden {
            self.changed(n, |node| node.hidden = false);
        }
    }

    pub(crate) fn moved(&mut self, node: usize, under: Option<usize>) {
        let subtree: Vec<usize> = std::iter::once(node).chain(below(self, node)).collect();
        match under {
            None => {
                let parent = self.node(node).parent;
                self.node_mut(parent).children.retain(|&c| c != node);
            }
            Some(parent) => {
                self.changed(node, |n| n.parent = parent);
                self.node_mut(parent).children.push_back(node);
            }
        }
        let gone = under.is_none();
        for n in subtree {
            if self.node(n).gone != gone {
                self.changed(n, |node| node.gone = gone);
            }
        }
    }

    pub fn paths(&self) -> Vec<String> {
        (1..self.len())
            .filter(|&n| self.node(n).children.is_empty() && !self.node(n).gone)
            .map(|leaf| {
                let mut names = Vec::new();
                let mut at = leaf;
                while at != 0 {
                    names.push(self.node(at).name.to_string());
                    at = self.node(at).parent;
                }
                names.reverse();
                names.join(PATH_MARK)
            })
            .collect()
    }
}

pub(crate) fn named_braced<'a>(tree: &'a NodeTree, word: &str) -> impl Iterator<Item = usize> + 'a {
    let braced = format!("{BRACE_OPEN_TEXT}{word}{}", crate::quiz::BRACE_CLOSE);
    tree.by_name.get(braced.as_str()).into_iter().flat_map(|list| list.iter().copied())
}
because!(named_braced, CursorTree, "every node named the braced form of a word, the relation written from it, asked of the index");

pub(crate) fn things_holding(tree: &NodeTree, relation: &str) -> Vec<usize> {
    let mut things: Vec<usize> = tree.named(relation).filter(|&r| !tree.node(r).gone).map(|r| tree.node(r).parent).filter(|&t| t != 0 && !tree.node(t).gone).collect();
    things.sort_unstable();
    things.dedup();
    things
}
because!(things_holding, CursorTree, "every thing under the world that holds a relation, in the order the things were added, found through \
     the nodes of that relation and never by walking every child of the world, which a large state gives by the hundred thousand");

pub(crate) fn counts_none(tree: &NodeTree, node: usize) -> bool {
    quantity_of(tree, node) == Some(zero())
}

pub(crate) fn quantity_tag(tree: &NodeTree, node: usize) -> Option<usize> {
    tree.node(node).children.iter().copied().filter(|&c| !tree.node(c).gone && super::is_quantity_tag(&tree.node(c).name)).find(|&c| stamped(tree, c).is_none()).or_else(|| tree.node(node).children.iter().copied().find(|&c| !tree.node(c).gone && super::is_quantity_tag(&tree.node(c).name)))
}
because!(quantity_tag, CursorTree, "the quantity tag a node holds, the one of the present before one filed in the past, so a count read is \
     the count now");

pub(crate) fn quantity_of(tree: &NodeTree, node: usize) -> Option<f32> {
    quantity_tag(tree, node).and_then(|tag| number_of(&tree.node(tag).name))
}
because!(quantity_of, CursorTree, "how many of a thing there are, the number of its quantity tag");
because!(counts_none, CursorTree, "whether a node carries a quantity of zero, which a text wrote for a thing someone has none of");

pub fn present(mind: &CursorMind, node: usize) -> bool {
    let tree = &mind.tree;
    let alive = !tree.node(node).gone && !counts_none(tree, node) && !asked_node(tree, node);
    let timed = |time: usize| *tree.node(tree.node(node).parent).name == *TIME_RELATION || newest(mind, node, time);
    alive && stamped(tree, node).is_none_or(|time| question_time(mind).is_some() || timed(time))
}

fn newest(mind: &CursorMind, node: usize, time: usize) -> bool {
    let tree = &mind.tree;
    let order = time_order(&mind.tree);
    let rank = |t: usize| order.iter().position(|o| *o == tree.node(t).name);
    let Some(here) = rank(time) else {
        return false;
    };
    tree.node(tree.node(node).parent).children.iter().copied().filter(|&c| c != node && !tree.node(c).gone).all(|c| stamped(tree, c).is_some_and(|t| rank(t).is_none_or(|r| r <= here)))
}
because!(newest, CursorTree, "whether a value with a named time is the newest of its relation: no sibling without a time, since such a \
     value is so now, and no sibling with a later named time by the order the seeds state, so where julie is after this morning's cinema \
     and yesterday's kitchen is the cinema, while a value with the time past is never newest");
because!(present, CursorTree, "whether a node is there to be met: not removed and not a thing with a quantity of zero, which a text wrote \
     for a thing someone has none of, so a find, a listing, a count or a walk never meets it while a step to it by its slot still reads \
     the zero back; a value with a time only while the question asks about the past, so who has the ball never meets the ball someone had, \
     while who had it meets every ball, the timed ones first, since who received the pen holds it now, or the newest of its relation by \
     the order of named times the seeds state while no sibling is so now, since the last time told is now; a time value itself always, \
     since it is what says when; the question node and what stands under it are never present, so no walk, count or listing reads the \
     question as a fact");

pub(crate) fn mention_relation(name: &str) -> bool {
    name == TIME_RELATION || name == QUANTITY || super::is_quantity_tag(name)
}
because!(mention_relation, CursorTree, "whether a relation belongs to the mention of a thing and not to the thing: its time and its \
     quantity, since when a fact was so and how many of a thing a holder has are facts of the holding, so a thing that adopts an earlier \
     mention leaves them on the mention, a path reads them on the mention, and a write puts them there");

pub(crate) fn stamped(tree: &NodeTree, node: usize) -> Option<usize> {
    child_named(tree, node, TIME_RELATION).and_then(|t| tree.node(t).children.iter().copied().find(|&c| !tree.node(c).gone))
}
because!(stamped, CursorTree, "the time value a node carries, past or a named time, so a value that is no longer so or was so at a named \
     time is told from one that is so now; none for a value with no time");

pub(crate) fn question_time(mind: &CursorMind) -> Option<Arc<str>> {
    asked_time(&mind.tree)
}

pub(crate) fn asked_time(tree: &NodeTree) -> Option<Arc<str>> {
    tree.thing_named(QUESTION_NODE).and_then(|q| stamped(tree, q)).map(|t| tree.node(t).name.clone())
}
because!(asked_time, CursorTree, "the time the question in its node asks about, read from the tree alone, for the hops that take a tree");

because!(question_time, CursorTree, "the time the question in its node asks about, past, later or a named time, written there when the \
     question says a past form, a future form or a time the seeds order, so the physics reads every value for such a question, those of \
     its time first, and only the values of now for any other");

pub(crate) fn of_question_time(mind: &CursorMind, node: usize) -> bool {
    stamped(&mind.tree, node).is_some_and(|t| question_time(mind).is_some_and(|asked| *asked == *PAST_TIME && *mind.tree.node(t).name != *LATER_TIME || *asked == *mind.tree.node(t).name))
}
because!(of_question_time, CursorTree, "whether a timed value is of the time the question asks about: any time but later for a question \
     about the past, and the very time for a question about later or a named time, so such values come first among the nodes of a name");

pub(crate) fn verb_of(tree: &NodeTree, word: &str) -> Option<Arc<str>> {
    told_past(tree, word).or_else(|| told_form(tree, ASKS_FORM, word)).or_else(|| told_form(tree, FUTURE_FORM, word)).map(|verb| tree.node(verb).name.clone())
}
because!(verb_of, CursorTree, "the verb the seeds state a word as a form of, see for saw or sees, so a deed a sentence tells by any form \
     of a verb is written under the verb");

pub(crate) fn told_past(tree: &NodeTree, word: &str) -> Option<usize> {
    told_form(tree, PAST_FORM, word)
}

pub(crate) fn told_form(tree: &NodeTree, form: &str, word: &str) -> Option<usize> {
    tree.named(word).filter(|&f| f != 0 && !tree.node(f).gone).map(|f| tree.node(f).parent).filter(|&held| held != 0 && *tree.node(held).name == *form).map(|held| tree.node(held).parent).filter(|&t| t != 0 && tree.node(t).parent == 0 && !tree.node(t).gone).min()
}
because!(told_form, CursorTree, "the verb a word is a form of, under the relation of that form the seeds state, so the past and the future \
     forms are read alike");
because!(told_past, CursorTree, "the verb whose past form a word is, as the seeds state it under the past of the verb, meet for met, so a \
     sentence that says such a word is known to tell what happened, and none for a word that is no past form");

pub(crate) fn word_form(tree: &NodeTree, node: usize) -> bool {
    let forms = |n: usize| child_named(tree, n, PAST_FORM).is_some() || child_named(tree, n, FUTURE_FORM).is_some() || child_named(tree, n, ASKS_FORM).is_some();
    let under_form = |n: usize| *tree.node(n).name == *PAST_FORM || *tree.node(n).name == *FUTURE_FORM || *tree.node(n).name == *ASKS_FORM;
    node != 0 && (forms(node) || under_form(tree.node(node).parent))
}
because!(word_form, CursorTree, "whether a node is a word the seeds state a form of, a verb holding its past or its form after one \
     subject, or such a form itself, met under meet and was under is, which names no thing and no value of the world, so a question that \
     says is, had or met is not charged a lookup of the word and never looks it up");

pub(crate) const SUCCESSOR: &str = "{successor}";
pub struct TimeSeeds;
source!(TimeSeeds, "the seeds files that state an order of times, the day-parts file for the parts of a day and yesterday, each time \
     holding its successor, one statement a line");

because!(SUCCESSOR, TimeSeeds, "the relation the seeds order the times by, morning before afternoon, so the values a text stamps with \
     named times are walked in the order of the times whatever order the text told them in");

fn time_order(tree: &NodeTree) -> Vec<Arc<str>> {
    let successor = |t: usize| child_named(tree, t, SUCCESSOR).and_then(|s| tree.node(s).children.iter().copied().find(|&c| !tree.node(c).gone)).map(|c| tree.node(c).name.clone());
    let things: Vec<usize> = things_holding(tree, SUCCESSOR);
    let named_after: Vec<Arc<str>> = things.iter().filter_map(|&t| successor(t)).collect();
    let within = |t: usize| child_named(tree, t, crate::quiz::IN_FORM.trim()).and_then(|r| tree.node(r).children.iter().copied().find(|&c| !tree.node(c).gone)).map(|c| tree.node(c).name.clone());
    let chain = |first: usize| {
        let mut order: Vec<Arc<str>> = vec![tree.node(first).name.clone()];
        while let Some(next) = order.last().and_then(|last| tree.thing_named(last)).and_then(successor) {
            if order.contains(&next) {
                break;
            }
            order.push(next);
        }
        order
    };
    let Some(first) = things.iter().copied().find(|&t| !named_after.contains(&tree.node(t).name) && within(t).is_none()) else {
        return Vec::new();
    };
    let mut order: Vec<Arc<str>> = Vec::new();
    for day in chain(first) {
        order.push(day.clone());
        if let Some(part) = things.iter().copied().find(|&t| !named_after.contains(&tree.node(t).name) && within(t).as_ref() == Some(&day)) {
            order.extend(chain(part));
        }
    }
    order
}
because!(time_order, CursorTree, "the named times in the order the seeds state: from the time no other time names as its successor and \
     that is in no other time, along the successors, yesterday, today, tomorrow, and right after each such time the times in it along \
     their own successors, so yesterday comes before this morning and this morning before this afternoon; none when the times form a ring, \
     as the hours do, since a ring has no first");

pub fn hidden_node(mind: &CursorMind, node: usize) -> bool {
    mind.tree.node(node).hidden && !seen_nodes(&mind.seen).any(|s| s == node)
}
because!(hidden_node, CursorTree, "whether a node is hidden from a reading: marked hidden in the tree and not among the nodes the reading \
     discovered");

pub(crate) fn child_named(tree: &NodeTree, under: usize, name: &str) -> Option<usize> {
    if name == QUANTITY {
        return quantity_tag(tree, under);
    }
    if under == 0 {
        return tree.thing_named(name);
    }
    let children = &tree.node(under).children;
    let own = if children.len() <= WIDE_NODE { children.iter().copied().find(|&c| *tree.node(c).name == *name) } else { tree.named(name).find(|&c| c != 0 && tree.node(c).parent == under && !tree.node(c).gone) };
    own.or_else(|| tree.node(under).link.filter(|&thing| thing != under).and_then(|thing| child_named(tree, thing, name)))
}
because!(child_named, CursorTree, "the first child of a node with a name, in the order its children were added, or, when the node is a \
     link to a thing and has no such child, the thing's, since a mention of a thing stands for the thing and what the thing holds is read \
     from the mention, as the successor of the monday a story said today is: read from the children when the node has few, and from the \
     nodes of that name when it has many, as the world with a thing for every fact of a large state, since a child kept its parent even \
     when it is gone and a gone child is no longer among the children");

pub struct TreeIndexes;
source!(TreeIndexes, "the indexes the tree keeps beside its nodes so that a search on a state of millions of nodes asks for a name, a \
     thing, a holder or a linker in constant time: the user asked for ten million objects in the permanent state, and a scan of every node \
     for each question of a search does not scale to that");

pub(crate) const WIDE_NODE: usize = 32;
because!(WIDE_NODE, TreeIndexes, "how many children a node may have before a child is looked up by its name through the index instead of \
     along the children, about the width at which walking the list costs more than the lookup");

pub(crate) fn below(tree: &NodeTree, at: usize) -> Vec<usize> {
    let mut out: Vec<usize> = tree.node(at).children.iter().copied().collect();
    let mut next = 0;
    while next < out.len() {
        let more: Vec<usize> = tree.node(out[next]).children.iter().copied().collect();
        out.extend(more);
        next += 1;
    }
    out
}
because!(below, CursorTree, "every node below a node, the nearest first, each depth in the order the children were added");

pub(crate) fn above(tree: &NodeTree, at: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut node = at;
    while node != 0 {
        node = tree.node(node).parent;
        if node != 0 {
            out.push(node);
        }
    }
    out
}
because!(above, CursorTree, "every node above a node, the nearest first, the world left out");

pub(crate) fn holds_relations(mind: &CursorMind, node: usize) -> bool {
    mind.tree.node(node).children.iter().any(|&c| present(mind, c) && mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT))
}
because!(holds_relations, CursorTree, "whether a node holds a relation of its own, which makes it a thing whatever its name, as an hour of \
     the clock that has a successor");

pub(crate) fn named_present<'a>(mind: &'a CursorMind, name: &str) -> impl Iterator<Item = usize> + 'a {
    let tree = &mind.tree;
    tree.named(name).filter(move |&n| n != 0 && present(mind, n) && !hidden_node(mind, n) && !asked_node(tree, n) && tree.node(n).link.is_none() && !word_form(tree, n))
}
because!(named_present, CursorTree, "every node with a name that is present, seen, no link, no part of the question and no word the seeds \
     state a form of, as is holding will or met under meet, since such a word names no thing a question asks about while every question \
     says one, in the order the nodes were added and one at a time, for a rule that asks whether any or every such node is some way and \
     stops at the first that answers, since a relation name stands on a node for every fact of a large state");

pub(crate) fn named_nodes(mind: &CursorMind, name: &str) -> Vec<usize> {
    let tree = &mind.tree;
    let story_mention = |n: usize| tree.story(n) && tree.node(n).link.is_some();
    let tags: Vec<usize> = number_of(name).map(|n| super::quantity_tag_named(n)).map(|tag| tree.named(&tag).filter(|&t| t != 0 && present(mind, t) && !hidden_node(mind, t)).collect()).unwrap_or_default();
    let mut named: Vec<usize> = named_present(mind, name).chain(tree.named(name).filter(|&n| n != 0 && present(mind, n) && !hidden_node(mind, n) && !asked_node(tree, n) && story_mention(n))).chain(tags).collect();
    let past = |n: usize| question_time(mind).is_none() && stamped(tree, n).is_some();
    named.sort_by_key(|&n| (!super::is_quantity_tag(&tree.node(n).name), !story_mention(n), !tree.story(n), !shared(tree, n), past(n), tree.node(n).parent != 0, !of_question_time(mind, n)));
    named
}

pub(crate) fn concept_named(tree: &NodeTree, word: &str) -> Option<usize> {
    let top = |name: &str| tree.named(name).find(|&m| !tree.story(m) && !tree.node(m).gone && tree.node(m).parent == 0);
    let inner = word.strip_prefix(BRACE_OPEN_TEXT).and_then(|rest| rest.strip_suffix(crate::quiz::BRACE_CLOSE)).unwrap_or(word);
    if inner.contains(crate::quiz::WORD_GAP) {
        return top(word);
    }
    top(&format!("{BRACE_OPEN_TEXT}{inner}{}", crate::quiz::BRACE_CLOSE)).or_else(|| top(inner).and_then(|w| tree.node(w).link).filter(|&c| tree.node(c).parent == 0 && !tree.node(c).gone)).or_else(|| top(inner))
}

pub struct ConceptTags;
source!(ConceptTags, "the user's design of a concept's name: its kind and its word in one tag, {mammal dog}, {country france}, the number \
     tag with its number, the kind a property of the concept and not a child of it");

pub(crate) fn tag_kind(name: &str) -> Option<&str> {
    name.strip_prefix(BRACE_OPEN_TEXT).and_then(|rest| rest.strip_suffix(crate::quiz::BRACE_CLOSE)).and_then(|inner| inner.split_once(crate::quiz::WORD_GAP)).map(|(kind, _)| kind).filter(|kind| *kind != NUMBER_TAG)
}
because!(tag_kind, ConceptTags, "the kind a concept's tag names, mammal for {mammal dog}, since the user groups a concept under its kind \
     in its own name; none for a concept with no kind and for a number, whose tag names no kind");

const NUMBER_TAG: &str = "number";
because!(NUMBER_TAG, ConceptTags, "the tag of a number written as a concept, which names what it is and no kind");
because!(concept_named, CursorTree, "the concept a word names: the node under the world the state holds by the word, red for the relation \
     {red}, since a braced relation word is the world's word itself, a braced name the state holds, or the node the word links to, since \
     the seeds talk in concepts and a word of the text is the English side of one");

pub(crate) fn concept_reached(tree: &NodeTree, node: usize) -> Option<usize> {
    child_named(tree, node, EQUAL_RELATION).and_then(|r| tree.node(r).children.iter().copied().filter(|&c| !tree.node(c).gone).find_map(|c| tree.node(c).link.or_else(|| concept_named(tree, &tree.node(c).name))))
}
because!(concept_reached, CursorTree, "the concept a node reaches through its equal, as the English word cat reaches {cat}, the node its \
     equal links to or the concept its equal names");

pub(crate) fn worth_of(tree: &NodeTree, thing: usize) -> Option<usize> {
    let number_under = |n: usize| child_named(tree, n, EQUAL_RELATION).or_else(|| child_named(tree, n, WORTH_FORM.trim())).and_then(|r| tree.node(r).children.iter().copied().find(|&c| !tree.node(c).gone && number_of(&tree.node(c).name).is_some()));
    number_under(thing).or_else(|| concept_reached(tree, thing).and_then(number_under))
}
because!(worth_of, CursorTree, "the number a thing is worth, the number under its equal, as the word zero holds its number and a dozen \
     holds twelve, read through one concept when the word reaches one first, as a numeral reaches its roman concept and that its number; \
     none for a thing worth no number");

pub(crate) fn word_name(name: &str) -> String {
    name.strip_prefix(BRACE_OPEN_TEXT).and_then(|rest| rest.strip_suffix(crate::quiz::BRACE_CLOSE)).map_or_else(|| name.to_string(), |inner| inner.rsplit(crate::quiz::WORD_GAP).next().unwrap_or(inner).to_string())
}
because!(word_name, CursorTree, "the name a mention of a node takes: the word of a concept, cat for {cat} and dog for {mammal dog}, the \
     number for a number tag, since a mention the story writes is the word the text said and a braced name would read as a relation, and \
     any other name as it is");

pub(crate) fn number_below(mind: &CursorMind, relation: usize) -> Option<f32> {
    let tree = &mind.tree;
    if super::is_quantity_tag(&tree.node(relation).name) {
        return number_of(&tree.node(relation).name);
    }
    let values: Vec<usize> = tree.node(relation).children.iter().copied().filter(|&c| present(mind, c)).collect();
    values.iter().find_map(|&c| number_of(&tree.node(c).name)).or_else(|| values.iter().find_map(|&c| quantity_of(tree, c)))
}
because!(number_below, CursorTree, "the number a relation holds: a number right under it, or the quantity of the unit mention under it, \
     since a cost is dollars with a quantity and an age is years with a quantity, so a comparison or a plan reads the amount through the \
     unit");

pub(crate) fn amount_of(tree: &NodeTree, unit: usize) -> Option<usize> {
    quantity_tag(tree, unit)
}
because!(amount_of, CursorTree, "the amount node of a unit concept, the number under its quantity, which holds under equal what that many \
     of the unit equal");

pub(crate) fn has_worth(tree: &NodeTree, thing: usize) -> bool {
    worth_of(tree, thing).is_some()
}
because!(has_worth, CursorTree, "whether a thing is worth a number");

pub(crate) fn shared(tree: &NodeTree, node: usize) -> bool {
    tree.node(node).parent == 0 && (number_of(&tree.node(node).name).is_some() || has_worth(tree, node) || tree.unit(node))
}
because!(shared, CursorTree, "whether a thing under the world belongs to every story alike, a number, a number word that holds its worth \
     or a unit, since the reader's numbers and units are its own vocabulary and no story's, so a story counts its cats with the ten every \
     tree starts with and states that two is a number of the two that is worth two, while a story's things are its own beside the things \
     the state held");

pub(crate) fn last_answered(mind: &CursorMind) -> Option<usize> {
    let tree = &mind.tree;
    let current = tree.thing_named(QUESTION_NODE).filter(|&q| tree.story(q) && child_named(tree, q, super::ANSWER_RELATION).is_none());
    tree.things.get(QUESTION_NODE).into_iter().flat_map(|list| list.iter().rev().copied()).filter(|&q| tree.story(q) && !tree.node(q).gone && Some(q) != current).find_map(|q| child_named(tree, q, super::ANSWER_RELATION)).and_then(|holder| tree.node(holder).children.iter().copied().find(|&c| !tree.node(c).gone))
}
because!(last_answered, CursorTree, "the value the story's newest answered question holds under its answer relation, the question of the \
     turn being read aside, so a turn that types a sign and a number alone carries the sum on from the number last answered, found in the \
     tree and not on the stack, which a new turn empties");

because!(named_nodes, CursorTree, "the story's counts of the number first, since who has five apples looks for a count while a number \
     itself is found by the step on numbers; the story's own nodes before the state's, since the ball the story put in the box is the ball \
     asked about even beside the seed that knows balls; what is so now before what was so unless the question asks about the past, since \
     who has the kite asks for the holder now; then every node with a name that is present, seen, no link unless the story wrote it, since \
     a link stands for its thing while the story's mention of a thing the state knows is where the story's facts of it stand, as the pens \
     each owner has beside a state that knows a thing called pen, which a question about the most pens must land on once the stack no \
     longer shows them, and no fragment of \n the past, since what was so is found by its own step: the story's mentions first, then the \
     shared things of that name, a number, a number word or a unit, since the three a story counts apples with is the three the seeds know \
     the parity of and the ten cats a story counts are ten by the ten every tree starts with, then the story's nodes before the state's, a \
     thing or a value alike, since the sock a hidden world puts in a basket is the sock asked about beside a state that knows a thing \
     called sock; among the rest the things of that name, right under the world, \n and then the values that say it, and among each the \
     nodes the story told before those the state held when the \n story began, since the kim a story gives a hat is the story's kim and \
     not the novel the seeds know, while the three \n a story counts apples with is still the three the seeds know the parity of, each in \
     the order the nodes were added, \n so a find lands first on the thing that holds the facts of the name");

pub(crate) fn is_kind(mind: &CursorMind, word: &str) -> bool {
    let mut nodes = named_present(mind, &word_name(word)).peekable();
    nodes.peek().is_some() && nodes.all(|n| *mind.tree.node(mind.tree.node(n).parent).name == *IS_FORM.trim())
}
because!(is_kind, CursorTree, "whether a word names only kinds, standing only as a value under is, as day under {day monday} and color \
     under {color red}, which the tags of the seeds' concepts put there, so a question that says it asks for a thing of that kind and \
     never for the kind itself");

pub(crate) fn quoted(tree: &NodeTree, at: usize) -> bool {
    let opaque = |n: usize| QUOTING.contains(&&*tree.node(n).name) || *tree.node(n).name == *QUESTION_NODE;
    at != 0 && (opaque(at) || above(tree, at).iter().any(|&a| opaque(a)))
}

because!(quoted, CursorTree, "whether a node stands under the relation that holds what a sayer said, that relation itself included, or \
     under the question node, the answer it got included, so a value written there stays a plain value and never links to the thing it \
     names");

pub(crate) fn asked_node(tree: &NodeTree, at: usize) -> bool {
    at != 0 && (*tree.node(at).name == *QUESTION_NODE || above(tree, at).iter().any(|&a| *tree.node(a).name == *QUESTION_NODE))
}
because!(asked_node, CursorTree, "whether a node is the question node or stands under it, the answer it got included, so a lookup by name \
     never finds a question's own words as facts, nor an answer said for a story's thing, and the question is read only through its node \
     while its answer is reached by the step to the last answer alone");
