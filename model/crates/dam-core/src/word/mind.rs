use super::moves::{WordMove, WordStep};
use super::WordReading;
use crate::cursor::{child_named, names_class, told_past, verb_of, CursorMind, BRACE_OPEN_TEXT, NAMES_FEATURE, TOKEN_TEXT, WORD_PLACE};
use crate::events::{feature, named_item, Item, ItemKind, Stack, ACTION_KIND, ACTION_NAME};
use crate::quiz::{APOSTROPHE, IS_FORM, OWNS_FORM, PLACE_RELATIONS};
use crate::words::number_of;
use patterns::because;
use std::sync::Arc;
use super::english::{MARK_SIGNS, ASKED_END, TOLD_END, OPERATORS, SOFT_BEFORE, LIST_FUNCTIONS, PLURAL_LEAST, HISSING_ENDS, LONG_PLURAL, SOFT_PLURAL, PLAIN_PLURALS, MORE_PLACES, LINKS, VERB_ENDS, MARK_CLASS, SPEAKER_CLASS, OWN_CLASS, FILLER_CLASS, NEGATION_CLASS, LINK_CLASS, OPERATOR_CLASS, POSSESSIVE_CLASS, COPULA_CLASS, HAVING_CLASS, ARTICLE_CLASS, SKIPPED_CLASS, ASKS_CLASS, HELPER_CLASS, THING_PRONOUN_CLASS, PERSON_PRONOUN_CLASS, PLACE_CLASS, MOVING_CLASS, PAST_CLASS, NUMBER_CLASS, KIND_CLASS, QUALITY_CLASS, OTHER_CLASS, STORY_OPENER, POINTED_NTH};
pub use super::english::{WordEnglish, COPULA, LIST_MARK, HAVING, SKIPPED, ODD_SIGN, EVEN_SIGN, RANGE_WORD, ALPHABET_WORD, PLACE_UNITS, REPLY_WORDS, DIGITS_SIGN, POWER_SIGN, BRACKET_OPENS, BRACKET_CLOSES, FUNCTIONS, COMPARING, CLOCK_OPENER, MEASURES, SUPERLATIVE_RELATION, TELLING, GIVING, ARTICLE_FLAGS, ASKING, HELPERS, THING_PRONOUNS, PERSON_PRONOUNS, MOVING, CONTAINING, PLURAL_END, FLAG_OWNED, function_word, SWITCHED, STATES, OWN_WORDS, YOU_WORDS, SELF_WORDS, FILLERS, TAKING, LEAVING, ORDINALS, OPPOSITE_WORD, COMPARED, COMPARISON_END, POSING, OWNING, BELONGING, LOSING, NEGATIONS, DROPPING, VERB_CLASS, THING_CLASS, VALUE_CLASS, GOING_END, ABLE, LABEL_OPENER};
pub use super::view::{found_word_item};
pub use super::english::{TIMED_VERBS, YEAR_LEAST, YEAR_NAME, ACTIVITY, INFINITIVE, TOWARD, CLAUSE_BREAKS, ANOTHER, LINES, BEEN, PRONOUN_GENDERS, DONE_ASKED, OBJECT_PRONOUNS, PARTING_MARKS, MORE_SIGN, LESS_SIGN, MOST_SIGN, LEAST_SIGN, MAKING, LAST_PLACE, DECIMAL_BASE, HALVING, ROMAN_LETTERS, FRONT, MADE, AROUND_ASKED, SIDE_NAMED, PLURAL_ASKED, SINGULAR_ASKED, IN_WORDS, NUMBER_ASKED, RUN_ASKED, DURING, GAINING, PARTING, WHO_ASKED, WEIGHT_COMPARISONS, WEIGHING, SPAN_ENDS, SPAN_ASKED, SHARING, EVENLY, GOAL, WHY_ASKED, CHOICE_ASKED, STATE_WORDS, MEANS, GOING, CLAIMING, RIGHT_ASKED, NAME_ROLE, SEQUENCE_WORDS, THERE_OPENER, CHANGE_ASKED, PAYING, COSTING, POSSESSIVE_S, BORN, HOUR_OPENER, AGE_WORDS, RUN_SUM, ROUNDING, ROUNDING_WORDS, LETTER_ASKED, LETTER_PLACES, SPELLING, VOWEL_LETTERS, QUOTE, MEDIAN_SIGN, MODE_SIGN, SPREAD_SIGN, LEVEL_SIGN, LIST_HALVES, MULTIPLE_SIGN, FACTOR_SIGN, MULTIPLIERS, SHOWN_THOUSANDTHS, SOLVE_NEAR, SOLVE_SPAN, EQUAL_SIGN, CHOICE_WORD, MIDDLE_SIGN, TOGETHER, CLOCK_WORD, TIME_ASKED, AGO, UNTIL, DIRECTIONS, DIGITS_ASKED, KIN, GRAND, PARENTS, FLAG_KIN, SUPERLATIVE_END, ORDER_RELATION, FLAG_HAND, FLAG_DEFINITE, FLAG_INDEFINITE, FLAG_QUANTITY, FLAG_PROPERTY, FLAG_TIME, FLAG_GIVE, FLAG_PLACE, FLAG_CONTAIN, ASSISTANT_NAME, JOINER, SOURCE, FLAG_FROM, COMPANION, FLAG_WITH, BELONGS_IN, DEED_TAG, RELATIVES, FLAG_GROUP, FLAG_STATE, PAST_END, USER_NAME, COUNTING, GENERAL_THINGS, RECEIVING, OUT_OF, WILL, AWAY_FROM, FLAG_LEAVE, TIMES_OF_DAY, FLAG_WHEN, FLAG_ROLE, SAMENESS, LIKENESS, SYMMETRIC_ROLES, PASSIVE_MARK, ABOUT, EARLIER, LATER, NO_LONGER, FLAG_TAKE, FLAG_RELEASE, GREETING_OPENERS, REPLY_RELATION, WEAK_DEGREES, DEGREE_WORDS, FLAG_DEGREE, CAUSE_WORD, GROUP_OBJECT, REFLEXIVES, LABEL_AS, NAMED_BY};
pub use super::view::{closing_mark, singular, verb_like, role_word, noun_word, node_kind, heard_word, open_question};

