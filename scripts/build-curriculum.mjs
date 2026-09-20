// Build the curriculum: every grade folder of model/data/train from the quiz files of model/data/quiz
// the grades list names, each converted to the statements the cursor tree holds. A file of many lines
// is written in parts. Usage: node scripts/build-curriculum.mjs [grade folder] [--overwrite]
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync, rmSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const model = fileURLToPath(new URL('../model/', import.meta.url));
const quiz = model + 'data/quiz/';
const train = model + 'data/train/';

export const grades = {
  '002_grade2': ['00-base', '10-places', '40-time-and-owning', '60-possessive', '62-classes', '66-days', '68-greetings', '70-indirect-questions', '72-negatives', '74-have-questions', '90-our-and-their', '92-family', '94-animals-and-sounds', '96-shapes-and-sides', '98-weather-and-seasons', '99-an', '99-plural-counts', '99-my-things', '99-them', '99-whose', '99-possessive', '99-letters', '99-vowels'],
  '003_grade3': ['20-arithmetic-words', '99-number-after', '99-number-between', '99-counting-between', '99-successor', '99-odd-and-even', '99-tens-and-hundreds', '99-place-value', '99-greater-less', '99-more-and-fewer', '99-more-less-than', '99-biggest-number', '99-totals', '99-counts-the', '99-counts-over-two-owners', 'next-counting', '99-season-after', '99-day-after', '99-month-after', '99-letter-after', '99-alphabetical-order', '99-ordinal-days', '88-order-and-time-words', '99-order-in-a-line'],
  '004_grade4': ['76-tell-me-about', '78-comparisons', '80-more-verbs', '82-more-qualities', '84-opposites-and-sameness', '99-doing-verbs', '99-verbs', '99-verbs-more', '99-verb-objects', '99-saw', '99-giving', '99-giving-who', '99-moving-and-carrying', '99-moving-again', '99-picking-up', '99-dropping', '99-carried', '99-carried-before', '99-counting-carried', '99-before', '99-after', '99-they', '99-earlier-later', '99-longer-heavier', '99-hotter-colder', '99-inverse-comparisons', '99-opposite-both-ways', '99-relation-words'],
  '005_grade5': ['64-costs', '99-coins', '99-dollars-and-cents', '99-costs-together', '99-change', '99-money-left-after-buying', '99-how-many-can-buy', '99-unit-price', '99-clock', '99-hours-between', '99-days-apart', '99-months-apart', '99-durations', '99-parts-of-an-hour', '99-mixed-time-units', '99-today-from-neighbours', '99-dozens-and-pairs', '99-half-and-double', '99-ages', '99-amounts-left'],
  '006_grade6': ['99-each', '99-sharing', '99-full-groups', '99-left-over-in-groups', '99-remainders', '99-multiples-and-factors', '99-fits-in', '99-units', '99-metric-units', '99-mixed-metric-units', '99-units-back', '99-scaling-amounts', '99-two-steps', '99-missing-number', '99-sequences', '99-digit-sums', '99-word-count'],
  '007_grade7': ['99-fractions', '99-comparing-fractions', '99-percent', '99-discount', '99-percent-increase', '99-area-and-perimeter', '99-triangle-perimeter', '99-roman-numerals', '50-rounding', '99-squares-and-cubes', '99-square-roots'],
  '008_grade8': ['99-averages', '99-median', '99-mode', '99-range', '99-speed-distance', '99-travel-time', '99-unknowns', '30-functions'],
  '009_grade9': ['86-capitals-and-materials', '99-titles-and-languages', '99-kind-words', '99-kinds-do', '99-kinds-with-a-quality', '99-inherit-kinds', '99-superlative-kinds', '99-where-countries-are'],
  '010_grade10': ['99-directions', '99-positions', '99-paths', '99-far-end', '99-kinship', '99-grandparents'],
  '011_grade11': ['99-deduction', '99-induction', '99-motives', '99-choices', '99-which-not', '99-absent', '99-yes-no-after-place', '99-many-questions', '99-colour-final', '99-plurals', '99-number-words'],
  '012_grade12': ['grade12'],
  '013_university': ['university'],
};

