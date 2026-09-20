---
license: agpl-3.0
language:
- en
library_name: dam1
pipeline_tag: question-answering
tags:
- dam-word
- llm
- symbolic
- reading-comprehension
- safetensors
- rust
- webassembly
---

# 🧠 DAM1

DAM1 is an LLM of a different design. You tell it things and ask it questions in English, as you
do with any LLM. It reads English one word at a time into a tree of things, places and the
relations between them, the world the words describe, and answers from that tree. Like any LLM
it reads a context of tokens and predicts the next token, one at a time. Its tokens are steps,
not pieces of text: grab a thing, drop it into a place, write a property, find what a question
names, get the answer, work a number, until it chooses to continue to the next word. The steps
are fixed operations of the engine; the network only chooses between them.

What it knows is in the tree, where it can be read and changed, and not in the weights. The
network learns only how to shape and read that memory. That is why it is small: it is not a
transformer, it was not trained on a large corpus, and it does not write free text. These files
hold 5,596,839 numbers, three networks of 1,865,613 numbers each that read by vote, and the
permanent state the project's seeds make. It runs on a processor, and in a browser through
WebAssembly, where the Datamine Network chat runs it with nothing sent to a server.

## 📊 How it compares

Every model was asked the same 2,669 questions: the questions of the held-out lines of DAM1's
curriculum, which no network was trained on. Each item is a short text and one question about it.
One rule scores every reply. The other models are also given an order and two worked examples,
which DAM1 is not; they run in float16 on an NVIDIA RTX 3080 Ti, DAM1 on the processor.

| Model | Parameters | Weights | Memory | Answered | Exact | One answer |
|---|---:|---:|---:|---:|---:|---:|
| **DAM1** | **5,596,839** | **22 MB** | **43 MB** | **100.0%** | **100.0%** | **3 ms** |
| Qwen2.5-0.5B-Instruct | 494,032,768 | 988 MB | 1.01 GB | 63.8% | 39.6% | 109 ms |
| Qwen3-0.6B | 596,049,920 | 1.50 GB | 1.22 GB | 52.7% | 41.4% | 80 ms |
| LFM2-350M | 354,483,968 | 709 MB | 727 MB | 52.2% | 44.1% | 53 ms |
| SmolLM2-360M-Instruct | 361,821,120 | 724 MB | 760 MB | 51.4% | 34.1% | 114 ms |
| SmolLM2-135M-Instruct | 134,515,008 | 269 MB | 285 MB | 36.0% | 14.5% | 228 ms |

**Weights** is the file the hub publishes. DAM1's page downloads the same network written
small, 10 MB. **Memory** is what answering one question takes: for DAM1 the one block of
WebAssembly memory that holds the network, the state, the tree and the stack; for the others the
most the card held over the same questions. **Answered** is the share of replies that say the
answer. **Exact** is the share that begin with the answer and put nothing before it. **One answer**
is the middle wait for a single question, for DAM1 on the processor and for the others on the card.