pub fn asked_sentence(mut words: Vec<String>) -> Vec<String> {
    let opener = words.iter().find(|w| (!FILLERS.contains(&w.as_str()) || *w == ABLE) && !mark_word(w)).is_some_and(|w| COPULA.contains(&w.as_str()) || HELPERS.contains(&w.as_str()) || w == ABLE);
    let asks = opener || words.iter().any(|w| ASKING.contains(&w.as_str()));
    if !asks {
        words.insert(0, ASKING[0].to_string());
    }
    if words.last().is_some_and(|w| !mark_word(w)) {
        words.push(ASKED_END.to_string());
    }
    words
}
because!(asked_sentence, WordReading, "a question as the reading takes it: one with no word that asks anywhere and no copula, helper or \
     can opening it after the fillers is a sentence to fill in, the color of the ball is, and is asked with what before it, and one with \
     no mark is closed with a question mark");

pub fn closed_sentence(mut words: Vec<String>) -> Vec<String> {
    if words.last().is_some_and(|w| !mark_word(w)) {
        let asks = words.iter().any(|w| ASKING.contains(&w.as_str()) || operation_of(w).is_some());
        words.push(if asks { ASKED_END } else { TOLD_END }.to_string());
    }
    words
}
because!(closed_sentence, WordReading, "a sentence closed with a mark when it was written without one, a question mark when it asks or \
     works out numbers, else a full stop, since the chat reads the end of a message as its end");

pub fn mark_word(word: &str) -> bool {
    MARK_SIGNS.contains(&word)
}
because!(mark_word, WordReading, "whether a word is a sign that ends or parts a sentence");

