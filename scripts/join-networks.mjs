// Join networks that read by vote into one weights file:
//   [HALF=1] node scripts/join-networks.mjs OUT.bin FIRST.bin SECOND.bin ...
// With HALF=1 every weight is written in two bytes instead of four, which halves what the page downloads and
// loads; rounding a trained network to that width and back scored the same on every lesson, learned and held out.
// The weights are written one after another into OUT.bin, and OUT.bin.json is the first network's classes
// file with the count of voters, so the console, the chat build and the page read them as one network file.
// Every network must have the same classes and shape, which networks trained on the same rows have.
import { readFileSync, writeFileSync } from 'node:fs';

const [out, ...parts] = process.argv.slice(2);
if (!out || parts.length === 0) {
  console.error('usage: node scripts/join-networks.mjs OUT.bin FIRST.bin SECOND.bin ...');
  process.exit(1);
}
const named = parts.map((path) => JSON.parse(readFileSync(`${path}.json`, 'utf8')));
const same = (a, b) => JSON.stringify({ ...a, voters: 0, half: false }) === JSON.stringify({ ...b, voters: 0, half: false });
for (const [at, one] of named.entries()) {
  if (!same(named[0], one)) {
    console.error(`${parts[at]} was trained on other classes or another shape than ${parts[0]}`);
    process.exit(1);
  }
}
const votersIn = (one) => Math.max(one.voters ?? 1, 1);
const voters = named.reduce((all, one) => all + votersIn(one), 0);
const half = process.env.HALF === '1';
const narrow = named.findIndex((one) => one.half);
if (narrow >= 0) {
  console.error(`${parts[narrow]} is written half as wide already, join the full networks it was made from`);
  process.exit(1);
}
// The head of a network's weights holds a feature's place beside its embedding, so only the weights are narrowed.
const idWidth = 8;
const width = 4;
const narrowed = (bytes, shape) => {
  const record = idWidth + shape.item * width;
  const head = shape.items * record;
  const weights = (bytes.length - head) / width + shape.items * shape.item;
  const out = Buffer.alloc(shape.items * (idWidth + shape.item * 2) + (bytes.length - head) / 2);
  const small = new Float16Array(1);
  let at = 0;
  const put = (from) => { small[0] = bytes.readFloatLE(from); out.writeUInt16LE(new Uint16Array(small.buffer)[0], at); at += 2; };
  for (let r = 0; r < shape.items; r += 1) {
    bytes.copy(out, at, r * record, r * record + idWidth);
    at += idWidth;
    for (let f = 0; f < shape.item; f += 1) put(r * record + idWidth + f * width);
  }
  for (let from = head; from + width <= bytes.length; from += width) put(from);
  console.log(`narrowed ${weights.toLocaleString('en-US')} weights to two bytes each`);
  return out;
};
// A file that holds several voters holds them one after another, each with its own head of places, so each is
// narrowed by itself.
const apart = parts.flatMap((path, at) => {
  const bytes = readFileSync(path);
  const each = bytes.length / votersIn(named[at]);
  return Array.from({ length: votersIn(named[at]) }, (_, voter) => bytes.subarray(voter * each, (voter + 1) * each));
});
const parted = apart.map((bytes) => (half ? narrowed(bytes, named[0].stacked) : bytes));
writeFileSync(out, Buffer.concat(parted));
writeFileSync(`${out}.json`, JSON.stringify({ ...named[0], voters, half }));
console.log(`wrote ${out} with ${voters} voters`);
