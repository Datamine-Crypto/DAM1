use super::mind::{action_item, word_at};
use super::moves::{WordStep};
use crate::cursor::{CursorMind, BRACE_OPEN_TEXT};
use patterns::{because, source};
pub use super::lookup::{ranked_in_seeds, question_match, things_named, claim_node, unseeded, seeded_class, holds, thing_number, activity_word, spans_told, converted, told_relation, told_thing, number_run, place_of, who_has, shifted_value, named_result, measure_asked, map_route};
pub use super::appear::{passive_deed, clause_done, listing, class_after_quality, rounding_here, appeared};
pub use super::lookup::{found_sight, value_told, told_with_article, holds_any, person_named, left_trace, activity_named, unit_worths, says_what, node_relates, owns_relation, has_relation, gained_by_word, traded, said_to_have, defined_name};
pub struct WordWorld;
source!(
    WordWorld,
    "the user's physics of the reading one word at a time: a word is a fact about the world before he moves, a thing word makes the thing \
     appear, the known one of its name or a new one carrying the flags the words before it set, and a mark drops what he holds and the \
     flags; his moves then place things, grab, drop and give, or write on the thing the cursor stands on, and the judge is the shape of \
     the world"
);

pub(super) const TRUE_TAG: &str = "true";

pub(super) const KIND_STEM: usize = 3;
because!(KIND_STEM, WordWorld, "how many letters a word of a question shares with the kind it asks for, feel with feeling, so how does kim \
     feel gets her feeling");

pub const ANSWER_TAG: &str = "{replied}";
because!(ANSWER_TAG, WordWorld, "the tag right under the world that holds what the assistant last answered, where no thing holds it, \
     written with every answer and replaced by the next question's, so a chat may ask what did you say");

pub const OWNER_TAG: &str = "{owner}";
because!(OWNER_TAG, WordWorld, "the tag a thing given takes, with a mention of the one it was given to under it, so the owner stays known \
     when the thing is put somewhere else: ann has a dog and the dog is in the garden");
because!(TRUE_TAG, WordWorld, "the value under the tag of an article, the or a, since the article is kept on the node as a tag");

pub const FOUND_SIGHTS: [&str; 5] = ["word.placed", "word.owned", "word.asked-of", "word.further", "word.relates"];
because!(FOUND_SIGHTS, WordWorld, "the feature places of what is seen of a found node, in the order the sight gives them");

pub const CAUSE_TAG: &str = "{because}";
because!(CAUSE_TAG, WordWorld, "the relation that holds why a thing is as the story told it, the thing the clause after because told of");

pub(super) const STEERED: [&str; 2] = ["fly", "drive"];
because!(STEERED, WordWorld, "the verbs of moving that are also done to a thing one steers, she drives a bus, he flies planes, where to climb a tree or go up a hill is still to go there");

pub(super) const NEAR_DAYS: [&str; 3] = ["today", "tomorrow", "yesterday"];
because!(NEAR_DAYS, WordWorld, "the days named from now, which said after a finished clause only tell when, it is raining today, and make no thing a later it could stand for");

pub const ASSUMED_TAG: &str = "{assumed}";
because!(ASSUMED_TAG, WordWorld, "the tag on a thing whose place the story never told and the world guessed: a thing someone lost is put where they stand, and the tag says so, so a person sees which facts are guesses");

pub const GENDER_TAG: &str = "{gender}";
because!(GENDER_TAG, WordWorld, "the relation the seeds give a person's name, so a name with a gender is a person he or she stands for and \
     it never does");

pub const FIND_SEES_PLACE: &str = "found.placed";
pub const FIND_SEES_OWNER: &str = "found.owned";
pub const FIND_SEES_CONTENTS: &str = "found.holds";
pub const FIND_SEES_ASKED: &str = "found.asked";
because!(FIND_SEES_PLACE, WordWorld, "that the thing a find landed on stands in a place, as a feature of what it found, so the network \
     sees whether where can be answered from it");
because!(FIND_SEES_OWNER, WordWorld, "that the thing a find landed on has an owner, so the network sees whether who has can be answered \
     from it");
because!(FIND_SEES_CONTENTS, WordWorld, "that the thing a find landed on holds or owns things, so the network sees whether what does it \
     hold can be answered from it");
