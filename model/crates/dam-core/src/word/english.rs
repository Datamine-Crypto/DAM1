use patterns::{because, source};

pub struct WordEnglish;
source!(
    WordEnglish,
    "the closed classes of English words the reading one word at a time knows by heart, since a network learns the pattern of a class and \
     never one word's context: the forms of to be, the forms of having, the articles and link words that do nothing, the words that open a \
     question, the helpers a question says, the pronouns, and the verbs whose deed moves a thing; every open class, the things, the values \
     and the deeds, comes from the tree and the seeds"
);

pub const COPULA: [&str; 10] = ["is", "are", "was", "were", "am", "be", "been", "became", "become", "becomes"];
because!(COPULA, WordEnglish, "the forms of to be a sentence says between a thing and its place or its quality; the past ones date the \
     fact");

pub const TIMED_VERBS: [&str; 6] = ["start", "end", "begin", "finish", "open", "close"];
because!(TIMED_VERBS, WordEnglish, "the verbs whose at names a time and no place, the film starts at three, so at after them makes no \
     activity of them");

pub const YEAR_LEAST: f32 = 1000.0;
because!(YEAR_LEAST, WordEnglish, "the least number that, said bare after is, names the year it is, it is twenty twenty");

pub const YEAR_NAME: &str = "year";
because!(YEAR_NAME, WordEnglish, "the thing a bare year after is is written under, year is twenty twenty");

pub const ACTIVITY: &str = "activity";
because!(ACTIVITY, WordEnglish, "the relation that holds what a thing does, loves or likes to do, under which where it is done stands, tom \
     loves to swim in the river");

pub const INFINITIVE: &str = "to";
because!(INFINITIVE, WordEnglish, "the word before a verb that names what is loved or liked, after which that verb is an activity of the \
     subject");

pub const TOWARD: &str = "of";
because!(TOWARD, WordEnglish, "the word after a feeling that names what the feeling is about, afraid of wolves, which makes the feeling a \
     relation of the subject");

pub const CLAUSE_BREAKS: [&str; 4] = ["but", "so", "although", "while"];
because!(CLAUSE_BREAKS, WordEnglish, "the words that part two clauses of one sentence, after which the next clause starts at the world as \
     after a full stop, tom had a car but ann had a bike");

pub const ANOTHER: [&str; 2] = ["other", "another"];
because!(ANOTHER, WordEnglish, "the words that say the thing named next is a new one of its name though the comes before them, the other \
     cat");

pub const LINES: [&str; 3] = ["line", "row", "queue"];
because!(LINES, WordEnglish, "the places a group stands in one after another, so each member is first, second or third by the order it was \
     named in, and the one named last is last");

pub const LIST_MARK: &str = ",";
because!(LIST_MARK, WordEnglish, "the mark between the things of a list, the key, the pen and the cup, which joins the thing before it to \
     the group as and does");

pub const BEEN: &str = "been";
because!(BEEN, WordEnglish, "the copula that follows has, where has is no having, the cat has been in the garden");

pub const PRONOUN_GENDERS: [(&str, &str); 5] = [("he", "male"), ("his", "male"), ("him", "male"), ("she", "female"), ("her", "female")];
because!(PRONOUN_GENDERS, WordEnglish, "the gender a pronoun stands for, so she finds the newest woman the story told and not the newest \
     person");

pub const DONE_ASKED: &str = "do";
because!(DONE_ASKED, WordEnglish, "the verb that asks what a person did, which reads their activity, what did kim do");

pub const OBJECT_PRONOUNS: [(&str, bool); 3] = [("him", true), ("her", true), ("them", false)];
because!(OBJECT_PRONOUNS, WordEnglish, "the pronouns that stand after a verb or with, each with whether it stands for a person, ann is \
     with him");

pub const HAVING: [&str; 3] = ["has", "have", "had"];
because!(HAVING, WordEnglish, "the forms of having: what a thing has stands inside it, so having is a give, the thing held as the receiver \
     of what appears next");

pub const SKIPPED: [&str; 8] = ["the", "a", "an", "to", "there", "and", "s", "of"];
because!(SKIPPED, WordEnglish, "the articles and the link words a step never points at: an article sets a flag, the rest point at nothing, \
     the s of a possessive among them");

pub const MARK_SIGNS: [&str; 7] = [".", "?", "!", ",", ";", ":", "="];
because!(MARK_SIGNS, WordEnglish, "the signs that end or part a sentence, the equals sign among them since six plus one equals asks its \
     answer there; the signs of arithmetic and the apostrophe of a possessive are words");

pub const ASKED_END: &str = "?";
because!(ASKED_END, WordEnglish, "the mark a sentence that asks is closed with when it was written without one");

pub const TOLD_END: &str = ".";
because!(TOLD_END, WordEnglish, "the mark a sentence that tells is closed with when it was written without one");

pub const PARTING_MARKS: [&str; 3] = [",", ";", ":"];
because!(PARTING_MARKS, WordEnglish, "the marks that part a sentence without ending it, so a question holding a list stays open over them, \
     the mode of nine, two and six");

pub const OPERATORS: [(&str, &str); 10] = [("+", "+"), ("plus", "+"), ("-", "-"), ("minus", "-"), ("*", "*"), ("times", "*"), ("x", "*"), ("/", "/"), ("divided", "/"), ("between", MIDDLE_SIGN)];
because!(OPERATORS, WordEnglish, "the words and signs of arithmetic, each with the operation it names, so plus and the plus sign are one \
     operation");

pub const MORE_SIGN: &str = ">";
because!(MORE_SIGN, WordEnglish, "the operation more or greater names over the numbers of a question: whether the first is the greater \
     when a copula asks, is twelve greater than twenty, and their sum otherwise, what is ten more than a number");

pub const LESS_SIGN: &str = "<";
because!(LESS_SIGN, WordEnglish, "the operation less or fewer names: whether the first is the smaller when a copula asks, and the second \
     less the first otherwise, what is one less than fifty");

pub const MOST_SIGN: &str = "max";
because!(MOST_SIGN, WordEnglish, "the operation biggest names, the greatest of the numbers of the question");

pub const LEAST_SIGN: &str = "min";
because!(LEAST_SIGN, WordEnglish, "the operation smallest names, the least of the numbers of the question");

pub const ODD_SIGN: &str = "odd";
because!(ODD_SIGN, WordEnglish, "the word that asks whether a number leaves one over when halved, or counts such numbers in a range");

pub const EVEN_SIGN: &str = "even";
because!(EVEN_SIGN, WordEnglish, "the word that asks whether a number halves with none over, or counts such numbers in a range");

pub const RANGE_WORD: &str = "numbers";
because!(RANGE_WORD, WordEnglish, "the word that asks how many numbers stand from one number of the question to the other");

pub const MAKING: &str = "make";
because!(MAKING, WordEnglish, "the word that asks how many of a unit make a number, how many tens make a hundred, where in asks the digit \
     of that unit");

pub const ALPHABET_WORD: &str = "alphabet";
because!(ALPHABET_WORD, WordEnglish, "the word that asks which of the words after it comes first, or last, by its letters");

pub const LAST_PLACE: &str = "last";
because!(LAST_PLACE, WordEnglish, "the word that turns an order question to the far end");

pub const PLACE_UNITS: [(&str, f32); 4] = [("ones", 1.0), ("tens", 10.0), ("hundreds", 100.0), ("thousands", 1000.0)];
because!(PLACE_UNITS, WordEnglish, "the units of place value, each with how many ones it is worth");

