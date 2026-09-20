"""The rule that says whether an answer is right, the same rule for every model.

The reply is put in lower case, its marks are taken off, and a leading article and an opener like
"the answer is" are dropped. A number counts in digits or in words. The reply is right when the
answer is in it (answered), and exact when the reply begins with the answer (right).
"""
import re

WORDS = {
    "0": "zero", "1": "one", "2": "two", "3": "three", "4": "four", "5": "five", "6": "six",
    "7": "seven", "8": "eight", "9": "nine", "10": "ten", "11": "eleven", "12": "twelve",
    "13": "thirteen", "14": "fourteen", "15": "fifteen", "16": "sixteen", "17": "seventeen",
    "18": "eighteen", "19": "nineteen", "20": "twenty", "30": "thirty", "40": "forty",
    "50": "fifty", "60": "sixty", "70": "seventy", "80": "eighty", "90": "ninety",
    "100": "hundred", "1000": "thousand",
}
DECLINES = (
    "nothing", "none", "no", "no one", "nobody", "not mentioned", "not said", "not stated",
    "unknown", "does not say", "doesn t say", "is not in the text", "not in the text",
    "cannot be answered", "can t be answered", "i don t know", "i do not know",
    "not mention", "does not mention", "no information", "not say",
)
ARTICLES = ("the ", "a ", "an ", "his ", "her ", "their ", "its ", "my ", "your ")
OPENERS = (
    "the answer is ", "answer: ", "it is ", "it's ", "there is ", "there are ", "i think ",
    "yes, ", "no, ",
)


def plain(text):
    text = text.lower().replace("{", "").replace("}", "")
    text = re.sub(r"[^a-z0-9 ]+", " ", text)
    text = re.sub(r"\s+", " ", text).strip()
    for opener in OPENERS:
        if text.startswith(opener):
            text = text[len(opener):].strip()
    for article in ARTICLES:
        if text.startswith(article):
            text = text[len(article):].strip()
    return text


def forms(answer):
    """The ways the expected answer may be written."""
    kept = {plain(answer)}
    bare = plain(answer)
    if bare in WORDS:
        kept.add(WORDS[bare])
    for digit, word in WORDS.items():
        if bare == word:
            kept.add(digit)
    if bare == "nothing":
        kept.update(DECLINES)
    return {form for form in kept if form}


def right(reply, answer):
    """True when the reply says the answer and nothing else first."""
    said = plain(reply)
    if not said:
        return False
    for form in forms(answer):
        if said == form:
            return True
        # The first words of the reply say the answer: the answer with what follows it in a sentence.
        if said.startswith(form + " "):
            return True
    return False


def loose(reply, answer):
    """True when the answer is anywhere in the reply, a kinder rule for a model that talks."""
    said = " " + plain(reply) + " "
    return any((" " + form + " ") in said for form in forms(answer))


def answered(reply, answer, kind=True):
    """The one rule used for every model.

    An answer of several parts is right when the reply says every part, in any order. An empty reply
    is right only when the answer is nothing.
    """
    if not plain(reply):
        return plain(answer) in ("nothing", "")
    parts = [part.strip() for part in answer.split(",") if part.strip()]
    if len(parts) > 1:
        return all(loose(reply, part) for part in parts)
    return loose(reply, answer) if kind else right(reply, answer)
