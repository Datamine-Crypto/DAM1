# The DAM1 model layout, version 5

A DAM1 model is a folder of three files. The same folder is what the Hugging Face repository
holds, what `dam1 export` writes and what `dam1::model::loaded` reads. The repository also holds
`README.md` (the model card), `LICENSE` and `NOTICE.md`. A loader does not read them, and
`dam1 pull` does not fetch them.

| File | Holds | Read by |
|---|---|---|
| `config.json` | What the folder is, the limit a reading runs under, the network's shape, voters and step classes, the build | any JSON reader |
| `model.safetensors` | Every number of every voter, as named tensors | any safetensors reader |
| `state.bin` | The permanent state every chat starts from: the tree the seeds files make | dam-core only |

No file holds code. A loader never runs anything from the folder.

DAM1 is not a tensor network that generates text. It reads a sentence one word at a time. At each
word the network chooses steps that shape a tree, the world the sentence describes, until it
chooses to go on to the next word. The steps themselves are the dam-core crate. So the folder
cannot be run by a tensor runtime such as transformers, Candle or ONNX: it is read by dam-core,
natively or compiled to WebAssembly.

Version 5 holds one word network, as one voter or several voters of one shape that read by vote,
and the permanent state. A loader refuses a folder of any other version.

## config.json

| Field | Meaning |
|---|---|
| `architecture` | Always `dam word network`. The weights file carries the same text under its `format` metadata key. |
| `format_version` | The layout version, `5` for this document. |
| `reading.steps` | The most steps the network may take at one word. A word that takes more ends the reading of the sentence as not finished. |
| `network.bits` | The width in bits of the hashed feature space the events' features are placed in. It must equal dam-core's `FEATURE_BITS`. |
| `network.slots` | How many events of the stack a voter reads, the newest first. |
| `network.item` | The width of an event's embedding. |
| `network.hidden` | How many hidden units. |
| `network.items` | How many feature places have an embedding. |
| `network.pointer` | Whether a voter points at the word a pointing step takes. |
| `network.voters` | How many networks of this one shape read by vote, `1` or more. |
| `network.classes` | The step each output names, in output order. See below. |
| `build` | The repository commit the model was exported at, the network file it was made of, and what its state was made of: the seeds folder alone when every seeds file was told, the folder and `none` for an empty tree, or the folder and the names of the seeds files told, joined by commas. |

A class is a step written as one braced record: the move and then its properties, each a name, a
colon and a value. `{grab}` holds the thing the cursor stands on, `{drop @}` puts the held thing
into the thing the pointed word names, `{setFlag type: the value: true}` marks the next thing as
definite, `{children add: @ type: property}` writes the pointed word under the thing as a
property, `{get owner}` answers who owns the thing, and `{continue}` goes on to the next word.
The mark `@` says the step takes a word of the sentence; the pointer chooses which one. A loader
refuses a class that names no step of its engine.

## model.safetensors

Every tensor is little-endian. The file holds these tensors for each voter and no other. The
voter's number `N` counts from `0` in the order the voters vote. `network.N.point_weights` is held
only when `network.pointer` is true. Every voter has the same shapes.

| Tensor | Type | Shape | Holds |
|---|---|---|---|
| `network.N.items` | `U64` | items | Each feature place that has an embedding, in rising order, each once |
| `network.N.item_table` | `F32` | items by item | The embedding of each place, in `network.N.items` order |
| `network.N.slot_weights` | `F32` | slots by hidden by item | Each slot's weights from an event's embedding into the hidden units |
| `network.N.fill_weights` | `F32` | slots by hidden | What each hidden unit adds when a slot is filled |
| `network.N.hidden_bias` | `F32` | hidden | The bias of each hidden unit |
| `network.N.output_weights` | `F32` | classes by hidden | The weights from the hidden units into each output, in `network.classes` order |
| `network.N.output_bias` | `F32` | classes | The bias of each output, in `network.classes` order |
| `network.N.point_weights` | `F32` | hidden by item | The weights the pointer scores a word's embedding with against the hidden units |

The names in the Shape column are the `network` fields of `config.json`; classes is the length of
`network.classes`.

A voter reads one event per slot. An event is a set of feature places. Its embedding is the sum of
the rows of `item_table` at those places; a place not in `items` adds nothing. Slot `s` adds
`slot_weights[s] · embedding + fill_weights[s]` to the hidden units, which start at `hidden_bias`.
An empty slot adds nothing. The hidden units are rectified, and each output is
`output_bias + output_weights · hidden`. The outputs take a softmax, so each voter gives every
step a share of one whole. The shares of the voters are added and the step with the largest sum
is taken. When that step points, each voter scores every word of the sentence on the stack by
its embedding through `point_weights` against its hidden units, those scores take a softmax over
the words, the shares are added, and the word with the largest sum is the one the step takes.

The console keeps the same numbers as one weights file with a classes file beside it (the same
name with `.json` added). The weights file holds the voters one after another. Each voter is, in
order: for each place of its `items`, the place as a `u64` followed by its row of `item_table`;
then the bytes of `slot_weights`, `fill_weights`, `hidden_bias`, `output_weights`, `output_bias`
and `point_weights`. The classes file holds `bits`, `half`, `classes`, `voters`, and `stacked`
with `slots`, `item`, `hidden`, `items` and `pointer`. A model is made from full-width numbers:
`dam1 export` refuses a weights file whose classes file says `half`. A loader in Rust rebuilds
these two files from the folder and reads them with `dam::network::Stacked::read_voters`, as the
console and the chat page do.

## state.bin

The tree after the seeds files `build.state` names were told into a fresh mind in name order, as
the bytes the console's state command writes and the chat page loads. With every seeds file it
holds what the model knows before anyone speaks: the number words, the measures, the verb forms,
the opposites and the rest of the seeds. The word network is taught on that state, so a model is
exported with every seeds file. With `none` the state is an empty tree. It is a cache:
`dam1 export` makes it from the seeds, and `dam1 check` makes it again from the record and
confirms it byte for byte. Text is split into tokens by `dam::words::simple_words`; there is no
tokenizer file.

## Loading rules

A loader following this layout must:

1. Refuse a folder whose `architecture` or `format_version` it does not know.
2. Refuse a folder whose `network.bits` is not the width its engine hashes features into.
3. Refuse a class that names no step of its engine.
4. Refuse a weights file that holds a tensor this layout does not name, or lacks one it names for
   any voter.
5. Refuse a tensor whose type is not the one above, or whose shape is not exactly the one the
   table gives from the `network` fields of `config.json`.
6. Refuse a `network.N.items` that is not in rising order with each place once.
7. Refuse a `state.bin` its engine cannot load.

## Versions

Any change that a loader of the current version would misread raises `format_version`.

Version 5 replaced the cursor network of version 4, which read a whole sentence as one input and
walked a cursor over the tree, with the word network: each word is one input, a step is a record
with one pointer at a word of the sentence, and several networks of one shape may read by vote.
The tensors took the voter's number into their names, `network.voters` joined the config, and
`reading.reach` and `reading.input_words` left it. A loader of version 5 refuses a version 4
folder.

Version 4 replaced the reader of version 3 (word values, choice costs, programs, a vocabulary and
a seeded memory, read with an event-stack network) with the cursor network alone. Feature places
went from 32 to 64 bits, and the pointer's weights joined the tensors.