pub const DECIMAL_BASE: f32 = 10.0;
because!(DECIMAL_BASE, WordEnglish, "how many of one place make one of the next, for the digit a place holds");

pub const HALVING: f32 = 2.0;
because!(HALVING, WordEnglish, "what a number is halved by to tell odd from even");

pub const ROMAN_LETTERS: &str = "ivxlcdm";
because!(ROMAN_LETTERS, WordEnglish, "the letters a roman numeral is written with, so a number in words is never spelled by one");

pub const FRONT: &str = "front";
because!(FRONT, WordEnglish, "the word after in that makes it no place a thing stands inside, in front of the garage");

pub const MADE: &str = "made";
because!(MADE, WordEnglish, "the word before of that names no thing, what is made of stone");

pub const AROUND_ASKED: &str = "perimeter";
because!(AROUND_ASKED, WordEnglish, "the word that asks how far it is around a shape, the perimeter of a rectangle");

pub const SIDE_NAMED: &str = "side";
because!(SIDE_NAMED, WordEnglish, "the part of a shape whose count says how many lengths go around it");

pub const PLURAL_ASKED: &str = "plural";
because!(PLURAL_ASKED, WordEnglish, "the word that asks the form of the last word of a question for more than one, the plural of box");

pub const SINGULAR_ASKED: &str = "singular";
because!(SINGULAR_ASKED, WordEnglish, "the word that asks the form of the last word of a question for one, the singular of boxes");

pub const IN_WORDS: &str = "words";
because!(IN_WORDS, WordEnglish, "the word that asks a number said in digits as the words that spell it, what is forty-two in words");

pub const NUMBER_ASKED: &str = "number";
because!(NUMBER_ASKED, WordEnglish, "the word that asks the number the number words of a question spell together, what number is twenty \
     one");

pub const SOFT_BEFORE: &str = "aeiou";
because!(SOFT_BEFORE, WordEnglish, "the letters before a final y after which the plural only adds its ending, days, while after any other \
     the y becomes ies, babies");

pub const RUN_ASKED: [&str; 6] = ["next", "missing", "first", "last", "largest", "smallest"];
because!(RUN_ASKED, WordEnglish, "the words that ask a number of the run of numbers the story told: the one that comes next, the one \
     missing, its ends and its extremes");

pub const DURING: &str = "during";
because!(DURING, WordEnglish, "the word before a stretch of time, which flags when the sentence holds, so the noun after it is that time \
     and no thing, during the day");

pub const WHEN_OPENER: &str = "when";
because!(WHEN_OPENER, WordEnglish, "the word that asks a time before a copula or a helper, when was tom born, and only opens a clause \
     before an article or a noun, when the cat is in the garden");

pub const GAINING: [&str; 2] = ["find", "buy"];
because!(GAINING, WordEnglish, "the deeds after which the one who did them has the thing, when it is a thing the story already placed or \
     lost, zoe finds the coin, while ann found a coin is only a deed");

pub const PARTING: [&str; 1] = ["sell"];
because!(PARTING, WordEnglish, "the deeds after which the one named after to has the thing, max sells the bike to sue");

pub const WHO_ASKED: &str = "who";
because!(WHO_ASKED, WordEnglish, "the question word that asks for people, so the past of a thing it asks of is every one that had it");

pub const WEIGHT_COMPARISONS: [(&str, bool); 4] = [("heavier", true), ("heaviest", true), ("lighter", false), ("lightest", false)];
because!(WEIGHT_COMPARISONS, WordEnglish, "the comparisons of weight and whether each asks the most, which are measured by what a thing \
     weighs, the box weighs ten kilos, and by no quality of their own stem");

pub const WEIGHING: &str = "weigh";
because!(WEIGHING, WordEnglish, "the relation a weight is told under, which heavier and lighter compare");

pub const SPAN_ENDS: [(&str, &str); 3] = [("start", "end"), ("begin", "finish"), ("open", "close")];
because!(SPAN_ENDS, WordEnglish, "the pairs of timed verbs that open and close a stretch of time, so how long a thing is reads the hour \
     under the second less the hour under the first");

pub const SPAN_ASKED: &str = "long";
because!(SPAN_ASKED, WordEnglish, "the measure that asks how long a thing told to start and end lasts");

pub const SHARING: &str = "share";
because!(SHARING, WordEnglish, "the deed whose value is the ones shared with, counted, so them, equally and with after it name nothing, he \
     shares them equally with three friends");

pub const EVENLY: &str = "equally";
because!(EVENLY, WordEnglish, "the word that says a sharing is even, which names no thing and no quality");

pub const GOAL: &str = "goal";
because!(GOAL, WordEnglish, "the relation the seeds give a state of a person to the place it sends them, the goal of thirsty is the \
     kitchen");

pub const WHY_ASKED: &str = "why";
because!(WHY_ASKED, WordEnglish, "the question word that asks the state that sent a person somewhere, why did sumit go to the kitchen");

pub const CHOICE_ASKED: &str = "which";
because!(CHOICE_ASKED, WordEnglish, "the question word that asks one of the things named, so a comparison under it answers the word and no \
     number");

pub const STATE_WORDS: [&str; 2] = ["on", "off"];
because!(STATE_WORDS, WordEnglish, "the place words that are also the state of a machine, said of a counted part of a group, three fans \
     are on");

pub const MEANS: &str = "means";
because!(MEANS, WordEnglish, "the word that defines a name by a function or by another defined name, kept as the relation it says, hj \
     means sqrt");

pub const GOING: &str = "go";
because!(GOING, WordEnglish, "the verb that asks a route between two places of a map of directions, how do you go from the garden to the \
     kitchen");

pub const CLAIMING: [&str; 3] = ["say", "think", "believe"];
because!(CLAIMING, WordEnglish, "the verbs after which a person's claim follows, held under the claimer apart from what is so, tom said \
     the ball is red");

pub const RIGHT_ASKED: &str = "right";
because!(RIGHT_ASKED, WordEnglish, "the word that asks whether what a person claimed is what the story tells, was tom right");

pub const NAME_ROLE: &str = "name";
because!(NAME_ROLE, WordEnglish, "the word after a possessive that is a relation of the owner to what they are called, as a word of kin \
     is, my name is kim");

pub const REPLY_WORDS: [&str; 2] = ["say", "answer"];
because!(REPLY_WORDS, WordEnglish, "the verbs that ask the assistant what it last answered, what did you say");

pub const SEQUENCE_WORDS: [&str; 3] = ["then", "after", "following"];
because!(SEQUENCE_WORDS, WordEnglish, "the words that open the next event of a story told without marks, which after a finished clause \
     send the cursor back to the world, daniel moved to the garden after that he went to the kitchen");

pub const THERE_OPENER: &str = "there";
because!(THERE_OPENER, WordEnglish, "the word that opens a clause of being somewhere, there is a key in the box, and so parts it from a \
     clause finished before it");

pub const CHANGE_ASKED: &str = "change";
because!(CHANGE_ASKED, WordEnglish, "the word that asks what a payer gets back, worth to a plan on the number the cost of the thing told \
     just before they paid, so the change is what they paid less that");

pub const PAYING: &str = "pay";
because!(PAYING, WordEnglish, "the relation a payment is told under, tom pays ten dollars");

pub const COSTING: &str = "cost";
because!(COSTING, WordEnglish, "the relation a price is told under, the book costs five dollars");