because!(FIND_SEES_ASKED, WordWorld, "that the thing a find landed on holds every other word of the question as a value, the blue of who \
     owns the blue ball, so the network sees whether it is the thing asked about");

pub(super) const ORDER_SPAN: usize = 31;
because!(ORDER_SPAN, WordWorld, "how many steps of an order a get walks at the most, the days of the longest month, so a ring of days or \
     months never runs on");

pub(super) const PLACE_DEPTH: usize = 8;
because!(PLACE_DEPTH, WordWorld, "how many places a check climbs from a thing to the places that hold it, the manuscript in the vault in \
     the library in oxford in england, so a loop of places never runs on");

pub(super) const CLASS_DEPTH: usize = 4;
because!(CLASS_DEPTH, WordWorld, "how many steps of what a thing is a check follows, an apple is a fruit and a fruit is food, so a chain \
     of classes is read and a loop in the seeds never runs on");

pub(super) const PAIR_LEAST: usize = 2;
because!(PAIR_LEAST, WordWorld, "how many things a group holds at the least, since one thing alone is no group");

pub fn word_stepped(mind: &mut CursorMind, step: &WordStep) -> bool {
    let word = match step.at {
        Some(depth) => match word_at(&mind.stack, depth) { Some(w) => Some(w), None => return false },
        None => None,
    };
    if step.act.points() && word.is_none() {
        return false;
    }
    let item = action_item(mind, step);
    mind.stack.push(item);
    mind.steps += 1;
    let word = word.unwrap_or_default();
    let stood = mind.at;
    let at = mind.at;
    let plain = at != 0 && !mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT);
    let braced = at != 0 && !plain;
    if !super::moving::carried(mind, step, &word, at, plain, braced) && !super::answering::told_back(mind, step, &word, at, plain, braced) {
        super::writing::wrote_down(mind, step, &word, at, plain, braced);
    }
    if mind.at != stood {
        let now = super::view::cursor_item(mind);
        mind.stack.push(now);
    }
    true
}
because!(
    word_stepped,
    WordWorld,
    "one step taken on the mind, which says where the cursor came to rest when it moved it, since the stack is all the next \
     step is chosen from and a step that walks to the thing a question asks about leaves the reader blind to what it walked to: \
     a compute pointed at a word of a part writes that part of the whole said after it, a compute pointed at a word of a part writes that part of the whole said after it, a number or what a thing named is worth, as many times as the number before the part says, two fifths of ten and a third of tom's apples, a get of a count of parts the thing holds none of reads the count the seeds write under the part as a relation of the thing or of the seeded thing of its name, a dog's four legs, a get of a choice between two things by a comparison writes the one the story put ahead, by the comparison said or by its opposite the other way, which is faster a car or a bike, a get of the first or the last of a kind that holds no such relation itself reads it off the seeded thing whose first or last is of that kind, the first month off the year, a value added under is right after a word the seeds never state takes that word under itself as what \
     describes it, a useful device, so what the thing is stays the device, a get of a name writes the name a thing goes by when one was \
     told, and for the speaker what they said they are when it is no class or quality, i am michael, and never the word user, a get of \
     what a thing of the seeds is writes the first class the seeds give it when they give several, a dog a mammal, a get of a reply writes \
     what the seeds say is said back to the first word of manners the input said, refused when it points at a slot that holds no word: the \
     step goes on the stack, then its move acts: a hand flags that the next thing appearing is handed on, its drop giving it to the one it \
     is dropped into; a grab, a give or a hold holds the thing the cursor stands on, the give flagging that what appears next is given to \
     it and the hold that it goes inside it; a drop puts the held thing inside the thing that appeared, which the pointed word names, or \
     with a give or a hold the thing that appeared inside the held one, except that a thing given that already stands in a place stays \
     there and only takes its owner, never a thing into itself, and lands on what moved with the flags and the hold dropped, a thing given \
     taking the owner tag with a mention of its receiver, and a thing that moved or changed hands leaving its trace where it was or with \
     who had it; a flag is set for the thing appearing next; the property add writes is under the thing, dated for a past form, or a bare \
     quality; the deed add writes a deed with no object under the thing's did, it broke down; a drop after the past flag leaves only the \
     trace of the held thing in the place, the cat was on the chair; the relate after a role takes the role off the world, or leaves it \
     under is, paris is a capital, to wait for what it is the role of, holding the one said to hold it, london is the capital of; a drop \
     with a role waiting writes the held one under the role as a relation of the thing that appears, england -> capital -> london; the \
     role add writes the waiting role as a relation of the thing the cursor stands on, the capital of england is london; a thing handed on \
     that had no owner leaves its trace with the giver, mary gave the milk to bill; a drop with the time flagged later places the thing \
     and tags it later, the cat will be in the garden; a companion named after an own word is given to the subject as well, with her dog; \
     the leave takes the held subject out of the place that appears, leaving its trace there; been after has lets go of what has held, \
     since it is a copula there, the cat has been in the garden; a value added beside a dated value of a verb's relation is dated too, \
     chased the cat and the bird; a write on a group goes on each member, the cat and the dog are black; the relation add writes the \
     verb's base under the thing, a past form flagging the past for the value that follows, tom met sally, a comparison with than after \
     its word, biggerthan, a verb the seeds do not know by its form without the s, dated for a past form; the value add writes the word \
     under the relation the cursor stands on, a mention when a thing of the name stands under the world, a new value taking the flags; a \
     set of a material in a sentence that said made takes any word, atoms, and stands beside the materials set before it, hydrogen and \
     oxygen; a get of a kind that holds several values writes them all, and one pointed at made gets the material; a set after not writes \
     the quality counted none beside the others of its kind, the cup is not blue, and a set of a quality denied before counts it again; a \
     set writes the quality of the kind under is, when the seeds class it so, replacing the quality of that kind said before, since a \
     thing has one color at a time; a step to the parent lands there, flagging the past when it leaves a dated is, so the place after was \
     is where the thing was, or on the world from a thing standing right in it, a step to the newest thing lands on what it stands for and \
     a step to the top on what he or she stands for; the question add makes the question node under the world, named by the word, and a \
     value add while a question is open writes the word under the question node; the find lands on the story's newest thing of the name \
     outside the question, or else on a value of the story the story told facts of, the cat under what tom sees, or on the story's \
     activity of the name, its mention in a place first, where does tom swim, or on the story's value of a plural name before the seeds' \
     thing of its singular, poems, and last on a value of the seeds, soft under what hard is the opposite of,, or its singular's, else on \
     the seeds' thing of the name, so a question asks about what the seeds know, else on the story's value of the name, who met sally, and \
     on the world when there is none; the question add made after a count said first takes the count as its first word, six plus one; the \
     amount get writes the count of the value a relation of the thing found holds, the dollars a book costs; the about get writes \
     everything the world holds of the thing found, its owner or else its place, what it has and its values, tell me about tom; a check \
     with the past flagged also finds the value as a place the thing left, was mary in the school; after and before on a number, the one \
     found or the one the question holds, write the number one more or one less; the compute of a perimeter writes the lengths told added \
     up, times the sides of the shape, over how many lengths were told, so a rectangle three by four is fourteen around and a square with \
     sides of three is twelve; the compute writes the plural or the singular of the last word of a question, a number said in digits as \
     the words the seeds spell it with, its tens and its ones, and the number that number words spell together; it writes the letter of \
     the last word of a question that stands at the place an ordinal names, the first letter of dog, counts the letters, vowels, \
     consonants or digits of the last word of a question, or the words between its quotes, pointed at a unit turns the one amount of the \
     question into that unit by what the seeds say the units are worth, fourteen days as two weeks, and works a question that is an \
     expression by the rules of arithmetic, and else writes the median, the mode or the range of the numbers a question lists and the sum \
     of a number's digits, which of the words after alphabet comes first or last, how many numbers, odd or even, stand from one number to \
     another, how many of a unit make a number or the digit of that unit, and whether a number is odd or even; with a word that compares \
     it writes yes or no for a copula's question, is twelve greater than twenty, the sum or the rest otherwise, what is ten more than a \
     number, and the greatest or the least for biggest or smallest; the most and least gets write the holder with the most or the fewest \
     of the things their word names, or of the measure a comparison asks by, who is older, who has more apples, over the things the \
     question names when it names several, counting what the seeds count under a relation too, which has more legs a dog or a spider; the \
     possess makes the name under is the thing the held subject is something of, and the relation added next holds the subject when it is \
     a word of kin, ann is tom's mother, while any other noun says what the subject is and gives it to the owner, rex is tom's dog; a \
     check or a subject get on a direction also reads the opposite direction from the other thing, is the chest right of the box; the \
     measure moves a counted unit under is to the relation its word names, tom is five years old; the shifted get writes the member of an \
     order the question asks for from the one the story told, and the distance get how many steps lead to the member its word names, from \
     the first number of a question that says two, the hours from eleven o'clock to two o'clock; the step to the newest for they holds the \
     people who stand together where the newest person stands, a name the seeds do not know counting as a person when no article was said \
     before it, as a group, so what they had or are is written on each; the activity move makes a deed the activity of its verb's stem, \
     and makes a verb's empty relation the activity it names, dated when the verb was past, kim swam, or after to adds the activity \
     relation beside the verb's, tom loves to swim, and a drop of what the seeds class as an activity into one a word of having gave it \
     to, and no possessive, writes it as their activity, they had a picnic, on each of a group, and a drop of an activity leaves it where \
     it is and puts a mention of it in the place, swim in the river; a relation get on a verb's relation that holds nothing reads the \
     activity, what does tom love; the regard makes a quality under is a relation of the subject, afraid of wolves; the children get of a \
     question with a word of having, asked of what is no person, writes only what that thing was said to have, the door of the house and \
     never the pen that is in the box; a drop after not places nothing, writes a mention of the thing counted none in the place and lands \
     on the thing, the cat is not in the house; a grab at the word at flags the place with that word, so the number after it is known for \
     an hour; a belong on a mention holds the thing it links to, and a drop while a belong is flagged places whatever word it points at; a \
     relation added for the word means, for a word of arithmetic, or while a count is flagged for a word that is no verb, keeps the word \
     as said, dead and no stem of it; the distance get on a place of a map of directions, pointed at another, writes the steps of the \
     route between them; a check of right on a person who claimed something holds when the story tells every quality and place they \
     claimed, was tom right; a find in a question of a claim lands on the thing of the claim of the claimer named, and a drop of the thing \
     of a claim leaves it where it is and mentions it in the place, as a drop of an activity does; a check of a superlative holds for the \
     thing at the end of the chain its comparison draws, is sam the shortest; a check of having does not count a thing the one only sold, \
     found or bought and no longer holds; the past get of a thing that left no trace writes the place it stands in, where was curie; the \
     past get of a question that asks who writes every one that had the thing before, in the order they had it, rose and then tim; a get \
     of a kind pointed at what, on a thing of the story with several values under is, plain or under a kind, writes them all, a fern is \
     alive and a plant, a puppy young and a dog; a get of a kind pointed at a measure, on a thing of the seeds that has none, writes \
     nothing, how old is rose; a get of a kind pointed at a class the seeds know takes only a value the seeds class so, what shape is the \
     tent is nothing when the tent is a shelter; a get of a kind pointed at why writes the state of the person whose goal the seeds give, \
     the one that sends them to a place the question names first, and a location get in a question that says will writes the place a state \
     sends the person to; a get of a kind pointed at who or what, on a thing told nothing it is, writes the word of kin it is the value \
     of, ann the sister; a get of a kind pointed at a quality reads the kind the seeds class the quality by, how big is tom his size; a \
     get of a kind passes over a value the story denied, a whale is no fish; a get of a kind the thing lacks, or a check of a quality of a \
     kind it lacks, takes it from another thing of the story that is what the thing is, brian is white as bernhard is; a group dropped \
     into a line is each told its place there, first, second, third, and the last one last; the grab of all holds everything the giver has \
     as a group, and the drop of a group handed on gives each to the one it is dropped into, he gave them to ann; the clock set gives the \
     subject above is its time, counted in clock by the number flagged, it is three o'clock; the moves on the number set it from a pointed \
     word, a number or the number the word's thing holds, work another pointed word into it by the operation of their kind, or change it \
     alone, each showing the number on the stack, and the say writes it into the output; the total get writes the count of the things its \
     word names over the holders the question names, or over every holder, and the difference get how many more the one has than the \
     other; an activity on the one value of a deed makes the deed an activity of its doer with the value under of, tom plays football with \
     ben, and a relation added on a mention of an activity, or a verb added on a value under a relation, is written on the thing of the \
     story the value names, made when there is none, the cat chased the bird after the dog chased the cat; a possess on the value my or \
     your takes the user or the assistant for the owner, ann is my sister; a release flags the verb said, so a count lost with no thing \
     named can be written under it; a drop of a counted thing handed on by one who has more of that name takes the count from what they \
     have, the fund gave part of its dollars; a find of a thing with a value takes the one had by someone the question names, tom's dog, \
     and none when the one named has none; the owner get of a thing with several owners, or of a mention of it, writes them all; a drop \
     that gives a thing to a group held places it in the last of them and mentions it in each of the others, all of them its owners; a \
     step to the newest for they counts as a person a thing told by its bare name and placed by the last sentence, sandra and daniel whom \
     the seeds do not know; a step to the newest for they, when the newest thing has several owners, holds those owners as a group, tom \
     and jane have a boat and they are happy; a find of they that finds only the value of a relation finds the newest person in its place, \
     where did they go after they had fun; the relation get pointed at the verb of the activity the cursor stands on writes what the \
     activity is of, and the location get on an activity writes the place its mention stands in; the relation get of a word of saying no \
     story told a saying by writes what it last answered; the relation get writes every value a relation holds, and only those of a kind \
     the question names, which fish does a wolf eat, reads a grandparent as the mother or the father of a parent, and reads the relation \
     of the thing found, of another thing of its name or of what the thing is, rex likes what a dog likes, the spring of the seeds for the \
     spring of the story; the relation get reads only forward, who did joe help, but a role that holds both ways also backward, what is \
     the opposite of cold; the subject get takes a subject of the story first, and one of the seeds only when the story never told the \
     relation, who eats the fish, or when the relation is a role, a fact of the world, what is the same as a sofa, and writes the thing \
     whose relation holds the thing found, read backward, or the value of the opposite relation of the thing found, who is ann liked by, \
     who is shorter than tom; a get of a place asked of a name alone writes the place of every thing of that name, where is the car, and a \
     subject get every subject of the story; a plain check also holds for the member of an order a word stands for from the one the story \
     told, is tomorrow tuesday, and with a number said it holds only when the count is that number, does a bird have four legs; a check \
     with a place word flagged holds only when the value is a place above the thing, is oxford in the vault, and one of a superlative also \
     needs the kind named after it right under is, is the pacific the largest planet; a check with a comparison of a measure flagged \
     compares the two counts, is ann older than tom; a check with a relation flagged as the property asked holds only under that relation, \
     of any node of the thing's name, of the thing or of what it is, either way for a role that holds both ways, is cold the opposite of \
     hot, through a chain of it or through its opposite read backward, does ann see tom, is ann shorter than tom; the given get writes \
     what the one found had that the one its word names has now, what did mary give to bill; the before and after gets write the place or \
     the holder the thing found was in or with before or after the one their word names, from its traces and where it is now, in the order \
     of the day when each move was told with its time; the past get writes where the thing found was, or who had it, from the newest trace \
     it left, or who has it when it never changed hands; the naming move closes the question just worked out and makes a thing of its word \
     whose is holds the result, one plus one equals sum; the relation get writes the value under the relation its word names, or the thing \
     whose relation holds the thing found, the opposite of cold; the compute of a word that asks of the run of numbers the story told \
     writes the next, the missing, the first, the last, the largest or the smallest of them, how many they are or their sum; the compute \
     pointed at long, on a thing told to start and to end, writes the hours between, and with several such things in the question their \
     sum, or with than how far apart they are, and nothing for a thing told no start and end; the most and the least get by the measure \
     long also count those hours; the compute of a comparing word in a question that asks which writes the word of the greater or the \
     smaller number, a half or a quarter; the compute of an expression writes a number that is not whole to three decimals, or to the \
     places the story dropped the rounding in, as does the compute of one operation, and a number divided by nothing as infinity; the \
     compute of an equation with one unknown letter names the letter with what solves it and goes back to the world, x plus one equals \
     two; the compute works out the numbers of the question by the operation its word names; a find for a pronoun lands on what the \
     pronoun stands for; the next find lands on the next older thing of the same name, the with find on the newest thing holding the value \
     its word names, the choice get writes which of the options beside or the thing found holds; the all get of a comparing word also \
     writes a thing that holds the word as a plain value, a bigger box; a superlative the story told of a thing outright stands before any \
     end of a chain, ben is the oldest; the far end of a chain told by the opposite comparison is the value under it that no thing of its \
     name goes on from, the lowest after the urals are higher than the downs; the all get leaves out a thing that is one of the others it \
     lists, told with the, tom is the cat and the cat is black; a kind named beside a superlative must stand right under is of the thing, \
     the fastest animal and no bird; the all get in a question that denies writes every thing the story denied the value of, which is not \
     a bird; it writes for a superlative the far end of the story's chain of its comparison, the tallest of tom, ann and kim, or the near \
     end of the opposite one, and else the name of every thing holding the value its word names and every other value the question names, \
     the largest planet, or holding the relation a comparison names, who is taller, the name get writes the name of the thing found, the \
     count get how many things of its word stand inside it or are owned by it, with holds the subject so that it and the next thing to \
     appear stand in one place, the thing that appears going to the subject's place or the subject to the place of the thing; the join \
     after a thing just placed ends that clause and starts the next at the world, the key is in the cup and the cup is in the bag, else it \
     puts the thing the cursor stands on in a group that moves together, or after a thing a holder was just said to hold takes the holder up again to hold more, the garage holds a bike and a car, or after a thing just given holds its owner again to give more, \
     the change of state on a verb of state flags the state it sets, and on the thing that appears after it sets that state under its is \
     and takes away the one it undoes, the step to the user lands on the user or the assistant the word says, made when the story has \
     none, every find adding what it sees of the thing to its event; a find of the word i in a question of a successor, when the story \
     told a thing of that name, finds the letter and not the user; the total and the difference get count a thing had by what it is worth \
     in the unit asked when it is not of that unit; the count get of a unit adds what the things had are worth in it, three nickels \
     fifteen cents, to those of the unit itself; the location get of a thing the story told by its bare name, with no place of its own, \
     writes the place the seeds give a thing of its name, china in asia; the location get of a thing placed for a time to come writes \
     nothing unless the question says will; the location get writes the place the thing found stands in, never a person who has it, or for \
     a thing of the seeds the place under its place relation, the owner get who has it, its owner or the person or owner of what holds it, \
     else its parent, or the thing above the relation it stands under, bird for wings, the count get nothing when none fits, but none for \
     every thing asked of a thing of the story that carries nothing, and none when the story said the thing has none, the children get \
     every thing inside it or owned by it, the property get the value under the kind its word names, or the kind its word shares its first \
     letters with, feel for feeling, or else the one quality the thing has when the seeds class it by the kind asked, or any quality when \
     the question asks by how or a verb, how does paul feel, silver being a color as well as a material, and nothing when it is of another \
     kind, as the small of a mouse asked its color, and the check yes or no by whether the thing holds the value its word names; pointing \
     at nothing passes over the place a moving thing came from, and continuing does nothing"
);