pub fn plural_of(word: &str) -> String {
    if let Some((many, _)) = PLAIN_PLURALS.iter().find(|(many, one)| *one == word && *many != *one) {
        return many.to_string();
    }
    if HISSING_ENDS.iter().any(|end| word.ends_with(end)) || word.ends_with(PLURAL_END) {
        return format!("{word}{LONG_PLURAL}");
    }
    match word.strip_suffix(SOFT_PLURAL.1) {
        Some(stem) if !stem.chars().last().is_some_and(|c| SOFT_BEFORE.contains(c)) => format!("{stem}{}", SOFT_PLURAL.0),
        _ => format!("{word}{PLURAL_END}"),
    }
}
because!(plural_of, WordEnglish, "the plural of a word: the one the table gives, es after a hissing sound or an s, ies for a y after a \
     consonant, else the word with s");

pub fn reply_word(mind: &CursorMind, word: &str) -> bool {
    REPLY_WORDS.contains(&word) || REPLY_WORDS.contains(&verb_stem(mind, word).as_str())
}
because!(reply_word, WordReading, "whether a word is a verb of replying, as said or as its stem, answer keeping its er");

pub fn list_function(word: &str) -> Option<&'static str> {
    operation_in(&LIST_FUNCTIONS, word)
}
because!(list_function, WordReading, "the operation over a list of numbers a word names, or none");

pub fn unary_operation(word: &str) -> bool {
    word == DIGITS_SIGN || word == ODD_SIGN || word == EVEN_SIGN || PLACE_UNITS.iter().any(|(unit, _)| *unit == word)
}
because!(unary_operation, WordReading, "whether a word names an operation over one number, odd, even or a unit of place value");

pub fn unknown_letter(word: &str) -> bool {
    word.len() == 1 && word.chars().all(|c| c.is_ascii_lowercase()) && !ARTICLE_FLAGS.contains(&word) && !SELF_WORDS.contains(&word) && !SKIPPED.contains(&word) && word != CLOCK_OPENER
}
because!(unknown_letter, WordReading, "whether a word is a single letter that may stand for the unknown of an equation, x, y or n, and no \
     article, speaker or skipped letter");

pub fn expression_word(word: &str) -> bool {
    word == POWER_SIGN || word == BRACKET_OPENS || word == BRACKET_CLOSES || FUNCTIONS.contains(&word)
}
because!(expression_word, WordReading, "whether a word is a bracket, the power sign or a function name, which make a sentence an \
     expression to work out by the rules of arithmetic");

pub fn number_operation(word: &str) -> Option<&'static str> {
    operation_of(word).or_else(|| list_function(word)).or_else(|| operation_in(&COMPARING, word)).or_else(|| [ODD_SIGN, EVEN_SIGN, RANGE_WORD, ALPHABET_WORD].into_iter().find(|w| *w == word)).or_else(|| PLACE_UNITS.iter().find(|(unit, _)| *unit == word).map(|(unit, _)| *unit))
}
because!(number_operation, WordReading, "the operation a word names over numbers, a sign of arithmetic or a word that compares");

pub fn operation_in(table: &[(&'static str, &'static str)], word: &str) -> Option<&'static str> {
    table.iter().find(|(w, _)| *w == word).map(|(_, op)| *op)
}
because!(operation_in, WordReading, "the operation a table of words gives a word, or none");

pub fn operation_of(word: &str) -> Option<&'static str> {
    operation_in(&OPERATORS, word).filter(|op| !op.is_empty())
}
because!(operation_of, WordReading, "the operation a word or sign of arithmetic names, or none");

pub fn flag_of<'a>(mind: &'a CursorMind, key: &str) -> Option<&'a str> {
    mind.flags.iter().rev().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}
because!(flag_of, WordReading, "the value of a flag set on the mind, the newest when set twice, or none when it is not set");

pub fn place_word(word: &str) -> bool {
    MORE_PLACES.contains(&word) || !CONTAINING.contains(&word) && PLACE_RELATIONS.iter().any(|r| r.trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE) == word)
}
because!(place_word, WordReading, "whether a word is a place word, in, on, under and the rest, which says the thing must be placed inside \
     what comes next; the verbs of holding the older place list also names are no place words, since the holder comes before them");