pub const POSSESSIVE_S: &str = "s";
because!(POSSESSIVE_S, WordEnglish, "the letter that ends a possessive when it follows the apostrophe, and is the letter of the alphabet \
     anywhere else, is s the successor of r");

pub const BORN: &str = "born";
because!(BORN, WordEnglish, "the word after was that is a relation of the person to a year and no quality of theirs, tom was born in a \
     year");

pub const HOUR_OPENER: &str = "at";
because!(HOUR_OPENER, WordEnglish, "the place word after which a number said in digits is an hour, a place in time the thing held is \
     dropped in, the meeting is at three");

pub const AGE_WORDS: [&str; 2] = ["old", "born"];
because!(AGE_WORDS, WordEnglish, "the words of a question that measure against the year it is, so each is worth that year to a plan on the \
     number, how old is tom and when was sam born");

pub const RUN_SUM: &str = "sum";
because!(RUN_SUM, WordEnglish, "the word that asks the numbers of the run the story told added up, the sum of the numbers");

pub const ROUNDING: &str = "round";
because!(ROUNDING, WordEnglish, "the word that, said first, is the thing whose place is how many decimals an answer is written to, round \
     to two decimal places");

pub const ROUNDING_WORDS: [&str; 3] = ["decimal", "places", "place"];
because!(ROUNDING_WORDS, WordEnglish, "the words after the number of a rounding that name no thing, two decimal places");

pub const LETTER_ASKED: &str = "letter";
because!(LETTER_ASKED, WordEnglish, "the word that asks one letter of the last word of a question by its place, the first letter of dog");

pub const LETTER_PLACES: [(&str, usize); 5] = [("first", 0), ("second", 1), ("third", 2), ("fourth", 3), ("fifth", 4)];
because!(LETTER_PLACES, WordEnglish, "the ordinals that pick a letter of a word, each with how many letters stand before it");

pub const SPELLING: [&str; 5] = ["letters", "vowels", "consonants", "digits", "words"];
because!(SPELLING, WordEnglish, "the words that ask a count of what a word, a number or a quoted sentence is written with, how many vowels \
     are in apple");

pub const VOWEL_LETTERS: &str = "aeiou";
because!(VOWEL_LETTERS, WordEnglish, "the letters that are vowels, every other letter being a consonant");

pub const QUOTE: &str = "\"";
because!(QUOTE, WordEnglish, "the mark around a sentence a question quotes, whose words are counted between two of it");

pub const MEDIAN_SIGN: &str = "median";
because!(MEDIAN_SIGN, WordEnglish, "the operation that writes the middle of the numbers of a question once they stand in order");

pub const MODE_SIGN: &str = "mode";
because!(MODE_SIGN, WordEnglish, "the operation that writes the number a question says most often");

pub const SPREAD_SIGN: &str = "range";
because!(SPREAD_SIGN, WordEnglish, "the operation that writes how far the greatest number of a question stands from the least");

pub const DIGITS_SIGN: &str = "digits";
because!(DIGITS_SIGN, WordEnglish, "the operation that writes the sum of the digits of a number");

pub const LEVEL_SIGN: &str = "average";
because!(LEVEL_SIGN, WordEnglish, "the operation that writes the numbers of a question added up and shared evenly, so an average is worked \
     out by the same one move as a median and never by a walk of its own");

pub const LIST_FUNCTIONS: [(&str, &str); 8] = [("average", LEVEL_SIGN), ("mean", LEVEL_SIGN), ("median", MEDIAN_SIGN), ("middle", MEDIAN_SIGN), ("mode", MODE_SIGN), ("often", MODE_SIGN), ("range", SPREAD_SIGN), ("digits", DIGITS_SIGN)];
because!(LIST_FUNCTIONS, WordEnglish, "the words that name an operation over the list of numbers a question says, each with the operation, \
     the middle number and the median being one");

pub const LIST_HALVES: usize = 2;
because!(LIST_HALVES, WordEnglish, "how many halves a list in order has around its middle, for the median");

pub const MULTIPLE_SIGN: &str = "multiple";
because!(MULTIPLE_SIGN, WordEnglish, "the operation that asks whether the first number of a question holds the second a whole number of \
     times");

pub const FACTOR_SIGN: &str = "factor";
because!(FACTOR_SIGN, WordEnglish, "the operation that asks whether the second number of a question holds the first a whole number of \
     times");

pub const MULTIPLIERS: [(&str, f32); 4] = [("double", 2.0), ("twice", 2.0), ("triple", 3.0), ("thrice", 3.0)];
because!(MULTIPLIERS, WordEnglish, "the words that are worth the number they multiply by, so a move on the number may point at them, \
     double five");

pub const SHOWN_THOUSANDTHS: f32 = 1000.0;
because!(SHOWN_THOUSANDTHS, WordEnglish, "how many parts of one a worked expression is written to when no places are set, a number that is \
     not whole answered to three decimals");

pub const SOLVE_NEAR: f32 = 0.001;
because!(SOLVE_NEAR, WordEnglish, "how near nothing the two sides of an equation must come for a guess to solve it");

pub const SOLVE_SPAN: i32 = 1000;
because!(SOLVE_SPAN, WordEnglish, "how far from nothing an unknown of an equation is looked for among the whole numbers, when the equation \
     is no straight line");

pub const POWER_SIGN: &str = "^";
because!(POWER_SIGN, WordEnglish, "the sign that raises the number before it to the number after it, worked before a product");

pub const BRACKET_OPENS: &str = "(";
because!(BRACKET_OPENS, WordEnglish, "the bracket that opens a part of an expression worked first");

pub const BRACKET_CLOSES: &str = ")";
because!(BRACKET_CLOSES, WordEnglish, "the sign after which the rest of an expression goes on at the level it left, so four times the sum \
     in the brackets multiplies the whole sum");

pub const FUNCTIONS: [&str; 5] = ["sqrt", "abs", "sin", "cos", "ln"];
because!(FUNCTIONS, WordEnglish, "the names of the functions an expression may call on the part in the brackets after them");

pub const EQUAL_SIGN: &str = "=";
because!(EQUAL_SIGN, WordEnglish, "the operation equal names, whether the two numbers of a question are the same");

pub const CHOICE_WORD: &str = "or";
because!(CHOICE_WORD, WordEnglish, "the word between the things a question offers, which is none of them");

pub const COMPARING: [(&str, &str); 27] = [("divisible", MULTIPLE_SIGN), ("hotter", MOST_SIGN), ("warmer", MOST_SIGN), ("colder", LEAST_SIGN), ("cooler", LEAST_SIGN), ("earlier", LEAST_SIGN), ("later", MOST_SIGN), ("multiple", MULTIPLE_SIGN), ("factor", FACTOR_SIGN), (">", MORE_SIGN), ("<", LESS_SIGN), ("equal", EQUAL_SIGN), ("=", EQUAL_SIGN), ("fewest", LEAST_SIGN), ("more", MORE_SIGN), ("greater", MORE_SIGN), ("bigger", MORE_SIGN), ("larger", MORE_SIGN), ("less", LESS_SIGN), ("fewer", LESS_SIGN), ("smaller", LESS_SIGN), ("biggest", MOST_SIGN), ("largest", MOST_SIGN), ("greatest", MOST_SIGN), ("most", MOST_SIGN), ("smallest", LEAST_SIGN), ("least", LEAST_SIGN)];
because!(COMPARING, WordEnglish, "the words that compare numbers or counts, each with the operation it names, kept apart from the signs of \
     arithmetic since a statement uses them as plain comparisons, the box is bigger than the bag");

