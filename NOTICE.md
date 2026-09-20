# Notice

Copyright (C) 2026 Datamine Network Inc.

This program is free software: you can redistribute it and/or modify it under the terms of the
GNU Affero General Public License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
Affero General Public License in [LICENSE](LICENSE) for more details.

## What the licence covers

- The code: the Rust crates under `model/crates/`, the page under `chat/`, the scripts under
  `scripts/`, and the documents in this repository.
- The quiz, the curriculum, the facts and the seeds, which are the source the weights are compiled
  from: `model/data/quiz/`, `model/data/train/`, `model/data/facts/` and `model/data/seeds/`.
- The weights: every cursor network (its weights file and the classes file beside it) and the
  page's `network.bin`, `network.json` and `seeds.txt` built from this repository, with or without a
  notice in the file. A release of weights names the commit it was built from, so the source it was
  compiled from is the repository at that commit.

The licence applies per file only where a file says otherwise. No file in this repository does.

## What it does not cover

- The names. "Datamine Network", "DAM1" and the Datamine logo are the names and mark of Datamine
  Network Inc. Under section 7(e) of the licence, no right is granted to use them. You may say that your
  work is derived from DAM1. You may not call a modified version DAM1, or present it as coming
  from Datamine Network.

## Running it for others

Section 13 of the licence: if you run a modified version and users interact with it through a
network, you must offer those users the source of your version. Serving the page from a site is
conveying it, so the site must also make its source available, as section 6 says. The unmodified
page satisfies this by pointing at this repository and the commit it was built from.

## No notice in the Rust files

The Rust is written under Premise, which forbids comments: the checker fails the build on one.
The per-file header that the licence suggests is therefore not in the Rust files. This file and
`LICENSE` carry the notice for every file in the repository.

## Contributions

A contribution to this repository is accepted under the same licence, with no separate agreement.
