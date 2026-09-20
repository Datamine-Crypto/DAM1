use crate::cursor::{CursorMind, InputLooks, NodeTree, TreeNode};
use crate::events::{Item, Stack};
use patterns::{because, source};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::sync::Arc;

pub struct StateFile;
source!(
    StateFile,
    "the permanent state every chat starts from, built once by the console from every seeds file and written as bytes the page loads \
     whole, so no page parses a statement: the tree's nodes in their order, each with its name, its parent, whether it is gone and whether \
     it is hidden, packed as MessagePack, the smallest form the workspace already reads"
);

#[derive(Serialize, Deserialize)]
struct Stored {
    names: Vec<String>,
    parents: Vec<u32>,
    gone: Vec<bool>,
    hidden: Vec<bool>,
    links: Vec<Option<u32>>,
}
because!(Stored, StateFile, "the tree as the file holds it: one entry per node in the tree's order, the world first, its children left out \
     since they follow from the parents, and the thing each value links to when it does");

fn stored_nodes(nodes: &[&TreeNode]) -> Result<Stored, String> {
    Ok(Stored {
        names: nodes.iter().map(|n| n.name.to_string()).collect(),
        parents: nodes.iter().map(|n| u32::try_from(n.parent).map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?,
        gone: nodes.iter().map(|n| n.gone).collect(),
        hidden: nodes.iter().map(|n| n.hidden).collect(),
        links: nodes.iter().map(|n| n.link.map(|l| u32::try_from(l).map_err(|e| e.to_string())).transpose()).collect::<Result<Vec<_>, _>>()?,
    })
}
because!(stored_nodes, StateFile, "a run of nodes as the file holds them, refused when a parent does not fit the file's numbers");

fn nodes_of(stored: Stored, offset: usize, total: usize) -> Result<Vec<TreeNode>, String> {
    let count = stored.names.len();
    if stored.parents.len() != count || stored.gone.len() != count || stored.hidden.len() != count || stored.links.len() != count {
        return Err("the file's columns differ in length".to_string());
    }
    let mut nodes: Vec<TreeNode> = Vec::with_capacity(count);
    for (at, name) in stored.names.into_iter().enumerate() {
        let parent = stored.parents[at] as usize;
        let place = offset + at;
        if parent >= total || (place > 0 && parent >= place) {
            return Err(format!("the file's node {place} names a parent that is not before it"));
        }
        let link = stored.links[at].map(|l| l as usize);
        if link.is_some_and(|l| l >= total) {
            return Err(format!("the file's node {place} links to a node the tree does not hold"));
        }
        nodes.push(TreeNode { name: Arc::from(name), parent, children: imbl::Vector::new(), gone: stored.gone[at], hidden: stored.hidden[at], link });
    }
    Ok(nodes)
}
because!(nodes_of, StateFile, "the nodes a file's run holds, read back from the place they start at, every parent coming before its child \
     within the whole tree of the given size, and every link to a node the tree holds");

fn with_children(nodes: Vec<TreeNode>) -> NodeTree {
    NodeTree::from_nodes(nodes)
}
because!(with_children, StateFile, "the nodes with their children listed again from their parents, a gone node kept in its place but not \
     among its parent's children, in the order the nodes were added");

pub fn state_bytes(mind: &CursorMind) -> Result<Vec<u8>, String> {
    rmp_serde::to_vec(&stored_nodes(&mind.tree.iter().collect::<Vec<_>>())?).map_err(|e| e.to_string())
}
because!(state_bytes, StateFile, "a mind's tree as the bytes of the state file");

pub fn state_mind(bytes: &[u8]) -> Result<CursorMind, String> {
    let stored: Stored = rmp_serde::from_slice(bytes).map_err(|e| e.to_string())?;
    let total = stored.names.len();
    let mut nodes = with_children(nodes_of(stored, 0, total)?);
    nodes.state = nodes.len();
    Ok(CursorMind { tree: nodes, ..CursorMind::default() })
}
because!(
    state_mind,
    StateFile,
    "the mind the state file holds: its tree read back node by node, every parent coming before its child, a gone node kept in its place \
     but not among its parent's children, the children in the order the nodes were added, with nothing on the stack, as the mind the seeds \
     make"
);

#[derive(Serialize, Deserialize)]
struct Context {
    pack: u64,
    base: u32,
    added: Stored,
    changed: Vec<(u32, bool, bool)>,
    at: u32,
    first_mark: Option<u32>,
    second_mark: Option<u32>,
    items: Vec<Item>,
    output: Vec<String>,
    ended: bool,
    number: Option<f32>,
    places: Option<u32>,
    held: Vec<u32>,
    steps: u32,
    looks: InputLooks,
}
because!(
    Context,
    StateFile,
    "a chat's mind as a delta on the pack: a fingerprint of the pack and how many nodes it has, the nodes the chat added after them, the \
     pack nodes whose gone or hidden mark the chat changed, where the cursor stands and what its pointers memorized, the stack's events \
     oldest first, the output and whether it ended, the number worked out and its places, the mentions of every name, the nodes held, the \
     steps taken and what the newest input's steps did"
);

fn pack_fingerprint(pack: &CursorMind) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    for node in pack.tree.iter() {
        node.name.hash(&mut hasher);
        node.parent.hash(&mut hasher);
    }
    hasher.finish()
}
because!(pack_fingerprint, StateFile, "a hash of the pack's names and parents, so a context saved on one pack is never laid over another");

