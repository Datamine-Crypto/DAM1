# The debug lessons

One case per file, in the quiz notation: the text, the statements
its tree should hold after the arrow, and its questions after the bar. Each file also holds the same case with
names no lesson and no seed says, marked `test:`, so a pass there is the pattern and never the word.

The rules the cases stand on:

- The stack is emptied at every word. It starts with the cursor, where you are, where you are pointing to and
  the flags you carry, `{cursor object: go type: movement pointingTo: {nothing} flags: the}`, then the word, then
  the steps taken for it.
- A step is a record: `{find name: corn orderBy: lastMentioned}`, `{children add: in type: location}`,
  `{children add: bag}`, `{children add: is type: property}`, `{children add: go type: movement time: past}`,
  `{children add: what type: question}`, `{setFlag type: the value: true}`, `{point to: {nothing}}`,
  `{answer}`, `{continue}`.
- Adding a node moves the cursor to it. A value that names a thing is a mention linked to it. A thing stands in
  one place, the newest, and the place it left keeps it with the time past. A question is a tree built like a statement under its question node, and the answer matches it
  against the story. A word before its head, `the`, `a`, a count, an adjective, sets a flag the next find or add
  uses.

The cases:

1. `001_corn_in_bag`
2. `002_cat_in_box`
3. `003_car_is_red`
4. `004_one_bag`: one bag named twice
5. `005_mary_went`: a moving verb with its tense
6. `006_mary_went_twice`: places kept, the newest is where she is
7. `007_ball_moved`: the same with a place word
8. `008_what_color`
9. `009_what_is_in`: what is in, and where
10. `010_the_plim`: the flag `the`
11. `011_a_plim`: the flag `a`
12. `012_three_red_apples`: the count and the adjective flags

No `seed:` lines: the runs read every seed file through `--state all`, which the word reading uses for two things
only, the verb forms and the kinds of the qualities.

Teach and report a file with `dam word teach --quiz <file> --out <rows> --state all` and
`dam word report --quiz <file> --network <network> --state all`.