pub fn verb_stem(mind: &CursorMind, word: &str) -> String {
    if let Some(base) = verb_base(mind, word) {
        return base;
    }
    if let Some(past) = told_past(&mind.tree, word) {
        return mind.tree.node(past).name.to_string();
    }
    if let Some(going) = word.strip_suffix(GOING_END) {
        let whole = format!("{going}e");
        if let Some(base) = [going, whole.as_str()].into_iter().find_map(|plain| verb_base(mind, plain)) {
            return base;
        }
    }
    let stems: Vec<&str> = VERB_ENDS.iter().filter_map(|end| word.strip_suffix(end).filter(|stem| stem.len() >= PLURAL_LEAST)).collect();
    let known = std::iter::once(&word).chain(stems.iter()).find(|stem| mind.tree.named(&crate::cursor::step_item(stem)).any(|n| n != 0 && !mind.tree.node(n).gone)).or_else(|| stems.iter().find(|stem| mind.tree.named(stem).any(|n| n != 0 && !mind.tree.node(n).gone)));
    let silent = stems.first().filter(|cut| word.ends_with(PAST_END) && ends_open(cut)).and_then(|_| stems.get(usize::from(true)));
    known.or(silent).or(stems.first()).map_or_else(|| word.to_string(), |stem| stem.to_string())
}

fn ends_open(stem: &str) -> bool {
    let letters: Vec<char> = stem.chars().collect();
    let vowel = |c: char| super::english::VOWEL_LETTERS.contains(c);
    match letters.as_slice() {
        [.., one, two, three] => !vowel(*one) && vowel(*two) && !vowel(*three) && !super::english::GLIDES.contains(*three),
        _ => false,
    }
}
because!(ends_open, WordReading, "whether a verb cut short at ed asks for the e its base ends with: its last three letters are a consonant, a vowel and a consonant, ador of adored and lik of liked, which read as adore and like, while help of helped ends with two consonants and stays as it is");

because!(verb_stem, WordReading, "the plain form of a verb: the one the world or the seeds know, the base of a past form the seeds state, \
     or the word without its verb's ending, the one a relation of the world is named by first, then the one the seeds know, loves as love \
     and not lov, read as read, painted as paint, and a going form by the verb the seeds know with its e put back, writing as write");

pub fn comparison_known(mind: &CursorMind, word: &str) -> bool {
    word.ends_with(COMPARISON_END) && mind.tree.named(word).any(|n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).children.iter().any(|&r| { let name = &*mind.tree.node(r).name; *name == *crate::cursor::step_item(OPPOSITE_WORD) || *name == *crate::cursor::step_item(SUPERLATIVE_RELATION) }))
}
because!(comparison_known, WordReading, "whether the seeds know a word as a comparison, by its opposite or its superlative, bigger, so it \
     waits before its noun as a quality does and teacher does not");

pub fn unknown_word(mind: &CursorMind, word: &str) -> bool {
    ORDINALS.contains(&word) || MEASURES.contains(&word) || comparison_known(mind, word) || word_classes(mind, word) == [OTHER_CLASS]
}
because!(unknown_word, WordReading, "whether a word is in no class at all, a name or an adjective the seeds never state, which waits \
     before its noun as a quality does");

pub fn known_base(word: &str) -> bool {
    [&MOVING[..], &CONTAINING[..], &GIVING[..], &TAKING[..], &DROPPING[..], &OWNING[..], &BELONGING[..], &LOSING[..], &POSING[..], &LEAVING[..]].iter().any(|list| list.contains(&word)) || STATES.iter().any(|(v, _, _)| *v == word)
}
because!(known_base, WordEnglish, "whether a word is the plain form of a verb whose deed the world knows, go, hold, give, take, drop or \
     open");

pub fn verb_base(mind: &CursorMind, word: &str) -> Option<String> {
    if known_base(word) {
        return Some(word.to_string());
    }
    if let Some(base) = VERB_ENDS.iter().filter_map(|end| word.strip_suffix(end)).find(|b| known_base(b)) {
        return Some(base.to_string());
    }
    if HAVING.contains(&word) {
        return Some(OWNS_FORM.trim().trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE).to_string());
    }
    if COPULA.contains(&word) {
        return Some(IS_FORM.trim().trim_matches(|c| c == crate::quiz::BRACE_OPEN || c == crate::quiz::BRACE_CLOSE).to_string());
    }
    if let Some(verb) = verb_of(&mind.tree, word) {
        return Some(verb.to_string());
    }
    let base = mind.tree.named(word).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0).any(|n| mind.tree.node(n).children.iter().any(|&c| [crate::cursor::PAST_FORM, crate::cursor::ASKS_FORM, crate::cursor::FUTURE_FORM].contains(&&*mind.tree.node(c).name)));
    base.then(|| word.to_string())
}
because!(verb_base, WordReading, "the verb a word is a form of: having for any form of having, being for any form of to be, the verb the \
     seeds state the word as a form of, see for saw, the word itself when the seeds state forms of it, and none for a word that is no verb \
     the tree knows");

