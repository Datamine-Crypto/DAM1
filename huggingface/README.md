# 🤗 DAM1 on Hugging Face

[![Publish model](https://github.com/Datamine-Crypto/DAM1/actions/workflows/publish-model.yml/badge.svg)](https://github.com/Datamine-Crypto/DAM1/actions/workflows/publish-model.yml)
[![Model on Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97%20Hugging%20Face-DAM1-yellow)](https://huggingface.co/DatamineNetwork/DAM1)

This folder turns the word network the console trains, one network or several that read by vote,
with the permanent state the seeds make,
into a model on the Hugging Face hub, and reads it back. The model repository is
https://huggingface.co/DatamineNetwork/DAM1. It is not public yet.

| Path | What it is |
|---|---|
| [FORMAT.md](FORMAT.md) | The model layout, version 5: every file, tensor and loading rule |
| `card/README.md` | The model card, copied into the model folder as its `README.md` |
| `crates/dam1` | The layout: `written` saves a model folder, `loaded` reads one, `check` compares it with its source |
| `crates/dam1-hub` | The tool's commands and the hub download, through hf-hub |
| `crates/dam1-cli` | The `dam1` binary, with the spec's settings wired in, and the round-trip tests |
| `Cargo.toml`, `premise.zones` | This folder's own workspace and its Premise zones |
| `model/` | The exported folder, written by `dam1 export` and ignored by git |

The three crates are a workspace of their own, beside the engine's workspace in `model/`. Under
Premise, `dam1` and `dam1-hub` are patterns crates and `dam1-cli` is an app crate. Because the engine
crates they depend on live in another workspace, the checker here reports those dependencies as
outside every zone, along with the diagnostics that follow from that. This was chosen to keep the
engine's workspace to itself.

## 📤 Export, check, publish

Build here, then export and check from `model/`, where the engine's data lives.

```sh
cd huggingface && cargo build --release && cd ../model
../huggingface/target/release/dam1 export --network NETWORK.bin --out ../huggingface/model --commit "$(git rev-parse HEAD)" --seeds none
../huggingface/target/release/dam1 check --model ../huggingface/model --network NETWORK.bin
cp ../huggingface/card/README.md ../LICENSE ../NOTICE.md ../huggingface/model/
hf upload DatamineNetwork/DAM1 ../huggingface/model
```

The same steps run on their own in the workflow `.github/workflows/publish-model.yml`: every push
to `main` builds the tool, runs its tests, exports the release network at
`model/data/network/release.bin`, checks the folder against its sources, grades it, adds the card,
the licence and the notice, and uploads the folder to the hub as one commit. A tag `v*` also tags
that version on the hub. So a new version is published by joining the trained networks that vote with
`scripts/join-networks.mjs` at full width (`HALF=0`) into `model/data/network/release.bin` and
`release.bin.json`, filling the card from the
check, and pushing. The workflow needs the secret `HF_TOKEN`, a write token of the hub account,
and takes the variable `HF_REPO` when the repository is not `DatamineNetwork/DAM1`. The chat
workflows build from the same network file.

`--network` names the weights file `dam word train` wrote, or the file the join made of several. The file with the same name and
`.json` added lists its step classes and its shape. `export` loads the network with dam-core's own
loader, refuses a class that names no step, makes the state from the seeds `--seeds` names (`none`
for an empty tree, names of seeds files joined by commas, or every file of `data/seeds` when the
option is not given), records that choice in the config, and writes the three files of the layout
with the step limit of the spec. It refuses weights written half as wide, since a model is made
from the full numbers. The seeds are found from the current folder, so start the tool under
`model/`. The release ships with every seeds file: the word network is taught on the state all
seeds make, and the chat starts from the same state.

`check` is the gate before a publish. It loads the folder and compares it with the network and the
seeds it came from:

- every tensor of `model.safetensors`, byte for byte, against the tensors the source files make;
- the network's feature width, step classes, shape and count of voters, and the weights file the
  folder's tensors make against the source's;
- the step limit against the spec the tool was built with;
- the state against the seeds told again.

It then reads every quiz file under `data/train` (or the folder `--quiz` names) with the model,
by the console's own reading of a line: a line is right when every word of every input continued
within the step limit and the tree holds its shape or the output says its answer. It prints the score per file, learned lines and held-out lines apart, and
fails on any difference from the source.

The round-trip tests need no data. Run them from this folder:

```sh
cargo test --release
```

They cover bit-exact save and load with and without a pointer, including NaN, infinity and
negative zero, a plain safetensors reading of the eight tensors and the config's shape, identical
chat turns before and after, the check finding one changed byte, a changed state found and a
broken one refused, and the refusals: another architecture or layout version, a class that is no
step, a shape that does not fit its tensors, places out of rising order, a tensor the layout does
not name, and a network over another feature width.

## 🛠️ Use

From the command line, with a signed-in Hugging Face token for a private repository:

```sh
huggingface/target/release/dam1 pull --repo DatamineNetwork/DAM1
huggingface/target/release/dam1 talk --repo DatamineNetwork/DAM1 "tom has a red car." "what color is tom's car?"
huggingface/target/release/dam1 talk --model huggingface/model "what is 2 + 3?"
```

`talk` says each text as one turn of one conversation. It prints each reply as JSON: `output` is
what the model said, `steps` is every step the network chose, and `ended` says whether the reading
continued within the step limit.

From Rust, with the crates taken from the GitHub repository:

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

`dam_page` is the engine the chat page runs, compiled natively. It holds one conversation: `read`
reads a turn on the mind the turns before it left, `forget` takes the mind back to the state, and
`context` and `restore` save and load a conversation.

## ✅ Release checklist

Checked against the Hugging Face model release checklist
(https://huggingface.co/docs/hub/model-release-checklist), for layout version 5.

Done in the card: metadata (`license`, `language`, `library_name`, `pipeline_tag`, `tags`), model
details, intended and out-of-scope uses, training data and procedure, compute, evaluation and how
it was measured, limitations and risks, citation, contact. Weights are in safetensors.
`base_model` and `datasets` are not given: the model is trained from nothing, and its data is not
on the hub. The quiz scores are not in `.eval_results`, because that format needs a Hub benchmark
dataset.

`pipeline_tag: question-answering` is the nearest task on the hub list. The hub's widget for it
sends a question and a context, and no inference provider runs `dam1`, so the model page has no
widget. `library_name: dam1` is not a registered library. The hub still shows it and filters by
it. Registration is a pull request to `model-libraries.ts` in huggingface.js, after one public
model names the library: see [HUB_LIBRARY.md](HUB_LIBRARY.md). Downloads are counted on
`config.json` without it.

Open:

| Item | Needs | Owner |
|---|---|---|
| Source repository | The new public repository. Then replace every `github.com/Datamine-Crypto/DAM1` link in the card, this file, the crates' manifests and the Rust dependency snippets. | User |
| Hub secret | Add `HF_TOKEN`, a write token of the hub account, to the source repository's secrets, so the publish workflow can upload. Then make the model public when ready, and test the card's commands and Rust snippet from a clean checkout. | User |
| Hub settings | Gated access (not needed for AGPL weights unless the user wants it), no inference providers, a Space demo that pulls from the hub if wanted, a collection if variants follow. | User |
| Library registration | Optional pull request to register `dam1`, after the model is public. | User |
| Emissions | The place and the power drawn were not recorded, so `co2_eq_emissions` is not given. | Later work |
| Notebook | Not applicable: the model runs in Rust, not in Colab or Kaggle. | None |

## 🕯️ Why not Candle

Candle runs tensor graphs. In DAM1 the network only chooses the next step; the steps, the tree and
the stack are the engine. A Candle port would run the choice and nothing else. The folder uses the
hub's own formats instead: safetensors for the numbers, JSON for the rest, and hf-hub for the
download. Those are the parts of the Hugging Face ecosystem that Candle itself loads models with.