fn cursor_thing(mind: &CursorMind) -> Vec<usize> {
    let at = mind.at;
    if at == 0 || mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) {
        return Vec::new();
    }
    let name = mind.tree.node(at).name.to_string();
    std::iter::once(at).chain(mind.tree.named(&name).filter(|&n| n != 0 && n != at && !mind.tree.node(n).gone && mind.tree.node(n).link.is_none())).collect()
}
because!(cursor_thing, WordWorld, "the thing a sight reads: the one the cursor stands on and every other node of its name, since a get \
     of a relation is read off the thing asked about and not off the word being heard, which at a question mark names nothing");

fn asked_relations(mind: &CursorMind) -> Vec<String> {
    mind.before.iter().map(|said| super::lookup::relation_named(mind, said)).collect()
}
because!(asked_relations, WordWorld, "the relations the sentence names, one for each word it has said, since the \
     relation a question asks by is said before the mark that answers it");

pub(super) fn holds_asked(mind: &CursorMind) -> bool {
    let asked = asked_relations(mind);
    cursor_thing(mind).into_iter().any(|n| asked.iter().any(|relation| super::lookup::own_child(mind, n, relation).is_some()))
}
because!(holds_asked, WordWorld, "whether the thing the cursor stands on holds a relation the sentence names, so the get that reads it \
     forward is told from the gets that must look elsewhere");