The item set is DAM1's own curriculum. The words, the names and the numbers of a held-out line are
new, but its sentence shapes are shapes DAM1 was taught, so this measures reading of a taught shape
and not general knowledge. Ask about the capital of Peru and these models answer while DAM1 does
not. The harness is in
[benchmarks/](https://github.com/Datamine-Crypto/DAM1/blob/main/benchmarks/README.md).

## 📋 Model details

- **Made by:** Datamine Network Inc.
- **Model type:** an LLM built as a reader of one word at a time over a tree, driven by networks
  over a stack: an embedding per feature place, a block of weights per stack slot, one hidden
  layer of rectified units, an output per step class and a pointer over the words of the
  sentence. Three networks of one shape, trained from different starts, read by vote. They are
  trained from nothing; there is no base model.
- **Language:** English.
- **Licence:** AGPL-3.0-or-later.
- **Files built at:** the commit `config.json` records under `build`, which the publish writes as it exports.
- **Layout:** version 5, described in
  [FORMAT.md](https://github.com/Datamine-Crypto/DAM1/blob/main/huggingface/FORMAT.md).
- **Source:** https://github.com/Datamine-Crypto/DAM1

## ✨ What it does

Tell it statements and ask it questions, across the turns of a conversation. It also knows what
its seeds state: the number words, the measures, the clock, verb forms, opposites, what common
things are and the like. These replies were said by these files with `dam1 talk`, each group one
conversation. A statement is answered with nothing; the reply comes at the question.

| You say | DAM1 says |
|---|---|
| tom has a red car. | |
| who has the car? | tom |
| what color is tom's car? | red |
| what is 12 * 3? | 36 |
| what is the opposite of hot? | cold |
| how many legs does a dog have? | 4 |
| the cat is in the box. the box is in the garden. | |
| where is the cat? | box |
| is the cat in the garden? | yes |
| john took the ball. john went to the hall. | |
| what is john carrying? | ball |
| where is the ball? | hall |
| ann has 3 apples. ben has 5 apples. | |
| how many apples do they have together? | 8 |
| who has more apples? | ben |
| sam is taller than ann. ann is taller than bob. | |
| who is the tallest? | sam |
| is bob taller than sam? | no |
| my sister lives in paris and she loves music. | |
| where does my sister live? | paris |
| what does she love? | music |

It gets things wrong too, and a wrong reply looks the same as a right one:

| You say | DAM1 says | Should be |
|---|---|---|
| there is a red box and a blue box. the red box is heavy. which box is heavy? | yes | the red box |
| tom turned on the lamp. is the lamp on? | no | yes |
| it rained all night. what happened last night? | (no reply) | it rained |

## 🎯 Uses

Intended uses:

- Research on reading into a tree with learned steps, as an alternative to a transformer.
- Short conversations in simple English about things the user tells it: who has what, where
  things are, qualities, counts, sums, measures and time.
- Running an LLM with no server and no graphics card: native or in a browser.

Out of scope:

- Any decision where a wrong answer causes harm: medical, legal, financial, safety or personal.
- A source of truth. It does not know when it is wrong.
- Free text generation, translation, summaries, or English outside the shapes its lessons taught.
- Languages other than English.

## ⚙️ How it works

1. Text is split into sentences, and a sentence into words, numbers and marks. There is no
   learned tokenizer.
2. Memory is one tree, and the tree is a space: a thing is a node that stands inside its place or
   its owner, a relation is a braced token such as `{is}` or `{has}`, a count or a time is a tag
   on the thing, and a mention of a thing told before is a link to the same node.
3. Each word is one input. A word that names a thing makes the thing appear, the known one of
   its name or a new one carrying the flags the words before it set, such as `the` or a count.
4. The stack holds one sentence: the cursor, each word heard, and two events for every step taken
   (the step, and what it found or made). It is emptied at the next sentence. Each event is a set
   of features hashed into a space of 2^64 places.
5. A network reads the stack, one event per slot of 200. An event's embedding is the sum of the
   16-wide embeddings of its places. Each slot has its own weights into 512 hidden rectified
   units. The outputs take a softmax over the 92 step classes. A class is a move and its
   properties, such as `{grab}`, `{drop @}`, `{children add: @ type: property}` or `{get owner}`.
6. The three networks vote: the shares each gives every step are added and the step with the
   largest sum is taken. When the step takes a word of the sentence (the `@`), the pointers score
   every word on the stack the same way and the best one is taken.
7. The step runs, its two events join the stack, and the networks choose again. `{continue}`
   goes on to the next word. A word may take at most 50 steps.
8. What a sentence wrote to the tree stays for the next sentence and the next turn, so a question
   is answered from everything the conversation said and from the seeds.

No rule decides an answer at reading time. Every step of every reading is the networks' choice.

## 🏋️ Training

### 📚 Data

All training data is in the source repository, written for this project in plain English. None of
it is a Hugging Face dataset, so the card lists no `datasets`.

- `model/data/train`: 274 lesson files in 16 folders, from basics and Grade 1 to Grade 12 and
  university, with the world lessons and the debug cases agreed while the design was made. Each
  line is text with the shape its tree should take and the answers expected. A line is either
  learned or held out of training: 2,575 learned lines and 942 held-out lines. The 12 debug
  cases are graded and never trained on.
- `model/data/facts` and `model/data/seeds`: plain fact sentences and the 60 seeds files built
  from them, told into the tree before anything is read. They are not used to train the network.
  They make the state every lesson and every chat starts from, which ships as `state.bin`.

#### What the lessons teach

One concept a file, from the basics up to a university exam:

- **Basics:** simple statements and questions, places, changes over time, referring back, yes and no, adjectives, searching the tree, which one, partial views and partial searches, checking again, tricks, conclusions, actions, hidden things, conversation.
- **Grade 1:** scenes with several things and owners, counts, forms of having, not having, decomposition.
- **Grade 2:** places, time and owning, possessives, classes, days, greetings, indirect questions, negatives, have questions, our and their, family, animals and their sounds, shapes and sides, weather and seasons, a and an, plural counts, my things, them, whose, letters and vowels, changing hands, two things alike, about me, stronger and weaker qualities, going to, story openers, colored things, things that happen, another one, labels and traits, always and never, how much.
- **Grade 3:** arithmetic in words, the number after, between, counting between, successors, odd and even, tens and hundreds, place value, greater and less, more and fewer, the biggest number, totals, counts over two owners, the next season, day, month and letter, alphabetical order, ordinal days, order and time words, order in a line, carrying a sum on, numbered ones, part of a group, on and off, my pocket.
- **Grade 4:** tell me about, comparisons and their inverses, qualities and opposites, verbs and their objects, seeing, giving, moving and carrying, picking up and dropping, what was carried before, counting what is carried, before and after, they, earlier and later, longer and heavier, hotter and colder, the passive, relation words, that clauses, can do, lost things, done by, because, telling, kin somewhere.
- **Grade 5:** costs, coins, dollars and cents, costs together, change, money left after buying, how many one can buy, unit prices, the clock, hours between, days and months apart, durations, parts of an hour, mixed time units, today from its neighbours, dozens and pairs, half and double, ages, amounts left, years and birthdays.
- **Grade 6:** each, sharing, full groups and what is left over, remainders, multiples and factors, what fits in, units, metric units and mixed units, units back, scaling amounts, two-step problems, the missing number, sequences, digit sums, counting words.
- **Grade 7:** fractions and comparing them, percent, discounts, percent increase, area and perimeter, the perimeter of a triangle, roman numerals, rounding, squares and cubes, square roots.
- **Grade 8:** averages, median, mode, range, speed and distance, travel time, unknowns, functions, definitions.
- **Grade 9:** capitals and materials, titles and languages, kind words, what kinds do, kinds with a quality, inherited kinds, superlative kinds, where countries are, what animals eat.
- **Grade 10:** directions, positions, paths, the far end, kinship, grandparents.
- **Grade 11:** deduction, induction, motives, choices, which is not, what is absent, yes or no after a place, many questions on one statement, a colour at the end of a question, plurals, numbers in words, chains of one relation, claims.
- **Grade 12:** scenes of two to five statements with many questions each, from several wordings, both directions, yes and no, and things the scene does not name.
- **University:** chains walked several steps, kinds three deep, counts summed, split or scaled, relations asked backwards, what follows from a rule, and arithmetic that needs a calculator.

### 🔁 Procedure

1. `dam word teach` is the teacher. It is deterministic: for every word of every line it writes
   the moves that shape the tree the line expects and answer its questions, each move run on the
   same world the network runs on. A line the teacher does not reach is never taught. It reaches
   every learned line and, as a control, every held-out line. It writes the stack and the step
   taken, at every step, as rows, and a second copy of each row with the names the lessons state
   blanked, so a name never seen reads as they do.
2. `dam word train` trains a network from the rows of the learned lines: 230,141 rows, item width
   16, 512 hidden units, AdaGrad with a learning rate of 0.05, softmax cross-entropy over the
   step classes and over the words the pointer may take. Three networks were trained, from seeds
   1, 2 and 3. Each run stopped after 300 epochs with no fewer rows taken wrongly, at between
   355 and 404 epochs, with 10 of the 230,141 rows not chosen as taught in each.

The rows are not committed; `dam word teach` makes them again.

### 🖥️ Compute

Each network trains in minutes on one consumer graphics card. The place and the power drawn were
not recorded, so the card gives no `co2_eq_emissions`.

## 📁 Files

| File | Holds |
|---|---|
| `config.json` | The step limit, the network's shape, its voters and its step classes, and the build commit |
| `model.safetensors` | The eight tensors of each of the three voters, listed below |
| `state.bin` | The permanent state every chat starts from: the tree the seeds make |
| `LICENSE`, `NOTICE.md` | The licence and what it covers |

The tensors of `model.safetensors`, for each voter `N` of 0, 1 and 2:

| Tensor | Type | Shape |
|---|---|---|
| `network.N.items` | `U64` | 4,240 |
| `network.N.item_table` | `F32` | 4,240 by 16 |
| `network.N.slot_weights` | `F32` | 200 by 512 by 16 |
| `network.N.fill_weights` | `F32` | 200 by 512 |
| `network.N.hidden_bias` | `F32` | 512 |
| `network.N.output_weights` | `F32` | 92 by 512 |
| `network.N.output_bias` | `F32` | 92 |
| `network.N.point_weights` | `F32` | 512 by 16 |

No file contains code. A tensor runtime cannot run DAM1; it is read by the dam-core crate.

## 🛠️ Use

The crates are not on crates.io. Take them from the source repository. The repository must be
public, or you must be signed in with `hf auth login`, for the download to work.

From the command line, in a checkout of the source repository:

```sh
cd huggingface && cargo build --release
target/release/dam1 pull --repo DatamineNetwork/DAM1
target/release/dam1 talk --repo DatamineNetwork/DAM1 "tom has a red car." "what color is tom's car?"
```

Each text after `talk` is one turn of the same conversation. `--revision` picks a branch, tag or
commit; `--model DIR` reads a folder already on disk. Each reply is printed as JSON: `output` is
what the model said, `steps` is every step the networks chose at every word, `told` is what the
turn wrote to the tree, and `ended` says whether every word continued within the step limit.

From Rust:

```toml
[dependencies]
dam1 = { git = "https://github.com/Datamine-Crypto/DAM1" }
dam1-hub = { git = "https://github.com/Datamine-Crypto/DAM1" }
dam-page = { git = "https://github.com/Datamine-Crypto/DAM1" }
```

```rust
fn main() -> Result<(), String> {
    let folder = dam1_hub::hub::pulled("DatamineNetwork/DAM1", "main")?;
    let model = dam1::model::loaded(&folder)?;
    dam_page::load(&model.network.weights, &model.network.classes)?;
    dam_page::state(&model.state)?;
    dam_page::read("tom has a red car.", model.reading.steps)?;
    println!("{}", dam_page::read("what color is tom's car?", model.reading.steps)?);
    Ok(())
}
```

`dam_page` holds one conversation. `forget` takes it back to the seeds' state; `context` and
`restore` save and load it.

## 📊 Evaluation

The lessons are the training data. Each file mixes learned lines with lines held out of training.
Measured with `dam1 check` on these files, run from `model/`:

```sh
dam1 check --model ../huggingface/model --network data/network/release.bin
```

The check compares every tensor byte for byte with the network the files were exported from, and
the state with the seeds told again. It then reads every line with the networks alone, no teacher
and no search, from the state every seed makes. A line is right when every word of every input
continued within the step limit and the tree holds the expected shape or the output says the
expected answer.

| Lessons | Learned lines | Held-out lines |
|---|---:|---:|
| Basics | 446 of 446 (100.0%) | 142 of 142 (100.0%) |
| Grade 1 | 73 of 73 (100.0%) | 24 of 24 (100.0%) |
| Grade 2 | 676 of 676 (100.0%) | 239 of 239 (100.0%) |
| Grade 3 | 246 of 246 (100.0%) | 97 of 97 (100.0%) |
| Grade 4 | 265 of 265 (100.0%) | 88 of 88 (100.0%) |
| Grade 5 | 172 of 172 (100.0%) | 72 of 72 (100.0%) |
| Grade 6 | 156 of 156 (100.0%) | 58 of 58 (100.0%) |
| Grade 7 | 83 of 83 (100.0%) | 35 of 35 (100.0%) |
| Grade 8 | 79 of 79 (100.0%) | 33 of 33 (100.0%) |
| Grade 9 | 149 of 149 (100.0%) | 65 of 66 (98.5%) |
| Grade 10 | 26 of 26 (100.0%) | 9 of 9 (100.0%) |
| Grade 11 | 82 of 82 (100.0%) | 23 of 23 (100.0%) |
| Grade 12 | 39 of 39 (100.0%) | 11 of 11 (100.0%) |
| University | 47 of 47 (100.0%) | 13 of 13 (100.0%) |
| World and debug cases | 36 of 36 (100.0%) | 32 of 32 (100.0%) |
| **All** | **2575 of 2575 (100.0%)** | **941 of 942 (99.9%)** |

It reads every learned line of the curriculum. The one held-out line it misses is in the lesson
on the parts a kind of thing has, which asks a part of a thing that is not a creature.

The held-out lines use the same sentence shapes and many of the same words as the learned lines.
They measure new words and numbers in taught shapes. They are not an independent benchmark. No
result is given in the Hub's evaluation format, because the lessons are not a Hub benchmark
dataset.

## ⚠️ Limitations and risks

- English only, and only the sentence shapes its lessons taught. A shape it was not taught often
  reads wrong.
- It knows what it is told and what the seeds state, nothing more. It has no general world
  knowledge.
- A wrong reply looks the same as a right one. Check what it says.
- It accepts what it is told as true and says it back. It has no filter for false, harmful or
  offensive statements.
- The seeds are the project's own sentences and reflect what the project wrote.
- The stack holds 200 events and one sentence, so a very long sentence loses its first words. A
  word that does not continue within 50 steps ends the turn as not ended.

## 📖 Citation

```bibtex
@software{dam1_2026,
  author = {{Datamine Network Inc.}},
  title  = {DAM1: an LLM that reads one word at a time into a tree, with networks over its stack},
  year   = {2026},
  url    = {https://github.com/Datamine-Crypto/DAM1},
  note   = {Model files built at the commit the config records}
}
```

There is no paper.

## ✉️ Contact

Open a discussion in the Community tab of this repository, or an issue in the source repository.

## ⚖️ Licence

Copyright 2026 Datamine Network Inc. The code, the lessons, the seeds and these weights are
released under the GNU Affero General Public License, version 3 or later. If you run a modified
version for users over a network, you must offer them its source. The names DAM1 and Datamine
Network are not licensed for use by modified versions. See `LICENSE` and `NOTICE.md`.