pub fn past_form(mind: &CursorMind, word: &str) -> bool {
    told_past(&mind.tree, word).is_some()
}
because!(past_form, WordReading, "whether a word is the past form of a verb the seeds state, was, had, went or saw");

pub fn kind_word(mind: &CursorMind, word: &str) -> bool {
    let is = IS_FORM.trim();
    let classes = mind.tree.named(word).any(|n| n != 0 && !mind.tree.node(n).gone && !mind.tree.story(n) && *mind.tree.node(mind.tree.node(n).parent).name == *is);
    let thing = mind.tree.named(word).any(|n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0);
    classes && !thing
}
because!(kind_word, WordReading, "whether a word names a kind the seeds class qualities by, color for red or size for tall: a value under \
     is in the seeds that is no thing under the world, since a car is what a taxi is and still a thing a question asks about");

pub fn kind_of(mind: &CursorMind, quality: &str) -> Option<Arc<str>> {
    let thing = mind.tree.named(quality).find(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0 && !mind.tree.story(n))?;
    let is = child_named(&mind.tree, thing, IS_FORM.trim())?;
    mind.tree.node(is).children.iter().copied().filter(|&c| !mind.tree.node(c).gone).map(|c| mind.tree.node(c).name.clone()).find(|k| super::moves::KINDS.iter().any(|(_, name)| **name == **k))
}
because!(kind_of, WordReading, "the kind the seeds class a quality by, color for red, read from the seed thing of that name under is, one \
     of the kinds a quality has, or none for a word the seeds class by no such kind, as an animal is a living thing");

pub fn compared_relation(word: &str) -> Option<String> {
    word.ends_with(COMPARISON_END).then(|| format!("{word}{COMPARED}"))
}
because!(compared_relation, WordReading, "the relation a comparison names, taller as tallerthan, as the lessons write it");

pub fn is_number(mind: &CursorMind, word: &str) -> Option<String> {
    if number_of(word).is_some() {
        return Some(word.to_string());
    }
    let thing = mind.tree.named(word).find(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0 && !mind.tree.story(n))?;
    let equal = child_named(&mind.tree, thing, crate::cursor::EQUAL_RELATION)?;
    mind.tree.node(equal).children.iter().copied().find(|&c| !mind.tree.node(c).gone).and_then(|c| number_of(&mind.tree.node(c).name)).map(|n| crate::cursor::worked_text(n, None))
}
because!(is_number, WordReading, "the number a word is worth as the tree writes it: a number written in digits as itself, a number word by \
     what the seeds state it equals, and none for any other word");

