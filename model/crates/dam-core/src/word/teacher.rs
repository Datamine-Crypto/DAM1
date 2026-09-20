use super::mind::{mark_word, FLAG_HAND, GIVING, noun_word, open_question, slot_of_text, flag_of, heard_word, is_number, kind_of, node_kind, past_form, place_word, quality_word, singular, slot_of_word, verb_base, word_classes, ARTICLE_FLAGS, ASKING, CONTAINING, COPULA, FLAG_GIVE, HAVING, HELPERS, MOVING, PERSON_PRONOUNS, SKIPPED, THING_PRONOUNS};
use super::moves::{WordMove, WordStep, KINDS};
use super::physics::word_stepped;
use super::WordReading;
use crate::cursor::{CursorMind, BRACE_OPEN_TEXT};
use crate::quiz::{APOSTROPHE, IS_FORM};
use patterns::{because, source};
use super::walks::{plan_answers, number_walk, kept_walks, walk_kept};
use super::asked::{asked_word};

pub struct WordGame;
source!(
    WordGame,
    "the user's game one word at a time: the word is a fact, a thing word making the thing appear, and he must take the fewest moves that \
     bring the world to the shape the lesson expects; the teacher writes those moves from the cursor's kind, what he holds, the flags and \
     the word's class, and every move is executed on the virtual world, so a lesson is taught only when the world it leaves has the shape"
);

const AT_WORLD: &str = "world";
const ON_THING: &str = "thing";
const ON_PROPERTY: &str = "property";
because!(AT_WORLD, WordGame, "the cursor's kind at the start of a sentence, where a thing word makes the thing appear");
because!(ON_THING, WordGame, "the cursor's kind on a thing, which a place word or a moving verb grabs and a copula or a verb writes on");
because!(ON_PROPERTY, WordGame, "the cursor's kind on the is a copula just added, from which a place word steps up to the thing before it \
     grabs it");

fn things_inside(mind: &CursorMind, at: usize) -> usize {
    mind.tree.node(at).children.iter().filter(|&&c| !mind.tree.node(c).gone && !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)).count()
}
because!(things_inside, WordGame, "how many things stand inside a node, for the teacher to see a thing that only just appeared");

pub(super) fn punctuation(word: &str) -> bool {
    mark_word(word)
}
because!(punctuation, WordReading, "whether a word is a mark that ends or parts the sentence and no word of it");

pub fn taught_words(mind: &mut CursorMind, words: &[String]) -> Vec<Vec<WordStep>> {
    taught_words_answered(mind, words, None)
}

#[derive(Clone, PartialEq)]
pub(super) enum WalkTarget {
    Asked(usize),
    Worked(usize),
}
because!(WalkTarget, WordGame, "what a move of a kept number walk points at, apart from the numbers of one question: the word of the \
     question by its place, or the result the walk had worked out by then, by how many moves came before it");

pub(super) type KeptWalk = Vec<(WordMove, Option<WalkTarget>)>;
because!(KeptWalk, WordGame, "a number walk kept apart from the numbers of the question it was found for");

pub(super) static KEPT_WALKS: std::sync::Mutex<Vec<(String, KeptWalk, usize)>> = std::sync::Mutex::new(Vec::new());
because!(KEPT_WALKS, WordGame, "the number walks the teacher found, each under the shape of its question with how many questions it \
     answered, so one shape of question is taught one walk and never a second one that numbers happening to agree let through, forty after \
     ten percent off as ten percent of forty times something");

pub(super) const SHAPE_DIGITS: &str = "#";
pub(super) const SHAPE_OPEN: &str = "_";
pub(super) const SHAPE_PART: &str = "%";
because!(SHAPE_PART, WordGame, "what stands for a word of a part in the shape of a question, a third or a quarter, so every fraction of a thing is taught one walk");
pub(super) const SHAPE_GAP: &str = " ";
because!(SHAPE_DIGITS, WordGame, "what stands for a number said in digits in the shape of a question");
because!(SHAPE_OPEN, WordGame, "what stands for a thing or a value in the shape of a question, a word a lesson may swap");
because!(SHAPE_GAP, WordGame, "what joins the words of the shape of a question");

pub fn taught_words_answered(mind: &mut CursorMind, words: &[String], answer: Option<&str>) -> Vec<Vec<WordStep>> {
    let mut all = Vec::new();
    for (index, word) in words.iter().enumerate() {
        let done = super::physics::clause_done(mind);
        heard_word(mind, word, index);
        let claiming = super::physics::claim_node(mind, mind.at) && open_question(mind).is_none() && COPULA.contains(&word.as_str());
        let start = !claiming && (index == 0 || words[..index].iter().all(|w| super::mind::FILLERS.contains(&w.as_str()) || mark_word(w)) || (ASKING.contains(&word.as_str()) && !super::mind::RELATIVES.contains(&word.as_str()) && open_question(mind).is_none()) || (done && (REQUESTS.contains(&word.as_str()) || QUESTION_OPENERS.contains(&word.as_str()) || COPULA.contains(&word.as_str()) && !mind.before.iter().any(|b| super::mind::RELATIVES.contains(&b.as_str())) && mind.before.iter().rev().skip(1).take_while(|b| !mark_word(b) && !super::mind::CLAUSE_BREAKS.contains(&b.as_str())).any(|b| COPULA.contains(&b.as_str()) || HAVING.contains(&b.as_str())))) || super::physics::clause_done(mind));
        let closing = super::mind::closing_mark(word) && open_question(mind).is_some();
        let numbered = answer.filter(|a| closing && crate::words::number_of(a).is_some());
        let taught: Vec<(WordMove, Option<String>)> = if let Some(answer) = numbered {
            let ruled = asked_word(mind, word, start);
            if plan_answers(mind, &ruled, index, answer) {
                ruled
            } else {
                let kept = kept_walks(mind).into_iter().find(|walk| plan_answers(mind, walk, index, answer));
                let walk = kept.or_else(|| number_walk(mind, words, answer).filter(|walk| plan_answers(mind, walk, index, answer)));
                if let Some(walk) = &walk {
                    walk_kept(mind, walk);
                }
                walk.unwrap_or(ruled)
            }
        } else if open_question(mind).is_some() || opens_question(word, start) { asked_word(mind, word, start) } else { taught_word(mind, word).into_iter().map(|m| (m, None)).collect() };
        let mut placed = Vec::new();
        for (act, target) in taught.into_iter().chain(std::iter::once((WordMove::Continue, None))) {
            let at = if !act.points() { None } else if let Some(text) = target { slot_of_text(&mind.stack, &text).or_else(|| super::mind::slot_of_worked(&mind.stack, &text)) } else { slot_of_word(&mind.stack, index) };
            let ws = WordStep { act, at };
            word_stepped(mind, &ws);
            placed.push(ws);
        }
        all.push(placed);
    }
    all
}
because!(taught_words, WordReading, "the moves the teacher writes for a sentence, one list per word, a question's by the question's rules, \
     each pointing at the word heard or at the word of the dumped question its walk names, each word's moves decided from the cursor, what \
     he holds, the flags and the word, and executed on the virtual world as they are written, since the stack is emptied at every word; \
     every word ends with continue");
