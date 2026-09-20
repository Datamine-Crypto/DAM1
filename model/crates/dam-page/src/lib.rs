#![deny(non_snake_case)]
#![deny(unreachable_patterns)]
#![forbid(unknown_lints)]

use dam::cursor::{CursorMind, PATH_MARK, settled};
use dam::events::STACK_SLOTS;
use dam::network::{NetworkClasses, Stacked};
use dam::quiz::{punctuated, sentences};
use dam::state::{context_bytes, context_mind, state_mind};
use dam::word::{read_by_words, taught_words, WordStep, OWNER_TAG};
use dam::words::simple_words;
use patterns::{because, source};
use serde::Serialize;
use std::cell::RefCell;

pub struct PageEngine;
source!(
    PageEngine,
    "the chat page's engine: the word network run in the page, loaded once from its weights, its step classes and the state every chat starts from, reading each turn as the console reads a turn line and keeping the tree and the stack from one turn to the next"
);

struct Engine {
    nets: Vec<Stacked>,
    words: Vec<WordStep>,
    seeded: CursorMind,
    mind: CursorMind,
}
because!(Engine, PageEngine, "what the page's engine holds: the networks that read by vote, the step each output names, the mind the state file holds, and the mind of the chat so far");

thread_local! {
    static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
}

const NOT_LOADED: &str = "the engine has no network loaded";
because!(NOT_LOADED, PageEngine, "what a call before the network is loaded says");

#[derive(Serialize)]
struct Reading {
    steps: Vec<(String, Vec<String>)>,
    output: Vec<String>,
    ended: bool,
    told: Vec<String>,
}
because!(Reading, PageEngine, "a turn as the page receives it: the steps taken at each input item by their classes, the output the turn said, whether every item continued within the step limit, and the paths of the tree the turn wrote, so a statement that says nothing back still shows what it told");

const TWIN_MARK: &str = "#";
because!(TWIN_MARK, PageEngine, "the mark between the name of a thing and its number among the things of that name under one holder, so the page draws two cars a person has as two");

const RELATION_OPEN: &str = "{";
because!(RELATION_OPEN, PageEngine, "the brace a relation's name opens with, which is never numbered since a holder has one relation of a name");

fn twin_name(mind: &CursorMind, at: usize) -> String {
    let tree = &mind.tree;
    let node = tree.node(at);
    let older = tree.node(node.parent).children.iter().filter(|&&other| other < at && other >= tree.state && !tree.node(other).gone && tree.node(other).name == node.name).count();
    let part = usize::from(node.parent != 0 && tree.node(node.parent).name == node.name);
    let older = older + part;
    if older == 0 || node.name.starts_with(RELATION_OPEN) || node.parent == 0 {
        node.name.to_string()
    } else {
        format!("{}{}{}", node.name, TWIN_MARK, older + 1)
    }
}
because!(twin_name, PageEngine, "the name a path writes a node by: its own, and for a thing with older things of its name under the same holder, or a smaller group inside a group of its name, its number among them after the mark, since two cars of one person are two nodes of one name");

const HAS_RELATION: &str = "{has}";
because!(HAS_RELATION, PageEngine, "the relation the page writes between an owner and a thing that stands with it, tom has a car, in place of the owner tag the tree keeps under the thing, as a person reads owning");

fn owned_here(mind: &CursorMind, thing: usize) -> bool {
    let tree = &mind.tree;
    let holder = tree.node(thing).parent;
    thing != 0 && holder != 0 && !tree.node(holder).name.starts_with(RELATION_OPEN) && tree.node(thing).children.iter().any(|&tag| !tree.node(tag).gone && &*tree.node(tag).name == OWNER_TAG && tree.node(tag).children.iter().any(|&owner| !tree.node(owner).gone && tree.node(owner).link == Some(holder)))
}
because!(owned_here, PageEngine, "whether a thing stands with its owner: its owner tag names the thing it stands in, so the page writes owner has thing and leaves the tag out");

fn story_paths(mind: &CursorMind) -> Vec<String> {
    let tree = &mind.tree;
    (tree.state..tree.len()).filter(|&n| tree.node(n).children.is_empty() && !tree.node(n).gone).filter_map(|leaf| {
        let mut names = Vec::new();
        let mut at = leaf;
        while at != 0 {
            if Some(at) == mind.question_start {
                return None;
            }
            if owned_here(mind, tree.node(at).parent) && &*tree.node(at).name == OWNER_TAG {
                names.retain(|name: &String| name.starts_with(RELATION_OPEN));
                at = tree.node(at).parent;
                continue;
            }
            names.push(twin_name(mind, at));
            if owned_here(mind, at) {
                names.push(HAS_RELATION.to_string());
            }
            at = tree.node(at).parent;
        }
        names.reverse();
        Some(names.join(PATH_MARK))
    }).fold(Vec::new(), |mut paths: Vec<String>, path| {
        if !paths.contains(&path) {
            paths.push(path);
        }
        paths
    })
}
because!(story_paths, PageEngine, "the paths of the tree from the world down to every leaf the story added, a thing that stands with its owner written as owner has thing with the past of the owning after the thing, each path once, the state's own nodes and the words of an open question left out, since a question tells nothing, as a person reads them");