pub fn word_classes(mind: &CursorMind, word: &str) -> Vec<&'static str> {
    let mut classes = Vec::new();
    if mark_word(word) {
        classes.push(MARK_CLASS);
    }
    if word == APOSTROPHE {
        classes.push(POSSESSIVE_CLASS);
    }
    if COPULA.contains(&word) {
        classes.push(COPULA_CLASS);
        if past_form(mind, word) {
            classes.push(PAST_CLASS);
        }
    }
    if HAVING.contains(&word) {
        classes.push(HAVING_CLASS);
    }
    if ARTICLE_FLAGS.contains(&word) {
        classes.push(ARTICLE_CLASS);
    } else if SKIPPED.contains(&word) {
        classes.push(SKIPPED_CLASS);
    }
    if ASKING.contains(&word) {
        classes.push(ASKS_CLASS);
    }
    if HELPERS.contains(&word) {
        classes.push(HELPER_CLASS);
    }
    if THING_PRONOUNS.contains(&word) {
        classes.push(THING_PRONOUN_CLASS);
    }
    if PERSON_PRONOUNS.contains(&word) {
        classes.push(PERSON_PRONOUN_CLASS);
    }
    if FILLERS.contains(&word) {
        classes.push(FILLER_CLASS);
    }
    if NEGATIONS.contains(&word) {
        classes.push(NEGATION_CLASS);
    }
    if LINKS.contains(&word) {
        classes.push(LINK_CLASS);
    }
    if operation_of(word).is_some() {
        classes.push(OPERATOR_CLASS);
    }
    if SELF_WORDS.contains(&word) || YOU_WORDS.contains(&word) {
        classes.push(SPEAKER_CLASS);
    }
    if OWN_WORDS.iter().any(|(w, _)| *w == word) {
        classes.push(OWN_CLASS);
    }
    if place_word(word) {
        classes.push(PLACE_CLASS);
    }
    if !classes.is_empty() {
        return classes;
    }
    if is_number(mind, word).is_some() {
        classes.push(NUMBER_CLASS);
    }
    if let Some(base) = verb_base(mind, word) {
        classes.push(VERB_CLASS);
        if MOVING.contains(&base.as_str()) {
            classes.push(MOVING_CLASS);
        }
    }
    if past_form(mind, word) {
        classes.push(PAST_CLASS);
    }
    if kind_word(mind, word) {
        classes.push(KIND_CLASS);
    }
    if quality_word(mind, word) {
        classes.push(QUALITY_CLASS);
    }
    let named = |n: usize| n != 0 && !mind.tree.node(n).gone;
    let standing = |n: usize| { let parent = mind.tree.node(n).parent; parent == 0 || !mind.tree.node(parent).name.starts_with(BRACE_OPEN_TEXT) };
    if mind.tree.named(word).any(|n| named(n) && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT) && standing(n)) {
        classes.push(THING_CLASS);
    } else if mind.tree.named(word).any(|n| named(n) && !mind.tree.node(n).name.starts_with(BRACE_OPEN_TEXT)) {
        classes.push(VALUE_CLASS);
    }
    if classes.is_empty() {
        classes.push(OTHER_CLASS);
    }
    classes
}
because!(word_classes, WordReading, "the classes a word is in: the closed classes first, a mark, a copula and whether it is a past form, a \
     form of having, an article or a link word, a question opener, a helper, a pronoun or a place word, which settle the word by \
     themselves; else what the tree makes of it, a number, a verb and whether it moves, a past form, a kind, a quality, a thing standing \
     in the world, under it or inside another thing, or a value the tree holds under a relation, and a word in none of these is other");

pub fn quality_word(mind: &CursorMind, word: &str) -> bool {
    let known = |relation: &str| mind.tree.named(word).filter(|&n| n != 0 && !mind.tree.node(n).gone && mind.tree.node(n).parent == 0 && !mind.tree.story(n)).any(|n| child_named(&mind.tree, n, relation).is_some());
    let _ = known;
    kind_of(mind, word).is_some_and(|k| super::moves::KINDS.iter().any(|(_, name)| **name == *k))
}
because!(quality_word, WordReading, "whether the seeds know a word as a quality, classed by one of the kinds a quality has, red under \
     color; a kitchen the seeds call a room and a brother set against a sister are things, not qualities");

pub fn slot_of_worked(stack: &Stack, text: &str) -> Option<usize> {
    stack.items().enumerate().filter(|(_, e)| e.item.kind == ItemKind::Found && e.item.node.is_none() && *e.item.text == *text && crate::words::number_of(text).is_some()).map(|(depth, _)| depth).last()
}
because!(slot_of_worked, WordReading, "the depth of the newest number a move on the number showed on the stack, so a later move works a \
     first part's result in, the sum of two sides before the third");

pub fn slot_of_text(stack: &Stack, text: &str) -> Option<usize> {
    stack.items().enumerate().filter(|(_, e)| e.item.kind == ItemKind::Input && *e.item.text == *text).map(|(depth, _)| depth).last()
}
because!(slot_of_text, WordReading, "the depth of the deepest word slot holding a text, the dumped word of the question rather than the \
     word just heard, so the teacher's walk points at the question");

