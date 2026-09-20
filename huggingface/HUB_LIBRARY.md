# Registering dam1 as a Hub library

The model repository `DatamineNetwork/DAM1` already names its library in the card:
`library_name: dam1`. Registering the library adds four things to the model page, as the Hub docs
say (https://huggingface.co/docs/hub/models-adding-libraries):

- the label `DAM1` in place of `dam1`;
- a link to the library's source repository;
- a code snippet under "Use this model";
- a download count rule of our own. This one is not needed: the folder holds `config.json`, which
  the Hub counts by default.

Registration is a pull request to `huggingface/huggingface.js`. It is optional. Nothing in the
release depends on it.

## Before opening the pull request

1. The model `DatamineNetwork/DAM1` is public.
2. It appears at https://huggingface.co/models?other=dam1. The Hub asks for at least one model
   there before a library is registered.
3. The source repository https://github.com/Datamine-Crypto/DAM1 is public, since the entry links
   to it.
4. The commands in the snippet work from a clean checkout of that repository.

## The change

Fork `huggingface/huggingface.js`, then edit two files in `packages/tasks/src/`.

### model-libraries.ts

Add this entry to `MODEL_LIBRARIES_UI_ELEMENTS`. Keep the keys in alphabetical order: check the
keys around `dam1` on the day, since the file changes often.

```ts
dam1: {
	prettyLabel: "DAM1",
	repoName: "DAM1",
	repoUrl: "https://github.com/Datamine-Crypto/DAM1",
	snippets: snippets.dam1,
	filter: false,
},
```

The docs ask for `filter: false`. The docs live in the source repository, so `docsUrl` is left
out. The default count on `config.json` is right, so `countDownloads` is left out.

### model-libraries-snippets.ts

Add this function, in alphabetical order with the others:

```ts
export const dam1 = (model: ModelData): string[] => [
	`# In a checkout of https://github.com/Datamine-Crypto/DAM1
cd huggingface && cargo build --release
target/release/dam1 talk --repo ${model.id} "tom has a red car." "what color is tom's car?"`,
	`// Cargo.toml: dam1, dam1-hub and dam-page, each from git = "https://github.com/Datamine-Crypto/DAM1"
fn main() -> Result<(), String> {
    let folder = dam1_hub::hub::pulled("${model.id}", "main")?;
    let model = dam1::model::loaded(&folder)?;
    dam_page::load(&model.network.weights, &model.network.classes)?;
    dam_page::state(&model.state)?;
    dam_page::read("tom has a red car.", model.reading.steps)?;
    println!("{}", dam_page::read("what color is tom's car?", model.reading.steps)?);
    Ok(())
}`,
];
```

The snippet uses the same commands and names as the Use section of `card/README.md`. If the card's
commands change, change the snippet before opening the pull request.

## The pull request

- Title: Add DAM1 library.
- Body: DAM1 reads English one word at a time into a tree and answers questions from it. Small
  networks over a stack choose by vote every step that shapes and reads the tree. It runs in Rust and
  WebAssembly. Model:
  https://huggingface.co/DatamineNetwork/DAM1. Source: https://github.com/Datamine-Crypto/DAM1.
- Run the package's lint and tests as its contributing guide says before pushing.
