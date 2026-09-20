# Working in this repository

## The discipline

The Rust is written under Premise. Its manual is the README of
https://github.com/Datamine-Crypto/premise, and `checker --explain E-CODE` prints the passage for
any check that fires. The short form:

- The Rust workspace is `model/`: run cargo, `checker` and `gate` there. Zones are in
  `model/premise.zones`:
  - `spec` holds facts: every tuned number, with `because!` when it was measured and
    `provisional!` saying what would settle it when it was not.
  - `patterns` crates (`dam-core`, `dam-console`, `dam-card`, `dam-page`) hold the logic; every
    public item carries a `because!`, and every constant cites a `source!` declared in the same file.
  - `app` crates (`app`, `dam-cli`, `dam-web`) wire spec values into pattern calls and may do
    nothing else.
  - `library` names the outside crates an app crate may use: `premise` and `wasm-bindgen`.
- No comments anywhere. A reason goes on the item as a `because!`; a fact goes into the spec.
- Run `checker` after every edit (a second); run `gate` before a commit (minutes: it compiles the
  workspace and runs every test).
- Tests live in `model/crates/app/tests/` and are the one place literals and loops are free;
  comments are still banned there. `card.rs` needs an NVIDIA card with CUDA.
- Speed first. An engine stays imperative inside a patterns crate; a generic pattern is extracted
  only where it costs nothing.

What the checker refuses that is easy to write: a float literal (use `dam::numbers::{zero, one}`),
a char literal (use a one character string), a digit inside any string, a string carried by an
attribute, `#[cfg]` other than `cfg(test)`, a `static` inside a function, two items of one name
anywhere in the workspace, two functions of one shape, a logic line in an app crate, and an
outside crate an app crate depends on that no zone names.

## Build and test

```sh
cd model                                                 # every cargo, dam, checker and gate command runs here
cargo test -p app --release                              # the project's tests
gate                                                     # everything
node ../scripts/build-curriculum.mjs [grade]             # a grade folder of data/train that does not exist yet, from data/quiz; the lessons are edited in data/train
node ../scripts/build-seeds.mjs NAME ...                 # data/seeds from data/facts
NETWORK=W[,W2,W3] sh ../scripts/build-chat.sh            # the chat's engine, its network or networks that vote, and the state of every seed (not committed)
cd ../chat && npm install && npm run dev                 # the chat: the engine in the browser, no backend
```

`dam` is `model/crates/dam-cli`. It has one command, `word`, the reading one word an input:

```sh
dam word teach --quiz F --out R --state all        # the teacher's moves for every word of every line of F, written as rows R, from the state all seeds make
dam word train --rows R --out W --card             # a network taught R, W and W.json written
dam word report --quiz F --network W[,W2,W3] --state all   # every line of F read word by word, by one network or by several that vote
dam word state --out S --seeds all                 # the state file every chat starts from, the tree the seeds make
```

Options:

- `teach` and `report`: `--state all` starts every line from the state all seeds make, as the chat
  does; seeds file names joined by commas start it from those.
- `train`:
  - `--card` works on the graphics card.
  - `--slices N` sets the rows a step sums.
  - `--limit S` stops after the epoch that passes S seconds and writes the network.
  - `--enough X` stops once that share of learned lines is taken as taught.
  - `--from W` grows a network from W.
  - `--epochs`, `--seed`, `--hidden`, `--patience` and `--settle` override the spec.
- `report`: `--all` prints every input, not only the failed ones.

## Rules that hold

1. **The quiz is the weights.** `model/data/quiz/` is the source and the network is the output.
   Changing what the system does is editing English. `scripts/build-curriculum.mjs` builds the
   grade folders of `model/data/train/` from it, and every run teaches and reports those files.
   `model/data/facts/` holds plain fact sentences, and `scripts/build-seeds.mjs` writes from them
   the seed files of `model/data/seeds/`, the statements a line or a chat starts from.