because!(taught_words_answered, WordGame, "a request, a helper or a copula said right after a finished clause starts a question, a copula \
     only when the clause had its own copula or word of having and no relative word of the sentence still waits for one, i have a dog do i \
     have a dog; the teacher's steps for a sentence, told the answer its question has when the lesson gives one: a number answer its own \
     rules do not reach is looked for as a plan of moves on the number");

pub(super) const MUCH_SPAN: usize = 2;
because!(MUCH_SPAN, WordGame, "how many words a question of how much about one thing holds, much and the thing, how much is the book");

pub(super) fn opens_question(w: &str, start: bool) -> bool {
    start && (ASKING.contains(&w) || COPULA.contains(&w) || QUESTION_OPENERS.contains(&w) || REQUESTS.contains(&w))
}
because!(opens_question, WordGame, "whether a word opens a question: the first word of a sentence that asks, what or where, or a copula or \
     a helper said first, is the car red or does tom have corn, the words a speaker adds before it, check again, not counted");

pub(super) const REQUESTS: [&str; 2] = ["tell", "say"];
because!(REQUESTS, WordGame, "the words that open a request for what the world holds or for the last answer, which is a question with no \
     mark, tell me about the bike and say it again");

pub(super) const QUESTION_OPENERS: [&str; 4] = ["does", "do", "did", super::mind::ABLE];
because!(QUESTION_OPENERS, WordGame, "the helpers that open a yes or no question about a deed");

pub(super) const WHERE_FORM: &str = "where";
pub(super) const WHEN_FORM: &str = "when";
because!(WHEN_FORM, WordGame, "the question that asks when a thing is, read as where it stands in time, the party is at noon");
pub(super) const WHOSE_FORM: &str = "whose";
pub(super) const WHAT_FORM: &str = "what";
pub(super) const CHOICE_FORM: &str = "which";
because!(WHAT_FORM, WordGame, "the question that asks a thing, its contents, its kind or, with a quality alone, the thing holding it");
because!(CHOICE_FORM, WordGame, "the question that asks one of the things it names, which has more legs a dog or a spider");
because!(WHOSE_FORM, WordGame, "the question that asks the owner of a thing named by its quality, whose cat is black");
pub(super) const HOW_FORM: &str = "how";
because!(HOW_FORM, WordGame, "the question that asks a kind through its verb, how does kim feel, which gets the property the verb names");
pub(super) const WHO_FORM: &str = "who";
because!(WHERE_FORM, WordGame, "the question that asks where a thing stands: its parent");
because!(WHO_FORM, WordGame, "the question that asks who holds a thing: its parent too, since what a thing has stands inside it");

fn present_children_counted(mind: &CursorMind, at: usize) -> bool {
    mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone && mind.tree.node(c).name.starts_with(crate::cursor::QUANTITY_TAG))
}
because!(present_children_counted, WordGame, "whether the value the cursor stands on carries a count, the years of five years old");

const ALIKE: &str = "like";
because!(ALIKE, WordGame, "the word that after is says what a thing resembles, time is like a river, written as a relation of its own as about is");

pub(super) const CURIOUS_WORDS: [&str; 2] = ["curious", "interested"];
because!(CURIOUS_WORDS, WordGame, "the words that ask the network what it wants to hear more of, what are you curious about, which it answers \
     with the thing the last sentence told of, the one that matters most to it");

const SELF_HOLDING: [&str; 3] = ["hold", "have", "carry"];
because!(SELF_HOLDING, WordGame, "the verbs of having whose going form at the head of a sentence with no subject tells what the speaker has, holding five cards");

fn going_open(mind: &CursorMind, at: usize) -> bool {
    let named = |n: usize| crate::cursor::bare_name(&mind.tree.node(n).name);
    at != 0 && mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) && (named(at) == super::mind::GOING_RELATION || super::mind::MOVING.contains(&named(at).as_str()) && named(at) != super::mind::FITTING)
}
because!(going_open, WordGame, "whether the cursor stands on a going that has taken no place yet, where the word from names where the thing came and not where it went");

fn going_moves(mind: &CursorMind, w: &str) -> Vec<WordMove> {
    if verb_base(mind, w).is_some_and(|base| base == super::mind::FITTING) {
        return vec![WordMove::Grab];
    }
    if past_form(mind, w) || w.ends_with(super::mind::PAST_END) {
        return vec![WordMove::AddRelationPast];
    }
    vec![WordMove::AddRelation]
}
because!(going_moves, WordGame, "what a verb of moving has the reading do: the thing is taken, so the place after to can hold it, and a deed told in the past is written as a relation of its own, dated, so what was done and when it was done stay on the tree");

fn going_bases(word: &str) -> Vec<String> {
    let Some(stem) = word.strip_suffix(super::mind::GOING_END) else { return Vec::new() };
    let mut bases = vec![stem.to_string(), format!("{stem}e")];
    let mut letters = stem.chars().rev();
    if letters.next().is_some_and(|last| letters.next() == Some(last)) {
        bases.push(stem[..stem.len() - usize::from(true)].to_string());
    }
    bases
}
because!(going_bases, WordGame, "the plain forms a going word may come from: the word without its ending, with an e put back, driving from drive, and \
     with a doubled last letter made single, travelling from travel");