pub(super) fn holds_doing(mind: &CursorMind) -> bool {
    let doing = crate::cursor::step_item(super::mind::ACTIVITY);
    cursor_thing(mind).into_iter().any(|n| super::lookup::present_children(mind, n).into_iter().any(|r| {
        mind.tree.node(r).name.starts_with(BRACE_OPEN_TEXT) && (*mind.tree.node(r).name == *doing || super::lookup::own_child(mind, r, &doing).is_some())
    }))
}
because!(holds_doing, WordWorld, "whether what the thing the cursor stands on holds is a doing of its own, tom loves to swim, so the \
     get that reads the doing is told from the get that reads a plain value");

pub(super) fn held_under(mind: &CursorMind) -> bool {
    let asked = asked_relations(mind);
    cursor_thing(mind).into_iter().any(|n| {
        let up = mind.tree.node(n).parent;
        up != 0 && asked.iter().any(|relation| *mind.tree.node(up).name == **relation) && mind.tree.node(up).parent != 0 && mind.tree.node(up).parent != mind.at
    })
}
because!(held_under, WordWorld, "whether the thing the cursor stands on stands under a relation the sentence names that another thing \
     holds, so the get that reads a relation backward is told from the one that reads it forward");

pub(super) fn reaches_through(mind: &CursorMind) -> bool {
    let at = mind.at;
    at != 0 && !mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) && mind.before.iter().any(|said| super::lookup::grandparent(mind, at, said).is_some())
}
because!(reaches_through, WordWorld, "whether a walk of two steps out of the thing the cursor stands on reaches a relation the \
     sentence names, the father of one of its parents for a grandfather, so the get that walks through is told from the answer of \
     nothing, where the thing has parents but none of them holds what is asked");

