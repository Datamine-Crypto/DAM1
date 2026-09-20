// The item set: every question of a held-out line of the curriculum, with the text it is asked about and
// the answer it must give. A held-out line is a line no network was trained on, marked test: in its lesson.
// Lines that state a world instead of a text are left out, since only DAM1 can be given a world.
import fs from 'fs';
import path from 'path';

const root = process.argv[2] ?? path.join(import.meta.dirname, '..', 'model', 'data', 'train');
const out = process.argv[3] ?? path.join(import.meta.dirname, 'out', 'items.jsonl');
const items = [];
for (const folder of fs.readdirSync(root).filter((name) => fs.statSync(path.join(root, name)).isDirectory()).sort()) {
  for (const file of fs.readdirSync(path.join(root, folder)).filter((name) => name.endsWith('.txt')).sort()) {
    const lines = fs.readFileSync(path.join(root, folder, file), 'utf8').split(/\r?\n/);
    for (const [at, line] of lines.entries()) {
      if (!line.startsWith('test: ')) continue;
      const said = line.slice('test: '.length);
      if (said.startsWith('world:') || said.startsWith('then:') || said.startsWith('next:')) continue;
      const bar = said.indexOf('|');
      if (bar < 0) continue;
      const arrow = said.slice(0, bar).indexOf('=>');
      if (arrow < 0) continue;
      const text = said.slice(0, arrow).trim();
      let ask = 0;
      for (const asked of said.slice(bar + 1).split(';')) {
        const equals = asked.indexOf('=');
        if (equals < 0) continue;
        const question = asked.slice(0, equals).trim();
        const answer = asked.slice(equals + 1).trim();
        if (!question) continue;
        const place = ask;
        ask += 1;
        // An answer that says what must not be said, and one written as a shape, is no item.
        if (!answer || answer.startsWith('not ') || answer.includes('->')) continue;
        items.push({ folder, file, line: at + 1, ask: place, text, question, answer });
      }
    }
  }
}
fs.mkdirSync(path.dirname(out), { recursive: true });
fs.writeFileSync(out, items.map((item) => JSON.stringify(item)).join('\n') + '\n');
console.log(`${items.length} questions written to ${out}`);
