import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const model = fileURLToPath(new URL('../model/', import.meta.url));

const article = '(?:the |a |an )?';
// The kinds a thing is classified by: subject, kind, value, written as one path under the classification
// relation, so one pair of steps writes any kind and one check reads any of them.
// Every named relation of a thing is subject, what, value: the what a node under is and the value under the
// what by is again, as germany to capital to berlin, so one shape serves kinds and relations alike. Only
// structure stays a relation of its own: in, owns, same, successor, and counts, whose values are numbers.
const STRUCTURAL = new Set(['in', 'same', 'successor', 'kind', 'direction', 'self', 'means', 'form', 'past', 'future', 'time', 'quantity', 'number', 'opposite', 'answer', 'say', 'think', 'believe', 'want', 'east', 'west', 'north', 'south', 'above', 'below', 'left', 'right']);
// A verb as a relation is written in its base form, helps as help, since the lessons write verbs that way and a
// relation the seeds spell otherwise would be a second relation with the same stem; is and was stay, has is owns.
const baseVerb = (verb) => ({ is: 'is', was: 'was', has: 'has', does: 'do', goes: 'go' }[verb] ?? verb.replace(/ies$/, 'y').replace(/(s|x|ch|sh)es$/, '$1').replace(/s$/, ''));
// One relation word per concept, the value straight under it: germany {capital} berlin, tom {gender} male.
const named = (subject, what, value) => `${subject} {${what}} ${value}`;
// A quality is its own relation with yes beneath it, gold {yellow} yes, as the lessons write the dog big; the
// quality words are the colors and the words the qualities facts class by a kind.
const qualities = new Set(readFileSync(`${model}data/facts/18-qualities.txt`, 'utf8').split(/\r?\n/).map((l) => l.match(/^([a-z]+) is an? [a-z]+\.$/)).filter(Boolean).map((m) => m[1]).concat(readFileSync(`${model}data/facts/90-everyday.txt`, 'utf8').split(/\r?\n/).map((l) => l.match(/^([a-z]+) is a color\.$/)).filter(Boolean).map((m) => m[1])));
// The kind of every word the facts class, from the first fact that says the word is a kind, over all the facts.
const kindOf = new Map();
for (const file of readdirSync(`${model}data/facts`).filter((f) => f.endsWith('.txt')).sort()) {
  for (const raw of readFileSync(`${model}data/facts/${file}`, 'utf8').split(/\r?\n/)) {
    const m = raw.trim().toLowerCase().replace(/\s*\.$/, '').match(/^(?:the |a |an )?([a-z]+) (?:is|are) (?:a|an) ([a-z]+)$/);
    if (m && !qualities.has(m[2]) && !kindOf.has(m[1]) && m[1] !== m[2]) kindOf.set(m[1], m[2]);
  }
}
// The things: what a fact says is a kind of thing, a country, a city, a person, a composer, a god, a novel,
// a planet, a company, the name a gender fact names, a capital and what it is the capital of, and a single letter.
const THING_KINDS = new Set(['country', 'city', 'town', 'village', 'continent', 'island', 'ocean', 'sea', 'lake', 'river', 'mountain', 'volcano', 'desert', 'forest', 'region', 'state', 'province', 'capital', 'landmark', 'building', 'bridge', 'tower', 'palace', 'castle', 'temple', 'church', 'cathedral', 'museum', 'university', 'company', 'website', 'language', 'currency', 'person', 'man', 'woman', 'king', 'queen', 'emperor', 'empress', 'president', 'leader', 'ruler', 'general', 'explorer', 'scientist', 'inventor', 'physicist', 'chemist', 'biologist', 'mathematician', 'astronomer', 'philosopher', 'writer', 'poet', 'author', 'novelist', 'playwright', 'painter', 'sculptor', 'artist', 'composer', 'musician', 'singer', 'actor', 'actress', 'director', 'athlete', 'player', 'footballer', 'boxer', 'runner', 'swimmer', 'god', 'goddess', 'hero', 'heroine', 'prophet', 'saint', 'pope', 'novel', 'play', 'poem', 'book', 'epic', 'painting', 'opera', 'symphony', 'song', 'film', 'battle', 'war', 'event', 'treaty', 'revolution', 'planet', 'star', 'moon', 'galaxy', 'comet', 'constellation', 'nationality', 'people', 'tribe', 'dynasty', 'empire', 'kingdom', 'republic', 'party', 'team', 'club', 'newspaper', 'magazine', 'holiday', 'festival', 'month', 'day', 'season', 'operating', 'program', 'game', 'sport']);
const things = new Set();
for (const file of readdirSync(`${model}data/facts`).filter((f) => f.endsWith('.txt')).map((f) => f.slice(0, -4))) {
  for (const raw of readFileSync(`${model}data/facts/${file}.txt`, 'utf8').split(/\r?\n/)) {
    const sentence = raw.trim().toLowerCase().replace(/\s*\.$/, '');
    if (!sentence || sentence.startsWith('#')) continue;
    const kind = sentence.match(/^(?:the )?([a-z]+) (?:is|was) (?:a|an|the) ([a-z]+)$/);
    if (kind && THING_KINDS.has(kind[2]) && !['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday', 'january', 'february', 'march', 'april', 'may', 'june', 'july', 'august', 'september', 'october', 'november', 'december', 'spring', 'summer', 'autumn', 'winter', 'today', 'tomorrow', 'yesterday'].includes(kind[1])) things.add(kind[1]);
    const capital = sentence.match(/^(?:the )?([a-z]+) is the capital of (?:the )?([a-z]+)$/);
    if (capital) { things.add(capital[1]); things.add(capital[2]); }
    const gender = sentence.match(/^the gender of ([a-z]+) is/);
    if (gender) things.add(gender[1]);
    const of = sentence.match(/^(?:the )?([a-z]+) is the ([a-z]+) of (?:the )?([a-z]+)$/);
    if (of && THING_KINDS.has(of[2])) { things.add(of[1]); things.add(of[3]); }
  }
}
const thing = (name) => things.has(name) || /^[a-z]$/.test(name);
// A quality of a kind the reading sets by a move of its own stands as a story tells it, under is and its
// kind, ice {is} {temperature} cold, so a question reads a seeded quality as it reads a told one; any other
// quality stays its own relation.
const WORLD_KINDS = new Set(['color', 'size', 'feeling', 'speed', 'temperature', 'age', 'material']);
const qualityKind = new Map();
for (const file of ['18-qualities', '90-everyday']) {
  for (const raw of readFileSync(`${model}data/facts/${file}.txt`, 'utf8').split(/\r?\n/)) {
    const m = raw.trim().toLowerCase().match(/^([a-z]+) is an? ([a-z]+)\.$/);
    if (m && WORLD_KINDS.has(m[2]) && !qualityKind.has(m[1])) qualityKind.set(m[1], m[2]);
  }
}
const isOrQuality = (subject, value, article) => (qualityKind.has(value) && qualities.has(value) ? `${subject} -> {is} -> {${qualityKind.get(value)}} -> ${value}` : qualities.has(value) || (article === false && !kindOf.has(value)) ? `${subject} -> {${value}}` : `${subject} {is} ${value}`);
// A unit of measure. A count of units under another unit (a day is 24 hours, there are 100 centimeters in a meter,
// 1 dollar = 4 quarters) is the user's equals form, two branches side by side under the unit: its own quantity, and
// under equal the other unit with its quantity, so the tree says 1 liter equals 1000 milliliters and never that a liter
// equals a milliliter, and a unit that is the target of one conversion and the base of another holds one quantity.
// A count of parts (a cat has 4 legs, a rainbow has 7 colors) is the part as the relation with the quantity beneath
// it, cat to leg to quantity 4, since the part is a concept and has says nothing more; a weight is the unit under weigh.
const units = new Set(['second', 'minute', 'hour', 'day', 'week', 'month', 'year', 'decade', 'century', 'millennium', 'millimeter', 'centimeter', 'meter', 'kilometer', 'inch', 'foot', 'feet', 'yard', 'mile', 'gram', 'kilogram', 'kilo', 'ton', 'tonne', 'pound', 'ounce', 'milliliter', 'liter', 'litre', 'gallon', 'pint', 'cup', 'cent', 'dollar', 'quarter', 'dime', 'nickel', 'penny', 'euro', 'pound', 'degree', 'byte', 'kilobyte', 'megabyte', 'gigabyte', 'bit']);
const singular = (word) => word.replace(/ies$/, 'y').replace(/(ch|sh|s|x)es$/, '$1').replace(/s$/, '');
const counted = (subject, relation, number, unit) => {
  const one = singular(unit);
  if (units.has(singular(subject)) && units.has(one) && relation !== 'weigh') return `${singular(subject)}:quantity(1) {equal} ${one}:quantity(${number})`;
  if (relation === 'has') return `${subject} -> {${one}:quantity(${number})}`;
  return `${subject} -> {${relation}} -> ${one} -> {quantity} -> ${number}`;
};
const shapes = [
  [/^(\S+) = (\S+)$/, (m) => `${m[1]} = ${m[2]}`],
  [/^(-?[\d.]+) (\S+) = (-?[\d.]+) (\S+)$/, (m) => `${singular(m[2])}:quantity(${m[1]}) {equal} ${singular(m[4])}:quantity(${m[3]})`],
  // What a thing is made of is its material, one path per material when a sentence names two.
  [new RegExp(`^${article}(\\S+) is made of ${article}(\\S+)(?: and ${article}(\\S+))?$`), (m) => [m[2], m[3]].filter(Boolean).map((material) => named(m[1], 'material', material)).join('\n')],
  [new RegExp(`^the (\\S+) of ${article}(\\S+) is ${article}(\\S+)$`), (m) => named(m[2], m[1], m[3])],
  [new RegExp(`^${article}(\\S+) (?:is|are) (-?[\\d.]+) (\\S+)$`), (m) => counted(m[1], 'is', m[2], m[3])],
  [new RegExp(`^there are (-?[\\d.]+) (\\S+) in ${article}(\\S+)$`), (m) => counted(m[3], 'has', m[1], m[2])],
  [new RegExp(`^${article}(\\S+) is the same as ${article}(\\S+)$`), (m) => `${m[1]} {same} ${m[2]}`],
  [new RegExp(`^${article}(\\S+) is in ${article}(\\S+)$`), (m) => `${m[1]} {in} ${m[2]}`],
  // A comparison word means more or less of a measure: heavier means more weight.
  [/^(\S+) means (more|less) (\S+)$/, (m) => `${m[1]} {${m[2]}} ${m[3]}`],
  [new RegExp(`^${article}(\\S+) is the capital of ${article}(\\S+)$`), (m) => named(m[2], 'capital', m[1])],
  // What a thing is the K of: the thing on the left is the K of the one on the right, english the language of britain.
  [new RegExp(`^${article}(\\S+) is the ([a-z]+) of ${article}(\\S+)$`), (m) => named(m[3], m[2], m[1])],
  [new RegExp(`^${article}(\\S+) (has|have|weighs|weigh) (-?[\\d.]+) (\\S+)$`), (m) => counted(m[1], m[2].startsWith('weigh') ? 'weigh' : 'has', m[3], m[4])],
  [new RegExp(`^${article}(\\S+) is (-?[\\d.]+)$`), (m) => `${m[1]} = ${m[2]}`],
  [new RegExp(`^the (\\S+) (\\S+) of the (\\S+) is (\\S+)$`), (m) => `${m[3]} {${m[1]}} ${m[4]}`],
  // A superlative or an ordinal with its kind, jupiter is the largest planet, earth is the third planet, ranks the thing under is and names its kind, the
  // way a story tells it, so what is the largest planet reads the seeds when the story ranks nothing.
  [new RegExp(`^${article}(\\S+) is the ([a-z]+est|first|second|third|fourth|fifth|sixth|seventh|eighth) (\\S+)$`), (m) => [`${m[1]} -> {is} -> ${m[2]}`, `${m[1]} {is} ${m[3]}`].join(LINE)],
  [/^(\S+) is (a |an |the )?(\S+)$/, (m) => isOrQuality(m[1], m[3], m[2] !== undefined)],
  // A verb as a relation is written in its base form, helps as help, since the lessons write verbs that way and a
  // relation the seeds spell otherwise would be a second relation with the same stem; has is owns.
  [new RegExp(`^${article}(\\S+) ([a-z]+s) (the |a |an )?(\\S+)$`), (m) => (m[2] === 'is' ? isOrQuality(m[1], m[4], m[3] !== undefined) : `${m[1]} {${baseVerb(m[2])}} ${m[4]}`)],
];
// Seeds talk in concepts and hold no English: every node is a braced concept, {cat} -> {is} -> {animal},
// {tall} -> {opposite} -> {short}, {paris} -> {capital} -> {france}, a number {number 5}, a letter {a}.
// The English side, how a word reaches its concept, is written apart: word -> {equal} -> {word} for every
// concept word, and the files that are only about words, the verb forms, the speakers' words, the numerals
// and the tens words, go there whole, as does a number word's worth, zero -> {equal} -> 0.
const INDIVIDUAL_FILES = new Set(['16-names', '30-geography', '31-countries', '32-cities-and-landmarks', '33-rivers-mountains-seas', '34-languages-and-money', '50-astronomy', '55-earth', '70-people-and-the-past', '71-famous-people', '72-inventions-and-discoveries', '73-events-and-years', '74-books-and-authors', '78-arts-and-music', '79-myths-and-religions', '85-technology', '86-computing']);
const ENGLISH_FILES = new Set(['21-verb-forms', '17-speakers', '13-roman-numerals', '22-tens-words']);
const LINE = String.fromCharCode(10);
const number = (name) => /^-?[\d.]+$/.test(name);
const tagged = (name) => (kindOf.has(name) ? `{${kindOf.get(name)} ${name}}` : `{${name}}`);
// A braced token with properties, {leg(quantity: 6)}: its word is tagged by its kind and the properties stay.
const retagged = (token) => { const m = token.match(/^\{([a-z]+)(\(.*\))\}$/); return m ? tagged(m[1]).slice(0, -1) + m[2] + '}' : token; };
const braced = (name) => (name.startsWith('{') || thing(name) ? name : number(name) ? `{number ${name}}` : tagged(name));
const statementsOf = (sentence) => {
  const shape = shapes.find(([re]) => re.test(sentence));
  return shape ? [shape[1](sentence.match(shape[0]))].flat().flatMap((st) => st.split('\n')) : [];
};
const numbered = (name) => (number(name) ? `{number:value(${name})}` : name);
// Every statement in plain English words, a number as the number tag, a numeral or a number word to equal to its
// number, a digit's tens word, and the speakers' words to their self.
const plain = (st, file) => {
  if (file === '13-roman-numerals') {
    const roman = st.match(/^(\d+) \{roman\} (\S+)$/);
    return roman ? [`${roman[2]} -> {equal} -> {number:value(${roman[1]})}`, `{number:value(${roman[1]})} -> {roman} -> ${roman[2]}`] : [];
  }
  if (file === '22-tens-words') {
    const tens = st.match(/^(\d+) \{tens\} (\S+)$/);
    return tens ? [`${tens[2]} -> {tens} -> {number:value(${tens[1]})}`] : [st];
  }
  const worth = st.match(/^(\S+) = (\S+)$/);
  if (worth) {
    const [, a, b] = worth;
    const n = number(a) ? a : b;
    const word = n === a ? b : a;
    return [`${word} -> {equal} -> {number:value(${n})}`];
  }
  if (st.includes(':quantity(')) return [st];
  const arrows = st.includes(' -> ');
  const names = arrows ? st.split(' -> ') : st.split(' ');
  const out = names.map((name, place) => (place % 2 === 0 || number(name) ? numbered(name) : name));
  for (let place = 0; place + 1 < out.length; place++) if (out[place] === '{quantity}' && out[place + 1].startsWith('{number:value(')) { out.splice(place, 2, `{quantity:value(${out[place + 1].slice(14, -2)})}`); }
  return [arrows ? out.join(' -> ') : out.join(' ')];
};
for (const name of process.argv.slice(2)) {
  const source = readFileSync(`${model}data/facts/${name}.txt`, 'utf8');
  const out = [`# The statements of data/facts/${name}.txt the cursor tree holds, one path each, written by build-seeds.mjs.`];
  const skipped = [];
  for (const raw of source.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith('#')) continue;
    const sentence = line.toLowerCase().replace(/\s*\.$/, '');
    const shape = shapes.find(([re]) => re.test(sentence));
    if (shape) {
      // A statement two facts both give (an hour is worth one hour in each of its conversions) is written once.
      for (const st of [shape[1](sentence.match(shape[0]))].flat().flatMap((st) => st.split(LINE))) if (!out.includes(st)) out.push(st);
    } else skipped.push(line);
  }
  // Statements that share a relation and run on from one another (a successor b, b successor c) are
  // written as one path, so the cursor walks the chain a step at a time instead of finding each link
  // again. A chain that comes round (the days) starts at the member the file names first and ends back
  // on it.
  const lower = source.toLowerCase();
  const seen = (word) => {
    const at = lower.search(new RegExp(`\\b${word}\\b`));
    return at < 0 ? Infinity : at;
  };
  const links = new Map();
  for (const st of out.slice(1)) {
    const w = st.split(' ');
    if (w.length !== 3 || ['{is}', '=', '{in}', '{has}'].includes(w[1])) continue;
    if (!links.has(w[1])) links.set(w[1], new Map());
    if (!links.get(w[1]).has(w[0])) links.get(w[1]).set(w[0], w[2]);
  }
  const chained = new Set();
  const chains = [];
  for (const [relation, next] of links) {
    const targets = new Set(next.values());
    const heads = [...next.keys()].filter((k) => [...next.values()].includes(k) && [...next.keys()].includes(next.get(k)));
    if (heads.length < 2) continue;
    const byFirst = (a, b) => seen(a) - seen(b);
    const starts = [...next.keys()].filter((k) => !targets.has(k)).sort(byFirst);
    const rounds = [...next.keys()].filter((k) => targets.has(k)).sort(byFirst);
    const used = new Set();
    for (const start of [...starts, ...rounds]) {
      if (used.has(start)) continue;
      const path = [start];
      let at = start;
      while (next.has(at) && !used.has(at)) {
        used.add(at);
        chained.add(`${at} ${relation} ${next.get(at)}`);
        at = next.get(at);
        path.push(relation, at);
        if (at === start) break;
      }
      if (path.length > 3) chains.push(path.join(' -> '));
      else chained.delete(`${start} ${relation} ${next.get(start)}`);
    }
  }
  const written = [...out.slice(1).filter((st) => !chained.has(st)), ...chains].flatMap((st) => plain(st, name));
  const header = `# The statements of data/facts/${name}.txt the cursor tree holds, one path each, written by build-seeds.mjs.`;
  if (written.length) writeFileSync(`${model}data/seeds/${name}.txt`, [header, ...written].join('\n') + '\n');
  else if (existsSync(`${model}data/seeds/${name}.txt`)) rmSync(`${model}data/seeds/${name}.txt`);
  for (const folder of ['concepts', 'english']) { const old = `${model}data/seeds/${folder}/${name}.txt`; if (existsSync(old)) rmSync(old); }
  if (chains.length) console.log(`${name}: ${chains.length} chains, the longest ${Math.max(...chains.map((c) => c.split(' -> ').length))} names`);
  console.log(`${name}: ${out.length - 1} statements, ${skipped.length} left out${skipped.length ? ': ' + skipped.slice(0, 3).join(' | ') : ''}`);
}