#[derive(Serialize)]
struct Settings {
    classes: Vec<String>,
    steps: usize,
    slots: usize,
    hidden: usize,
    voters: usize,
}
because!(Settings, PageEngine, "the network's step classes, the step limit the engine reads with, how much of the stack a network reads, its hidden units and how many networks read by vote, as the page shows them");

fn with_engine<T>(work: impl FnOnce(&mut Engine) -> Result<T, String>) -> Result<T, String> {
    ENGINE.with(|held| held.borrow_mut().as_mut().ok_or_else(|| NOT_LOADED.to_string()).and_then(work))
}
because!(with_engine, PageEngine, "a piece of work on the loaded engine, refused when no network is loaded");

pub fn load(weights: &[u8], classes: &str) -> Result<usize, String> {
    let named: NetworkClasses = serde_json::from_str(classes).map_err(|e| e.to_string())?;
    let words = named.classes.iter().map(|c| WordStep::of_class(c).ok_or(format!("{c} is no step of the word network"))).collect::<Result<Vec<_>, _>>()?;
    let nets = Stacked::read_voters_wide(weights, named.stacked, named.classes.len(), named.voters.max(usize::from(true)), named.half)?;
    let numbers = nets.iter().map(Stacked::numbers).sum();
    ENGINE.with(|held| *held.borrow_mut() = Some(Engine { nets, words, seeded: CursorMind::default(), mind: CursorMind::default() }));
    Ok(numbers)
}
because!(load, PageEngine, "the network loaded from its weights and its classes file, or the several networks that read by vote when the classes file counts more than one and the weights hold them one after another, every class read back as the step of the reading one word at a time it names, refused when one names no step, and the count of its numbers");

pub fn state(bytes: &[u8]) -> Result<usize, String> {
    let mind = state_mind(bytes)?;
    let nodes = mind.tree.len();
    with_engine(|engine| {
        engine.seeded = mind.clone();
        engine.mind = mind;
        Ok(nodes)
    })
}
because!(state, PageEngine, "the mind every chat starts from, read whole from the state file the console wrote, and how many nodes its tree holds");

pub fn context() -> Result<Vec<u8>, String> {
    with_engine(|engine| {
        settled(&mut engine.mind);
        context_bytes(&engine.seeded, &engine.mind)
    })
}
because!(context, PageEngine, "the chat's mind as the bytes of its context, a delta on the state every chat starts from, the nodes its newest turn discovered marked seen in its tree first, since the context holds the tree and not the list of a reading, for the page to keep with the chat");

pub fn restore(bytes: &[u8]) -> Result<(), String> {
    with_engine(|engine| {
        engine.mind = context_mind(&engine.seeded, bytes)?;
        Ok(())
    })
}
because!(restore, PageEngine, "the chat's mind put back from its context laid over the state every chat starts from, so a reopened chat goes on where it was without reading its prompts again");

pub fn forget() -> Result<(), String> {
    with_engine(|engine| {
        engine.mind = engine.seeded.clone();
        Ok(())
    })
}
because!(forget, PageEngine, "the chat's mind taken back to the one the state file holds");

pub fn described(text: &str) -> Result<String, String> {
    with_engine(|engine| {
        let before = story_paths(&engine.mind);
        for sentence in sentences(simple_words(text)) {
            taught_words(&mut engine.mind, &punctuated(sentence));
        }
        let told = story_paths(&engine.mind).into_iter().filter(|path| !before.contains(path)).collect();
        serde_json::to_string(&Reading { steps: Vec::new(), output: Vec::new(), ended: true, told }).map_err(|e| e.to_string())
    })
}
because!(described, PageEngine, "a world described to the page and not said to the network: the teacher's own rules write each sentence into the chat's mind, exactly and with no move of the network, so a game can set a world the network has to explore, and what the description wrote is given back");

pub fn read(text: &str, steps: usize) -> Result<String, String> {
    with_engine(|engine| {
        let mut answer = Reading { steps: Vec::new(), output: Vec::new(), ended: true, told: Vec::new() };
        let before = story_paths(&engine.mind);
        for sentence in sentences(simple_words(text)) {
            let words = punctuated(sentence);
            let reading = read_by_words(&engine.mind, &words, (&engine.nets, &engine.words), steps);
            answer.steps.extend(reading.trail);
            answer.output.extend(reading.mind.output.iter().cloned());
            answer.ended &= reading.ended;
            engine.mind = reading.mind;
        }
        answer.told = story_paths(&engine.mind).into_iter().filter(|path| !before.contains(path)).collect();
        serde_json::to_string(&answer).map_err(|e| e.to_string())
    })
}
because!(
    read,
    PageEngine,
    "a turn read by the networks on the chat's mind within the steps given, sentence by sentence of the message, each closed with a full stop or a question mark when it has none, each word of a sentence one input; the steps and the outputs of every sentence gathered as one reading returned as JSON, the mind after the last kept for the next turn"
);

