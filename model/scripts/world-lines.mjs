import fs from 'fs';
// Lines for the world lessons: every shape of data/train/world, statements and questions, with invented names.
// Usage (from model/): node world-lines.mjs <count per shape> <learn|test> <seed> [avoid-file] > file.txt
// A test run skips every name in the avoid file, so held-out names never appear in the learned lines.
const per = Number(process.argv[2] || 10);
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
const colors = ['yellow', 'black', 'white', 'orange', 'purple', 'pink', 'brown', 'red', 'blue', 'green'];
const other = (c) => { for (;;) { const o = pick(colors); if (o !== c) return o; } };
const count = () => String(2 + Math.floor(rand() * 8));
const shapes = [
  () => { const a = name(), b = name(); return `${a} in ${b}. => ${b} -> ${a}`; },
  () => { const a = name(), b = name(); return `the ${a} is in the ${b}. => ${b}:the(true) -> ${a}:the(true)`; },
  () => { const a = name(), c = pick(colors); return `${a} is ${c}. => ${a} -> {is} -> {color} -> ${c}`; },
  () => { const a = name(), c = pick(colors); return `the ${a} was ${c}. => ${a}:the(true) -> {is}:time(past) -> {color} -> ${c}`; },
  () => { const a = name(), b = name(), p = name(), c = pick(colors); return `${a} in ${p}. ${b} in ${p}. ${p} is ${c}. => ${p} -> ${a}; ${p} -> ${b}; ${p} -> {is} -> {color} -> ${c}`; },
  () => { const a = name(), b = name(); return `${a} went to the ${b}. => ${b}:the(true) -> ${a}`; },
  () => { const a = name(), b = name(), c = name(); return `${a} went to the ${b}. ${a} went to the ${c}. => ${c}:the(true) -> ${a}; not ${b}:the(true) -> ${a}`; },
  () => { const a = name(), b = name(), c = name(); return `the ${a} is in the ${b}. the ${a} is in the ${c}. => ${c}:the(true) -> ${a}:the(true); not ${b}:the(true) -> ${a}:the(true)`; },
  () => { const a = name(), b = name(), n = count(); return `${a} has ${n} ${b}s. => ${a} -> ${b}:quantity(${n})`; },
  () => { const a = name(), b = name(), n = count(), c = pick(colors); return `${a} has ${n} ${c} ${b}s. => ${a} -> ${b}:quantity(${n}) -> {is} -> {color} -> ${c}`; },
  () => { const a = name(), c = pick(colors); return `there was a ${a}. it was ${c}. => ${a}:a(true):time(past) -> {is}:time(past) -> {color} -> ${c}`; },
  () => { const a = name(), b = name(); return `a ${a} goes to a ${b}. => ${b}:a(true) -> ${a}:a(true)`; },
  () => { const a = name(), b = name(); return `${a} in ${b}. => ${b} -> ${a} | where is ${a}? = ${b}`; },
  () => { const a = name(), b = name(); return `${a} in ${b}. => ${b} -> ${a} | what is in the ${b}? = ${a}`; },
  () => { const a = name(), b = name(), c = name(); return `${a} went to the ${b}. ${a} went to the ${c}. => ${c}:the(true) -> ${a} | where is ${a}? = ${c}`; },
  () => { const a = name(), c = pick(colors); return `${a} is ${c}. => ${a} -> {is} -> {color} -> ${c} | what color is ${a}? = ${c}`; },
  () => { const a = name(), b = name(), n = count(); return `${a} has ${n} ${b}s. => ${a} -> ${b}:quantity(${n}) | what does ${a} have? = ${b}; who has ${b}s? = ${a}`; },
  () => { const a = name(), c = pick(colors); return `the ${a} is ${c}. => ${a}:the(true) -> {is} -> {color} -> ${c} | is the ${a} ${c}? = yes; is the ${a} ${other(c)}? = no`; },
  () => { const a = name(), b = name(), d = name(), n = count(); return `${a} has ${n} ${b}s. => ${a} -> ${b}:quantity(${n}) | does ${a} have ${b}s? = yes; does ${a} have ${d}s? = no`; },
  () => { const a = name(), c = pick(colors); return `there was a ${a}. it was ${c}. => ${a}:a(true):time(past) -> {is}:time(past) -> {color} -> ${c} | what color was the ${a}? = ${c}`; },
];
const lines = [`# generated ${test ? 'held-out' : 'learned'} lines for the world lessons: every shape with invented names`];
if (test) lines.push('corn in bag. => bag -> corn');
for (const shape of shapes) for (let i = 0; i < per; i++) lines.push((test ? 'test: ' : '') + shape());
console.log(lines.join('\n'));