pub(super) fn holds_said(mind: &CursorMind) -> bool {
    cursor_thing(mind).into_iter().any(|n| super::lookup::present_children(mind, n).into_iter().flat_map(|c| std::iter::once(c).chain(super::lookup::present_children(mind, c))).any(|m| {
        let name = crate::cursor::bare_name(&mind.tree.node(m).name);
        !mind.tree.node(m).name.starts_with(BRACE_OPEN_TEXT) && mind.before.iter().any(|said| *super::view::singular(said) == *super::view::singular(&name))
    }))
}
because!(holds_said, WordWorld, "whether the thing the cursor stands on holds a thing a word of the sentence names, under itself or \
     under one of its relations, so a dog asked whether it has a tail is told from a dog asked whether it has a horn, which with the \
     names taken away are the same question");

pub(super) fn manners_said(mind: &CursorMind) -> bool {
    mind.before.iter().any(|said| super::lookup::has_relation(mind, said, super::english::REPLY_RELATION))
}
because!(manners_said, WordWorld, "whether a word the sentence has said is one of manners the seeds give a reply, good evening, since \
     the get of a reply is chosen at the mark that closes the turn and the mark is no word of manners itself");

pub(super) fn ranked_kind(mind: &CursorMind) -> bool {
    let at = mind.at;
    if at == 0 || mind.tree.node(at).name.starts_with(BRACE_OPEN_TEXT) {
        return false;
    }
    let name = mind.tree.node(at).name.to_string();
    mind.before.iter().any(|said| (*said == *super::mind::ORDINALS[0] || *said == *super::mind::LAST_PLACE) && super::answering::ranked_value(mind, &name, &super::lookup::relation_named(mind, said)).is_some())
}
because!(ranked_kind, WordWorld, "whether the seeds rank the kind the cursor stands on, for a first or a last the sentence asks, the \
     days of a week or the months of a year, so the get that writes the first or the last of a kind is told from the get that writes \
     what a thing is");

pub(super) fn owned_twice(mind: &CursorMind) -> bool {
    cursor_thing(mind).into_iter().any(|n| { let thing = mind.tree.node(n).link.unwrap_or(n); super::lookup::own_child(mind, thing, OWNER_TAG).is_some_and(|tag| super::lookup::present_children(mind, tag).len() > 1) })
}
because!(owned_twice, WordWorld, "whether the thing the cursor stands on has been handed on more than once, so the get that writes \
     every owner it has had is told from the get that writes the owner it has");

pub(super) fn owned_thing(mind: &CursorMind) -> bool {
    cursor_thing(mind).into_iter().any(|n| super::lookup::own_child(mind, n, OWNER_TAG).is_some())
}
because!(owned_thing, WordWorld, "whether the thing the cursor stands on has an owner written on it, so the get of an owner is told \
     from the others");