pub const MIDDLE_SIGN: &str = "~";
because!(MIDDLE_SIGN, WordEnglish, "the operation between names, the number in the middle of the numbers of the question, what number is \
     between six and eight");

pub const TOGETHER: [&str; 4] = ["together", "total", "altogether", "all"];
because!(TOGETHER, WordEnglish, "the words that ask a count over every holder, how many apples do they have together");

pub const CLOCK_WORD: &str = "clock";
because!(CLOCK_WORD, WordEnglish, "the word that makes a number after is the hour it is, three o'clock, written as a time of the subject \
     counted in clock");

pub const CLOCK_OPENER: &str = "o";
because!(CLOCK_OPENER, WordEnglish, "the letter before the apostrophe of o'clock, which names nothing");

pub const TIME_ASKED: &str = "time";
because!(TIME_ASKED, WordEnglish, "the word that asks the hour, what time will it be");

pub const AGO: &str = "ago";
because!(AGO, WordEnglish, "the word that turns a count of days or months backward, what day was it three days ago");

pub const UNTIL: &str = "until";
because!(UNTIL, WordEnglish, "the word that asks how many steps of an order lead to the thing named after it, how many days until friday");

pub const MEASURES: [&str; 7] = ["old", "tall", "long", "wide", "deep", "high", "heavy"];
because!(MEASURES, WordEnglish, "the words that name what a counted unit after is measures, tom is five years old, so the count stands \
     under their relation");

pub const DIRECTIONS: [&str; 8] = ["north", "south", "east", "west", "left", "right", "above", "below"];
because!(DIRECTIONS, WordEnglish, "the words that say where one thing stands from another, which name a relation of the subject after is, \
     the office is south of the hallway");

pub const DIGITS_ASKED: &str = "digits";
because!(DIGITS_ASKED, WordEnglish, "the count that is taken of the number a question says and of no word of it, how many digits does \
     seventy have");

pub const KIN: [&str; 13] = ["mother", "father", "brother", "sister", "uncle", "aunt", "cousin", "son", "daughter", "grandmother", "grandfather", "husband", "wife"];
because!(KIN, WordEnglish, "the words that name what one person is to another, which after a possessive name a relation of the owner, ann \
     is tom's mother, where any other noun names what the subject is, rex is tom's dog");

pub const GRAND: &str = "grand";
because!(GRAND, WordEnglish, "the opening of a word of kin one generation further, a grandmother being the mother of a parent");

pub const PARENTS: [&str; 2] = ["mother", "father"];
because!(PARENTS, WordEnglish, "the relations that lead one generation up, through which a grandparent is read");

pub const FLAG_KIN: &str = "kin";
because!(FLAG_KIN, WordEnglish, "the flag that the held subject is what the next word names of the thing the cursor stands on, ann is \
     tom's mother");

pub const SUPERLATIVE_RELATION: &str = "top";
because!(SUPERLATIVE_RELATION, WordEnglish, "the relation the seeds give from a comparison to its superlative, taller to tallest, so the \
     far end of a chain of comparisons answers a superlative");

pub const SUPERLATIVE_END: &str = "est";
because!(SUPERLATIVE_END, WordEnglish, "the ending of a superlative, largest, after which the noun names the kind of the subject, the \
     largest planet");

pub const ORDER_RELATION: &str = "successor";
because!(ORDER_RELATION, WordEnglish, "the relation the seeds chain an order by, monday to tuesday, which after reads forward and before \
     backward");

pub const GIVING: [&str; 6] = ["give", "hand", "pass", "send", "lend", "offer"];

pub const TELLING: [&str; 3] = ["tell", "teach", "read"];
because!(TELLING, WordEnglish, "the verbs whose deed reaches someone without giving them the thing, tom told ann a story, so the one named right after the verb hears it and the thing stays the doer's deed");
because!(GIVING, WordEnglish, "the verbs whose deed moves the thing named next to the one after to, who then has it: ana gives the book to \
     leo");

pub const FLAG_HAND: &str = "hand";
because!(FLAG_HAND, WordEnglish, "the flag that the thing appearing next is handed on, grabbed and dropped into the one after to, who then \
     owns it");

pub const ARTICLE_FLAGS: [&str; 3] = ["the", "a", "an"];
because!(ARTICLE_FLAGS, WordEnglish, "the articles, the setting the flag that the thing appearing next is known and a or an that it is new");

pub const ASKING: [&str; 8] = ["what", "where", "who", "whose", "how", "why", "when", "which"];
because!(ASKING, WordEnglish, "the words that open a question and say what kind of answer it wants");

pub const HELPERS: [&str; 5] = ["does", "do", "did", "many", "much"];
because!(HELPERS, WordEnglish, "the helper words a question says beside its form, which name no thing");

pub const THING_PRONOUNS: [&str; 2] = ["it", "they"];
because!(THING_PRONOUNS, WordEnglish, "the pronouns that stand for the newest thing the story told");

pub const PERSON_PRONOUNS: [&str; 2] = ["he", "she"];
because!(PERSON_PRONOUNS, WordEnglish, "the pronouns that stand for the newest thing under the world, the person the story told of");

pub const MOVING: [&str; 17] = ["fit", "move", "go", "come", "came", "walk", "walked", "fly", "flew", "return", "returned", "travel", "travelled", "traveled", "journey", "drive", "drove"];
because!(MOVING, WordEnglish, "the verbs whose deed puts the thing at the place after to: the thing is grabbed and dropped into the place \
     that appears, leaving where it was");

pub const CONTAINING: [&str; 2] = ["hold", "contain"];
because!(CONTAINING, WordEnglish, "the verbs that say the subject holds what comes next inside it, the bag holds corn, the box contains a \
     hat");

pub const PLURAL_END: &str = "s";
because!(PLURAL_END, WordEnglish, "what a plural ends with, taken off when a count was said before it, so the apples counted are the thing \
     apple with the count as its quantity");

pub(super) const PLURAL_LEAST: usize = 3;
because!(PLURAL_LEAST, WordEnglish, "how many letters a word must keep for its ending to be read as the plural's, so a short word such as \
     is stays whole");

pub const HISSING_ENDS: [&str; 4] = ["x", "ch", "sh", "ss"];
because!(HISSING_ENDS, WordEnglish, "the endings a noun takes es after for its plural, box and boxes, dish and dishes");

pub const LONG_PLURAL: &str = "es";
because!(LONG_PLURAL, WordEnglish, "the plural ending after a hissing sound, boxes");

pub const SOFT_PLURAL: (&str, &str) = ("ies", "y");
because!(SOFT_PLURAL, WordEnglish, "the plural ending that stands for a final y, and the y it stands for, cookies as the lessons' cooky");

pub const PLAIN_PLURALS: [(&str, &str); 20] = [("hooves", "hoof"), ("shelves", "shelf"), ("pennies", "penny"), ("buses", "bus"), ("cookies", "cookie"), ("movies", "movie"), ("pies", "pie"), ("ties", "tie"), ("lies", "lie"), ("series", "series"), ("mice", "mouse"), ("wolves", "wolf"), ("geese", "goose"), ("men", "man"), ("women", "woman"), ("children", "child"), ("feet", "foot"), ("teeth", "tooth"), ("leaves", "leaf"), ("knives", "knife")];
because!(PLAIN_PLURALS, WordEnglish, "the words ending like a soft plural that only drop their last letter, cookies as cookie, or stay as \
     they are, and the plurals that change the word, mice as mouse and wolves as wolf, each with its singular");

