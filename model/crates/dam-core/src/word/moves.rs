use crate::quiz::{BRACE_CLOSE, BRACE_OPEN};
use patterns::{because, source};

pub struct RecordClasses;
source!(
    RecordClasses,
    "the user's way of writing a step: a record in braces, the family of the move, its kind and its arguments each with a name, the \
     pointed word standing in for the pointer mark, setFlag type: quantity value: and a count, and children add: is type: property time: \
     past, so a step reads as one thing with its properties; and the user's game: a word is a fact about the world, a thing word makes the \
     thing appear, and his moves are only what the facts leave to do, grab, drop and give, so corn in bag is two moves"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WordMove {
    Grab,
    Drop,
    SetProperty,
    Give,
    Contain,
    Hand,
    Take,
    Release,
    Join,
    Together,
    Relate,
    AddRole,
    ChangeState,
    Belong,
    Accompany,
    Leave,
    FlagThe,
    FlagA,
    FlagQuantity,
    FlagProperty,
    FlagTime,
    FlagLater,
    FlagFrom,
    FlagWhen,
    FlagWhose,
    AddProperty,
    AddPropertyPast,
    AddRelation,
    AddRelationPast,
    AddDeed,
    AddValue,
    StepParent,
    StepNewest,
    StepTop,
    PointNothing,
    AddQuestion,
    FindAsked,
    GetLocation,
    GetChildren,
    GetOwner,
    GetName,
    GetCount,
    NumberSet,
    NumberAdd,
    NumberSubtract,
    NumberMultiply,
    NumberDivide,
    NumberLarger,
    NumberSmaller,
    NumberRemainder,
    NumberPower,
    NumberPercent,
    NumberAddPercent,
    NumberLessPercent,
    NumberRound,
    NumberNegate,
    NumberMean,
    NumberWhole,
    NumberRoot,
    NumberRoundDefault,
    NumberSay,
    GetTotal,
    GetDifference,
    GetMost,
    GetLeast,
    FindNext,
    FindWith,
    GetAllWith,
    StepUser,
    GetKind,
    GetRelation,
    GetRelationDoing,
    GetRelationThrough,
    GetRelationRanked,
    GetRelationBackward,
    GetLocationClaimed,
    GetLocationMotive,
    GetKindWhy,
    GetKindMeasure,
    CheckRelation,
    CheckRight,
    GetRelationSaid,
    GetRelationToward,
    ActivityNamed,
    ActivityDone,
    ActivityUnder,
    FindAskedStood,
    GetDistanceRoute,
    GetOwnerEvery,
    GetSubject,
    GetPast,
    GetShifted,
    GetReply,
    GetChoice,
    GetDistance,
    Measure,
    Possess,
    GrabAll,
    SetClock,
    Activity,
    Regard,
    GetAbout,
    GetBefore,
    GetAfter,
    GetGiven,
    GetAmount,
    Compute,
    NameResult,
    Check,
    Continue,
}
because!(
    WordMove,
    RecordClasses,
    "the moves the network takes one word at a time, as the user agreed them on the debug page: grab the thing the cursor stands on, since \
     a place word or a moving verb says it must be placed; drop it into the thing that appeared, or with a give the thing that appeared \
     into what he holds; give, holding the thing as the receiver of what comes; set a flag the next thing appearing takes, the article the \
     or a, a count, a quality said before its noun, or the past of a copula said before any thing; add the property relation is, dated \
     when the copula is a past form, or a bare property; add a relation from a verb; add a value under the relation the cursor stands on; \
     set a quality of a kind the seeds name, color, size, feeling, speed, temperature, age or material, as the property of that kind under \
     is; step to the parent, to the newest thing or to the newest top thing; point at nothing for a word that does nothing; for a \
     question, add the question node named by its first word and its words under it, and at the question mark, the question dumped on the \
     stack, find the thing asked by a word of it, get where the thing stands, what stands inside it or its property of a kind the question \
     names, or check that a value is on it, each writing the answer into the output; and continue to the next word"
);

pub const RECORD_MARK: &str = "@";
because!(RECORD_MARK, RecordClasses, "what stands in a move's class for the word the step points at, so one class serves every word");

const KEY_MARK: &str = ": ";
because!(KEY_MARK, RecordClasses, "what follows the name of an argument in a move's class, as the user writes a record");

const PART_MARK: &str = " ";
because!(PART_MARK, RecordClasses, "what parts the family of a move from its kind and one argument from the next in a move's class");

const TYPE_KEY: &str = "type";
because!(TYPE_KEY, RecordClasses, "the argument of an add that says what kind of node it adds, left out for a value, the default, and of a \
     flag that says which flag");

const VALUE_KEY: &str = "value";
because!(VALUE_KEY, RecordClasses, "the argument of a flag that holds what the flag is set to");

const TIME_KEY: &str = "time";
because!(TIME_KEY, RecordClasses, "the argument of an add that dates the node it adds, and the flag that dates the thing appearing next");

const ADD_KIND: &str = "add";
because!(ADD_KIND, RecordClasses, "the kind of a children move that adds a child");

const RELATION_TYPE: &str = "relation";
const DEED_TYPE: &str = "deed";
because!(DEED_TYPE, RecordClasses, "the type of an added node that holds a deed the subject did with no object, it broke down");
const PROPERTY_TYPE: &str = "property";
const QUESTION_TYPE: &str = "question";
because!(QUESTION_TYPE, RecordClasses, "the type of the node a question opens with, named by its first word, what or where, under which \
     the question's words stand");
because!(RELATION_TYPE, RecordClasses, "the type of an added node written from a verb, see or like");
because!(PROPERTY_TYPE, RecordClasses, "the type of an added node that holds what the copula says of the thing, is, or a bare quality, red \
     or big");

const TRUE_VALUE: &str = "true";
because!(TRUE_VALUE, RecordClasses, "the value of a flag that is set or not, the or a");

const PAST_VALUE: &str = "past";
because!(PAST_VALUE, RecordClasses, "the value of the time flag and of the time argument, the past a copula's past form says");

const DEFINITE_FLAG: &str = "the";
const INDEFINITE_FLAG: &str = "a";
const QUANTITY_FLAG: &str = "quantity";
const PROPERTY_FLAG: &str = "property";
const LATER_VALUE: &str = "later";
because!(LATER_VALUE, RecordClasses, "the value of the time flag will sets: what is placed next will be there later");
const FROM_FLAG: &str = "from";
const WHOSE_FLAG: &str = "owned";
because!(WHOSE_FLAG, RecordClasses, "the key of the flag that names who the thing appearing next belongs to, as the user writes a record");

const WHEN_FLAG: &str = "when";
because!(WHEN_FLAG, RecordClasses, "the flag a time of day sets: the move of the sentence was made at the time it names");
because!(FROM_FLAG, RecordClasses, "the flag from sets: the thing appearing next is where the held thing came from");
because!(DEFINITE_FLAG, RecordClasses, "the flag the article the sets: the thing appearing next is one already known");
because!(INDEFINITE_FLAG, RecordClasses, "the flag the article a sets: the thing appearing next is a new one");
because!(QUANTITY_FLAG, RecordClasses, "the flag a count sets: the thing appearing next takes it as its quantity");
because!(PROPERTY_FLAG, RecordClasses, "the flag a quality said before its noun sets: the thing appearing next takes it as a value under is");

const GRAB_FAMILY: &str = "grab";
const DROP_FAMILY: &str = "drop";
const GIVE_FAMILY: &str = "give";
const CONTAIN_FAMILY: &str = "contain";
const HAND_FAMILY: &str = "hand";
const TAKE_FAMILY: &str = "take";
const GROUP_FAMILY: &str = "group";
because!(GROUP_FAMILY, RecordClasses, "the family of the move that makes a group of the things held: they become one thing of their own, john and i are best friends, and what the sentence says of them is written on the group and not on each of them");

const JOIN_FAMILY: &str = "join";
const ROLE_FAMILY: &str = "role";
const ROLE_KIND: &str = "role";
because!(ROLE_FAMILY, RecordClasses, "the family of the move for of after a role: the role is taken off the world and waits for the thing \
     it is the role of, and the one said to hold it is held");
because!(ROLE_KIND, RecordClasses, "the kind of the add that writes the waiting role as a relation of the thing the cursor stands on, the \
     capital of england is");
const BELONG_FAMILY: &str = "belong";
const WITH_FAMILY: &str = "with";
const LEAVE_FAMILY: &str = "leave";
because!(LEAVE_FAMILY, RecordClasses, "the family of the move for a verb of leaving: the subject is held and taken out of the place \
     appearing next");
because!(WITH_FAMILY, RecordClasses, "the family of the move for with: the subject is held so that it and the next thing to appear stand \
     in one place");
because!(BELONG_FAMILY, RecordClasses, "the family of the move for a verb of belonging: the subject is held and given to the next thing to \
     appear, the ball belongs to lily");
const STATE_FAMILY: &str = "setState";
because!(JOIN_FAMILY, RecordClasses, "the family of the move for and: the thing the cursor stands on joins a group that moves together, or \
     what was just given is followed by more of the same giving");
because!(STATE_FAMILY, RecordClasses, "the family of the move for a verb of state: the thing appearing next takes the state the verb sets, \
     tom opens the box");
const RELEASE_FAMILY: &str = "release";
because!(TAKE_FAMILY, RecordClasses, "the family of the move for a verb of taking, tom takes the key: the subject is held and the thing \
     named next moves into it");
because!(RELEASE_FAMILY, RecordClasses, "the family of the move for a verb of dropping, sam drops the ball: the subject is held and the \
     thing named next is put where the subject stands");
because!(HAND_FAMILY, RecordClasses, "the family of the move for a verb of giving on to someone, ana gives the book to leo: the thing \
     named next is grabbed and dropped into the one after to");
because!(CONTAIN_FAMILY, RecordClasses, "the family of the move that holds the thing the cursor stands on as the container of what appears \
     next, for a verb of holding, the bag holds corn");
const FLAG_FAMILY: &str = "setFlag";
const CHILDREN_FAMILY: &str = "children";
const SET_FAMILY: &str = "setProperty";
const STEP_FAMILY: &str = "step";
const POINT_FAMILY: &str = "point";
const CONTINUE_FAMILY: &str = "continue";
const FIND_FAMILY: &str = "find";
const GET_FAMILY: &str = "get";
const KIND_GET_FAMILY: &str = "getPropertyValue";
const CHECK_FAMILY: &str = "check";
because!(FIND_FAMILY, RecordClasses, "the family of the move that finds the thing a question asks about by a word of the question");
because!(GET_FAMILY, RecordClasses, "the family of the moves that write into the output what is around the thing found, where it stands or \
     what stands inside it");
because!(KIND_GET_FAMILY, RecordClasses, "the family of the move that writes into the output the property of the kind a word of the \
     question names, the user's getPropertyValue");
because!(CHECK_FAMILY, RecordClasses, "the family of the move that writes yes or no into the output by whether the thing found holds the \
     value a word of the question names");
const NAME_KEY: &str = "name";
const ORDER_KEY: &str = "orderBy";
const LAST_MENTIONED: &str = "lastMentioned";
const LOCATION_KIND: &str = "location";
const CHILDREN_KIND: &str = "children";
const OWNER_KIND: &str = "owner";
const NAME_KIND: &str = "name";
const NEXT_KIND: &str = "next";
const WITH_KEY: &str = "with";
const ALL_KIND: &str = "all";
because!(ALL_KIND, RecordClasses, "the kind of the get that writes every thing holding the value its word names, what is white");
const COUNT_KEY: &str = "count";
const TOTAL_KEY: &str = "total";
const DIFFERENCE_KEY: &str = "difference";
const MOST_KEY: &str = "most";
const LEAST_KEY: &str = "least";
because!(MOST_KEY, RecordClasses, "the argument of the get that writes the holder with the most of the things its word names, who has more \
     apples");
because!(LEAST_KEY, RecordClasses, "the argument of the get that writes the holder with the fewest of the things its word names, who has \
     fewer apples");
because!(DIFFERENCE_KEY, RecordClasses, "the argument of the get that writes how many more of the things its word names one holder the \
     question names has than the other");
because!(TOTAL_KEY, RecordClasses, "the argument of the get that writes the count of the things its word names over every holder the \
     question names, or over every holder when it names none");
const USER_KIND: &str = "user";
const RELATION_KEY: &str = "relation";
const SUBJECT_KEY: &str = "subject";
because!(SUBJECT_KEY, RecordClasses, "the argument of the get that writes the thing whose relation holds the thing found, read backward, \
     who is ann liked by, who is taller than tom");
const DOING_KIND: &str = "doing";
because!(DOING_KIND, RecordClasses, "the kind of the get that writes what the thing found is doing when it holds no such relation of its \
     own, what does tom love answered from the swimming he loves to do");
const THROUGH_KIND: &str = "through";
because!(THROUGH_KIND, RecordClasses, "the kind of the get that writes the thing two steps up, so a relation asked of a thing inside \
     another is answered by the one that holds them both");
const RANKED_KIND: &str = "ranked";
because!(RANKED_KIND, RecordClasses, "the kind of the get that writes the first or the last of a kind the seeds order, the first day of the \
     week");
const CLAIMED_KIND: &str = "claimed";
because!(CLAIMED_KIND, RecordClasses, "the kind of the get of a place that writes where a mention of the thing stands, for a thing a \
     claim or a doing holds rather than a place");

const MOTIVE_KIND: &str = "motive";
because!(MOTIVE_KIND, RecordClasses, "the kind of the get of a place that writes the place a motive names, for a move still to come");

const WHY_KIND: &str = "why";
because!(WHY_KIND, RecordClasses, "the kind of the get that writes why a thing is as it is, the cause a because gave it or the state \
     a motive sent it for");

const MEASURE_KIND: &str = "measure";
because!(MEASURE_KIND, RecordClasses, "the kind of the get that writes nothing for a measure asked of a thing the story never gave \
     one, so a height asked of a thing that has none is answered nothing and not by a measure of another");

const STOOD_KIND: &str = "stood";
because!(STOOD_KIND, RecordClasses, "the kind of the find that walks to what a pronoun stands for, the newest thing or person it can \
     mean, rather than to a thing the word names");

const ROUTE_KIND: &str = "route";
because!(ROUTE_KIND, RecordClasses, "the kind of the get of a distance that adds the steps of a route the map knows, rather than \
     reading one distance told outright");

const NAMED_KIND: &str = "named";
because!(NAMED_KIND, RecordClasses, "the kind of the nesting that turns a deed already written into a relation of the thing and opens \
     a doing under it, for a verb said in its plain form after to");

const DONE_KIND: &str = "done";
because!(DONE_KIND, RecordClasses, "the kind of the nesting that takes the deed the thing carries and writes it as the doing itself");

const UNDER_KIND: &str = "under";
because!(UNDER_KIND, RecordClasses, "the kind of the nesting that writes the doing on the doer above, where the thing the cursor \
     stands on is held by a deed");

const SAID_KIND: &str = "said";
because!(SAID_KIND, RecordClasses, "the kind of the get of a relation that writes what was said back last, for a word of manners the \
     story never told as a relation of the thing");

const TOWARD_KIND: &str = "toward";
because!(TOWARD_KIND, RecordClasses, "the kind of the get of a relation that writes what a doing is done toward, where the thing the \
     cursor stands on is the doing itself");

const RELATION_CHECK: &str = "relation";
because!(RELATION_CHECK, RecordClasses, "the kind of the check that reads a relation flagged before the word, is tom taller than \
     ann, rather than a value the thing holds");

const RIGHT_CHECK: &str = "right";
because!(RIGHT_CHECK, RecordClasses, "the kind of the check that reads whether a claim the story made was right");

const BACKWARD_KIND: &str = "backward";
because!(BACKWARD_KIND, RecordClasses, "the kind of the get that writes the holder of a relation that goes both ways, read from the value \
     back to the thing that holds it, who is jane friends with");

const PAST_KIND: &str = "past";

const CHOICE_KIND: &str = "choice";
because!(CHOICE_KIND, RecordClasses, "the kind of the get that writes which of the options a question offers the thing found holds, red or \
     blue");

const SHIFTED_KIND: &str = "shifted";

const REPLY_KIND: &str = "reply";
because!(REPLY_KIND, RecordClasses, "the kind of the get that writes what the seeds say is said back to a word of manners, hello to hi, \
     when the input held nothing else");

const DISTANCE_KEY: &str = "distance";

const POSSESS_FAMILY: &str = "possess";
because!(POSSESS_FAMILY, RecordClasses, "the family of the move for an apostrophe after a name under is: the name becomes the thing the \
     subject is something of, ann is tom's mother");

const ACTIVITY_FAMILY: &str = "activity";
because!(ACTIVITY_FAMILY, RecordClasses, "the family of the move that makes what the cursor stands on an activity of the thing above it: a \
     verb's empty relation becomes the activity it names, and any other relation gets the activity relation beside it");

const REGARD_FAMILY: &str = "regard";
because!(REGARD_FAMILY, RecordClasses, "the family of the move for of after a quality under is: the quality becomes a relation of the \
     subject, whose value is what it is about, afraid of wolves");

const GRAB_ALL_FAMILY: &str = "grabAll";
because!(GRAB_ALL_FAMILY, RecordClasses, "the family of the move for them after a verb of giving: everything the giver has is held as a \
     group, to be dropped into the one named after to");

const CLOCK_FAMILY: &str = "clock";
because!(CLOCK_FAMILY, RecordClasses, "the family of the move for clock after a number under is: the subject gets its time, counted in \
     clock by the number flagged");

const MEASURE_FAMILY: &str = "measure";
because!(SHIFTED_KIND, RecordClasses, "the kind of the get that writes the member of an order as many steps from the one the story told as \
     the question counts, or as the word found stands from the word the story told it of");
because!(DISTANCE_KEY, RecordClasses, "the argument of the get that writes how many steps of an order lead from the member the story told \
     to the one its word names");
because!(MEASURE_FAMILY, RecordClasses, "the family of the move for a word of measure after a counted unit: the count moves under the \
     relation the word names, five years old");

const AMOUNT_KIND: &str = "amount";

const ABOUT_KIND: &str = "about";

const BEFORE_KEY: &str = "before";

const AFTER_KEY: &str = "after";

const GIVEN_KEY: &str = "given";
because!(GIVEN_KEY, RecordClasses, "the argument of the get that writes what the one found had and the one its word names has now, what \
     did mary give to bill");
because!(ABOUT_KIND, RecordClasses, "the kind of the get that writes everything the world holds of the thing found, its place, what it \
     has, its qualities and its relations");
because!(BEFORE_KEY, RecordClasses, "the argument of the get that writes the place the thing found was in before the place its word names");
because!(AFTER_KEY, RecordClasses, "the argument of the get that writes the place the thing found went to after the place its word names");
because!(AMOUNT_KIND, RecordClasses, "the kind of the get that writes the count of the value a relation of the thing found holds, how much \
     is the book");
because!(PAST_KIND, RecordClasses, "the kind of the get that writes where the thing found was, or who had it, before it moved, read from \
     the trace it left");

const NUMBER_FAMILY: &str = "number";
because!(NUMBER_FAMILY, RecordClasses, "the family of the moves that work on the number of the input: set it from a word, work another \
     word's number into it, or say it into the output");

pub const PLANNED_NUMBER_MOVES: [(crate::cursor::CursorMove, WordMove); 18] = [(crate::cursor::CursorMove::SetNumber, WordMove::NumberSet), (crate::cursor::CursorMove::AddNumber, WordMove::NumberAdd), (crate::cursor::CursorMove::SubtractNumber, WordMove::NumberSubtract), (crate::cursor::CursorMove::MultiplyNumber, WordMove::NumberMultiply), (crate::cursor::CursorMove::DivideNumber, WordMove::NumberDivide), (crate::cursor::CursorMove::LargerNumber, WordMove::NumberLarger), (crate::cursor::CursorMove::SmallerNumber, WordMove::NumberSmaller), (crate::cursor::CursorMove::RemainderNumber, WordMove::NumberRemainder), (crate::cursor::CursorMove::PowerNumber, WordMove::NumberPower), (crate::cursor::CursorMove::TakePercent, WordMove::NumberPercent), (crate::cursor::CursorMove::AddPercent, WordMove::NumberAddPercent), (crate::cursor::CursorMove::TakeAwayPercent, WordMove::NumberLessPercent), (crate::cursor::CursorMove::RoundNumber, WordMove::NumberRound), (crate::cursor::CursorMove::NegateNumber, WordMove::NumberNegate), (crate::cursor::CursorMove::DivideByCount, WordMove::NumberMean), (crate::cursor::CursorMove::WholeNumber, WordMove::NumberWhole), (crate::cursor::CursorMove::RootNumber, WordMove::NumberRoot), (crate::cursor::CursorMove::RoundDefault, WordMove::NumberRoundDefault)];
because!(PLANNED_NUMBER_MOVES, RecordClasses, "each operation a number plan uses with the word move that does it, so the plan the teacher \
     finds for an answer is taught as word moves");

const COMPUTE_FAMILY: &str = "compute";

const NAME_FAMILY: &str = "name";

const RESULT_KEY: &str = "result";
because!(NAME_FAMILY, RecordClasses, "the family of the move that names what was just worked out, one plus one equals sum");
because!(RESULT_KEY, RecordClasses, "the argument of the naming move that holds the word the result is named by");
because!(RELATION_KEY, RecordClasses, "the argument of the get that writes the value under a relation of the thing found, the capital of \
     france");
because!(COMPUTE_FAMILY, RecordClasses, "the family of the move that works out the numbers of the question by the operation its word \
     names, what is two plus three");
because!(NAME_KIND, RecordClasses, "the kind of the get that writes the name of the thing found, what is big");
because!(NEXT_KIND, RecordClasses, "the kind of the find that looks at the next older thing of the same name, as a person looks in the \
     next box when the first holds nothing");
because!(WITH_KEY, RecordClasses, "the argument of the find that looks for the thing holding a value, what is big");
because!(COUNT_KEY, RecordClasses, "the argument of the get that counts the things of a name inside the thing found, how many cats are in \
     the box");
because!(USER_KIND, RecordClasses, "the kind of the step to a person of the talk, the user i stands for or the assistant you stands for, \
     pointing at the word said");
because!(OWNER_KIND, RecordClasses, "the kind of the get that writes who owns the thing found, the one it was given to, wherever it stands \
     now");

const CHILD_KEY: &str = "withChild";
because!(NAME_KEY, RecordClasses, "the argument of a find that holds the name looked for");
because!(ORDER_KEY, RecordClasses, "the argument of a find that says which of several things of one name to take");
because!(LAST_MENTIONED, RecordClasses, "the order a find takes things of one name in: the one the story touched last");
because!(LOCATION_KIND, RecordClasses, "the kind of the get that writes the parent of the thing found, where it stands or who holds it");
because!(CHILDREN_KIND, RecordClasses, "the kind of the get that writes the things standing inside the thing found");
because!(CHILD_KEY, RecordClasses, "the argument of the property get that names the kind asked, color or size");
because!(GRAB_FAMILY, RecordClasses, "the family of the move that picks up the thing the cursor stands on");
because!(DROP_FAMILY, RecordClasses, "the family of the move that puts the held thing down inside the thing that appeared, or the thing \
     that appeared inside the held receiver");
because!(GIVE_FAMILY, RecordClasses, "the family of the move that holds the thing the cursor stands on as the receiver of what appears \
     next");
because!(FLAG_FAMILY, RecordClasses, "the family of the moves that set a flag the thing appearing next takes");
because!(CHILDREN_FAMILY, RecordClasses, "the family of the moves that add a child under the cursor");
because!(SET_FAMILY, RecordClasses, "the family of the moves that set a quality as the property of its kind under is");
because!(STEP_FAMILY, RecordClasses, "the family of the moves that step to the parent, the newest thing or the newest top thing");
because!(POINT_FAMILY, RecordClasses, "the family of the move a word that does nothing takes, pointing at nothing");
because!(CONTINUE_FAMILY, RecordClasses, "the family of the move that ends the steps of a word");

const PARENT_KIND: &str = "parent";

const NEWEST_KIND: &str = "newest";

const TOP_KIND: &str = "top";

const TO_KIND: &str = "to";
because!(PARENT_KIND, RecordClasses, "the kind of the step move to the parent of the node the cursor stands on, the thing a relation \
     belongs to");
because!(NEWEST_KIND, RecordClasses, "the kind of the step move to the newest thing the story told, what it stands for");
because!(TOP_KIND, RecordClasses, "the kind of the step move to the newest thing under the world, what he or she stands for");
because!(TO_KIND, RecordClasses, "the kind of the point move, which points to nothing");

pub const NUMBER_KINDS: [(WordMove, &str); 19] = [(WordMove::NumberSet, "set"), (WordMove::NumberAdd, "add"), (WordMove::NumberSubtract, "subtract"), (WordMove::NumberMultiply, "multiply"), (WordMove::NumberDivide, "divide"), (WordMove::NumberLarger, "larger"), (WordMove::NumberSmaller, "smaller"), (WordMove::NumberRemainder, "remainder"), (WordMove::NumberPower, "power"), (WordMove::NumberPercent, "percent"), (WordMove::NumberAddPercent, "addPercent"), (WordMove::NumberLessPercent, "lessPercent"), (WordMove::NumberRound, "round"), (WordMove::NumberNegate, "negate"), (WordMove::NumberMean, "mean"), (WordMove::NumberWhole, "whole"), (WordMove::NumberRoot, "root"), (WordMove::NumberRoundDefault, "roundDefault"), (WordMove::NumberSay, "say")];
because!(NUMBER_KINDS, RecordClasses, "each move on the number with the operation its class names, as the user writes a record, number add");

const EVERY_KIND: &str = "every";
because!(EVERY_KIND, RecordClasses, "the kind of a get that writes every answer a thing has of what is asked, where it has more \
     than one");

pub const MATERIAL_KIND: &str = "material";
because!(MATERIAL_KIND, RecordClasses, "the kind of the quality a thing is made of, one of the kinds a quality is classed by, named \
     on its own since a word said after made of sets that kind whatever else the seeds class the word by");
pub const KINDS: [&str; 8] = ["color", "size", "feeling", "temperature", "speed", "age", "material", "trait"];
because!(KINDS, RecordClasses, "the kinds the seeds class qualities by, red is a color, big is a size and shy is a trait, each with the move that sets a \
     quality of that kind, so the class says the kind as the user writes it, setProperty color: yellow");

fn argument(key: &str, value: &str) -> String {
    format!("{key}{KEY_MARK}{value}")
}
because!(argument, RecordClasses, "one argument of a move's class: its key and its value as the user writes a record");

fn class_parts(family: &str, rest: &[String]) -> String {
    let inner = std::iter::once(family.to_string()).chain(rest.iter().cloned()).collect::<Vec<_>>().join(PART_MARK);
    format!("{BRACE_OPEN}{inner}{BRACE_CLOSE}")
}
because!(class_parts, RecordClasses, "a move's class from its family and the parts after it, braced as one item");

impl WordMove {
    pub fn family(self) -> &'static str {
        match self {
            WordMove::Grab => GRAB_FAMILY,
            WordMove::Drop => DROP_FAMILY,
            WordMove::Give => GIVE_FAMILY,
            WordMove::Contain => CONTAIN_FAMILY,
            WordMove::Hand => HAND_FAMILY,
            WordMove::Take => TAKE_FAMILY,
            WordMove::Join => JOIN_FAMILY,
            WordMove::Together => GROUP_FAMILY,
            WordMove::Relate => ROLE_FAMILY,
            WordMove::Belong => BELONG_FAMILY,
            WordMove::Accompany => WITH_FAMILY,
            WordMove::Leave => LEAVE_FAMILY,

            WordMove::ChangeState => STATE_FAMILY,
            WordMove::Release => RELEASE_FAMILY,
            WordMove::FlagThe | WordMove::FlagA | WordMove::FlagQuantity | WordMove::FlagProperty | WordMove::FlagTime | WordMove::FlagLater | WordMove::FlagFrom | WordMove::FlagWhen | WordMove::FlagWhose => FLAG_FAMILY,
            WordMove::AddProperty | WordMove::AddPropertyPast | WordMove::AddRelation | WordMove::AddRelationPast | WordMove::AddDeed | WordMove::AddValue | WordMove::AddRole => CHILDREN_FAMILY,
            WordMove::SetProperty => SET_FAMILY,
            WordMove::StepParent | WordMove::StepNewest | WordMove::StepTop => STEP_FAMILY,
            WordMove::PointNothing => POINT_FAMILY,
            WordMove::AddQuestion => CHILDREN_FAMILY,
            WordMove::FindAsked | WordMove::FindAskedStood => FIND_FAMILY,
            WordMove::GetDistanceRoute | WordMove::GetLocation | WordMove::GetLocationClaimed | WordMove::GetLocationMotive |  WordMove::GetChildren | WordMove::GetOwner | WordMove::GetOwnerEvery | WordMove::GetName | WordMove::GetCount | WordMove::GetTotal | WordMove::GetDifference | WordMove::GetMost | WordMove::GetLeast => GET_FAMILY,
            WordMove::FindNext | WordMove::FindWith => FIND_FAMILY,
            WordMove::GetAllWith => GET_FAMILY,
            WordMove::StepUser => STEP_FAMILY,
            WordMove::GetKind | WordMove::GetKindWhy | WordMove::GetKindMeasure => KIND_GET_FAMILY,
            WordMove::Measure => MEASURE_FAMILY,
            WordMove::Possess => POSSESS_FAMILY,
            WordMove::GrabAll => GRAB_ALL_FAMILY,
            WordMove::SetClock => CLOCK_FAMILY,
            WordMove::Activity | WordMove::ActivityNamed | WordMove::ActivityDone | WordMove::ActivityUnder => ACTIVITY_FAMILY,
            WordMove::Regard => REGARD_FAMILY,
            WordMove::GetRelation | WordMove::GetRelationDoing | WordMove::GetRelationThrough | WordMove::GetRelationRanked | WordMove::GetRelationBackward | WordMove::GetRelationSaid | WordMove::GetRelationToward | WordMove::GetSubject | WordMove::GetPast | WordMove::GetShifted | WordMove::GetReply | WordMove::GetChoice | WordMove::GetDistance | WordMove::GetAmount | WordMove::GetAbout | WordMove::GetBefore | WordMove::GetAfter | WordMove::GetGiven => GET_FAMILY,
            WordMove::Compute => COMPUTE_FAMILY,
            WordMove::NumberSet | WordMove::NumberAdd | WordMove::NumberSubtract | WordMove::NumberMultiply | WordMove::NumberDivide | WordMove::NumberLarger | WordMove::NumberSmaller | WordMove::NumberRemainder | WordMove::NumberPower | WordMove::NumberPercent | WordMove::NumberAddPercent | WordMove::NumberLessPercent | WordMove::NumberRound | WordMove::NumberNegate | WordMove::NumberMean | WordMove::NumberWhole | WordMove::NumberRoot | WordMove::NumberRoundDefault | WordMove::NumberSay => NUMBER_FAMILY,
            WordMove::NameResult => NAME_FAMILY,
            WordMove::Check => CHECK_FAMILY,
            WordMove::CheckRelation | WordMove::CheckRight => CHECK_FAMILY,
            WordMove::Continue => CONTINUE_FAMILY,
        }
    }

    pub fn points(self) -> bool {
        matches!(self, WordMove::GetDistance | WordMove::GetDistanceRoute | WordMove::FindAskedStood | WordMove::Measure | WordMove::SetClock | WordMove::NumberSet | WordMove::NumberAdd | WordMove::NumberSubtract | WordMove::NumberMultiply | WordMove::NumberDivide | WordMove::NumberLarger | WordMove::NumberSmaller | WordMove::NumberRemainder | WordMove::NumberPower | WordMove::NumberPercent | WordMove::NumberAddPercent | WordMove::NumberLessPercent | WordMove::NumberRound | WordMove::Drop | WordMove::FlagQuantity | WordMove::FlagProperty | WordMove::FlagWhen | WordMove::FlagWhose | WordMove::AddProperty | WordMove::AddPropertyPast | WordMove::AddRelation | WordMove::AddRelationPast | WordMove::AddDeed | WordMove::AddValue | WordMove::AddQuestion | WordMove::FindAsked | WordMove::GetKind | WordMove::GetKindWhy | WordMove::GetKindMeasure | WordMove::Check | WordMove::CheckRelation | WordMove::CheckRight | WordMove::FindWith | WordMove::GetAllWith | WordMove::GetCount | WordMove::GetRelation | WordMove::GetRelationDoing | WordMove::GetRelationThrough | WordMove::GetRelationRanked | WordMove::GetRelationBackward | WordMove::GetRelationSaid | WordMove::GetRelationToward | WordMove::GetSubject | WordMove::Compute | WordMove::NameResult | WordMove::GetBefore | WordMove::GetAfter | WordMove::GetGiven | WordMove::GetTotal | WordMove::GetDifference | WordMove::GetMost | WordMove::GetLeast | WordMove::ChangeState | WordMove::StepUser | WordMove::SetProperty)
    }



    pub fn class(self) -> String {
        let flag = |name: &str, value: &str| class_parts(self.family(), &[argument(TYPE_KEY, name), argument(VALUE_KEY, value)]);
        let typed = |kind: &str| class_parts(self.family(), &[argument(ADD_KIND, RECORD_MARK), argument(TYPE_KEY, kind)]);
        match self {
            WordMove::Grab | WordMove::Give | WordMove::Contain | WordMove::Hand | WordMove::Take | WordMove::Release | WordMove::Join | WordMove::Relate | WordMove::GrabAll | WordMove::Activity | WordMove::Regard | WordMove::Possess | WordMove::Belong | WordMove::Accompany | WordMove::Leave | WordMove::Continue => class_parts(self.family(), &[]),

            WordMove::ChangeState => class_parts(self.family(), &[RECORD_MARK.to_string()]),
            WordMove::AddRole => class_parts(self.family(), &[ROLE_KIND.to_string()]),
            WordMove::Drop => class_parts(self.family(), &[RECORD_MARK.to_string()]),
            WordMove::FlagThe => flag(DEFINITE_FLAG, TRUE_VALUE),
            WordMove::FlagA => flag(INDEFINITE_FLAG, TRUE_VALUE),
            WordMove::FlagQuantity => flag(QUANTITY_FLAG, RECORD_MARK),
            WordMove::FlagProperty => flag(PROPERTY_FLAG, RECORD_MARK),
            WordMove::FlagTime => flag(TIME_KEY, PAST_VALUE),
            WordMove::FlagLater => flag(TIME_KEY, LATER_VALUE),
            WordMove::FlagFrom => flag(FROM_FLAG, TRUE_VALUE),
            WordMove::FlagWhen => flag(WHEN_FLAG, RECORD_MARK),
            WordMove::FlagWhose => flag(WHOSE_FLAG, RECORD_MARK),
            WordMove::AddProperty => typed(PROPERTY_TYPE),
            WordMove::AddPropertyPast => class_parts(self.family(), &[argument(ADD_KIND, RECORD_MARK), argument(TYPE_KEY, PROPERTY_TYPE), argument(TIME_KEY, PAST_VALUE)]),
            WordMove::AddRelation => typed(RELATION_TYPE),
            WordMove::AddRelationPast => class_parts(self.family(), &[argument(ADD_KIND, RECORD_MARK), argument(TYPE_KEY, RELATION_TYPE), argument(TIME_KEY, PAST_VALUE)]),
            WordMove::AddDeed => typed(DEED_TYPE),
            WordMove::AddValue => class_parts(self.family(), &[argument(ADD_KIND, RECORD_MARK)]),
            WordMove::ActivityNamed => class_parts(self.family(), &[NAMED_KIND.to_string()]),
            WordMove::ActivityDone => class_parts(self.family(), &[DONE_KIND.to_string()]),
            WordMove::ActivityUnder => class_parts(self.family(), &[UNDER_KIND.to_string()]),
            WordMove::StepParent => class_parts(self.family(), &[PARENT_KIND.to_string()]),
            WordMove::StepNewest => class_parts(self.family(), &[NEWEST_KIND.to_string()]),
            WordMove::StepTop => class_parts(self.family(), &[TOP_KIND.to_string()]),
            WordMove::PointNothing => class_parts(self.family(), &[argument(TO_KIND, crate::cursor::CURSOR_NOTHING)]),
            WordMove::AddQuestion => typed(QUESTION_TYPE),
            WordMove::FindAsked => class_parts(self.family(), &[argument(NAME_KEY, RECORD_MARK), argument(ORDER_KEY, LAST_MENTIONED)]),
            WordMove::FindAskedStood => class_parts(self.family(), &[STOOD_KIND.to_string(), argument(NAME_KEY, RECORD_MARK), argument(ORDER_KEY, LAST_MENTIONED)]),
            WordMove::GetLocation => class_parts(self.family(), &[LOCATION_KIND.to_string()]),
            WordMove::GetDistanceRoute => class_parts(self.family(), &[ROUTE_KIND.to_string(), argument(CHILD_KEY, RECORD_MARK)]),
            WordMove::GetChildren => class_parts(self.family(), &[CHILDREN_KIND.to_string()]),
            WordMove::GetLocationClaimed => class_parts(self.family(), &[LOCATION_KIND.to_string(), CLAIMED_KIND.to_string()]),
            WordMove::GetLocationMotive => class_parts(self.family(), &[LOCATION_KIND.to_string(), MOTIVE_KIND.to_string()]),
            WordMove::GetOwner => class_parts(self.family(), &[OWNER_KIND.to_string()]),
            WordMove::GetOwnerEvery => class_parts(self.family(), &[OWNER_KIND.to_string(), EVERY_KIND.to_string()]),
            WordMove::GetName => class_parts(self.family(), &[NAME_KIND.to_string()]),
            WordMove::GetCount => class_parts(self.family(), &[argument(COUNT_KEY, RECORD_MARK)]),
            WordMove::GetTotal => class_parts(self.family(), &[argument(TOTAL_KEY, RECORD_MARK)]),
            WordMove::GetDifference => class_parts(self.family(), &[argument(DIFFERENCE_KEY, RECORD_MARK)]),
            WordMove::GetMost => class_parts(self.family(), &[argument(MOST_KEY, RECORD_MARK)]),
            WordMove::GetLeast => class_parts(self.family(), &[argument(LEAST_KEY, RECORD_MARK)]),
            WordMove::FindNext => class_parts(self.family(), &[NEXT_KIND.to_string()]),
            WordMove::FindWith => class_parts(self.family(), &[argument(WITH_KEY, RECORD_MARK)]),
            WordMove::GetAllWith => class_parts(self.family(), &[ALL_KIND.to_string(), argument(WITH_KEY, RECORD_MARK)]),
            WordMove::StepUser => class_parts(self.family(), &[argument(USER_KIND, RECORD_MARK)]),
            WordMove::GetKind => class_parts(self.family(), &[argument(CHILD_KEY, RECORD_MARK)]),
            WordMove::GetKindWhy => class_parts(self.family(), &[WHY_KIND.to_string(), argument(CHILD_KEY, RECORD_MARK)]),
            WordMove::GetKindMeasure => class_parts(self.family(), &[MEASURE_KIND.to_string(), argument(CHILD_KEY, RECORD_MARK)]),
            WordMove::GetRelation => class_parts(self.family(), &[argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetRelationDoing => class_parts(self.family(), &[DOING_KIND.to_string(), argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetRelationThrough => class_parts(self.family(), &[THROUGH_KIND.to_string(), argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetRelationRanked => class_parts(self.family(), &[RANKED_KIND.to_string(), argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetRelationBackward => class_parts(self.family(), &[BACKWARD_KIND.to_string(), argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetRelationSaid => class_parts(self.family(), &[SAID_KIND.to_string(), argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetRelationToward => class_parts(self.family(), &[TOWARD_KIND.to_string(), argument(RELATION_KEY, RECORD_MARK)]),
            WordMove::GetSubject => class_parts(self.family(), &[argument(SUBJECT_KEY, RECORD_MARK)]),
            WordMove::GetPast => class_parts(self.family(), &[PAST_KIND.to_string()]),
            WordMove::GetShifted => class_parts(self.family(), &[SHIFTED_KIND.to_string()]),
            WordMove::GetReply => class_parts(self.family(), &[REPLY_KIND.to_string()]),
            WordMove::GetChoice => class_parts(self.family(), &[CHOICE_KIND.to_string()]),
            WordMove::GetDistance => class_parts(self.family(), &[argument(DISTANCE_KEY, RECORD_MARK)]),
            WordMove::Measure | WordMove::SetClock => class_parts(self.family(), &[RECORD_MARK.to_string()]),
            WordMove::GetAmount => class_parts(self.family(), &[AMOUNT_KIND.to_string()]),
            WordMove::GetAbout => class_parts(self.family(), &[ABOUT_KIND.to_string()]),
            WordMove::GetBefore => class_parts(self.family(), &[argument(BEFORE_KEY, RECORD_MARK)]),
            WordMove::GetAfter => class_parts(self.family(), &[argument(AFTER_KEY, RECORD_MARK)]),
            WordMove::GetGiven => class_parts(self.family(), &[argument(GIVEN_KEY, RECORD_MARK)]),
            WordMove::Compute => class_parts(self.family(), &[RECORD_MARK.to_string()]),
            WordMove::NumberSet | WordMove::NumberAdd | WordMove::NumberSubtract | WordMove::NumberMultiply | WordMove::NumberDivide | WordMove::NumberLarger | WordMove::NumberSmaller | WordMove::NumberRemainder | WordMove::NumberPower | WordMove::NumberPercent | WordMove::NumberAddPercent | WordMove::NumberLessPercent | WordMove::NumberRound | WordMove::NumberNegate | WordMove::NumberMean | WordMove::NumberWhole | WordMove::NumberRoot | WordMove::NumberRoundDefault | WordMove::NumberSay => {
                let kind = NUMBER_KINDS.iter().find(|(m, _)| *m == self).map(|(_, kind)| *kind).unwrap_or_default();
                if self.points() { class_parts(self.family(), &[argument(kind, RECORD_MARK)]) } else { class_parts(self.family(), &[kind.to_string()]) }
            }
            WordMove::NameResult => class_parts(self.family(), &[argument(RESULT_KEY, RECORD_MARK)]),
            WordMove::Check => class_parts(self.family(), &[RECORD_MARK.to_string()]),
            WordMove::CheckRelation => class_parts(self.family(), &[RELATION_CHECK.to_string(), RECORD_MARK.to_string()]),
            WordMove::CheckRight => class_parts(self.family(), &[RIGHT_CHECK.to_string(), RECORD_MARK.to_string()]),
            WordMove::SetProperty => class_parts(self.family(), &[argument(PROPERTY_FLAG, RECORD_MARK)]),
            _ => self.family().to_string(),
        }
    }

    pub fn of_class(class: &str) -> Option<WordMove> {
        ALL.iter().copied().find(|m| m.class() == class)
    }
}

pub const ALL: [WordMove; 110] = [
    WordMove::Grab,
    WordMove::Drop,
    WordMove::SetProperty,
    WordMove::Give,
    WordMove::Contain,
    WordMove::Hand,
    WordMove::Take,
    WordMove::Release,
    WordMove::Join,
    WordMove::Together,
    WordMove::Relate,
    WordMove::AddRole,
    WordMove::ChangeState,
    WordMove::Belong,
    WordMove::Accompany,
    WordMove::Leave,
    WordMove::FlagThe,
    WordMove::FlagA,
    WordMove::FlagQuantity,
    WordMove::FlagProperty,
    WordMove::FlagTime,
    WordMove::FlagLater,
    WordMove::FlagFrom,
    WordMove::FlagWhen,
    WordMove::FlagWhose,
    WordMove::AddProperty,
    WordMove::AddPropertyPast,
    WordMove::AddRelation,
    WordMove::AddRelationPast,
    WordMove::AddDeed,
    WordMove::AddValue,
    WordMove::StepParent,
    WordMove::StepNewest,
    WordMove::StepTop,
    WordMove::PointNothing,
    WordMove::AddQuestion,
    WordMove::FindAsked,
    WordMove::FindAskedStood,
    WordMove::GetLocation,
    WordMove::GetChildren,
    WordMove::GetKindWhy,
    WordMove::GetKindMeasure,
    WordMove::GetLocationClaimed,
    WordMove::GetLocationMotive,
    WordMove::GetOwner,
    WordMove::GetOwnerEvery,
    WordMove::GetName,
    WordMove::GetCount,
    WordMove::NumberSet,
    WordMove::NumberAdd,
    WordMove::NumberSubtract,
    WordMove::NumberMultiply,
    WordMove::NumberDivide,
    WordMove::NumberLarger,
    WordMove::NumberSmaller,
    WordMove::NumberRemainder,
    WordMove::NumberPower,
    WordMove::NumberPercent,
    WordMove::NumberAddPercent,
    WordMove::NumberLessPercent,
    WordMove::NumberRound,
    WordMove::NumberNegate,
    WordMove::NumberMean,
    WordMove::NumberWhole,
    WordMove::NumberRoot,
    WordMove::NumberRoundDefault,
    WordMove::NumberSay,
    WordMove::GetTotal,
    WordMove::GetDifference,
    WordMove::GetMost,
    WordMove::GetLeast,
    WordMove::FindNext,
    WordMove::FindWith,
    WordMove::GetAllWith,
    WordMove::StepUser,
    WordMove::GetKind,
    WordMove::GetRelation,
    WordMove::GetRelationDoing,
    WordMove::GetRelationThrough,
    WordMove::GetRelationRanked,
    WordMove::GetRelationBackward,
    WordMove::GetRelationSaid,
    WordMove::GetRelationToward,
    WordMove::GetSubject,
    WordMove::GetPast,
    WordMove::GetShifted,
    WordMove::GetReply,
    WordMove::GetChoice,
    WordMove::GetDistance,
    WordMove::GetDistanceRoute,
    WordMove::Measure,
    WordMove::Possess,
    WordMove::GrabAll,
    WordMove::SetClock,
    WordMove::Activity,
    WordMove::ActivityNamed,
    WordMove::ActivityDone,
    WordMove::ActivityUnder,
    WordMove::Regard,
    WordMove::GetAbout,
    WordMove::GetBefore,
    WordMove::GetAfter,
    WordMove::GetGiven,
    WordMove::GetAmount,
    WordMove::Compute,
    WordMove::NameResult,
    WordMove::Check,
    WordMove::CheckRelation,
    WordMove::CheckRight,
    WordMove::Continue,
];
because!(ALL, RecordClasses, "every move, so a class read from a network's file is matched against each move's own class");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WordStep {
    pub act: WordMove,
    pub at: Option<usize>,
}
because!(WordStep, RecordClasses, "one step: a move and, for a move that points, the depth of the stack slot it points at, the newest slot \
     at nothing; none when the pointer has not chosen yet");

impl WordStep {
    pub fn class(&self) -> String {
        self.act.class()
    }

    pub fn of_class(class: &str) -> Option<WordStep> {
        WordMove::of_class(class).map(|act| WordStep { act, at: None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_read_back() {
        for act in ALL {
            assert_eq!(WordMove::of_class(&act.class()), Some(act), "{}", act.class());
        }
        assert_eq!(WordMove::Grab.class(), "{grab}");
        assert_eq!(WordMove::Drop.class(), "{drop @}");
        assert_eq!(WordMove::FlagThe.class(), "{setFlag type: the value: true}");
        assert_eq!(WordMove::FlagQuantity.class(), "{setFlag type: quantity value: @}");
        assert_eq!(WordMove::FlagTime.class(), "{setFlag type: time value: past}");
        assert_eq!(WordMove::AddProperty.class(), "{children add: @ type: property}");
        assert_eq!(WordMove::AddValue.class(), "{children add: @}");
        assert_eq!(WordMove::SetColor.class(), "{setProperty color: @}");
        assert_eq!(WordMove::PointNothing.class(), "{point to: {nothing}}");
        assert_eq!(WordMove::StepNewest.class(), "{step newest}");
        assert_eq!(WordMove::Continue.class(), "{continue}");
        assert_eq!(WordMove::FindAsked.class(), "{find name: @ orderBy: lastMentioned}");
        assert_eq!(WordMove::GetLocation.class(), "{get location}");
        assert_eq!(WordMove::GetKind.class(), "{getPropertyValue withChild: @}");
    }
}
