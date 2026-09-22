# 📊 Benchmarks

DAM1 beside small language models on the same questions, with one rule for every reply.

## What is measured

Every question of a held-out line of the curriculum: 2,673 questions over 16 lesson folders, from
the basics to university. A held-out line is a line marked `test:` in its lesson, which no network
was trained on. Each item is the text of the line, the question, and the answer the lesson wants.

Each model gets the same text and the same question. The other models also get an order and two
worked examples, so the shape of an answer is stated and not guessed. DAM1 gets neither: it reads
the sentences of the text, one turn each, and then the question, as a person types them into the
chat page.

The item set is DAM1's own curriculum. Its lines were held out of training, so the words, the names
and the numbers are new, but the sentence shapes are shapes DAM1 was taught. It measures reading of
a shape it knows, not general knowledge, and DAM1 is at home here in a way the other models are not.
Read the table for what a 5.6 million number reader can do on the reading it was built for, not as a
ranking of language models.

## The rule

One rule for every model, in `score.py`. The reply is put in lower case, its marks are taken off, a
leading article and an opener like "the answer is" are dropped, and a number counts in digits or in
words. The reply is right when the answer is in it. An answer of several parts needs every part, in
any order. An empty reply is right only when the answer is nothing. The table also gives the exact
share, where the reply must begin with the answer and add nothing before it.

DAM1 is measured twice, by two paths. `dam1.mjs` asks the browser build as a person does, and it
answers 2,669 of the 2,673. Reading the same lines with the console, `dam word report` from the
state the model ships, answers 3,113 of the 3,116 questions it counts, which include the chat turns
the benchmark leaves out. The other models were run when the curriculum held 2,669 questions.

## Run it

```sh
node benchmarks/items.mjs                                  # writes out/items.jsonl
node benchmarks/dam1.mjs                                   # the build the browser runs
python benchmarks/ask.py Qwen/Qwen2.5-0.5B-Instruct qwen25-05b
python benchmarks/ask.py Qwen/Qwen3-0.6B qwen3-06b
python benchmarks/ask.py HuggingFaceTB/SmolLM2-360M-Instruct smollm2-360m
python benchmarks/ask.py HuggingFaceTB/SmolLM2-135M-Instruct smollm2-135m
python benchmarks/ask.py LiquidAI/LFM2-350M lfm2-350m
python benchmarks/table.py                                 # the table, and the shares by lessons
```

`dam1.mjs` needs the chat's built engine in `chat/src/wasm` and its files in `chat/public`, which
`scripts/build-chat.sh` writes. `ask.py` needs `torch` and `transformers`, and a card with about 4 GB
free. Everything is written to `benchmarks/out`, which is not committed.

## How each model was run

| | |
|---|---|
| DAM1 | the WebAssembly build, on the processor, one conversation an item |
| the others | float16 on an NVIDIA RTX 3080 Ti, greedy, at most 32 new tokens, 16 items a batch |

The time in the table is the middle of 60 answers taken one at a time, the wait for one question and
not the throughput of a batch. DAM1's time is on the processor; the other times are on the card.

The weights are the file the hub publishes, in the type it publishes. DAM1's page downloads the same
network written small, since the page serves it gzipped. The memory is what one answer takes: for
DAM1 the one block of WebAssembly memory it works in, which holds the network, the state, the tree
and the stack, and for the others the most the card held over the same one at a time pass.