pub const FLAG_DEFINITE: &str = "the";
because!(FLAG_DEFINITE, WordEnglish, "the flag that the thing appearing next is one already known, which it keeps as a tag");

pub const FLAG_INDEFINITE: &str = "a";
because!(FLAG_INDEFINITE, WordEnglish, "the flag that the thing appearing next is a new one even when one of the name exists, which it \
     keeps as a tag");

pub const FLAG_QUANTITY: &str = "quantity";
because!(FLAG_QUANTITY, WordEnglish, "the flag holding the count the thing appearing next takes as its quantity");

pub const FLAG_PROPERTY: &str = "property";
because!(FLAG_PROPERTY, WordEnglish, "the flag holding a quality the thing appearing next takes as a value under is, set once per quality \
     said");

pub const FLAG_TIME: &str = "time";
because!(FLAG_TIME, WordEnglish, "the flag holding the time the thing appearing next takes as a tag, past after there was");

pub const FLAG_GIVE: &str = "give";
because!(FLAG_GIVE, WordEnglish, "the flag that the thing appearing next is given to the held thing, so the drop puts it inside the \
     receiver");

pub const FLAG_PLACE: &str = "place";
because!(FLAG_PLACE, WordEnglish, "the flag that the held thing goes inside the next thing to appear, set by a grab");
pub const FLAG_CONTAIN: &str = "contain";
because!(FLAG_CONTAIN, WordEnglish, "the flag that the thing appearing next goes inside the held container, with no owner, the bag holds \
     corn");

pub const MORE_PLACES: [&str; 6] = ["into", "onto", "inside", "within", "over", "through"];
because!(MORE_PLACES, WordEnglish, "the place words the older place list lacks, which put the thing inside what comes next as in does, the \
     ball went into the box");

pub fn function_word(word: &str) -> bool {
    FILLERS.contains(&word) || DEGREE_WORDS.contains(&word) || DIRECTIONS.contains(&word) || word == MEANS || SKIPPED.contains(&word) || COPULA.contains(&word) || HAVING.contains(&word) || NEGATIONS.contains(&word)
        || ARTICLE_FLAGS.contains(&word) || super::mind::place_word(word) || SWITCHED.iter().any(|(said, _)| *said == word) || ASKING.contains(&word) || PERSON_PRONOUNS.contains(&word) || THING_PRONOUNS.contains(&word)
}
because!(function_word, WordEnglish, "whether a word belongs to a closed class of the language, an article, a copula, a place word, a direction, a word of degree, of working or of meaning: such a word says what to do by being itself, so a lesson that names one in a shape does not make the reading blank it as it blanks a name");

pub const FLAG_OWNED: &str = "owned";
because!(FLAG_OWNED, WordEnglish, "the flag that names who the thing appearing next belongs to, set by my or his while the reading holds something else, the keys are in my pocket");

pub const SWITCHED: [(&str, &str); 2] = [("on", "off"), ("off", "on")];
because!(SWITCHED, WordEnglish, "the words that say a thing is working or not, the lamp is on, each with the one it undoes, which read as a state and not as a place when the sentence ends right after them");

pub const STATES: [(&str, &str, &str); 4] = [("open", "open", "closed"), ("close", "closed", "open"), ("lock", "locked", "unlocked"), ("unlock", "unlocked", "locked")];
because!(STATES, WordEnglish, "the verbs whose deed sets a state of the thing named next, each with the state it sets and the state it \
     takes away, tom opens the box and the box is no longer closed");

pub const OWN_WORDS: [(&str, &str); 6] = [("my", "i"), ("his", "he"), ("her", "she"), ("its", "it"), ("their", "they"), ("your", "you")];
because!(OWN_WORDS, WordEnglish, "the possessive words, each with the word that says who owns, my dog is the dog of the speaker");

pub const YOU_WORDS: [&str; 1] = ["you"];
because!(YOU_WORDS, WordEnglish, "the word the speaker says for the one spoken to, the assistant");

pub const ASSISTANT_NAME: &str = "you";
because!(ASSISTANT_NAME, WordEnglish, "the thing you stands for, the one the speaker talks to, named you as the lessons write it");

pub const JOINER: &str = "and";
because!(JOINER, WordEnglish, "the word that joins two things or two clauses");

pub const LINKS: [&str; 6] = ["for", "with", "from", "than", "which", "that"];
because!(LINKS, WordEnglish, "the link words the lessons use between things, which never name a thing themselves");

pub const SOURCE: &str = "from";
because!(SOURCE, WordEnglish, "the word that names where a moving thing came from, which is no place it goes into, the mouse went from the \
     box to the house");

pub const FLAG_FROM: &str = "from";
because!(FLAG_FROM, WordEnglish, "the flag that the thing appearing next is where the held thing came from");

pub const COMPANION: &str = "with";
because!(COMPANION, WordEnglish, "the word that puts the thing after it where the subject is, tom went to the park with his dog, or the \
     subject where the thing after it is, the cat is with you");

pub const FLAG_WITH: &str = "with";
because!(FLAG_WITH, WordEnglish, "the flag that the thing appearing next and the held subject are to stand in one place");

pub const BELONGS_IN: [&str; 2] = ["of", "for"];
because!(BELONGS_IN, WordEnglish, "the word that puts the thing before it inside the thing after it, the door of the house");

pub const DEED_TAG: &str = "{did}";
because!(DEED_TAG, WordEnglish, "the relation a deed with no object is written under, the car broke, as the lessons write it");

pub const RELATIVES: [&str; 2] = ["which", "that"];
because!(RELATIVES, WordEnglish, "the words that open a clause about the place just named, the bag which is on the bed");

pub const GROUP_NAME: &str = "group";
because!(GROUP_NAME, WordEnglish, "what the thing made of the things said together is called, john and i are best friends, so what is true of them together is held by one thing and not by each of them");

pub const ITEMS_NAME: &str = "items";
because!(ITEMS_NAME, WordEnglish, "the relation a group holds its members under, each a mention of the thing itself, so a question about a member finds them through the group");

pub const FLAG_GROUP: &str = "group";
because!(FLAG_GROUP, WordEnglish, "the flag that the things held are a group joined by and, which move together, tom and jane went to the \
     park");

pub const FLAG_STATE: &str = "state";
because!(FLAG_STATE, WordEnglish, "the flag holding the verb of state whose state the thing appearing next takes");

pub(super) const GLIDES: &str = "wxy";
because!(GLIDES, WordEnglish, "the letters that end a verb which takes no e when its ending is cut, played and fixed, so only the others read as a base that ends with e, adored as adore");

pub(super) const VERB_ENDS: [&str; 5] = ["es", "s", "ed", "d", "ing"];
because!(VERB_ENDS, WordEnglish, "the endings a verb takes after he or she or in the past, goes, takes, moved, journeyed and carrying, \
     taken off to find the verb");

pub const PAST_END: &str = "ed";
because!(PAST_END, WordEnglish, "the ending of a regular past form, painted or helped");

pub const SELF_WORDS: [&str; 2] = ["i", "me"];
because!(SELF_WORDS, WordEnglish, "the words the person who speaks says for themself, which stand for the user");

pub const USER_NAME: &str = "user";
because!(USER_NAME, WordEnglish, "the thing the speaker words stand for, the user, as the seeds name the speaker");

pub const COUNTING: [&str; 2] = ["many", "much"];
because!(COUNTING, WordEnglish, "the words after how that ask a count, how many cats are in the box");

