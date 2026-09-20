# The world lessons

Phase 1: the network shapes the world one word at a time. Statements only, no questions yet.

The world is a space. A thing inside a place is a child of it: `corn in bag` leaves `bag -> corn`. What a thing
has stands inside it too: `tom has 3 apples` leaves `tom -> apple:quantity(3)`.

Each word is a fact before the network moves. A thing word makes the thing appear, the known one of its name or a
new one. The network's moves then do what the facts leave to do:

- `{grab}` holds the thing the cursor stands on, for a place word or a moving verb.
- `{drop @}` puts the held thing inside the thing that appeared. After `{give}`, it puts the thing that appeared
  inside the held one.
- `{setFlag type: the value: true}` and the other flags are taken by the next thing to appear: the article, a
  count, a quality said before its noun, the past after `there was`.
- `{children add: @ type: property}` adds `is`, with `time: past` for `was`. `{setProperty color: @}` sets a
  quality under it by the kind the seeds give it.
- `{step newest}`, `{step parent}`, `{point to: {nothing}}` and `{continue}`.

A tag is written after its name: `cat:the(true)`, `{is}:time(past)`, `apple:quantity(3)`. A line passes when the
world holds every path it expects and none it denies with `not`.

Each file holds one case and the same case in made-up names as its held-out line. Teach, train and report with:

```
dam word teach --quiz <file> --out <rows> --state all
dam word train --rows <rows> --out <network> --card --slices 192 --enough 1 --epochs 2000 --patience 300 --limit 300
dam word report --quiz <file> --network <network> --state all
```
