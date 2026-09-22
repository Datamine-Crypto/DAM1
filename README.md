# 🧠 Datamine Network

[![Check](https://github.com/Datamine-Crypto/DAM1/actions/workflows/check.yml/badge.svg)](https://github.com/Datamine-Crypto/DAM1/actions/workflows/check.yml)
[![Deploy Web](https://github.com/Datamine-Crypto/DAM1/actions/workflows/deploy-web.yml/badge.svg)](https://github.com/Datamine-Crypto/DAM1/actions/workflows/deploy-web.yml)
[![Publish model](https://github.com/Datamine-Crypto/DAM1/actions/workflows/publish-model.yml/badge.svg)](https://github.com/Datamine-Crypto/DAM1/actions/workflows/publish-model.yml)
[![Model on Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97%20Hugging%20Face-DAM1-yellow)](https://huggingface.co/DatamineNetwork/DAM1)
[![Discord](https://img.shields.io/badge/Discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/2dQ7XAB22u)
[![Rust](https://img.shields.io/badge/Rust-1.97-orange?logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/runs%20in-WebAssembly-654ff0?logo=webassembly&logoColor=white)](chat/README.md)
[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-green)](LICENSE)

🟢 **Live now at [datamine.network](https://datamine.network/)**

DAM1 is the World's Most Efficient LLM: one network of 1.9 million numbers, under 4 MB to
download, and it runs in a browser with no server and no graphics card. It
learns to read English without any word meaning written in the code. It reads one word at a time
and shapes a tree of things, places and relations, the world the words describe. At each word a
neural network looks at the stack of the sentence so far (the words heard, the steps taken and
what each step found) and chooses the next step, until it continues to the next word. Everything
the network does is learned from a curriculum of quiz lines.

The Rust is written under [Premise](https://github.com/Datamine-Crypto/premise): every fact lives
in code once, every value carries the reason it was chosen, there are no comments, and a checker
fails the build when either slips. [AGENTS.md](AGENTS.md) says how to work here.

## 📊 How it compares

Every model was asked the questions of the held-out lines of the curriculum, which no network was
trained on: 2,673 for DAM1, and the 2,669 that existed when the others were run. Each item is a short text and one question about it.
One rule scores every reply. The other models are also given an order and two worked examples,
which DAM1 is not; they run in float16 on an NVIDIA RTX 3080 Ti, DAM1 on the processor.

| Model | Parameters | Weights | Memory | Answered | Exact | One answer |
|---|---:|---:|---:|---:|---:|---:|
| **DAM1** | **1,870,616** | **8 MB** | **23 MB** | **99.9%** | **99.9%** | **2 ms** |
| Qwen2.5-0.5B-Instruct | 494,032,768 | 988 MB | 1.01 GB | 63.8% | 39.6% | 109 ms |
| Qwen3-0.6B | 596,049,920 | 1.50 GB | 1.22 GB | 52.7% | 41.4% | 80 ms |
| LFM2-350M | 354,483,968 | 709 MB | 727 MB | 52.2% | 44.1% | 53 ms |
| SmolLM2-360M-Instruct | 361,821,120 | 724 MB | 760 MB | 51.4% | 34.1% | 114 ms |
| SmolLM2-135M-Instruct | 134,515,008 | 269 MB | 285 MB | 36.0% | 14.5% | 228 ms |

**Weights** is the file the hub publishes. DAM1's page downloads the same network written
small, 3.3 MB. **Memory** is what answering one question takes: for DAM1 the one block of
WebAssembly memory that holds the network, the state, the tree and the stack; for the others the
most the card held over the same questions. **Answered** is the share of replies that say the
answer. **Exact** is the share that begin with the answer and put nothing before it. **One answer**
is the middle wait for a single question, for DAM1 on the processor and for the others on the card.

The item set is DAM1's own curriculum. The words, the names and the numbers of a held-out line are
new, but its sentence shapes are shapes DAM1 was taught, so this measures reading of a taught shape
and not general knowledge. Ask about the capital of Peru and these models answer while DAM1 does
not. [benchmarks/](benchmarks/README.md) holds the item set, the
harness and the full method.

## 🗂️ Layout

| folder | holds |
|---|---|
| `chat/` | the chat app: the word network runs in the browser, built into it by `scripts/build-chat.sh` and deployed by GitHub Actions |
| `model/` | the Rust workspace (`crates/`, `Cargo.toml`) and the curriculum, facts and seeds in `data/` |
| `scripts/` | `build-curriculum.mjs` (the grades from the quiz), `build-seeds.mjs` (the seeds from the facts) and `build-chat.sh` (the chat's engine, network and seeds) |
| `huggingface/` | the export of the release network to the Hugging Face layout, with its check and its round-trip tests |
| `benchmarks/` | the head to head: the item set, DAM1 and the other models asked the same questions, one rule for every reply |

## 📦 Crates

| crate | zone | holds |
|---|---|---|
| `model/crates/spec` | spec | every tuned number, each with its reason or marked provisional until a sweep settles it |
| `model/crates/dam-core` (lib `dam`) | patterns | the tree, the reading one word at a time with its moves and its teacher, the stack events, the network over the stack, the quiz format, word shapes, work on threads |
| `model/crates/dam-console` | patterns | the `word` command: teach, train, report, state; the trainer and the files |
| `model/crates/dam-card` | patterns | training steps and checks on an NVIDIA card through CUDA |
| `model/crates/dam-page` | patterns | the chat page's engine: the network and seeds loaded, each turn read on the chat's tree |
| `model/crates/dam-web` | app | the page's engine exported to WebAssembly with the spec's settings |
| `model/crates/dam-cli` | app | the `dam` binary: the spec's values wired into the console |
| `model/crates/app` | app | the tests |

## 📚 Data

| folder | holds |
|---|---|
| `model/data/quiz/` | the quiz files: the source of what the network does |
| `model/data/train/` | the curriculum: grade folders in order. The lessons are edited here; `scripts/build-curriculum.mjs` first built them from the quiz and now leaves a folder that exists alone unless `--overwrite` is given |
| `model/data/facts/` | fact sentences by field: geography, countries, science, living things, people, words and more |
| `model/data/seeds/` | statements a line or a chat starts from, written from the facts by `scripts/build-seeds.mjs` |

## 🚀 Run

```sh
cd model
gate                                                   # the compiler, the laws and every test
checker                                                # the laws alone, fast
cargo test -p app --release                            # the project's tests
cargo run --release -p dam-cli -- word teach --quiz data/train/000_basics/001_simple.txt --out rows.jsonl --state all
cargo run --release -p dam-cli -- word train --rows rows.jsonl --out net.bin --card
cargo run --release -p dam-cli -- word report --quiz data/train/000_basics/001_simple.txt --network net.bin --state all
```

`gate` and `checker` are installed from the Premise repository:
`cargo install --git https://github.com/Datamine-Crypto/premise premise_gate premise_checker`. Run
them from `model/`. The console reads `data/` from wherever it is started, so start it in `model/`.

## 🧭 Reading one word at a time

DAM1 reads one word an input (`dam word teach`, `train`, `report` and `state`). The tree is a space: a
thing stands inside its place or its owner, a word heard makes or finds its thing before any move, and
the network chooses moves such as grab, drop, add a relation, find, get and compute until it continues.
A deterministic teacher writes the moves for every line of the curriculum, each move run on the same
world the network runs on, and the rows it writes are what the network learns. The network sees the
cursor, the word with its classes, its ending and what the story shows of it, the words said before by
class, and what each step found. `--state all` starts every line from the state all seeds make, as the
chat does. `--network` takes one file, or several joined by commas: networks trained from different starts
then read by vote, which removes most of the misses one network makes by chance, at three
times the size; the release ships one.

## 🧭 The word network

- **The mind.** A tree, a cursor on one of its nodes, and a stack that holds one sentence: every
  word heard, every step taken and what each step found. The stack is emptied at the next
  sentence, so the tree is the memory.
- **A word.** Each word is one input. The word makes or finds its thing, then the network takes
  steps: grab a thing, drop it into a place, give it to an owner, write a property or a relation,
  find what a question names, get an answer off the tree, work a number. It stops when it
  continues to the next word.
- **Pointers.** A step that takes a word of the sentence chooses it with a pointer.
- **The teacher.** A deterministic teacher writes the steps for every word of every line. The
  goal is the statements the tree should hold after a text and the answer the output should say
  after a question, and a line the teacher does not reach is never taught.
- **Rows and training.** Every step becomes a row: the stack and the step. The network learns the
  rows by AdaGrad, on the CPU or on the card, and a row counts as learned only when its step and
  its pointed word are both right.
- **The vote.** Several networks trained from different starts can read together: each gives every
  step a share, the shares are added, and the step with the largest sum is taken. The release ships
  one network and no vote.

A wrong answer is fixed in the curriculum, the stack or the moves, never with a rule that answers
it in code.

### 🎓 The curriculum

`model/data/train/` holds grade folders from basics to university, each file one concept, with
learned lines and held-out lines. A line is a text, the statements its tree should hold, and
questions with their answers. Every relation is a braced token, `test:` holds a line out of
training, and `turn:` is a chat turn with the answer it should say:

```
tom has a car. the car is red. => tom -> {owns} -> car(is: red) | who has the car? = tom; what color is the car? = red
test: ann is in china. => ann {in} china | where is ann? = china
turn: what is 2 + 3 => | 5
```

AGENTS.md describes the whole format.

The lessons teach these concepts, one file each, from the basics up to a university exam:

- **Basics:** simple statements and questions, places, changes over time, referring back, yes and no, adjectives, searching the tree, which one, partial views and partial searches, checking again, tricks, conclusions, actions, hidden things, conversation.
- **Grade 1:** scenes with several things and owners, counts, forms of having, not having, decomposition.
- **Grade 2:** places, time and owning, possessives, classes, days, greetings, indirect questions, negatives, have questions, our and their, family, animals and their sounds, shapes and sides, weather and seasons, a and an, plural counts, my things, them, whose, letters and vowels, changing hands.
- **Grade 3:** arithmetic in words, the number after, between, counting between, successors, odd and even, tens and hundreds, place value, greater and less, more and fewer, the biggest number, totals, counts over two owners, the next season, day, month and letter, alphabetical order, ordinal days, order and time words, order in a line.
- **Grade 4:** tell me about, comparisons and their inverses, qualities and opposites, verbs and their objects, seeing, giving, moving and carrying, picking up and dropping, what was carried before, counting what is carried, before and after, they, earlier and later, longer and heavier, hotter and colder, the passive, relation words.
- **Grade 5:** costs, coins, dollars and cents, costs together, change, money left after buying, how many one can buy, unit prices, the clock, hours between, days and months apart, durations, parts of an hour, mixed time units, today from its neighbours, dozens and pairs, half and double, ages, amounts left, years and birthdays.
- **Grade 6:** each, sharing, full groups and what is left over, remainders, multiples and factors, what fits in, units, metric units and mixed units, units back, scaling amounts, two-step problems, the missing number, sequences, digit sums, counting words.
- **Grade 7:** fractions and comparing them, percent, discounts, percent increase, area and perimeter, the perimeter of a triangle, roman numerals, rounding, squares and cubes, square roots.
- **Grade 8:** averages, median, mode, range, speed and distance, travel time, unknowns, functions, definitions.
- **Grade 9:** capitals and materials, titles and languages, kind words, what kinds do, kinds with a quality, inherited kinds, superlative kinds, where countries are, what animals eat.
- **Grade 10:** directions, positions, paths, the far end, kinship, grandparents.
- **Grade 11:** deduction, induction, motives, choices, which is not, what is absent, yes or no after a place, many questions on one statement, a colour at the end of a question, plurals, numbers in words, chains of one relation, claims.
- **Grade 12:** scenes of two to five statements with many questions each, from several wordings, both directions, yes and no, and things the scene does not name.
- **University:** chains walked several steps, kinds three deep, counts summed, split or scaled, relations asked backwards, what follows from a rule, and arithmetic that needs a calculator.

### 🌱 Facts and seeds

`model/data/facts/` holds plain fact sentences in files by field. `scripts/build-seeds.mjs` writes
the sentences it can read as statements into `model/data/seeds/`, one file per facts file. A quiz
line names the seeds it starts from. The chat page and the published model start from the seeds
`scripts/build-chat.sh` is given with `SEEDS`: by default the chat starts from every seed, since the
network is taught on the state all seeds make, and `NETWORK` takes one network, or several joined by
commas that read by vote.

### 💬 The chat

The chat page runs `dam-web` in the browser. Every prompt is read by the network on the tree the
chat has built so far, and the reply is what the network says. Nothing leaves the browser.

## ⚖️ License

Copyright 2026 Datamine Network Inc. The code, the quiz, the facts and every trained network built
from them are released under the GNU Affero General Public License, version 3 or later:
[LICENSE](LICENSE). [NOTICE.md](NOTICE.md) says what the licence covers, what it does not
(third-party data, and the names DAM1 and Datamine Network), and what running the model for others
requires.