pub const GENERAL_THINGS: [&str; 3] = ["things", "items", "objects"];
because!(GENERAL_THINGS, WordEnglish, "the words a count uses for every thing at once, how many things does john have");

pub const FILLERS: [&str; 57] = ["always", "sometimes", "usually", "super", "totally", "only", "just", "very", "really", "quite", "extremely", "rather", "fairly", "most", "every", "each", "following", "now", "check", "again", "look", "still", "please", "ok", "okay", "well", "then", "down", "up", "away", "back", "out", "off", "also", "too", "after", "while", "although", "during", "because", "so", "but", "would", "may", "can", "could", "might", "will", "should", "must", "this", "these", "those", "more", "several", "some", "few"];
because!(FILLERS, WordEnglish, "the words a speaker adds around what is said, check again, look again, still, and the words that only \
     make a quality stronger, very or really, which point at nothing and name no thing");

pub const TAKING: [&str; 13] = ["take", "took", "get", "got", "grab", "grabbed", "pick", "picked", "carry", "buy", "bought", "win", "won"];
because!(TAKING, WordEnglish, "the verbs whose deed moves the thing named next into the subject, who then has it, tom takes the key");

pub const RECEIVING: [&str; 2] = ["receive", "hold"];
because!(RECEIVING, WordEnglish, "the verbs that ask who a thing came to or is with, who received the apple, who is holding the apple");

pub const LEAVING: [&str; 2] = ["leave", "left"];
because!(LEAVING, WordEnglish, "the verbs whose deed takes the subject out of the place named next, the dog left the garden, which keeps \
     its trace");

pub const OUT_OF: &str = "out";
because!(OUT_OF, WordEnglish, "the word after is that takes the subject out of the place named after of, the key is out of the box");

pub const WILL: &str = "will";
because!(WILL, WordEnglish, "the helper that says what follows will be so, the cat will be in the garden, which flags the time later");

pub const AWAY_FROM: &str = "off";
because!(AWAY_FROM, WordEnglish, "the word that after a copula takes the subject out of the place named next, the cat is off the chair");

pub const FLAG_LEAVE: &str = "leave";
because!(FLAG_LEAVE, WordEnglish, "the flag that the held subject leaves the place appearing next");

pub const ORDINALS: [&str; 11] = ["first", "second", "third", "fourth", "fifth", "last", "next", "other", "sixth", "seventh", "eighth"];
because!(ORDINALS, WordEnglish, "the words that pick one of a kind by its place, the second box, which wait before their noun as a quality \
     does");

pub const TIMES_OF_DAY: [&str; 5] = ["yesterday", "morning", "afternoon", "evening", "night"];
because!(TIMES_OF_DAY, WordEnglish, "the words that open a sentence with when its move was made, in the order of the day, so this \
     afternoon mary went to the cinema was after this morning whatever order the text told them in");

pub const GOING_RELATION: &str = "go";
because!(GOING_RELATION, WordEnglish, "the name every verb of moving writes its deed under, so a thing that walked, flew or journeyed to a      place carries one relation of going and a question about where it is reads them all alike");

pub const FITTING: &str = "fit";
because!(FITTING, WordEnglish, "the verb of moving that says a thing is held by a place and not that it went there, the pen fits in the      box, so it puts the thing inside as a place word does and writes no going");

pub const METHOD_RELATION: &str = "method";
because!(METHOD_RELATION, WordEnglish, "the name a going writes the way it was made under, walked, flew or drove, so every going is one      relation and how a thing got to a place is still on the tree to be asked for");

pub const FLAG_WHEN: &str = "when";
because!(FLAG_WHEN, WordEnglish, "the flag that the move of the sentence was made at the time its value names, kept on the thing that \
     moved");

pub const FLAG_ROLE: &str = "role";
because!(FLAG_ROLE, WordEnglish, "the flag that a role the sentence named waits for the thing it is the role of, the capital of england");

pub const SAMENESS: &str = "same";
because!(SAMENESS, WordEnglish, "the word that names a relation of one thing being another after is, a couch is the same as a sofa");

pub const LIKENESS: &str = "as";
because!(LIKENESS, WordEnglish, "the word after same, which names nothing of its own");

pub const SYMMETRIC_ROLES: [&str; 2] = ["opposite", "same"];
because!(SYMMETRIC_ROLES, WordEnglish, "the roles that hold both ways, so the opposite of hot is cold tells that the opposite of cold is hot");

pub const OPPOSITE_WORD: &str = "opposite";
because!(OPPOSITE_WORD, WordEnglish, "the relation the seeds give between two words of opposite meaning, taller and shorter");

pub const PASSIVE_MARK: &str = "by";
because!(PASSIVE_MARK, WordEnglish, "the word that names the doer after a verb read backward, ann is liked by tom");

pub const ABOUT: &str = "about";
because!(ABOUT, WordEnglish, "the word that asks everything the world holds of a thing, tell me about tom");

pub const EARLIER: &str = "before";
because!(EARLIER, WordEnglish, "the word that asks the place a thing was in before a place it names, where was mary before the cinema");

pub const LATER: &str = "after";
because!(LATER, WordEnglish, "the word that asks the place a thing went to after a place it names, where was mary after the kitchen");

pub const COMPARED: &str = "than";
because!(COMPARED, WordEnglish, "the word after a comparison, bigger than, whose relation is named by the comparison and this word, \
     biggerthan as the lessons write it");

pub const COMPARISON_END: &str = "er";
because!(COMPARISON_END, WordEnglish, "the ending of a comparison, bigger or taller");

pub const POSING: [&str; 15] = ["sit", "sat", "stand", "stood", "lie", "live", "stay", "sleep", "slept", "hide", "hid", "rest", "rise", "set", "shine"];
because!(POSING, WordEnglish, "the verbs that say where the subject is, the cat sat on the mat, which point at nothing and leave the place \
     word after them to place the subject");

pub const OWNING: [&str; 1] = ["own"];
because!(OWNING, WordEnglish, "the verbs of owning, which give what comes next to the subject as having does, sam owns a kite");

pub const BELONGING: [&str; 1] = ["belong"];
because!(BELONGING, WordEnglish, "the verbs that give the subject to the one named after to, the ball belongs to lily");

pub const LOSING: [&str; 2] = ["lose", "lost"];
because!(LOSING, WordEnglish, "the verbs whose deed takes the thing named next from the subject, sue lost the cup, which is then put down \
     where the subject stands");

pub const NEGATIONS: [&str; 3] = ["no", "not", "never"];
because!(NEGATIONS, WordEnglish, "the words that deny what comes next, tom has no bike, which set a count of none on the thing named next, \
     a thing that is never found, held or counted");

pub const NO_LONGER: &str = "longer";
because!(NO_LONGER, WordEnglish, "the word that turns no into losing, lily no longer has the ball");

pub const DROPPING: [&str; 4] = ["drop", "dropped", "put", "discard"];
because!(DROPPING, WordEnglish, "the verbs whose deed puts the thing named next where the subject stands, sam drops the ball");

pub const FLAG_TAKE: &str = "take";
because!(FLAG_TAKE, WordEnglish, "the flag that the thing appearing next is taken by the held subject, moved into it wherever it stood");

pub const FLAG_RELEASE: &str = "release";
because!(FLAG_RELEASE, WordEnglish, "the flag that the thing appearing next is put down where the held subject stands");