#[derive(Serialize)]
struct WorldNode {
    name: String,
    told: bool,
    names: Option<String>,
    children: Vec<WorldNode>,
}
because!(WorldNode, PageEngine, "one node of the mind's tree as the page's explorer shows it: its name, whether the chat told it or the state held it, the thing a mention names, and the nodes inside it");

fn world_node(mind: &CursorMind, n: usize) -> WorldNode {
    let tree = &mind.tree;
    let children = tree.node(n).children.iter().copied().filter(|&c| !tree.node(c).gone).map(|c| world_node(mind, c)).collect();
    WorldNode { name: tree.node(n).name.to_string(), told: tree.story(n), names: tree.node(n).link.map(|l| tree.node(l).name.to_string()), children }
}
because!(world_node, PageEngine, "a node of the tree with everything inside it, the removed nodes left out");

pub fn world() -> Result<String, String> {
    with_engine(|engine| {
        let tree = &engine.mind.tree;
        let tops: Vec<WorldNode> = tree.node(0).children.iter().copied().filter(|&c| !tree.node(c).gone).map(|c| world_node(&engine.mind, c)).collect();
        serde_json::to_string(&tops).map_err(|e| e.to_string())
    })
}
because!(world, PageEngine, "the whole tree of the chat's mind as JSON, every thing under the world with what stands inside it, for the page to show what the model holds");

const KIND_STEPS: usize = 5;
because!(KIND_STEPS, PageEngine, "how many kinds above a thing the page is told, a rat a rodent a mammal an animal, which is as far as the facts go");

fn kind_above(mind: &CursorMind, name: &str) -> Option<String> {
    let tree = &mind.tree;
    let is = dam::quiz::IS_FORM.trim();
    let named = |n: usize, wanted: &str| tree.node(n).children.iter().copied().find(|&c| !tree.node(c).gone && &*tree.node(c).name == wanted);
    let tops = tree.named(name).filter(|&n| n != 0 && !tree.node(n).gone && tree.node(n).parent == 0);
    tops.filter_map(|n| named(n, is)).flat_map(|under| tree.node(under).children.iter().copied().filter(|&c| !tree.node(c).gone).collect::<Vec<_>>()).map(|c| tree.node(c).name.to_string()).find(|kind| !kind.starts_with(dam::quiz::BRACE_OPEN))
}
because!(kind_above, PageEngine, "what the tree says a thing of a name is, the first plain name under its is, a rat a rodent, or none when the tree says nothing of it");

const GENDER_RELATION: &str = "{gender}";
because!(GENDER_RELATION, PageEngine, "the relation the facts give a first name, tom and anna, by which the page knows a name with no kind of its own is a person's");

const PERSON_KIND: &str = "person";
because!(PERSON_KIND, PageEngine, "the kind the page is told for a first name the facts give a gender and nothing else");

fn first_name(mind: &CursorMind, name: &str) -> bool {
    let tree = &mind.tree;
    tree.named(name).any(|n| n != 0 && !tree.node(n).gone && !tree.story(n) && tree.node(n).parent == 0 && tree.node(n).children.iter().any(|&c| !tree.node(c).gone && &*tree.node(c).name == GENDER_RELATION))
}
because!(first_name, PageEngine, "whether the facts hold a name as a first name, a thing under the world with a gender");

pub fn kinds(names: &str) -> Result<String, String> {
    with_engine(|engine| {
        let mut told = std::collections::BTreeMap::new();
        for name in names.split_whitespace() {
            let mut chain: Vec<String> = Vec::new();
            let mut at = name.to_string();
            while chain.len() < KIND_STEPS {
                let Some(kind) = kind_above(&engine.mind, &at).filter(|kind| !chain.contains(kind) && kind != name) else { break };
                chain.push(kind.clone());
                at = kind;
            }
            if chain.is_empty() && first_name(&engine.mind, name) {
                chain.push(PERSON_KIND.to_string());
            }
            told.insert(name.to_string(), chain);
        }
        serde_json::to_string(&told).map_err(|e| e.to_string())
    })
}
because!(kinds, PageEngine, "the kinds the tree gives each of some names, one above another, as JSON, so the page groups the things it draws by what the network's own facts say they are and not by a list of its own");

pub fn settings(steps: usize) -> Result<String, String> {
    with_engine(|engine| {
        let classes = engine.words.iter().map(WordStep::class).collect();
        let shown = Settings { classes, steps, slots: STACK_SLOTS, hidden: engine.nets[0].hidden, voters: engine.nets.len() };
        serde_json::to_string(&shown).map_err(|e| e.to_string())
    })
}
because!(settings, PageEngine, "the network's step classes and the settings the engine reads with, as JSON");
