import fs from 'fs';
// Long stories for the world lessons: many objects in one world, things put inside things that move later, and
// questions asked after other sentences. Each story is played on a small simulation of the world, so every expected
// shape and answer comes from running the moves, never from a template.
// Usage (from model/): node scripts/world-stories.mjs <stories> <learn|test> <seed> [avoid-file] [story|world] > file.txt
// world writes the same simulated world as a world: line, seeded before the questions, with no text read.
const stories = Number(process.argv[2] || 50);
const test = process.argv[3] === 'test';
let seed = Number(process.argv[4] || 7);
const rand = () => { seed = (seed * 1103515245 + 12345) % 2147483648; return seed / 2147483648; };
const pick = (xs) => xs[Math.floor(rand() * xs.length)];
const onsets = ['b', 'd', 'f', 'g', 'k', 'l', 'm', 'n', 'p', 'r', 't', 'v', 'z', 'bl', 'gr', 'tr', 'kr', 'pl', 'sn', 'fl'];
const vowels = ['a', 'e', 'i', 'o', 'u', 'oo', 'ai'];
const codas = ['b', 'd', 'g', 'k', 'm', 'n', 'p', 'rt', 'nd', 'x', 'zz', 'lk'];
const used = new Set();
for (const f of fs.readdirSync('data/seeds')) for (const w of fs.readFileSync('data/seeds/' + f, 'utf8').toLowerCase().split(/[^a-z]+/)) used.add(w);
for (const w of ['did', 'do', 'does', 'has', 'had', 'is', 'was', 'am', 'in', 'on', 'to', 'of', 'the', 'a', 'an', 'it', 'he', 'she', 'they', 'who', 'what', 'where', 'whose', 'how', 'grab', 'take', 'get', 'pick', 'drop', 'leave', 'put', 'hand', 'pass', 'send', 'give', 'hold', 'contain', 'open', 'close', 'lock', 'unlock', 'check', 'again', 'look', 'still', 'please', 'ok', 'okay', 'well', 'many', 'much', 'things', 'items', 'my', 'his', 'her', 'its', 'their', 'your', 'you', 'me', 'x', 'plus', 'minus', 'times', 'divided', 'go', 'move', 'went']) used.add(w);
if (process.argv[5]) for (const w of fs.readFileSync(process.argv[5], 'utf8').toLowerCase().split(/[^a-z]+/)) used.add(w);
let spare = 0;
const name = () => { for (let k = 0; ; k++) { let w = pick(onsets) + pick(vowels) + pick(codas) + (rand() < 0.4 ? pick(vowels) + pick(codas) : ''); if (k > 50) w += pick(vowels) + codas[spare++ % codas.length] + pick(vowels) + pick(codas); if (!used.has(w) && !w.endsWith('s')) { used.add(w); return w; } } };
const seeded = process.argv[6] === 'world';
const colors = ['yellow', 'black', 'white', 'orange', 'purple', 'pink', 'brown', 'red', 'blue', 'green'];

function story() {
  const world = new Map();
  const node = (n) => { if (!world.has(n)) world.set(n, { parent: null, color: null, count: null, the: false }); return world.get(n); };
  const inside = (a, b) => { for (let x = b; x; x = world.get(x)?.parent) if (x === a) return true; return false; };
  const chain = (n) => { const out = []; for (let x = n; x; x = world.get(x).parent) out.unshift(x); return out; };
  const shown = (n) => { const o = world.get(n); return n + (o.the ? ':the(true)' : '') + (o.count ? `:quantity(${o.count})` : ''); };
  const said = [];
  const pool = Array.from({ length: 4 + Math.floor(rand() * 5) }, name);
  const people = Array.from({ length: 2 }, name);
  const places = Array.from({ length: 2 }, name);
  const facts = 5 + Math.floor(rand() * 6);
  let tries = 0;
  for (let i = 0; i < facts && tries++ < 200; i++) {
    const kind = rand();
    if (kind < 0.4) {
      const a = pick(pool), b = pick(pool);
      if (a === b || inside(a, b) || world.get(a)?.count) { i--; continue; }
      node(b); node(a).parent = b;
      said.push(`${a} in ${b}.`);
    } else if (kind < 0.6) {
      const p = pick(people), place = pick(places);
      node(place).the = true; node(p).parent = place;
      said.push(`${p} went to the ${place}.`);
    } else if (kind < 0.8) {
      const a = pick(pool), c = pick(colors);
      if (world.get(a)?.count) { i--; continue; }
      node(a).color = c;
      said.push(`${a} is ${c}.`);
    } else {
      const p = pick(people), b = name(), n = 2 + Math.floor(rand() * 8);
      const o = node(b); o.count = n; o.parent = p; node(p);
      said.push(`${p} has ${n} ${b}s.`);
    }
  }
  const expect = [];
  const questions = [];
  for (const [n, o] of world) {
    if (o.parent) {
      expect.push(chain(n).map(shown).join(' -> '));
      questions.push(`${o.count ? `who has ${n}s` : `where is ${n}`}? = ${o.parent}`);
    }
    if (o.color) {
      expect.push([...chain(n).map(shown), '{is}', '{color}', o.color].join(' -> '));
      questions.push(`what color is ${n}? = ${o.color}`);
    }
  }
  for (const [n] of world) {
    const inner = [...world].filter(([, o]) => o.parent === n).map(([c]) => c);
    if (inner.length === 1 && !people.includes(n)) questions.push(`what is in ${world.get(n).the ? 'the ' : ''}${n}? = ${inner[0]}`);
  }
  const asked = questions.sort(() => rand() - 0.5).slice(0, 4);
  if (seeded) return `world: ${expect.join('; ')} | ${(asked.length ? asked : questions).join('; ')}`;
  return `${said.join(' ')} => ${expect.join('; ')}${asked.length ? ' | ' + asked.join('; ') : ''}`;
}

const lines = [`# generated ${test ? 'held-out' : 'learned'} long stories for the world lessons, played on a simulation of the world`];
if (test) lines.push('corn in bag. => bag -> corn');
for (let i = 0; i < stories; i++) lines.push((test ? 'test: ' : '') + story());
console.log(lines.join('\n'));