pub const WORD_CLASS: &str = "word.class";
because!(WORD_CLASS, WordEnglish, "the classes a word heard belongs to, as features of its event, one per class it is in, so the network \
     reads a place word, a copula or a quality by its class and never by its text alone, and a word never seen still reads by the classes \
     the tree and the closed lists put it in");

pub const WORD_KIND: &str = "word.kind";
because!(WORD_KIND, WordEnglish, "the kind the seeds class a quality heard by, color for red, as a feature of its event, so the network \
     sets the property of that kind");

pub(super) const MARK_CLASS: &str = "mark";
because!(MARK_CLASS, WordEnglish, "the class of a full stop, a question mark or a comma, which ends or parts a sentence and names nothing");

pub(super) const SPEAKER_CLASS: &str = "speaker";
because!(SPEAKER_CLASS, WordEnglish, "the class of i and me, the person who speaks");

pub(super) const OWN_CLASS: &str = "own";
because!(OWN_CLASS, WordEnglish, "the class of a possessive word, my or her, which gives the thing named next to its owner");
pub(super) const FILLER_CLASS: &str = "filler";
because!(FILLER_CLASS, WordEnglish, "the class of a word a speaker adds around what is said, again or still");

pub(super) const NEGATION_CLASS: &str = "negation";
because!(NEGATION_CLASS, WordEnglish, "the class of no and not, which deny what comes next");

pub(super) const LINK_CLASS: &str = "link";
because!(LINK_CLASS, WordEnglish, "the class of a link word between things, with, from or than");
pub(super) const OPERATOR_CLASS: &str = "operator";
because!(OPERATOR_CLASS, WordEnglish, "the class of a word or sign of arithmetic, plus or the plus sign");
pub(super) const POSSESSIVE_CLASS: &str = "possessive";
because!(POSSESSIVE_CLASS, WordEnglish, "the class of the apostrophe of a possessive, which gives what comes next to the thing before it");
pub(super) const COPULA_CLASS: &str = "copula";
because!(COPULA_CLASS, WordEnglish, "the class of is, are, was and were, the words between a thing and its place or quality");

pub(super) const HAVING_CLASS: &str = "having";
because!(HAVING_CLASS, WordEnglish, "the class of has, have and had, the words of owning");

pub(super) const ARTICLE_CLASS: &str = "article";
because!(ARTICLE_CLASS, WordEnglish, "the class of the, a and an, which set a flag on the thing to come");

pub(super) const SKIPPED_CLASS: &str = "skipped";
because!(SKIPPED_CLASS, WordEnglish, "the class of a link word that points at nothing");

pub(super) const ASKS_CLASS: &str = "asks";
because!(ASKS_CLASS, WordEnglish, "the class of a word that opens a question");

pub(super) const HELPER_CLASS: &str = "helper";
because!(HELPER_CLASS, WordEnglish, "the class of does, do, did, many and much, which shape a question and name nothing");

pub(super) const THING_PRONOUN_CLASS: &str = "it";
because!(THING_PRONOUN_CLASS, WordEnglish, "the class of the words that stand for the thing the story told last");

pub(super) const PERSON_PRONOUN_CLASS: &str = "he";
because!(PERSON_PRONOUN_CLASS, WordEnglish, "the class of the words that stand for the person the story told of last");

pub(super) const PLACE_CLASS: &str = "place";
because!(PLACE_CLASS, WordEnglish, "the class of a place word, which says the thing must be placed");

pub const VERB_CLASS: &str = "verb";
because!(VERB_CLASS, WordEnglish, "the class of a verb the tree knows by any form");

pub(super) const MOVING_CLASS: &str = "moving";
because!(MOVING_CLASS, WordEnglish, "the class of a verb whose deed moves the thing, went or moves");

pub(super) const PAST_CLASS: &str = "past";
because!(PAST_CLASS, WordEnglish, "the class of a past form the seeds state");

pub(super) const NUMBER_CLASS: &str = "number";
because!(NUMBER_CLASS, WordEnglish, "the class of a word worth a count, in digits or as the seeds spell it");

pub(super) const KIND_CLASS: &str = "kind";
because!(KIND_CLASS, WordEnglish, "the class of a word naming a kind the seeds class qualities by");

pub(super) const QUALITY_CLASS: &str = "quality";
because!(QUALITY_CLASS, WordEnglish, "the class of a quality the seeds know, classed by a kind or set against its opposite");

pub const THING_CLASS: &str = "thing";
because!(THING_CLASS, WordEnglish, "the class of a word naming a thing under the world, the story's or the seeds'");

pub const VALUE_CLASS: &str = "value";
because!(VALUE_CLASS, WordEnglish, "the class of a word the tree holds only as a value");

pub(super) const OTHER_CLASS: &str = "other";
because!(OTHER_CLASS, WordEnglish, "the class of a word in no other class, a name never seen");

pub const CURSOR_KIND: &str = "cursor.kind";
because!(CURSOR_KIND, WordEnglish, "what kind of node the cursor stands on when a word arrives, as a feature of the cursor's event: the \
     world, a thing, a value, a relation or a property, so the network reads where it is from the one item that says so");

pub const CURSOR_HOLDING: &str = "cursor.holding";
because!(CURSOR_HOLDING, WordEnglish, "that the cursor holds a thing when a word arrives, grabbed and not yet dropped, as a feature of the \
     cursor's event, so the network knows the next thing to appear is where it goes");

pub const CURSOR_ASKING: &str = "cursor.asking";
because!(CURSOR_ASKING, WordEnglish, "that a question is open when a word arrives, as a feature of the cursor's event, so a word of a \
     question reads apart from the same word in a statement");

pub const WORD_ASKED: &str = "word.asked";
because!(WORD_ASKED, WordEnglish, "that an item on the stack is a word of the question dumped at the question mark, so the network reads \
     the question it must answer apart from the mark it arrived with");

pub const CURSOR_FLAG: &str = "cursor.flag";
because!(CURSOR_FLAG, WordEnglish, "a flag set on the cursor when a word arrives, as a feature of the cursor's event, one per flag, so the \
     network reads what the words before the thing said");

pub(super) const CURSOR_BY: &str = "cursor";
because!(CURSOR_BY, WordEnglish, "what brought the cursor's event onto the stack: the arrival of a word, not a move");

pub(super) const FACT_BY: &str = "fact";
because!(FACT_BY, WordEnglish, "what brought a thing's event onto the stack: the word itself, since a thing word makes the thing appear \
     before he moves");

pub(super) const WORLD_KIND: &str = "world";
because!(WORLD_KIND, WordEnglish, "the cursor's kind at the root, where every sentence starts");

pub(super) const THING_KIND: &str = "thing";
because!(THING_KIND, WordEnglish, "the cursor's kind on a thing, under the world or inside another thing");

pub(super) const VALUE_KIND: &str = "value";
because!(VALUE_KIND, WordEnglish, "the cursor's kind on a value under a relation, a mention or a plain value");

pub(super) const RELATION_KIND: &str = "relation";
because!(RELATION_KIND, WordEnglish, "the cursor's kind on a relation written from a verb or a tag");

pub(super) const PROPERTY_KIND: &str = "property";
because!(PROPERTY_KIND, WordEnglish, "the cursor's kind on the is relation or a kind under it");

pub(super) const QUESTION_KIND: &str = "question";
because!(QUESTION_KIND, WordEnglish, "the cursor's kind on the question node, named by the word that opened the question");

pub(super) const RECORD_OBJECT: &str = "object";
because!(RECORD_OBJECT, WordEnglish, "the argument of the cursor's record that names the node it stands on");