fn taught_word(mind: &CursorMind, w: &str) -> Vec<WordMove> {
    let at = mind.at;
    let kind = node_kind(mind, at);
    let held = mind.held.last().copied().filter(|&h| !mind.tree.node(h).gone);
    if at == 0 && held.is_none() && mind.before.split_last().is_some_and(|(_, earlier)| super::mind::opens_story(earlier, w)) {
        return vec![WordMove::PointNothing];
    }
    let plain = at != 0 && !mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT);
    if at == 0 && held.is_none() && mind.before.len() == 1 && going_bases(w).iter().any(|base| SELF_HOLDING.contains(&base.as_str())) {
        return vec![WordMove::StepUser, WordMove::Give];
    }
    if w == super::mind::PASSIVE_MARK && super::physics::passive_deed(mind).is_some() {
        return vec![WordMove::PointNothing];
    }
    if w == super::mind::LABEL_OPENER && at == 0 && held.is_none() && mind.before.len() == 1 {
        return vec![WordMove::PointNothing];
    }
    if w == super::mind::LABEL_AS && plain && held.is_none() && super::mind::labelling(&mind.before) {
        return vec![WordMove::AddProperty];
    }
    if plain && held.is_none() && super::mind::ANOTHER.contains(&w) && mind.flags.is_empty() && mind.first_mark == Some(at) {
        return Vec::new();
    }
    if held.is_some() && super::mind::ANOTHER.contains(&w) && flag_of(mind, FLAG_GIVE).is_some() {
        return vec![WordMove::FlagProperty];
    }
    let on_is = at != 0 && *mind.tree.node(at).name == *IS_FORM.trim();
    let said: Vec<&String> = mind.before.iter().filter(|b| !mark_word(b)).collect();
    let mannered = |b: &str| super::physics::has_relation(mind, b, super::mind::REPLY_RELATION);
    let polite = |b: &str| mannered(b) || super::mind::FILLERS.contains(&b) || super::mind::YOU_WORDS.contains(&b) || super::mind::GREETING_OPENERS.contains(&b);
    if mark_word(w) && kind == AT_WORLD && held.is_none() && open_question(mind).is_none() && said.len() <= super::mind::LIST_HALVES && said.iter().any(|b| mannered(b)) && said.iter().all(|b| polite(b)) {
        return vec![WordMove::GetReply];
    }
    if plain && held.is_none() && mind.flags.is_empty() && mind.before.iter().rev().nth(1).is_some_and(|b| b == super::mind::ABLE) && !punctuation(w) && !COPULA.contains(&w) {
        return vec![WordMove::AddRelation];
    }
    let described_class = kind == "value" && at != 0 && held.is_none() && *mind.tree.node(mind.tree.node(at).parent).name == *IS_FORM.trim() && mind.tree.story(at) && mind.before.iter().rev().nth(1).is_some_and(|b| *mind.tree.node(at).name == **b && super::physics::unseeded(mind, b)) && mind.before.iter().rev().nth(super::mind::LIST_HALVES).is_some_and(|b| ARTICLE_FLAGS.contains(&b.as_str()));
    if described_class && !punctuation(w) && !COPULA.contains(&w) && !place_word(w) && !super::mind::RELATIVES.contains(&w) && crate::words::number_of(w).is_none() && verb_base(mind, w).is_none() && (noun_word(mind, w) || super::mind::unknown_word(mind, w)) {
        return vec![WordMove::StepParent, WordMove::AddValue];
    }
    let group_named = mind.stack.items().next().is_some_and(|newest| newest.item.node == Some(at));
    if plain && held.is_none() && w == super::mind::INFINITIVE && flag_of(mind, FLAG_HAND).is_some() && group_named {
        return vec![WordMove::Grab];
    }
    if plain && held.is_none() && *mind.tree.node(at).name == *super::mind::ROUNDING && w == super::mind::INFINITIVE {
        return vec![WordMove::Grab];
    }
    if held.is_some_and(|h| h != at) && plain && *mind.tree.node(at).name == *w && is_number(mind, w).is_some() {
        return vec![WordMove::Drop];
    }
    if super::physics::rounding_here(mind) && super::mind::ROUNDING_WORDS.contains(&w) {
        return vec![WordMove::PointNothing];
    }
    if w == super::mind::DURING && kind == AT_WORLD && held.is_none() && mind.flags.is_empty() {
        return vec![WordMove::FlagWhen];
    }
    if flag_of(mind, super::mind::FLAG_WHEN) == Some(w) && kind == AT_WORLD && held.is_none() && !super::mind::TIMES_OF_DAY.contains(&w) && w != super::mind::DURING {
        return vec![WordMove::PointNothing];
    }
    if super::mind::unknown_letter(w) && kind == AT_WORLD && held.is_none() && mind.flags.is_empty() && super::physics::named_result(mind, w).is_none() {
        return vec![WordMove::AddQuestion];
    }
    let on_relation = at != 0 && !plain;
    let bare_deed = on_relation && !on_is && held.is_none() && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone) && verb_base(mind, &crate::cursor::bare_name(&mind.tree.node(at).name)).is_some();
    if bare_deed && w == THING_PRONOUNS[0] {
        return vec![WordMove::AddValue];
    }
    if bare_deed && (w == super::mind::ABOUT || w == super::mind::TOWARD) {
        return vec![WordMove::PointNothing];
    }
    if on_relation && !on_is && held.is_none() && super::mind::REFLEXIVES.contains(&w) {
        return vec![WordMove::AddValue];
    }
    if plain && held.is_none() && mind.tree.node(at).parent == 0 && verb_base(mind, w).is_some_and(|b| super::mind::CLAIMING.contains(&b.as_str())) {
        return vec![WordMove::AddRelation];
    }
    if on_relation && super::mind::CLAIMING.contains(&crate::cursor::bare_name(&mind.tree.node(at).name).as_str()) && ARTICLE_FLAGS.contains(&w) {
        return vec![if w == ARTICLE_FLAGS[0] { WordMove::FlagThe } else { WordMove::FlagA }];
    }
    if on_relation && COPULA.contains(&w) && (super::mind::KIN.contains(&crate::cursor::bare_name(&mind.tree.node(at).name).as_str()) || crate::cursor::bare_name(&mind.tree.node(at).name) == super::mind::NAME_ROLE) && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone) {
        return vec![WordMove::PointNothing];
    }
    if super::physics::class_after_quality(mind, 1) && noun_word(mind, w) {
        return vec![WordMove::StepParent, WordMove::StepParent, WordMove::AddValue];
    }
    if w == super::mind::MEANS && plain && held.is_none() {
        return vec![WordMove::AddRelation];
    }
    let defining = kind == "value" && at != 0 && crate::cursor::bare_name(&mind.tree.node(mind.tree.node(at).parent).name) == super::mind::MEANS;
    if defining && super::mind::operation_of(w).is_some() {
        return vec![WordMove::StepParent, WordMove::StepParent, WordMove::AddRelation];
    }
    if on_relation && super::physics::gained_by_word(mind, w) {
        return vec![WordMove::AddValue, WordMove::Belong, WordMove::StepParent, WordMove::StepParent, WordMove::Drop];
    }
    if w == super::mind::SOURCE && plain && held.is_none() && mind.before.iter().any(|b| verb_base(mind, b).is_some_and(|v| super::mind::GAINING.contains(&v.as_str()))) {
        return vec![WordMove::PointNothing];
    }
    if super::physics::traded(mind, &super::mind::PARTING) && w == super::mind::INFINITIVE {
        return vec![WordMove::Belong];
    }
    if on_relation && crate::cursor::bare_name(&mind.tree.node(at).name) == super::mind::SHARING && (super::mind::OBJECT_PRONOUNS.iter().any(|(p, _)| *p == w) || w == super::mind::EVENLY || w == super::mind::COMPANION) {
        return vec![WordMove::PointNothing];
    }
    if on_is && flag_of(mind, super::mind::FLAG_QUANTITY).is_some() && mind.before.first().is_some_and(|b| is_number(mind, b).is_some() && !super::mind::SELF_WORDS.contains(&b.as_str())) && mind.first_mark == Some(mind.tree.node(at).parent) && !punctuation(w) && (!place_word(w) || super::mind::STATE_WORDS.contains(&w)) && !ARTICLE_FLAGS.contains(&w) && is_number(mind, w).is_none() && !super::mind::NEGATIONS.contains(&w) && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone) {
        return vec![WordMove::StepParent, WordMove::AddRelation];
    }
    if on_is && super::mind::LETTER_PLACES.iter().any(|(ordinal, _)| *ordinal == w) && flag_of(mind, super::mind::FLAG_QUANTITY).is_none() {
        return vec![WordMove::AddValue];
    }
    if on_is && (w == super::mind::BORN || w == super::mind::ABOUT || w == ALIKE) && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone && !mind.tree.node(c).name.starts_with(BRACE_OPEN_TEXT)) {
        return vec![WordMove::StepParent, WordMove::AddRelation];
    }
    if on_relation && crate::cursor::bare_name(&mind.tree.node(at).name) == super::mind::ABOUT && !punctuation(w) && !ARTICLE_FLAGS.contains(&w) {
        return vec![WordMove::AddValue];
    }
    if on_relation && crate::cursor::bare_name(&mind.tree.node(at).name) == super::mind::BORN && place_word(w) {
        return vec![WordMove::PointNothing];
    }
    if on_relation && crate::cursor::bare_name(&mind.tree.node(at).name) == super::mind::BORN && crate::words::number_of(w).is_some() {
        return vec![WordMove::AddValue];
    }
    if flag_of(mind, super::mind::FLAG_KIN).is_some() && plain && held.is_some() && !punctuation(w) && !SKIPPED.contains(&w) {
        return vec![WordMove::AddRelation];
    }
    if (super::mind::KIN.contains(&w) || w == super::mind::NAME_ROLE) && plain && held == Some(at) && flag_of(mind, FLAG_GIVE).is_some() {
        return vec![WordMove::AddRelation];
    }
    let activity = crate::cursor::step_item(super::mind::ACTIVITY);
    let empty_verb = on_relation && !on_is && held.is_none() && *mind.tree.node(at).name != *activity && mind.tree.node(at).parent != 0 && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone) && { let named = crate::cursor::bare_name(&mind.tree.node(at).name); !place_word(&named) && !super::mind::DIRECTIONS.contains(&named.as_str()) && !super::mind::role_word(mind, &named) && !super::mind::KIN.contains(&named.as_str()) && !named.ends_with(super::mind::COMPARED) } && flag_of(mind, super::mind::FLAG_QUANTITY).is_none() && flag_of(mind, super::mind::FLAG_KIN).is_none();
    let deed_value = plain && held.is_none() && mind.tree.node(at).children.iter().any(|&d| !mind.tree.node(d).gone && *mind.tree.node(d).name == *super::mind::DEED_TAG && mind.tree.node(d).children.iter().any(|&v| !mind.tree.node(v).gone));
    if deed_value && place_word(w) {
        return vec![WordMove::Activity, WordMove::Grab];
    }
    let only_value = plain && held.is_none() && mind.tree.node(at).parent != 0 && { let deed = mind.tree.node(at).parent; let name = &*mind.tree.node(deed).name; name.starts_with(BRACE_OPEN_TEXT) && *name != *IS_FORM.trim() && *name != *activity && mind.tree.node(deed).parent != 0 && mind.tree.node(deed).children.iter().filter(|&&c| !mind.tree.node(c).gone).count() == 1 && verb_base(mind, &crate::cursor::bare_name(name)).is_some() };
    if (deed_value || empty_verb || only_value) && w == super::mind::COMPANION {
        return vec![WordMove::Activity, WordMove::AddRelation];
    }
    let done_here = plain && held.is_none() && (mind.tree.node(at).link.is_some_and(|l| *mind.tree.node(mind.tree.node(l).parent).name == *activity) || (mind.tree.node(at).parent != 0 && *mind.tree.node(mind.tree.node(at).parent).name == *activity));
    if done_here && w == super::mind::COMPANION {
        return vec![WordMove::AddRelation];
    }
    let going_here = { let named = crate::cursor::bare_name(&mind.tree.node(at).name); mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) && (named == super::mind::GOING_RELATION || super::mind::MOVING.contains(&named.as_str()) && named != super::mind::FITTING) };
    if empty_verb && w == super::mind::INFINITIVE && !going_here {
        return vec![WordMove::Activity];
    }
    let done_object = plain && held.is_none() && !punctuation(w) && (noun_word(mind, w) || super::mind::OBJECT_PRONOUNS.iter().any(|(said, _)| *said == w)) && !COPULA.contains(&w) && !place_word(w) && verb_base(mind, w).is_none()
        && super::lookup::own_child(mind, at, super::mind::DEED_TAG).is_some_and(|deed| super::lookup::present_children(mind, deed).last().is_some_and(|&value| { let said = mind.tree.node(value).name.to_string(); past_form(mind, &said) || said.ends_with(super::mind::PAST_END) }));
    if done_object {
        return vec![WordMove::AddValue];
    }
    let done_verb = plain && held.is_none() && super::lookup::own_child(mind, at, super::mind::DEED_TAG).is_some_and(|deed| super::lookup::present_children(mind, deed).last().is_some_and(|&value| { let said = mind.tree.node(value).name.to_string(); past_form(mind, &said) || said.ends_with(super::mind::PAST_END) }));
    if done_verb && w == super::mind::INFINITIVE {
        return vec![WordMove::Activity];
    }
    let timed = on_relation && held.is_none() && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone) && super::mind::TIMED_VERBS.contains(&crate::cursor::bare_name(&mind.tree.node(at).name).as_str());
    if timed && place_word(w) {
        return vec![WordMove::PointNothing];
    }
    if empty_verb && place_word(w) {
        return vec![WordMove::Activity, WordMove::Grab];
    }
    if empty_verb && punctuation(w) {
        return vec![WordMove::Activity];
    }
    if on_relation && *mind.tree.node(at).name == *activity && !punctuation(w) && !ARTICLE_FLAGS.contains(&w) {
        return vec![WordMove::AddValue];
    }
    if kind == "value" && place_word(w) && held.is_none() && at != 0 && *mind.tree.node(mind.tree.node(at).parent).name == *activity {
        return vec![WordMove::Grab];
    }
    if flag_of(mind, FLAG_HAND).is_some() && held.is_none() && plain && w == THING_PRONOUNS[0] {
        return vec![WordMove::StepNewest, WordMove::Grab];
    }
    if flag_of(mind, FLAG_HAND).is_some() && held.is_none() && plain && super::mind::OBJECT_PRONOUNS.iter().any(|(p, person)| *p == w && !*person) {
        return vec![WordMove::GrabAll];
    }
    let group_kind = flag_of(mind, super::mind::FLAG_GROUP).is_some() && mind.held.len() >= super::physics::PAIR_LEAST
        && w.ends_with(super::mind::PLURAL_END) && noun_word(mind, w) && !quality_word(mind, w) && super::mind::kind_of(mind, w).is_none();
    if group_kind {
        return vec![WordMove::Together, WordMove::AddValue];
    }
    if super::physics::listing(mind, w) {
        return vec![WordMove::Join];
    }
    if super::mind::TIMES_OF_DAY.contains(&w) && at == 0 && held.is_none() {
        return vec![WordMove::FlagWhen];
    }
    if kind == "value" && at != 0 && (mind.tree.node(at).name.ends_with(super::mind::SUPERLATIVE_END) || super::mind::ORDINALS.contains(&&*mind.tree.node(at).name)) && *mind.tree.node(mind.tree.node(at).parent).name == *IS_FORM.trim() && noun_word(mind, w) {
        return vec![WordMove::StepParent, WordMove::AddValue];
    }
    if w == super::mind::TOWARD && kind == "value" && held.is_none() && at != 0 && quality_word(mind, &mind.tree.node(at).name) && mind.tree.node(mind.tree.node(at).parent).name.starts_with(BRACE_OPEN_TEXT) {
        return vec![WordMove::Regard];
    }
    if super::mind::MEASURES.contains(&w) && kind == "value" && at != 0 && *mind.tree.node(mind.tree.node(at).parent).name == *IS_FORM.trim() && present_children_counted(mind, at) {
        return vec![WordMove::Measure];
    }
    if punctuation(w) {
        return match held {
            Some(h) if plain && h != at => vec![WordMove::Drop],
            _ => Vec::new(),
        };
    }
    let appeared = plain && { let name = &*mind.tree.node(at).name; *name == *w || *name == *singular(w) };
    if appeared && flag_of(mind, super::mind::FLAG_FROM).is_some() {
        return vec![WordMove::PointNothing];
    }
    if appeared {
        if flag_of(mind, super::mind::FLAG_STATE).is_some() {
            return vec![WordMove::ChangeState];
        }
        return match held {
            Some(h) if h != at && flag_of(mind, FLAG_GIVE).is_some() => vec![WordMove::Drop],
            Some(h) if h != at && flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none() => vec![WordMove::Join],
            Some(h) if h != at => vec![WordMove::Drop],
            None if flag_of(mind, FLAG_HAND).is_some() && super::physics::person_named(mind, w) && mind.before.iter().rev().nth(usize::from(true)).is_some_and(|b| GIVING.contains(&verb_base(mind, b).unwrap_or_else(|| b.clone()).as_str())) => vec![WordMove::Give],
            None if flag_of(mind, FLAG_HAND).is_some() => vec![WordMove::Grab],
            _ => Vec::new(),
        };
    }
    if on_is && !mind.tree.node(at).children.iter().any(|&c| !mind.tree.node(c).gone) && super::mind::OWN_WORDS.iter().any(|(own, owner)| *own == w && (super::mind::SELF_WORDS.contains(owner) || super::mind::YOU_WORDS.contains(owner))) {
        return vec![WordMove::AddValue, WordMove::Possess];
    }
    if held.is_some() && flag_of(mind, super::mind::FLAG_WITH).is_some() && plain && super::mind::OWN_WORDS.iter().any(|(o, _)| *o == w) {
        return vec![WordMove::Give];
    }
    if let Some((_, owner)) = super::mind::OWN_WORDS.iter().find(|(o, _)| *o == w) {
        if held.is_some() && kind != AT_WORLD && flag_of(mind, super::mind::FLAG_PLACE).is_some() {
            return vec![WordMove::FlagWhose];
        }
        if kind != AT_WORLD {
            return vec![WordMove::PointNothing];
        }
        let step = if super::mind::SELF_WORDS.contains(owner) || super::mind::YOU_WORDS.contains(owner) { WordMove::StepUser } else if PERSON_PRONOUNS.contains(owner) { WordMove::StepTop } else { WordMove::StepNewest };
        return vec![step, WordMove::Give];
    }
    if super::mind::NEGATIONS.contains(&w) {
        return vec![WordMove::FlagQuantity];
    }
    if w == super::mind::NO_LONGER && plain && flag_of(mind, super::mind::FLAG_QUANTITY).is_some() {
        return vec![WordMove::Release];
    }
    if held.is_some() && flag_of(mind, super::mind::FLAG_WITH).is_some() {
        if super::mind::SELF_WORDS.contains(&w) || super::mind::YOU_WORDS.contains(&w) {
            return vec![WordMove::StepUser, WordMove::Drop];
        }
        if let Some((_, person)) = super::mind::OBJECT_PRONOUNS.iter().find(|(p, _)| *p == w) {
            return vec![if *person { WordMove::StepTop } else { WordMove::StepNewest }, WordMove::Drop];
        }
    }
    if w == super::mind::BEEN && plain && held == Some(at) && flag_of(mind, FLAG_GIVE).is_some() {
        return vec![WordMove::AddProperty];
    }
    if w == super::mind::SAMENESS && kind == AT_WORLD {
        return vec![WordMove::PointNothing];
    }
    if w == super::mind::COMPANION && plain && held.is_none() {
        return vec![WordMove::Accompany];
    }
    if w == super::mind::COMPANION && on_is && held.is_none() {
        return vec![WordMove::StepParent, WordMove::Accompany];
    }
    if ARTICLE_FLAGS.contains(&w) && plain && held.is_none() && mind.flags.is_empty() && mind.first_mark == Some(at) && mind.tree.node(at).parent == 0 && things_inside(mind, at) == 0 {
        return vec![WordMove::StepParent, if w == ARTICLE_FLAGS[0] { WordMove::FlagThe } else { WordMove::FlagA }];
    }
    if w == super::mind::BELONGS_IN[0] && plain && held.is_none() && super::mind::role_word(mind, &crate::cursor::bare_name(&mind.tree.node(at).name)) {
        return vec![WordMove::Relate];
    }
    if COPULA.contains(&w) && plain && held.is_none() && flag_of(mind, super::mind::FLAG_ROLE).is_some() {
        return vec![WordMove::AddRole];
    }
    if on_relation && !punctuation(w) && crate::words::number_of(w).is_none() && !ARTICLE_FLAGS.contains(&w) && !super::mind::BELONGS_IN.contains(&w) && w != super::mind::INFINITIVE && w != super::mind::COMPARED && w != super::mind::LIKENESS && super::mind::role_word(mind, &crate::cursor::bare_name(&mind.tree.node(at).name)) {
        return vec![WordMove::AddValue];
    }
    if on_is && flag_of(mind, super::mind::FLAG_QUANTITY).is_some() && w == super::mind::CLOCK_OPENER {
        return vec![WordMove::PointNothing];
    }
    if on_is && flag_of(mind, super::mind::FLAG_QUANTITY).is_some() && w == super::mind::CLOCK_WORD {
        return vec![WordMove::SetClock];
    }
    if on_is && super::mind::DIRECTIONS.contains(&w) {
        return vec![WordMove::StepParent, WordMove::AddRelation];
    }
    if w == APOSTROPHE && kind == "value" && at != 0 && *mind.tree.node(mind.tree.node(at).parent).name == *IS_FORM.trim() {
        return vec![WordMove::Possess];
    }
    if on_is && w == super::mind::SAMENESS {
        return vec![WordMove::StepParent, WordMove::AddRelation];
    }
    if w == super::mind::LIKENESS && at != 0 && *mind.tree.node(at).name == *crate::cursor::step_item(super::mind::SAMENESS) {
        return vec![WordMove::PointNothing];
    }
    if super::mind::BELONGS_IN.contains(&w) && plain && held.is_none() {
        return vec![WordMove::Grab];
    }
    if w == super::mind::JOINER && plain && kind == "value" {
        return vec![WordMove::StepParent];
    }
    if w == super::mind::JOINER && plain {
        return vec![WordMove::Join];
    }
    if w == super::mind::SOURCE && (held.is_some() || going_open(mind, at)) {
        return vec![WordMove::FlagFrom];
    }
    if THING_PRONOUNS.contains(&w) && held.is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_some() {
        return vec![WordMove::StepNewest, WordMove::Drop];
    }
    if super::mind::RELATIVES.contains(&w) && plain && held.is_none() && mind.tree.node(at).parent == 0 {
        return vec![WordMove::PointNothing];
    }
    if w == super::mind::WILL && plain && held.is_none() {
        return vec![WordMove::FlagLater];
    }
    if w == super::mind::OUT_OF && on_is && held.is_none() {
        return vec![WordMove::StepParent, WordMove::Leave];
    }
    if super::mind::RELATIVES.contains(&w) && plain && held.is_none() && mind.first_mark == Some(at) && mind.tree.node(at).parent != 0 {
        return vec![WordMove::StepParent];
    }
    if w == APOSTROPHE {
        return if plain && held.is_none() { vec![WordMove::Give] } else { vec![WordMove::PointNothing] };
    }
    if ARTICLE_FLAGS.contains(&w) {
        return vec![if w == ARTICLE_FLAGS[0] { WordMove::FlagThe } else { WordMove::FlagA }];
    }
    if (super::mind::SELF_WORDS.contains(&w) || super::mind::YOU_WORDS.contains(&w)) && held.is_some() && flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none() {
        return vec![WordMove::StepUser, WordMove::Join];
    }
    if super::mind::SELF_WORDS.contains(&w) || super::mind::YOU_WORDS.contains(&w) {
        return vec![if kind == AT_WORLD { WordMove::StepUser } else { WordMove::PointNothing }];
    }
    let placed = super::mind::ORDINALS.contains(&w) && flag_of(mind, ARTICLE_FLAGS[0]).is_some();
    if is_number(mind, w).is_some() && !placed {
        return vec![WordMove::FlagQuantity];
    }
    if super::mind::operation_of(w).is_some() || w == super::mind::POWER_SIGN {
        let carries = mind.number.is_some() && mind.flags.is_empty() && held.is_none();
        return vec![if kind == AT_WORLD && (flag_of(mind, super::mind::FLAG_QUANTITY).is_some() || carries) { WordMove::AddQuestion } else { WordMove::PointNothing }];
    }
    if (w == super::mind::BRACKET_OPENS || super::mind::FUNCTIONS.contains(&w)) && kind == AT_WORLD && held.is_none() {
        return vec![WordMove::AddQuestion];
    }
    if super::mind::RELATIVES.contains(&w) && kind == "value" && at != 0 && held.is_none() && mind.tree.story(at) && *mind.tree.node(mind.tree.node(at).parent).name == *IS_FORM.trim() {
        return vec![WordMove::StepParent, WordMove::StepParent];
    }
    if super::mind::ORDINALS.contains(&w) && kind == AT_WORLD && held.is_none() && mind.flags.is_empty() && mind.before.len() <= usize::from(true) && open_question(mind).is_none() {
        return vec![WordMove::PointNothing];
    }
    if bare_deed && super::physics::person_named(mind, w) && super::mind::TELLING.contains(&crate::cursor::bare_name(&mind.tree.node(at).name).as_str()) {
        return vec![WordMove::PointNothing];
    }
    if bare_deed && quality_word(mind, w) && ARTICLE_FLAGS.iter().all(|article| flag_of(mind, article).is_none()) {
        return vec![WordMove::AddValue];
    }
    let waiting = super::mind::unknown_word(mind, w) && held.is_none() && kind == AT_WORLD && flag_of(mind, super::mind::FLAG_QUANTITY).is_none() && flag_of(mind, super::mind::FLAG_PROPERTY).is_none();
    if ((held.is_some() || kind == AT_WORLD) && quality_word(mind, w) && !on_is || waiting) && flag_of(mind, FLAG_HAND).is_none() {
        return vec![WordMove::FlagProperty];
    }
    if super::physics::NEAR_DAYS.contains(&w) && at != 0 && held.is_none() && super::physics::clause_done(mind) {
        return vec![WordMove::PointNothing];
    }
    if held.is_none() && flag_of(mind, super::mind::FLAG_DEGREE).is_some_and(|said| said == w) {
        return vec![WordMove::PointNothing];
    }
    let setting_off = on_is && held.is_none() && going_bases(w).iter().any(|base| super::mind::MOVING.contains(&base.as_str()));
    if setting_off {
        return vec![WordMove::StepParent, WordMove::Grab];
    }
    let doing_now = on_is && held.is_none() && !quality_word(mind, w) && going_bases(w).iter().any(|base| verb_base(mind, base).is_some() || super::physics::activity_word(mind, base));
    if doing_now {
        return vec![WordMove::StepParent, WordMove::AddRelation];
    }
    if w == super::mind::AWAY_FROM && on_is && held.is_none() {
        return vec![WordMove::StepParent, WordMove::Leave];
    }
    if super::mind::FILLERS.contains(&w) || SKIPPED.contains(&w) || HELPERS.contains(&w) || ASKING.contains(&w) {
        return vec![WordMove::PointNothing];
    }
    let group = flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none() && held.is_some();
    if COPULA.contains(&w) {
        let clause_thing = plain && held.is_none() && mind.tree.node(at).parent != 0 && super::physics::said_to_have(mind, at, mind.tree.node(at).parent) && mind.before.iter().any(|b| super::mind::RELATIVES.contains(&b.as_str()));
        if clause_thing {
            return vec![WordMove::StepParent, if past_form(mind, w) { WordMove::AddPropertyPast } else { WordMove::AddProperty }];
        }
        if plain && (held.is_none() || group) {
            return vec![if past_form(mind, w) { WordMove::AddPropertyPast } else { WordMove::AddProperty }];
        }
        if kind == AT_WORLD && past_form(mind, w) {
            return vec![WordMove::FlagTime];
        }
        return vec![WordMove::PointNothing];
    }
    if THING_PRONOUNS.contains(&w) {
        return vec![if kind == AT_WORLD { WordMove::StepNewest } else { WordMove::PointNothing }];
    }
    if super::mind::SELF_WORDS.contains(&w) || super::mind::YOU_WORDS.contains(&w) {
        return vec![if kind == AT_WORLD { WordMove::StepUser } else { WordMove::PointNothing }];
    }
    if PERSON_PRONOUNS.contains(&w) {
        return vec![if kind == AT_WORLD { WordMove::StepTop } else { WordMove::PointNothing }];
    }
    if HAVING.contains(&w) {
        if plain && held.as_ref().is_some() && mind.held.contains(&at) && flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none() {
            return vec![WordMove::Give];
        }
        return if plain && held.is_none() { vec![WordMove::Give] } else { vec![WordMove::PointNothing] };
    }
    if place_word(w) {
        if held.is_some() && flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none() && plain {
            return vec![WordMove::Grab];
        }
        if held.is_some() && flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none() && on_is {
            return vec![WordMove::StepParent, WordMove::Grab];
        }
        if held.is_some() {
            return vec![WordMove::PointNothing];
        }
        if kind == ON_THING {
            return vec![WordMove::Grab];
        }
        if kind == ON_PROPERTY && on_is {
            return vec![WordMove::StepParent, WordMove::Grab];
        }
        if kind == "value" && at != 0 && *mind.tree.node(mind.tree.node(at).parent).name == *IS_FORM.trim() && mind.tree.story(at) {
            return vec![WordMove::StepParent, WordMove::StepParent, WordMove::Grab];
        }
        return vec![WordMove::PointNothing];
    }
    if on_is && verb_base(mind, w).is_some_and(|b| super::mind::STATES.iter().any(|(v, _, _)| *v == b)) {
        return vec![WordMove::AddValue];
    }
    if let Some(base) = verb_base(mind, w) {
        let group = flag_of(mind, super::mind::FLAG_GROUP).is_some() && flag_of(mind, super::mind::FLAG_PLACE).is_none();
        if group && plain && MOVING.contains(&base.as_str()) {
            return going_moves(mind, w);
        }
        if group && plain && super::mind::POSING.contains(&base.as_str()) {
            return vec![WordMove::PointNothing];
        }
        if group && plain && !MOVING.contains(&base.as_str()) {
            return vec![WordMove::AddRelation];
        }
        if held.is_some() || !plain {
            return vec![WordMove::PointNothing];
        }
        if super::mind::STATES.iter().any(|(v, _, _)| *v == base) {
            return vec![WordMove::ChangeState];
        }
        if super::mind::LEAVING.contains(&base.as_str()) {
            return vec![WordMove::Leave];
        }
        if super::mind::POSING.contains(&base.as_str()) {
            return vec![WordMove::PointNothing];
        }
        if super::mind::OWNING.contains(&base.as_str()) {
            return vec![WordMove::Give];
        }
        if super::mind::BELONGING.contains(&base.as_str()) {
            return vec![WordMove::Belong];
        }
        if super::mind::LOSING.contains(&base.as_str()) {
            return vec![WordMove::Release];
        }
        if MOVING.contains(&base.as_str()) {
            return going_moves(mind, w);
        }
        return vec![if false { WordMove::Grab } else if CONTAINING.contains(&base.as_str()) { WordMove::Contain } else if GIVING.contains(&base.as_str()) { WordMove::Hand } else if super::mind::TAKING.contains(&base.as_str()) { WordMove::Take } else if super::mind::DROPPING.contains(&base.as_str()) { WordMove::Release } else { WordMove::AddRelation }];
    }
    let seeded_noun = mind.tree.named(w).any(|n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && mind.tree.node(n).parent == 0) && !super::mind::comparison_known(mind, w);
    if on_is && !seeded_noun && w.ends_with(super::mind::COMPARISON_END) && (super::mind::comparison_known(mind, w) || (flag_of(mind, super::mind::FLAG_DEFINITE).is_none() && flag_of(mind, super::mind::FLAG_INDEFINITE).is_none())) && !quality_word(mind, w) && verb_base(mind, w).is_none() && !super::mind::role_word(mind, w) && !super::physics::owns_relation(mind, w, IS_FORM.trim().trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE)) {
        return vec![WordMove::StepParent, WordMove::AddRelation];
    }
    if w == super::mind::COMPARED {
        return vec![WordMove::PointNothing];
    }
    if on_is {
        let made_of = mind.before.iter().any(|b| b == super::mind::MADE) && mind.before.iter().rev().nth(1).is_some_and(|b| b == super::mind::TOWARD);
        let set = if made_of { Some(WordMove::SetMaterial) } else { kind_of(mind, w).and_then(|k| KINDS.iter().find(|(_, name)| **name == *k).map(|(m, _)| *m)) };
        return vec![set.unwrap_or(WordMove::AddValue)];
    }
    let described_value = on_relation && super::mind::quality_word(mind, w) && super::mind::ARTICLE_FLAGS.iter().skip(usize::from(true)).any(|article| flag_of(mind, article).is_some());
    if described_value {
        return vec![WordMove::FlagProperty];
    }
    if on_relation {
        return vec![WordMove::AddValue];
    }
    if kind == AT_WORLD || (held.is_some() && flag_of(mind, FLAG_GIVE).is_none() && !plain) {
        return Vec::new();
    }
    let other = word_classes(mind, w) == ["other"];
    if plain && held.is_none() && other && w.ends_with(super::mind::PLURAL_END) {
        return vec![WordMove::AddRelation];
    }
    if plain && held.is_none() && (other || past_form(mind, w)) && mind.first_mark == Some(at) {
        return vec![WordMove::AddDeed];
    }
    if plain && held.is_none() && !word_classes(mind, w).is_empty() {
        return vec![WordMove::AddProperty];
    }
    Vec::new()
}
because!(
    taught_word,
    WordGame,
    "the fewest moves for one word, a plural kind said of things joined by and makes a group of them that holds what they are together, john and i are best friends, while a quality said of them stays on each one, a thing named while the doer of a deed is stood on is what the deed was done to, everyone adored him, so the verb becomes a relation of the doer with that thing as its value, a past verb written as a deed takes the activity move at the word to, beep loved to go fast, so what is loved is an activity of the one who loves it, as the present form reads it, an own word said while the reading holds something on its way to a place flags whose the place is, the keys are in my pocket, the one told something named right after a verb of telling points at nothing, so what was told stays the deed of the one who told it, a quality after a verb with nothing under it, with no article said, is added as what the verb says, the bathroom smells bad, a going form of a verb of having at the head of a sentence steps to the speaker and gives, holding five cards, a going form of a known verb after is steps back to the doer and adds the verb, anna is reading a book, it right after a verb is added as the newest thing told that is no person and not the doer, he loves it, by after a deed a thing had done to it points at nothing since the doer named next takes the deed, about or of right after a verb points at nothing so what is cared about or smelled of goes under the verb, and a word that names the doer again is added as the value the doer is, he only cares about himself, label opening an order points at nothing and as in that order adds the is the label goes under, label grandma as a cool person, the other that the physics already stepped to its thing for needs no move, another or other after having waits as a quality so the thing named next is a new one of its name, i have another car, the opening of a tale, once upon a time, points at nothing word by word, a day named from now after a finished clause only tells when and points at nothing, the sky is red today, a bit or a little after a, under is, only weakens the quality said next and names no thing, a place in an order said after the is that place and no fraction, the third car, a going word of a verb of moving after is sets the thing off as the verb does, tom is going to \
     rome, a person said right after a verb of giving is the one given to and takes what comes next as has \
     does, mary gave john a book, a quality said after a or an under a verb waits for its noun as it does after has, i bought a new \
     phone, a noun said right after an article and a word the seeds never state, under is, is what the thing is \
     and the word before only describes it, a useful device, the word after can, said of a thing, is what the thing is able to do and is \
     added as its relation, a bird can fly, the mark after an input of manners alone gets the reply the seeds give, hello to hi, to said \
     after a count given grabs the group the count named, she gave two to max, a relative word after what a thing is steps back to the \
     thing, so what follows is said of it, a clock is a device that measures time, an ordinal that opens a sentence orders the telling and \
     is passed over, first the dog was in the garden, a place word after a quality told under is steps back to the thing and grabs it to \
     be placed, the shop is open in the morning, the facts of the word already done: a thing the story placed or lost, named under a deed \
     of finding or buying, is written as its value, held, and dropped in the doer two steps up, all at its own word, zoe finds the coin, \
     the from after it taking nothing, and one named under a deed of selling is held at to and dropped in the one who appears next; during \
     said first flags when, and the noun after it, which the physics made the time, takes nothing, during the day; a copula said after a \
     relative word of the sentence, while the cursor stands on what the subject was just said to have, steps back to the subject first, \
     the box that has a key is red; them, equally and with after a deed of sharing take nothing, so the counted ones after them are its \
     value; with after the one value of a deed makes the deed an activity and adds the companion to it, tom plays football with ben; the \
     word after made of sets the material whatever the seeds class it as, a molecule is made of atoms; a verb of saying, thinking or \
     believing after a person adds its relation, and the thing named next is its value, which the copula and the quality or place after it \
     are written on, tom said the ball is red; means after a name adds the relation of its definition, and a word of arithmetic after what \
     it means steps back to the name and adds the operation the definition carries; a quality said of a counted part of a group steps back \
     to the group and adds the quality as a relation, whose value the count becomes at the mark, five students are absent; an ordinal \
     after a copula is a value, third and no fraction, and the kind named after it stands beside it under is, the second element; my or \
     your right after a copula is written as its value and possessed at once, so the kin word after it is a relation of the speaker, ann \
     is my sister; a copula on an empty relation of kin takes nothing, so the name after it is its value, ann's son is tom; a noun after \
     an article and a quality of a copula steps back to is and is its value, a kitten is a young cat; it after a verb of giving steps to \
     the newest thing and grabs it, he gave it to ann; a lone letter said first at the world opens the question of an equation, as does a \
     sign of arithmetic said first when a number is carried from the last sum; the rounding is grabbed at to, dropped in the number said \
     next, and decimal and places after it take nothing; a number that appeared as a thing while something is held takes the drop, the \
     meeting is at three; born after was, or about after a copula, steps back to the thing and adds the relation, any word after about \
     being its value, a song about love, in after it takes nothing and the year is its value; a mark takes none, the physics drops what he \
     holds, unless a thing appeared at it while he places something, which he drops; a thing that appeared after a verb of state takes the \
     state; a possessive word at the world steps to the owner it names and gives what comes next to them; no or not sets a count of none \
     on what comes next, and no longer turns it into losing; a verb of owning gives as having does, a verb of belonging gives the subject \
     to the next thing, a verb of losing puts the next thing down; and after a value under a relation steps back to the relation for the \
     next value, ann likes tom and ben; from while holding flags that the next thing is where the held thing came from, which he then \
     leaves aside; it while placing drops what he holds into what it stands for, the coin is in it; a copula or a verb on a group writes \
     on each member; and on a thing joins it to a group, a thing appearing in a group joins it too until a moving verb grabs the whole \
     group, whose place then takes them all; a thing that appeared while he holds something is where it goes, a drop pointing at the word, \
     and a thing that appeared after a verb of handing on is grabbed; the apostrophe of a possessive gives what comes next to the thing \
     before it; an article sets the flag the or a; a count sets the quantity flag; a word of arithmetic after a count said first opens a \
     question, six plus one equals; a quality at the world or while he holds something, or a word in no class said first in a clause, sets \
     the property flag for the thing to come; which or that after a thing just placed steps to its place, the bag which is on the bed; a \
     link word, a helper or a question word points at nothing; a copula on a thing adds is, dated when it is a past form, and a past \
     copula at the world sets the time flag; it or they at the world step to the newest thing, he or she to the newest top thing; a form \
     of having on a thing is a give; a place word on a thing grabs it, and on the is just added steps up to the thing first; a moving verb \
     on a thing grabs it, clock after a number under is sets the subject's time, the o before it passed over, it is three o'clock; a \
     pronoun after with steps to what it stands for and drops there, ann is with him; been after has is the copula; same before a noun is \
     passed over; to after a verb's empty relation opens the activity beside it and the verb after it is its value, tom loves to swim; a \
     place word or a mark after a verb's empty relation, or a place word after a deed, makes the verb the activity, kim swam in the lake, \
     with after an activity is a relation of it, swims with sam, and a place word after an activity holds it to be dropped in the place; \
     them after a verb of giving holds everything the giver has, he gave them to ann; a comma after a thing just named joins it to the \
     group, the key, the pen and the cup; that or which after a thing at the world is passed over, the pen that is in the bag; will flags \
     the time later; out after is leaves the place named after of; an own word after with gives the companion to the subject; a direction \
     after is is a relation of the subject, south of the hallway; a word of kin after a possessive is a relation of the owner, tom's \
     brother is sam; an apostrophe after a name under is makes that name the thing and the next noun its relation to the subject, ann is \
     tom's mother; of after a quality under is makes the quality a relation, afraid of wolves; a word of a state after is is the value, \
     the door is open; a noun after a superlative under is is a value beside it, the largest planet; a verb of leaving holds the subject \
     to take it out of the place named next, as off does after is, the cat is off the chair, a verb of posture points at nothing, since \
     the place word after it places the subject, the cat sat on the mat, a verb of holding holds it as the container of what comes, a verb \
     of giving on flags that the next thing is handed on, a verb of taking holds the subject for the next thing to move into, a verb of \
     dropping holds it for the next thing to be put where it stands, and any other verb adds its relation; a quality on is sets the \
     property of its kind, or is a plain value when the seeds class it by no kind; a word on a relation is its value; a thing word at the \
     world appeared by itself and takes no move; of after a thing puts it inside the thing after, the door of the house; with holds the \
     subject so it and the next thing stand in one place, tom went to the park with his dog; an article after a thing that only just \
     appeared right in the world sends the cursor back to the world, since that word was a time said before the clause, today the cat is \
     in the garden; a place word after a group grabs the whole group; a place word after is on a group holds the group to place it, the \
     cat and the dog are in the park; a comparison after is adds its relation to the thing, bigger than, when the seeds know it or no \
     article stands before it, a teacher being no comparison, unless the seeds class the word, october being a month, and than points at \
     nothing; a word no class knows, ending in s, said of a thing is a verb the seeds do not know, and any other such word or a past form \
     said of the thing the sentence is about is a deed with no object, it broke down, a cup costs three dollars, whose relation it adds; \
     and any other word said of a thing with no relation is a bare property"
);
