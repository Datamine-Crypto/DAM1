// The item set asked of the build the browser runs: the engine, the network and the state the chat page
// ships. One conversation an item, the text said a sentence a turn and then the question, as a person
// types it. The time each answer takes is measured on the processor, one answer at a time.
import fs from 'fs';
import path from 'path';
import zlib from 'zlib';
import { pathToFileURL } from 'url';

const here = import.meta.dirname;
const chat = path.join(here, '..', 'chat');
const out = process.argv[2] ?? path.join(here, 'out');
const mod = await import(pathToFileURL(path.join(chat, 'src', 'wasm', 'dam_web.js')).href);
const engine = await mod.default({ module_or_path: fs.readFileSync(path.join(chat, 'src', 'wasm', 'dam_web_bg.wasm')) });
const weights = zlib.gunzipSync(fs.readFileSync(path.join(chat, 'public', 'network.bin')));
const numbers = mod.load(new Uint8Array(weights), fs.readFileSync(path.join(chat, 'public', 'network.json'), 'utf8'));
const seed = new Uint8Array(fs.readFileSync(path.join(chat, 'public', 'state.bin')));
const items = fs.readFileSync(path.join(out, 'items.jsonl'), 'utf8').trim().split('\n').map((line) => JSON.parse(line));
const said = [];
const times = [];
for (const item of items) {
  mod.forget();
  mod.state(seed);
  for (const sentence of item.text.split(/(?<=[.!?])\s+/).filter(Boolean)) {
    // A sentence the reader cannot take leaves the tree as it was, and the question is still asked.
    try { mod.read(sentence); } catch { /* left as it was */ }
  }
  const at = process.hrtime.bigint();
  let reply = '';
  try { reply = (JSON.parse(mod.read(item.question)).output ?? []).join(' '); } catch { reply = ''; }
  times.push(Number(process.hrtime.bigint() - at) / 1e6);
  said.push({ ...item, reply });
}
const middle = [...times].sort((a, b) => a - b)[Math.floor(times.length / 2)];
fs.writeFileSync(path.join(out, 'replies-dam1.jsonl'), said.map((one) => JSON.stringify(one)).join('\n') + '\n');
// The weights as the hub publishes them, and as the page downloads them, which is the same network
// written small: the page fetches it gzipped.
const published = path.join(here, '..', 'huggingface', 'model', 'model.safetensors');
fs.writeFileSync(path.join(out, 'facts-dam1.json'), JSON.stringify({
  name: 'DAM1', parameters: numbers, items: items.length, one_at_a_time: middle / 1000,
  per_item: times.reduce((sum, one) => sum + one, 0) / times.length / 1000,
  weights: fs.existsSync(published) ? fs.statSync(published).size : null,
  download: fs.statSync(path.join(chat, 'public', 'network.bin')).size,
  // All the memory the reader works in: the one block WebAssembly grew, which holds the network, the
  // state, the tree and the stack.
  memory: engine.memory.buffer.byteLength,
}));
console.log(`${numbers} numbers, ${middle.toFixed(1)} ms the middle answer, ${(engine.memory.buffer.byteLength / 1e6).toFixed(1)} MB of memory`);