2. **No word has a meaning in the code.** The moves are fixed operations the network chooses
   between; what a word does is the network's choice over the stack. A concept the network
   does not answer is taught with quiz lines, never answered by a rule.

3. **A wrong answer has three causes.** The stack did not hold what the answer needs, or no move
   can do the step, or the network chose wrong. Check the quiz line first, then the stack, then
   the moves; only then the training. A reading the teacher finds that answers by chance is a fault
   in the teacher's constraints, not good data.

4. **Learned lines must all read.** The target is every learned line of every file. Held-out lines
   (`test:`) measure what the network does with lines it never trained on.

5. **Runs are held to five minutes.** A teaching or training run that passes five minutes is
   stopped and the input it was stuck on is read from its progress lines. It gets five more
   minutes only when the errors it reports are not real ones.

6. **Measure rather than assert.** Quote the measurement, not the expectation. Two seeds, not
   one: a single run is not a measurement.

7. **On Windows the test binary cannot be rebuilt while a test from it runs** (`LNK1104`). Start
   the long test last, or wait for it.

## The quiz format

A line is a text, the statements its tree should hold, and optional questions with their answers:

```
tom has a car. the car is red. => tom {owns} car; car {is} red | who has the car? = tom; what color is the car? = red
test: ann is in china. => ann {in} china | where is ann? = china
ann has 3 apples. => ann -> {owns} -> apple -> {quantity} -> 3 | how many apples does ann have? = 3
```

- A statement is three names (`car {is} red`), a path of names and relations
  (`fred -> {in} -> school -> {before} -> bedroom`), a worth (`three = 3`), or a statement the tree
  must not hold (`not tom {owns} car`). Every relation is a braced token, so no relation ever
  competes with a word of a text; `=` is the one relation written as a sign. A count is a
  `{quantity}` under the thing, and the thing is written in the singular (`apple`), which the
  reader builds from the word it heard by dropping the ending.
- An answer is a list of names, `nothing` when the tree holds nothing the question asks for, a
  chain the output must say in order (`color->red`), or a name after `not` the output must not say.
- `test:` holds a line out. `then:` reads the line on the tree and stack the line above it left.
- `turn:` is a chat turn read as one input: after the arrow the statements its tree should hold,
  after the bar the answer it should say.
- `next:` asks what comes after a text.
- `world:` states a world instead of a text: the statements before the bar are in the tree before
  the line starts, in the form a reading leaves, so each question is answered from the tree
  (`world: tom {owns} ball; ball {in} box | where is the ball? = box`). A `then: world:` line writes its
  statements into the tree the line above left, with nothing on the stack, so the world changes after
  the network looked; a statement starting with `not` removes its path.
- `#` starts a comment.
- `seed: NAME` names a seeds file; every line after it in the file starts from those statements,
  and a bare `seed:` ends them.

## The word network

- **Tree and stack.** The tree's root is `{world}` and the tree is a space: a thing stands inside
  its place or its owner. The stack holds one sentence: the cursor, every word heard, every step
  and what each step found. It is emptied at the next sentence, so the tree is the memory.
- **Reading a word.** Each word is one input. A word heard makes or finds its thing before any
  move. Then the network takes steps, moves such as grab, drop, add a relation, find, get and
  compute, until it continues to the next word. A step that takes a word of the sentence chooses
  it with a pointer.
- **Teaching.** `word teach` runs a deterministic teacher: for every word it writes the moves that
  shape the tree the line expects and answer its questions, each move run on the same world the
  network runs on, and every step becomes a row with the stack it was taken on.
- **Training and reading.** `word train` teaches a network those rows by AdaGrad on the CPU or
  the card. `word report` reads every line with the network, or with several that vote.

## Not source

`model/crates/dam-web/pkg/`, `chat/dist/`, `node_modules/`, `chat/src/wasm/`, `chat/public/*.bin`,
`chat/public/network.json` and `chat/public/state.bin`: all generated and ignored.