fn narrowed(value: usize) -> Result<u32, String> {
    u32::try_from(value).map_err(|e| e.to_string())
}
because!(narrowed, StateFile, "a place in the tree or on the stack as the file's number, refused when it does not fit");

pub fn context_bytes(pack: &CursorMind, mind: &CursorMind) -> Result<Vec<u8>, String> {
    let base = pack.tree.len();
    if mind.tree.len() < base || mind.tree.iter().zip(pack.tree.iter()).any(|(m, p)| m.name != p.name || m.parent != p.parent) {
        return Err("the mind does not start from the pack".to_string());
    }
    let changed = mind.tree.iter().zip(pack.tree.iter()).enumerate().filter(|(_, (m, p))| m.gone != p.gone || m.hidden != p.hidden).map(|(at, (m, _))| Ok((narrowed(at)?, m.gone, m.hidden))).collect::<Result<Vec<_>, String>>()?;
    let mut items: Vec<Item> = mind.stack.items().map(|e| e.item.clone()).collect();
    items.reverse();
    let context = Context {
        pack: pack_fingerprint(pack),
        base: narrowed(base)?,
        added: stored_nodes(&mind.tree.iter().skip(base).collect::<Vec<_>>())?,
        changed,
        at: narrowed(mind.at)?,
        first_mark: mind.first_mark.map(narrowed).transpose()?,
        second_mark: mind.second_mark.map(narrowed).transpose()?,
        items,
        output: mind.output.clone(),
        ended: mind.ended,
        number: mind.number,
        places: mind.places.map(narrowed).transpose()?,
        held: mind.held.iter().map(|&n| narrowed(n)).collect::<Result<Vec<_>, String>>()?,
        steps: narrowed(mind.steps)?,
        looks: mind.looks.clone(),
    };
    rmp_serde::to_vec(&context).map_err(|e| e.to_string())
}
because!(context_bytes, StateFile, "a chat's mind as the bytes of its context, a delta on the pack it started from, refused when the mind \
     does not start from that pack");

pub fn context_mind(pack: &CursorMind, bytes: &[u8]) -> Result<CursorMind, String> {
    let context: Context = rmp_serde::from_slice(bytes).map_err(|e| e.to_string())?;
    let base = pack.tree.len();
    if context.pack != pack_fingerprint(pack) || context.base as usize != base {
        return Err("the context was saved on another pack".to_string());
    }
    let total = base + context.added.names.len();
    let mut nodes: Vec<TreeNode> = pack.tree.iter().cloned().collect();
    nodes.extend(nodes_of(context.added, base, total)?);
    for (at, gone, hidden) in context.changed {
        let node = nodes.get_mut(at as usize).filter(|_| (at as usize) < base).ok_or_else(|| format!("the context changes node {at}, which the pack lacks"))?;
        node.gone = gone;
        node.hidden = hidden;
    }
    let nodes = with_children(nodes);
    let within = |place: u32| -> Result<usize, String> { ((place as usize) < total).then_some(place as usize).ok_or_else(|| format!("the context points at node {place}, which the tree lacks")) };
    let mut stack = Stack::default();
    for item in context.items {
        stack.push(item);
    }
    Ok(CursorMind {
        tree: nodes,
        seen: None,
        at: within(context.at)?,
        first_mark: context.first_mark.map(within).transpose()?,
        second_mark: context.second_mark.map(within).transpose()?,
        stack,
        output: context.output,
        ended: context.ended,
        number: context.number,
        places: context.places.map(|p| p as usize),
        held: context.held.into_iter().map(within).collect::<Result<Vec<_>, _>>()?,
        flags: Vec::new(),
        steps: context.steps as usize,
        looks: context.looks,
        known: Arc::new(HashSet::new()),
        asked: Vec::new(),
        question_start: None,
        preferred: Arc::new(Vec::new()),
        own_relations: Arc::new(HashSet::new()),
        before: Vec::new(),
        ahead: Vec::new(),
        plan: None,
        last_topic: None,
        sentence_from: usize::default(),
        cause_of: None,
    })
}
because!(
    context_mind,
    StateFile,
    "a chat's mind read back from its context laid over the pack: the pack's nodes, the added nodes after them, the changed marks applied, \
     the children listed again, no node discovered and not yet marked, since a context is saved after the marks are set, the stack pushed \
     event by event oldest first so every depth and number comes back as it was, and every place checked against the tree, refused when \
     the context was saved on another pack"
);