pub fn word_at(stack: &Stack, depth: usize) -> Option<String> {
    stack.items().nth(depth).filter(|e| e.item.kind == ItemKind::Input || (e.item.kind == ItemKind::Found && e.item.node.is_none() && crate::words::number_of(&e.item.text).is_some())).map(|e| e.item.text.to_string())
}
because!(word_at, WordReading, "the word a slot holds, or the number a move on the number showed there, none when the slot is neither");

pub fn heard_text(mind: &CursorMind) -> String {
    mind.stack.items().find(|e| e.item.kind == ItemKind::Input).map(|e| e.item.text.to_string()).unwrap_or_default()
}
because!(heard_text, WordReading, "the word the stack was made for, the one input item on it");

pub fn sentence_slots(stack: &Stack) -> Vec<usize> {
    stack.items().enumerate().filter(|(_, e)| e.item.kind == ItemKind::Input).map(|(depth, _)| depth).collect()
}
because!(sentence_slots, WordReading, "the depths of the slots that hold the words of the sentence, the slots a step may point at");

pub fn slot_of_word(stack: &Stack, index: usize) -> Option<usize> {
    let place = feature(WORD_PLACE, index);
    stack.items().enumerate().find(|(_, e)| e.item.kind == ItemKind::Input && e.item.ids.contains(&place)).map(|(depth, _)| depth)
}
because!(slot_of_word, WordReading, "the depth of the slot that holds the word at a place of the sentence, so the teacher's word becomes \
     the pointer's slot");

pub fn action_item(mind: &CursorMind, step: &WordStep) -> Item {
    let class = step.class();
    let mut item = named_item(ItemKind::Action, (ACTION_NAME, step.act.family()));
    item.text = class.clone().into();
    item.ids.push(feature(ACTION_KIND, &class));
    if let Some(word) = step.at.and_then(|depth| word_at(&mind.stack, depth)) {
        item.ids.push(match names_class(mind, &word) {
            Some(named) => feature(NAMES_FEATURE, named),
            None => feature(TOKEN_TEXT, &word),
        });
        let below = step.at.map(|depth| mind.stack.items().skip(depth + 1).filter(|e| e.item.kind == ItemKind::Input).count());
        if let Some(nth) = below {
            item.ids.push(feature(POINTED_NTH, nth));
        }
        item.token = Some(word.into());
        item.bound = step.at.map(|depth| depth.to_string().into());
    }
    item
}
because!(action_item, WordReading, "the event of a step taken: its family and its class, and for a step that points the word pointed at, \
     as what it names when the tree names it, else as its text, with the slot it was taken from and how many words heard stand below it");

pub fn labelling(earlier: &[String]) -> bool {
    earlier.first().is_some_and(|first| first == LABEL_OPENER)
}
because!(labelling, WordReading, "whether the sentence opened with the order to label, given the words heard so far");

pub fn opens_story(earlier: &[String], word: &str) -> bool {
    let said = earlier.len();
    let phrase = said < STORY_OPENER.len() && STORY_OPENER[said] == word && earlier.iter().zip(STORY_OPENER).all(|(heard, opener)| heard == opener);
    let pause = said == STORY_OPENER.len() && word == LIST_MARK && earlier.iter().zip(STORY_OPENER).all(|(heard, opener)| heard == opener);
    phrase || pause
}
because!(opens_story, WordReading, "whether a word is the next word of the opening of a tale said from the start of the sentence, or the comma \
     right after it, given the words heard before it");

pub fn landed_on(mind: &mut CursorMind, by: WordMove, node: Option<usize>) {
    let item = match node {
        Some(n) => { mind.at = n; let text = crate::cursor::bare_name(&mind.tree.node(n).name); found_word_item(mind, &text, by.family(), Some(n)) }
        None => found_word_item(mind, crate::cursor::CURSOR_NOTHING, by.family(), None),
    };
    mind.stack.push(item);
}
because!(landed_on, WordReading, "where a step landed: the cursor moved to the node and the node put on the stack, or the nothing token \
     when the step found nothing");