const relation = new Set(['in', 'owns', 'is', 'color', 'size', 'day']);
const bare = (answer, question = '') =>
  answer
    .split(', ')
    .map((part) => part.replace(/^(-?\d+(?:\.\d+)?) (?:degrees|o'clock)$/, '$1'))
    .map((part) => (/^[nsew](,[nsew])*$/.test(part) && /\bgo from\b/.test(question) ? part.split(',').map((d) => ({ n: 'north', s: 'south', e: 'east', w: 'west' })[d]).join('->') : part))
    .map((part) => part.replace(/^([a-z0-9]+) (?:is in|is|has|likes) ([a-z0-9]+)$/, (all, first, second) => (question.split(/[^a-z0-9]+/).includes(second) ? first : second)))
    .map((part) => (part.startsWith('not ') || !part.includes('->') ? part : part.split('->').filter((t) => !relation.has(t)).join('->') || part))
    .join(', ');

// A count told as two statements (tom owns apples; apples is 7) holds on any node named apples, so a
// text may share one node between owners. Each count becomes one path under its own owner instead.
// The listed order is not the text order (a line may list ann before tom), so an owner takes the count
// its text says after the owner's name and before the thing's name (tom has 3 apples). A thing where any
// owner finds no such count, or two owners find the same one, stays as it was.
const counted = (own, text) => {
  const words = text.toLowerCase().match(/[a-z]+|-?\d+(?:\.\d+)?/g) || [];
  const things = new Map();
  own.forEach((s, i) => {
    const owns = s.match(/^(\S+) owns (\S+)$/);
    const count = s.match(/^(\S+) is (-?\d[\d.,]*)$/);
    const thing = owns ? owns[2] : count ? count[1] : null;
    if (!thing) return;
    if (!things.has(thing)) things.set(thing, { owners: [], counts: [] });
    if (owns) things.get(thing).owners.push([i, owns[1]]);
    else things.get(thing).counts.push([i, count[2]]);
  });
  const said = (owner, thing, counts) => {
    for (let at = words.indexOf(owner); at >= 0; at = words.indexOf(owner, at + 1)) {
      for (let k = at + 1; k < words.length && words[k] !== thing && words[k] !== owner; k++) {
        const hit = counts.find(([, n]) => n === words[k]);
        if (hit) return hit;
      }
    }
    return null;
  };
  const out = [...own];
  const dropped = new Set();
  for (const [thing, { owners, counts }] of things) {
    if (!counts.length || owners.length !== counts.length) continue;
    const pairs = owners.map(([i, owner]) => [i, owner, said(owner, thing, counts)]);
    if (pairs.some(([, , hit]) => !hit) || new Set(pairs.map(([, , hit]) => hit[0])).size !== pairs.length) continue;
    for (const [i, owner, [j, n]] of pairs) {
      out[i] = `${owner} -> owns -> ${thing} -> is -> ${n}`;
      dropped.add(j);
    }
  }
  return out.filter((_, i) => !dropped.has(i));
};
// A text that says who gave a thing to whom keeps only the new owner in its statements, so no question
// about the giver has anything in the tree to find. Each giving is kept as a path too, listed just before
// the owning it causes (or last), which is where the text says it.
const given = (own, text) => {
  const out = [...own];
  for (const m of text.toLowerCase().matchAll(/\b([a-z]+) gave (?:the |a |an )?([a-z]+) to ([a-z]+)\b/g)) {
    const path = `${m[1]} -> gave -> ${m[2]} -> to -> ${m[3]}`;
    if (out.includes(path) || m.slice(1).some((w) => /^(he|she|they|it|them|him|her|we|us|you|i|me)$/.test(w))) continue;
    const owning = out.indexOf(`${m[3]} owns ${m[2]}`);
    out.splice(owning < 0 ? out.length : owning, 0, path);
  }
  return out;
};
// A story that moves people keeps only where each one is now, so a question about where someone or
// something was before has nothing in the tree to find. The text is walked one sentence at a time (he,
// she and they stand for the last person named; then and after that are dropped); each person's places
// and each carried thing's places are kept, and a statement of where a person is now becomes the path of
// every place, newest first (fred -> in -> school -> before -> bedroom). A carried thing that was in two
// places or more gets its own path, listed after its owning. A sentence with a moving or carrying word in
// a shape not read here leaves the line as it was. A move may open with a time word (this morning fred
// went ...); when every move of a person names its time, that person's places are ordered by the times,
// yesterday first and this evening last, not by the telling. They stands for the last group named
// (john and daniel), he and she for the last person.
const TIMES = ['yesterday', 'this morning', 'this afternoon', 'this evening'];
const MOVES = '(?:went|moved|journeyed|travelled|traveled|ran|walked)';
const placed = (own, text) => {
  const sentences = text.toLowerCase().replace(/^(test: )?((then|turn): )?/, '').split(/[.?!]/).map((s) => s.trim()).filter(Boolean);
  const places = new Map();
  const carried = new Map();
  const thingPlaces = new Map();
  const times = new Map();
  const push = (map, key, place, time = -1) => {
    if (!map.has(key)) map.set(key, []);
    const list = map.get(key);
    if (list[list.length - 1] !== place) {
      list.push(place);
      if (map === places) {
        if (!times.has(key)) times.set(key, []);
        times.get(key).push(time);
      }
    }
  };
  let last = null;
  let group = null;
  for (const raw of sentences) {
    let opened = raw.replace(/^(then|after that|afterwards|following that) /, '');
    const time = TIMES.findIndex((t) => opened.startsWith(t + ' '));
    if (time >= 0) opened = opened.slice(TIMES[time].length + 1);
    const s = opened.replace(/^(he|she) /, () => (last ? last + ' ' : '§ ')).replace(/^they /, () => (group || last ? (group || last) + ' ' : '§ '));
    let m;
    if ((m = s.match(new RegExp(`^([a-z]+(?: and [a-z]+)*) ${MOVES} (?:back )?to (?:the )?([a-z]+)$`)))) {
      for (const person of m[1].split(' and ')) {
        push(places, person, m[2], time);
        for (const thing of carried.get(person) || []) push(thingPlaces, thing, m[2]);
      }
    } else if ((m = s.match(/^([a-z]+) (?:picked up|grabbed|took|got) (?:the |a |an )?([a-z]+)(?: there)?$/))) {
      if (!carried.has(m[1])) carried.set(m[1], new Set());
      carried.get(m[1]).add(m[2]);
      const at = places.get(m[1]);
      if (at) push(thingPlaces, m[2], at[at.length - 1]);
    } else if ((m = s.match(/^([a-z]+) (?:dropped|put down|discarded|left) (?:the |a |an )?([a-z]+)(?: there)?$/))) {
      carried.get(m[1])?.delete(m[2]);
    } else if ((m = s.match(/^([a-z]+) (?:gave|handed|passed) (?:the |a |an )?([a-z]+) to ([a-z]+)$/))) {
      carried.get(m[1])?.delete(m[2]);
      if (!carried.has(m[3])) carried.set(m[3], new Set());
      carried.get(m[3]).add(m[2]);
    } else if (new RegExp(`\\b(${MOVES.slice(3, -1)}|picked|grabbed|took|got|dropped|put|discarded|left|gave|handed|passed)\\b`).test(s)) {
      return own;
    }
    if (m && m[1] !== '§') {
      last = m[1];
      if (m[1].includes(' and ')) group = m[1];
    }
    if (s.startsWith('§')) return own;
  }
  const out = [...own];
  const chain = (key, list) => `${key} -> in -> ${[...list].reverse().join(' -> before -> ')}`;
  for (const [person, told] of places) {
    const when = times.get(person) || [];
    const list = when.length === told.length && when.every((t) => t >= 0) ? told.map((p, i) => [p, when[i], i]).sort((a, b) => a[1] - b[1] || a[2] - b[2]).map(([p]) => p) : told;
    if (list.length < 2) continue;
    const at = out.findIndex((st) => new RegExp(`^${person} in [a-z]+$`).test(st));
    if (at >= 0) out[at] = chain(person, list);
  }
  for (const [thing, list] of thingPlaces) {
    if (list.length < 2) continue;
    const owning = out.findIndex((st) => new RegExp(`^[a-z]+ owns ${thing}$`).test(st));
    out.splice(owning < 0 ? out.length : owning + 1, 0, chain(thing, list));
  }
  return out;
};
const ownsOf = (s) => s.match(/^(\S+) owns (\S+)$/) || s.match(/^(\S+) -> owns -> (\S+)(?: -> .*)?$/);
const placeOf = (s) => s.match(/^(\S+) in (\S+)$/) || s.match(/^(\S+) -> in -> (\S+)(?: -> .*)?$/);

function converted(source) {
  let last = [];
  // A statement of what a thing did (pen: broke, bike: broke down) is no shape the tree takes, so the
  // question what broke has nothing to find: it is kept as the thing and the verb under a did relation.
  const did = (s) => s.replace(/^([a-z]+): ([a-z]+)(?: [a-z]+)*$/, '$1 did $2');
  const statements = (expect, text) => counted(expect.split(';').map((s) => did(s.trim())).filter(Boolean), text);
  return source.split(/\r?\n/)
    .map((line) => {
      const arrow = line.indexOf(' => ');
      if (line.startsWith('#') || arrow < 0) return line;
      const bar = line.indexOf(' | ', arrow);
      const end = bar < 0 ? line.length : bar;
      const said = line.slice(0, arrow);
      const own = placed(given(statements(line.slice(arrow + 4, end), said), said), said);
      let head = own.length ? line.slice(0, arrow + 4) + own.join('; ') : line.slice(0, end);
      if (/^(test: )?then: /.test(line)) {
        const added = [];
        const stillOwns = (owner, thing) => own.some((st) => { const o = ownsOf(st); return o && o[1] === owner && o[2] === thing; });
        for (const s of own) {
          const owns = ownsOf(s);
          const place = placeOf(s);
          for (const old of last) {
            const oldOwns = ownsOf(old);
            const oldPlace = placeOf(old);
            if (owns && oldOwns && oldOwns[2] === owns[2] && oldOwns[1] !== owns[1] && !stillOwns(oldOwns[1], oldOwns[2])) added.push(`not ${oldOwns[1]} owns ${oldOwns[2]}`);
            if (place && oldPlace && oldPlace[1] === place[1] && oldPlace[2] !== place[2]) added.push(`not ${oldPlace[1]} in ${oldPlace[2]}`);
          }
        }
        const fresh = added.filter((a) => !own.includes(a));
        if (fresh.length) head += '; ' + fresh.join('; ');
        last = [...last.filter((old) => !fresh.includes(`not ${old}`)), ...own.filter((s) => !s.startsWith('not '))];
      } else {
        last = own;
      }
      if (bar < 0) return head;
      const tail = line.slice(bar + 3);
      const turn = /^(test: )?turn: /.test(line);
      const answers = turn
        ? bare(tail, line.slice(0, arrow))
        : tail
            .split('; ')
            .map((qa) => {
              const eq = qa.lastIndexOf(' = ');
              return eq < 0 ? qa : qa.slice(0, eq + 3) + bare(qa.slice(eq + 3), qa.slice(0, eq));
            })
            .join('; ');
      return head + ' | ' + answers;
    })
    .join('\n');
}

// The lessons in model/data/train are edited in place since the word reading: answers follow the seeds,
// shapes follow the world, and whole lessons were added there. Building a grade again would throw those
// edits away, so a grade folder that exists is left alone unless --overwrite is given.
const flags = process.argv.slice(2).filter((arg) => arg.startsWith('--'));
const only = process.argv.slice(2).find((arg) => !arg.startsWith('--'));
const overwrite = flags.includes('--overwrite');
let copied = 0;
for (const [grade, files] of Object.entries(grades)) {
  if (only && grade !== only) continue;
  const folder = train + grade + '/';
  if (existsSync(folder) && !overwrite) {
    console.log(`${grade}: kept, the folder holds edited lessons (give --overwrite to build it again from the quiz)`);
    continue;
  }
  if (existsSync(folder)) rmSync(folder, { recursive: true });
  mkdirSync(folder, { recursive: true });
  files.forEach((name, i) => {
    const source = quiz + name + '.txt';
    if (!existsSync(source)) throw new Error('missing ' + source);
    // A file of many lines takes longer to teach than one teaching run is given, so it is written in
    // parts of about PART_LINES lines each, cut only before a line that does not carry on from the one
    // above (then: lines and the lines under a seed stay with what they need).
    const PART_LINES = 60;
    // Seeds a quiz file needs on the cursor that its source does not name: the facts its questions look up.
    const EXTRA_SEEDS = { '99-odd-and-even': 'parity' };
    const sourced = readFileSync(source, 'utf8');
    const whole = converted(EXTRA_SEEDS[name] ? `seed: ${EXTRA_SEEDS[name]}\n${sourced}` : sourced).split('\n');
    const parts = [[]];
    let seed = null;
    let taught = 0;
    for (const line of whole) {
      if (/^seed:/.test(line)) seed = line;
      const starts = / => /.test(line) && !/^(test: )?then: /.test(line);
      if (starts && taught >= PART_LINES) {
        parts.push(seed && !/^seed:/.test(line) ? [seed] : []);
        taught = 0;
      }
      parts[parts.length - 1].push(line);
      if (/ => /.test(line)) taught++;
    }
    const base = `${folder}${String(i + 1).padStart(3, '0')}_${name.replace(/^\d+-/, '')}`;
    parts.forEach((part, p) => {
      writeFileSync(parts.length > 1 ? `${base}-part${p + 1}.txt` : `${base}.txt`, part.join('\n'));
      copied++;
    });
  });
  console.log(`${grade}: ${files.length} files`);
}
const listed = Object.values(grades).flat();
const unused = readdirSync(quiz).filter((f) => f.endsWith('.txt')).map((f) => f.slice(0, -4)).filter((n) => !listed.includes(n));
console.log(`copied ${copied} files; quiz files in no grade: ${unused.join(', ') || 'none'}`);