pub(super) const RECORD_TYPE: &str = "type";
because!(RECORD_TYPE, WordEnglish, "the argument of the cursor's record that says the kind of that node");

pub(super) const RECORD_FLAGS: &str = "flags";
because!(RECORD_FLAGS, WordEnglish, "the argument of the cursor's record that lists the flags set");

pub(super) const RECORD_HOLDING: &str = "holding";
because!(RECORD_HOLDING, WordEnglish, "the argument of the cursor's record that names what he holds");

pub const WORD_ENDING: &str = "word.ending";
because!(WORD_ENDING, WordEnglish, "the ending a word heard is written with, as a feature of its event, so a word the seeds never state \
     still reads as the form its ending makes it, measures as a verb said of one and oldest as a superlative");

pub const GOING_END: &str = "ing";
because!(GOING_END, WordEnglish, "the ending of a verb that tells what is going on");

pub(super) const SEEN_ENDS: [&str; 5] = [SUPERLATIVE_END, GOING_END, PAST_END, COMPARISON_END, PLURAL_END];
because!(SEEN_ENDS, WordEnglish, "the endings the view shows of a word, the longest first so a word shows one: the superlative, the going \
     on, the past, the comparison, and the s of a plural or of a verb said of one");

pub const GREETING_OPENERS: [&str; 1] = ["good"];
because!(GREETING_OPENERS, WordEnglish, "the word that opens a greeting by the time of day, good morning, and is no quality of anything there");

pub const REPLY_RELATION: &str = "reply";
because!(REPLY_RELATION, WordEnglish, "the relation the seeds hold what is said back to a word of manners under, hi to hello and thanks to \
     welcome");

pub const ABLE: &str = "can";
because!(ABLE, WordEnglish, "the word that says a thing is able to do what the verb after it names, a fish can swim, and that opens a \
     question of it, can a fish swim");

pub const WEAK_DEGREES: [&str; 2] = ["bit", "little"];
because!(WEAK_DEGREES, WordEnglish, "the words of degree that say how little of a quality a thing has, which are a degree only after an article, a bit tired, since little is a size of its own");

pub const DEGREE_WORDS: [&str; 10] = ["very", "really", "quite", "extremely", "rather", "fairly", "super", "totally", "bit", "little"];
because!(DEGREE_WORDS, WordEnglish, "the words that say how much of a quality a thing has, very hot, totally disorganized, or how little of it, a bit tired, kept as the degree of the quality named next");

pub const FLAG_DEGREE: &str = "degree";
because!(FLAG_DEGREE, WordEnglish, "the flag that holds a word of degree until the quality it strengthens is written, which then carries it");

pub const CAUSE_WORD: &str = "because";
because!(CAUSE_WORD, WordEnglish, "the word after which a sentence tells why what it told before is so, he is sad because he lost his car");

pub const GROUP_OBJECT: &str = "them";
because!(GROUP_OBJECT, WordEnglish, "the word that stands for the group the sentence before told of when a count of it is named, two of them are red");

pub const REFLEXIVES: [&str; 6] = ["himself", "herself", "itself", "themselves", "myself", "yourself"];
because!(REFLEXIVES, WordEnglish, "the words that name the doer again as what the deed is done to, he only cares about himself, which name no thing of their own");

pub const LABEL_OPENER: &str = "label";
because!(LABEL_OPENER, WordEnglish, "the word a person opens an order to label a thing with, label grandma as a cool person, which names no thing itself");

pub const LABEL_AS: &str = "as";
because!(LABEL_AS, WordEnglish, "the word that joins a thing to its label in an order to label it, which there says what is says: the thing is what the label names");

pub const STORY_OPENER: [&str; 4] = ["once", "upon", "a", "time"];
because!(STORY_OPENER, WordEnglish, "the words a tale opens with, once upon a time, which tell nothing of the world and name no thing");

pub const NAMED_BY: [&str; 2] = ["called", "named"];
because!(NAMED_BY, WordEnglish, "the words that ask or tell the name a thing goes by, what is my sister called");

pub const AFTER_SIGHT: &str = "word.after-sight";
because!(AFTER_SIGHT, WordEnglish, "what the story shows of the word said just before a word, as features of the word's event, so a helper \
     after a role reads apart from a helper after any other thing, what time will it be");

pub const CURSOR_SUBJECT: &str = "cursor.subject";
because!(CURSOR_SUBJECT, WordEnglish, "that the cursor stands on the thing the sentence first named, as a feature of the cursor's event, \
     so a verb right after its subject reads as the subject's deed");

pub const CURSOR_NUMBER: &str = "cursor.number";
because!(CURSOR_NUMBER, WordEnglish, "that a number is worked on, as a feature of the cursor's event, so an operation said next carries on \
     the sum");

pub const CURSOR_QUOTING: &str = "cursor.quoting";
because!(CURSOR_QUOTING, WordEnglish, "that the open question holds a quote mark with no second one yet, as a feature of the cursor's \
     event, so every word inside the quote is kept, an article too");

pub const ASKED_NTH: &str = "word.nth";
because!(ASKED_NTH, WordEnglish, "how many words heard stand below a word of the question on the stack, as a feature of its event, a place \
     that stays the same while steps push more events on top");

pub const POINTED_NTH: &str = "word.pointed-nth";
because!(POINTED_NTH, WordEnglish, "how many words heard stand below the word a step pointed at, as a feature of the step's event, so a \
     plan over several numbers sees which words it has used and points at the next");

pub const CURSOR_RELATION: &str = "cursor.relation";
because!(CURSOR_RELATION, WordEnglish, "the relation the cursor stands on, as a feature of the cursor's event, so a word after is reads \
     apart from a word after has");

pub const CURSOR_UNDER: &str = "cursor.under";
because!(CURSOR_UNDER, WordEnglish, "the relation the cursor's node stands under, as a feature of the cursor's event, so a value under is \
     or a deed under an activity reads as what it is");

pub const CURSOR_DONE: &str = "cursor.clause-done";
because!(CURSOR_DONE, WordEnglish, "whether the clause said so far is complete, as a feature of the cursor's event, which the teacher's \
     rules read at the start of every clause of a turn");

pub const SAID_SIGHTS: [&str; 1] = ["word.said"];
because!(SAID_SIGHTS, WordEnglish, "the feature place of the words said before in the sentence, a word of a closed class by its text and \
     any other by its first class, as the word just before is seen too, so a rule that reads an earlier word, a relative word or an \
     apostrophe, can be learned, and never by a word a lesson may swap");

pub(super) const OPEN_CLASSES: [&str; 5] = [THING_CLASS, VALUE_CLASS, QUALITY_CLASS, OTHER_CLASS, NUMBER_CLASS];
because!(OPEN_CLASSES, WordEnglish, "the classes of the words a lesson may swap for others, which are seen among the words said before by \
     their class and never by their text");

pub const WORD_SIGHTS: [&str; 12] = ["word.told", "word.value-told", "word.relation-told", "word.person", "word.activity", "word.result", "word.measure-told", "word.numbered", "word.role", "word.told-article", "word.mannered", "word.classed"];
because!(WORD_SIGHTS, WordEnglish, "the feature places of what the story shows of a word, in the order the sight gives them");

pub(super) const DOING_LETTERS: usize = 4;
because!(DOING_LETTERS, WordEnglish, "the fewest letters of a word of doing with its ending, hunts, under which a word ending so is a small word of its own, as or yes");
