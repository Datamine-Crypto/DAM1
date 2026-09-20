// A game played back: the turns of a chat, and the frames of the map they make, one for each move the network took.
import type { Steps } from '../agent';
import { regionOf, type Region } from '../emoji';
import type { Chat } from '../store';

// The gap between a relation's line and the border of the tile it ends at, in pixels.
export const lineGap = 4;
const continueMove = '{continue}';
// A name on a written path that is a plain thing, not a counted or valued form.
// The names of a written path in order: a braced name stays whole though it holds a space, {quantity 3}.
export function partsOf(path: string): string[] {
  return path.match(/\{[^}]*\}|[^\s{}]+/g)?.filter((part) => part !== '->') ?? [];
}

// The one form two spellings of a thing share, so apples and apple are the same tile.
export function singular(name: string): string {
  if (name.endsWith('ies') && name.length > 4) return `${name.slice(0, -3)}y`;
  if (name.endsWith('s') && !name.endsWith('ss') && name.length > 3) return name.slice(0, -1);
  return name;
}

// The past form of a relation's name, for a line the story tells in the past: the common irregular ones, else ed.
const pastForms: Record<string, string> = { does: 'did',
  is: 'was', has: 'had', have: 'had', do: 'did', go: 'went', give: 'gave', take: 'took', see: 'saw', make: 'made', buy: 'bought', find: 'found', eat: 'ate',
  say: 'said', tell: 'told', know: 'knew', get: 'got', come: 'came', run: 'ran', sit: 'sat', stand: 'stood', hold: 'held', keep: 'kept', leave: 'left', meet: 'met',
  pay: 'paid', sell: 'sold', send: 'sent', sing: 'sang', sleep: 'slept', swim: 'swam', teach: 'taught', think: 'thought', win: 'won', write: 'wrote', read: 'read',
};
function pastForm(label: string): string {
  const [first, ...rest] = label.split(' ');
  const past = pastForms[first] ?? (first.endsWith('e') ? `${first}d` : first.endsWith('y') && !/[aeiou]y$/.test(first) ? `${first.slice(0, -1)}ied` : `${first}ed`);
  return [past, ...rest].join(' ');
}

// The verbs of moving a turn may say, which the map shows on the thing that moved.
const movingWords = ['went', 'go', 'goes', 'going', 'walked', 'walks', 'ran', 'runs', 'drove', 'drives', 'flew', 'flies', 'moved', 'moves', 'travelled', 'traveled', 'journeyed', 'came', 'comes', 'returned', 'arrived', 'rode', 'sailed', 'climbed', 'jumped', 'swam'];

// The relation a deed is written under, and the word its line is named with.
const deedRelation = 'activity';
const deedLabel = 'does';

export const plainName = /^([a-z][a-z0-9'-]*(#[0-9]+)?|[0-9][0-9.]*)$/i;
// A second thing of a name under one holder is written with its number after a mark, car#2: it is a tile of its own, drawn as the plain name.
export const baseName = (name: string): string => name.replace(/#[0-9]+$/, '');
// A counted thing, a count on its own and a number, as a written path spells them.
const countedName = /^([a-z][a-z0-9'-]*):quantity\(([^)]+)\)$/i;
const amountValue = /^\{quantity(?::value\(| )([^)}]+)\)?\}$/;
const numberValue = /^\{number:value\(([^)]+)\)\}$/;
const inputMarks = ['{input start}', '{input end}'];
const marks = ['.', '?', '!'];
const skipped = [
  'the', 'a', 'an', 'is', 'are', 'was', 'were', 'has', 'have', 'had', 'in', 'on', 'to', 'of', 'at', 'and', 'it', 'what', 'who', 'where', 'how', 'which',
  'when', 'why', 'do', 'does', 'did', 'can', 'not', 'that', 'this', 'with', 'from', 'by', 'for', 'am', 'be', 'been', 'will', 'would',
];
// The words a person says of themselves: on the map they are the one tile of the user.
const selfWords = ['i', 'me', 'my', 'mine', 'myself'];
export const worldInk = '#ffd166';
// The two ways to use the input, each with what it means for him, so the choice explains itself.
export const modes = [
  { world: false, title: '💬 Talk to DAM1 LLM', tells: 'He hears it, acts on it and remembers it.' },
  { world: true, title: '🌍 Change the world', tells: 'He does not hear it. He must look to find it.' },
];
// The word for his own house: he goes there, and no tile is drawn for it.
export const groupWord = 'group';
const itemsWord = 'items';
const houseWord = 'home';
export const selfName = 'user';
export const networkNames = ['you', 'assistant', 'dam1'];
export const networkName = 'DAM1';
const articles = ['a', 'an', 'the'];
export const articleFlags = ['{a}', '{an}', '{the}'];
// How long one frame of the play-back stays at normal speed, in milliseconds.
export const frameTime = 560;
// The picture of each kind of action, shown beside him while he does it.
export const pictures = { moved: '🚶',
  next: 'nod', point: '👉', grab: '✊', give: '🤝', drop: '📥', keep: '📌', note: '🏷️', question: '❓', write: '✍️', look: '🔎', step: '👣',
  compute: '🧮', read: '👀', other: '⚙️', kept: 'shades', heart: '❤️', unseen: '🙈 not seen yet', answer: '💬', count: '×', time: '🕒', silent: '🤷',
};

export interface Place {
  name: string;
  inside: number | null;
  // Whether the tree holds this thing at its top, where a path opens at it: the map draws such a thing
  // at the top of its group as well as inside whatever holds it, and the reader walks to the top one.
  rooted?: boolean;
  region: Region;
  // Small notes on the tile: how many, when, or a flag the story set on it.
  tags: string[];
  // The article the story said the thing with, shown before its name: a clock.
  article?: string;
  // True for a thing the story tells in the past: it is drawn faded, as an old picture, with no label for it.
  past?: boolean;
  // How many of the thing there are, shown after its name: apples (3).
  count?: string;
  // True for a thing a described world made that no turn he heard has named yet: he has not seen it.
  unseen?: boolean;
  // A region the tables gave is kept; one that was guessed may change when a move shows more.
  guessed: boolean;
  // True for a thing the story named, a little car named beep: it stands on the map as itself and is never
  // drawn inside another thing, since what it does somewhere is a line and not a place.
  named?: boolean;
}

export interface Scene {
  places: Place[];
  at: number | null;
  carries: number | null;
  // A holder that has or is given a thing takes it in; anything else carried goes into where it is dropped.
  owns: boolean;
  born: number | null;
  // The tile a move points at, drawn as a line from him to it before the move is done.
  points: number | null;
  // The relations written so far, each a line between two tiles with the relation's name on it.
  links: Link[];
  // Where he stood in this turn, in order, home being none: drawn as the trail he left.
  trail: (number | null)[];
  // What he keeps in mind for the words to come, the flags his moves set: an article, a quality, a count.
  notes: string[];
  // The tiles the turn being played has named so far: they are lit, so what a person just said stands out.
  lit: number[];
  // What he is curious about: the newest thing the last sentence he heard told of.
  curious: string[];
}

interface Link {
  from: number;
  to: number;
  label: string;
  // True for a relation the story tells in the past: it reads was and is drawn faded.
  past?: boolean;
}

interface StackItem {
  text: string;
  kind: 'cursor' | 'word' | 'move' | 'pointed';
}

export interface Frame {
  turn: number;
  word: number;
  scene: Scene;
  stack: StackItem[];
  note: string;
  // What he said so far in this turn, the newest last.
  says: string[];
  // The picture of what he does at this frame, or none.
  doing: string | null;
  // True on the frame where he gives the turn's answer.
  answers: boolean;
  heardSoFar: number;
  movedSoFar: number;
  // How many turns have been written into memory when this frame shows.
  kept: number;
}

export interface GameTurn {
  // The message of the chat the turn was said in, by which a turn is taken out again.
  id: string;
  asked: string;
  heard: string[];
  steps: Steps;
  said: string;
  wrote: string[];
  // True when the turn described the world: the tree took it and he never heard it.
  world: boolean;
  // True when the turn asked something: a move of it wrote a question.
  asks: boolean;
  // When it was said, in milliseconds.
  at: number;
}

export function turnsOf(chat: Chat | null): GameTurn[] {
  if (!chat) return [];
  const turns: GameTurn[] = [];
  chat.messages.forEach((said, at) => {
    const next = chat.messages[at + 1];
    if (said.role !== 'user' || !next || next.role !== 'agent' || !next.steps) return;
    const steps = next.steps.filter(([word]) => !inputMarks.includes(word));
    turns.push({ asked: said.text, heard: steps.map(([word]) => word), steps, said: next.text, wrote: next.kind === 'noted' ? next.detail ?? [] : [], world: said.world === true, asks: steps.some(([, moves]) => moves.some((move) => move.includes('type: question'))), at: said.at, id: said.id });
  });
  return turns;
}

// True when a chat has a reply whose moves were never kept: it was read before the game stored them.
export function lacksMoves(chat: Chat | null): boolean {
  if (!chat) return false;
  return chat.messages.some((said, at) => said.role === 'user' && chat.messages[at + 1]?.role === 'agent' && !chat.messages[at + 1].steps);
}

// The frames of a game. A thing's region can show only late, when a move puts something into it or gives it
// something, so the game is walked twice: once to learn where every thing ends, then with those regions from
// the first frame, and a thing never changes region after he pointed at it.
export function framesOf(turns: GameTurn[], known: Map<string, Region>): Frame[] {
  const first = walked(turns, known);
  const ends = new Map<string, Region>(known);
  // What the network's facts say a thing is wins over what a move showed of it.
  for (const place of first[first.length - 1]?.scene.places ?? []) if (!known.has(place.name)) ends.set(place.name, place.region);
  return walked(turns, ends);
}

function walked(turns: GameTurn[], ends: Map<string, Region>): Frame[] {
  const frames: Frame[] = [];
  let scene: Scene = { places: [], at: null, carries: null, owns: false, born: null, points: null, links: [], trail: [], notes: [], lit: [], curious: [] };
  let heardSoFar = 0;
  let movedSoFar = 0;
  // The tile of a name, made when it is new: the index is worked out before the scene is written again.
  // True while a turn that described the world is walked: what it makes he has not seen.
  let describing = false;
  const tileOf = (said: string): number => {
    const name = selfWords.includes(said) ? selfName : said;
    const known = scene.places.findIndex((place) => place.name === name || singular(place.name) === singular(name));
    if (known >= 0) {
      const places = !describing && scene.places[known].unseen ? scene.places.map((place, at) => (at === known ? { ...place, unseen: false } : place)) : scene.places;
      scene = { ...scene, places, born: null, lit: scene.lit.includes(known) ? scene.lit : [...scene.lit, known] };
      return known;
    }
    const region = ends.get(baseName(name)) ?? regionOf(baseName(name));
    scene = { ...scene, places: [...scene.places, { name, inside: null, region: region ?? 'things', guessed: region === null, tags: [], unseen: describing || undefined }], born: scene.places.length, lit: [...scene.lit, scene.places.length] };
    return scene.places.length - 1;
  };
  // A guessed region gives way to what a move showed: a holder is living, a thing others are put into is a place.
  const shown = (nth: number | null, region: Region): void => {
    if (nth === null || !scene.places[nth]?.guessed) return;
    scene = { ...scene, places: scene.places.map((place, at) => (at === nth ? { ...place, region, guessed: false } : place)) };
  };

  turns.forEach((turn, turnAt) => {
    let says: string[] = [];
    // Qualities said before their thing wait here, and are joined to the thing by a line once it is put down.
    let waiting: string[] = [];
    let doing: string | null = null;
    // Everything the turn did, every word with its moves in order, shown whole when the turn has ended.
    const record: StackItem[] = [];
    scene = { ...scene, trail: [scene.at], lit: [] };
    const track = (): void => {
      if (scene.trail[scene.trail.length - 1] !== scene.at) scene = { ...scene, trail: [...scene.trail, scene.at] };
    };
    const say = (text: string): void => { says = [text]; doing = null; };
    // An action is a picture beside him, not words over his head.
    const act = (picture: string): void => { doing = picture; };
    describing = turn.world;
    // True at the start of a sentence: he still stands where the last one left him, and its first thing calls him over.
    let opened = true;
    turn.steps.forEach(([word, moves], at) => {
      const stack: StackItem[] = [{ text: 'cursor', kind: 'cursor' }, { text: word, kind: 'word' }];
      record.push({ text: word, kind: 'word' });
      const mark = marks.includes(word);
      heardSoFar += 1;
      if (!turn.world && !mark) {
        const named = selfWords.includes(word) ? selfName : word;
        const found = scene.places.findIndex((place) => place.unseen && (place.name === named || singular(place.name) === singular(named)));
        if (found >= 0) scene = { ...scene, places: scene.places.map((place, nth) => (nth === found ? { ...place, unseen: false } : place)), lit: scene.lit.includes(found) ? scene.lit : [...scene.lit, found] };
      }
      scene = { ...scene, born: null, points: null };
      // A word that names a thing with no move of its own still makes the thing appear: the walker goes to it.
      const silent = moves.every((taken) => taken.split('->')[0] === continueMove);
      if (word === houseWord && !turn.world) {
        scene = { ...scene, at: null };
        say(`"${word}"`);
      } else if (silent && !mark && !skipped.includes(word) && (scene.at === null || opened) && scene.carries === null) {
        const tile = tileOf(word);
        scene = { ...scene, at: tile };
        if (selfWords.includes(word)) says = []; else say(`"${word}"`);
      } else {
        if (selfWords.includes(word)) says = []; else say(`"${word}"`);
      }
      const push = (note: string, answers = false): void => {
        track();
        frames.push({ turn: turnAt, word: at, scene, stack: [...stack], note, says, doing, answers, heardSoFar, movedSoFar, kept: turnAt });
      };
      push(`hears "${word}" on a fresh stack`);
      if (!mark && !skipped.includes(word)) opened = false;
      for (const taken of moves) {
        const [move, pointed] = taken.split('->');
        movedSoFar += 1;
        if (move === continueMove) {
          if (mark) { scene = { ...scene, carries: null, owns: false, born: null, points: null, notes: [] }; opened = true; }
          if (!mark) act(pictures.next); else doing = null;
          push(mark ? 'continues, and stays where the talk was' : 'continues to the next word');
          continue;
        }
        const named = pointed ?? word;
        // A move that points is shown first as a line to the thing pointed at, and says nothing of its own.
        // A relation word, or a word such as is or was written as the verb of a quality, is no thing of its own.
        const relation = move.includes('type: relation') || move.includes('type: quantity') || (move.includes('children add') && skipped.includes(named)) || (named === houseWord && !turn.world);
        // A word of the question, what or who, is no thing: pointing at it draws no tile that would stay on the map.
        const asksOnly = pointed !== undefined && pointed !== null && skipped.includes(pointed) && !scene.places.some((place) => place.name === pointed);
        if (pointed && !relation && !marks.includes(pointed) && !asksOnly) {
          const aimed = tileOf(pointed);
          scene = { ...scene, points: aimed };
          frames.push({ turn: turnAt, word: at, scene, stack: [...stack, { text: move, kind: 'move' }, { text: pointed, kind: 'pointed' }], note: `points at "${pointed}"`, says, doing: pictures.point, answers: false, heardSoFar, movedSoFar, kept: turnAt });
          scene = { ...scene, points: null, born: null };
        }
        if (move.includes('grab') || move === '{give}' || move.includes('hand')) {
          const owns = move === '{give}';
          if (owns) shown(scene.at, 'people');
          scene = { ...scene, carries: scene.at, owns, born: null };
          act(owns ? pictures.give : pictures.grab);
        } else if (move.includes('drop')) {
          const target = tileOf(named);
          const carried = scene.carries;
          const owns = scene.owns;
          if (carried !== null && carried !== target && !owns) shown(target, 'places');
          const places = scene.places.map((place, nth) => {
            if (carried === null || carried === target) return place;
            if (owns) return nth === target ? { ...place, inside: carried } : place;
            return nth === carried ? { ...place, inside: target } : place;
          });
          act(pictures.drop);
          scene = { ...scene, places, at: owns && carried !== null ? carried : target, carries: null, owns: false };
          for (const quality of waiting) {
            // A quality is no part of the thing: it stands in its own group and a line says the thing is so.
            const tile = tileOf(quality);
            if (tile !== target && !scene.links.some((link) => link.from === target && link.to === tile)) scene = { ...scene, links: [...scene.links, { from: target, to: tile, label: 'is' }] };
          }
          waiting = [];
          scene = { ...scene, notes: [] };
        } else if (move.includes('setFlag') && pointed) {
          scene = { ...scene, notes: [...scene.notes, pointed] };
          if (!move.includes('type: quantity')) waiting = [...waiting, pointed];
          scene = { ...scene, born: null };
          act(pictures.keep);
        } else if (move.includes('setFlag') && !move.includes('type: question')) {
          scene = { ...scene, notes: [...scene.notes, word] };
          scene = { ...scene, born: null };
          act(pictures.note);
        } else if (move.includes('type: question')) {
          scene = { ...scene, born: null };
          act(pictures.question);
        } else if (relation) {
          // A relation word is no thing of its own: it becomes the name on a line once the sentence is kept.
          scene = { ...scene, born: null };
          act(pictures.write);
        } else if (move.includes('children add') || move.includes('setProperty')) {
          const tile = tileOf(named);
          const holder = scene.at;
          const fresh = scene.born === tile;
          const places = scene.places.map((place, nth) => (nth === tile && holder !== null && holder !== tile && place.inside === null && fresh ? { ...place, inside: holder } : place));
          scene = { ...scene, places, at: tile };
          act(pictures.write);
        } else if (move.includes('find') || move.includes('step')) {
          const tile = pointed && !marks.includes(pointed) ? tileOf(pointed) : scene.at;
          scene = { ...scene, at: tile };
          // Going to the person playing is a heart, not a search.
          act(move.includes('step user') || (tile !== null && scene.places[tile]?.name === selfName) ? pictures.heart : pointed ? pictures.look : pictures.step);
        } else if (move.includes('compute') || move.includes('number')) {
          scene = { ...scene, born: null };
          act(pictures.compute);
        } else if (move.includes('get') || move.includes('check')) {
          scene = { ...scene, born: null };
          act(pictures.read);
        } else {
          scene = { ...scene, born: null };
          act(pictures.other);
        }
        stack.push({ text: move, kind: 'move' });
        record.push({ text: move, kind: 'move' });
        if (pointed) { stack.push({ text: pointed, kind: 'pointed' }); record.push({ text: pointed, kind: 'pointed' }); }
        push(pointed ? `${move} pointing at "${pointed}"` : move);
      }
    });
    // What the turn wrote is drawn from its paths. Two names with braced names between them are joined by a
    // line with those names on it. A name right after another stands inside it. A count, and what ends a path
    // with no thing after it, a time or a flag, is a tag on the tile.
    const tag = (nth: number, text: string): void => {
      if (scene.places[nth].tags.includes(text)) return;
      scene = { ...scene, places: scene.places.map((place, at) => (at === nth ? { ...place, tags: [...place.tags, text] } : place)) };
    };
    const counts = (nth: number, amount: string): void => {
      scene = { ...scene, places: scene.places.map((place, at) => (at === nth ? { ...place, count: amount } : place)) };
    };
    const within = (inner: number, outer: number): boolean => {
      for (let at: number | null = outer, hops = 0; at !== null && hops <= scene.places.length; at = scene.places[at].inside, hops += 1) if (at === inner) return true;
      return false;
    };
    // The things whose is this turn tells in the past, and the links this turn drew, to tell which was meant.
    const pastIs: number[] = [];
    const linksBefore = scene.links.length;
    // What the tree says stands in what: a plain name straight after another, with no braced name between them.
    // A path opens at a thing of its own, so a thing that opens a path stands in nothing, whatever the walk did
    // while it was being read.
    const holds = new Set<string>();
    for (const path of turn.wrote) {
      let above: string | null = null;
      for (const part of partsOf(path)) {
        if (part.startsWith('{')) { above = null; continue; }
        const only = part.match(countedName);
        const name = only ? only[1] : part;
        if (!plainName.test(name)) { above = null; continue; }
        if (above !== null) holds.add(`${singular(name)}>${singular(above)}`);
        above = name;
      }
    }
    const freed = new Set<string>();
    for (const path of turn.wrote) {
      const first = partsOf(path)[0];
      if (first === undefined) continue;
      // A path opens at a thing or at the group the story made, which is a thing of its own.
      if (first.startsWith('{')) {
        if (first.replace(/[{}]/g, '') === groupWord) freed.add(singular(groupWord));
        continue;
      }
      const only = first.match(countedName);
      const name = only ? only[1] : first;
      if (plainName.test(name)) freed.add(singular(name));
    }
    scene = { ...scene, places: scene.places.map((place) => {
      if (!freed.has(singular(place.name))) return place;
      const top = { ...place, rooted: true };
      if (top.inside === null) return top;
      const holder = scene.places[top.inside];
      return holds.has(`${singular(place.name)}>${singular(holder.name)}`) ? top : { ...top, inside: null };
    }) };
    for (const path of turn.wrote) {
      const parts = partsOf(path);
      let from: number | null = null;
      let label: string[] = [];
      // The line the path's last two things are joined by, when there is one: a time after them is that line's.
      let lastLink: number | null = null;
      // Whether the last thing of the path stands with its owner, written owner has thing: a past after it is the owning's.
      let owned = false;
      for (const part of parts) {
        const counted = part.match(countedName);
        const amount = part.match(amountValue);
        const worth = part.match(numberValue);
        if (amount) { if (from !== null) counts(from, amount[1]); continue; }
        const braced = part.startsWith('{') && !worth;
        // The group the story made is a thing of its own, so the path opens at its tile and what it holds
        // stands inside it, as a thing stands in a place.
        if (braced && part.replace(/[{}]/g, '') === groupWord && from === null) { from = tileOf(groupWord); continue; }
        if (braced && part.replace(/[{}]/g, '') === itemsWord && from !== null) { label = []; continue; }
        if (braced) { label.push(part.replace(/[{}]/g, '')); continue; }
        // A flag set true is no thing: an article goes before the name, any other flag is a tag.
        if (part === 'true' && from !== null && label.length > 0) {
          const flagged = from;
          const flag = label.join(' ');
          if (articles.includes(flag)) scene = { ...scene, places: scene.places.map((place, at) => (at === flagged ? { ...place, article: flag } : place)) };
          else tag(flagged, flag);
          label = [];
          continue;
        }
        if (part === houseWord && parts.length === 1) continue;
        const name = counted ? counted[1] : worth ? worth[1] : part;
        if (!counted && !worth && !plainName.test(part)) { from = null; label = []; continue; }
        // How much of a quality a thing has is written on the quality itself, very or a bit, and draws no tile of its own.
        if (from !== null && label.join(' ') === 'degree') { tag(from, name); label = []; continue; }
        // A thing the story gives a name takes that name: the car named beep is drawn as beep, and what the car
        // is or does is drawn on it, since the two are one thing.
        if (from !== null && label.join(' ') === 'name') {
          const named = from;
          const gone = scene.places.findIndex((place, at) => at !== named && (place.name === name || singular(place.name) === singular(name)));
          scene = { ...scene, places: scene.places.map((place, at) => (at === named ? { ...place, name, named: true, inside: null } : place)) };
          if (gone >= 0) {
            const kept = (at: number): number => (at === gone ? named : at);
            scene = {
              ...scene,
              places: scene.places.filter((_, at) => at !== gone).map((place) => ({ ...place, inside: place.inside === null ? null : kept(place.inside) })),
              links: scene.links.map((link) => ({ ...link, from: kept(link.from), to: kept(link.to) })).filter((link) => link.from !== link.to),
              lit: [...new Set(scene.lit.map(kept))],
            };
          }
          label = [];
          continue;
        }
        const tile = tileOf(name);
        if (counted) counts(tile, counted[2]);
        if (from !== null && from !== tile) {
          const holder: number = from;
          if (label.length === 0) {
            if (scene.places[tile].inside === null && !scene.places[tile].named && !within(tile, holder)) scene = { ...scene, places: scene.places.map((place, at) => (at === tile ? { ...place, inside: holder } : place)) };
            // How the thing came to be there, when the turn said it with a verb of moving: john went to the store.
            const came = turn.heard.find((word) => movingWords.includes(word.toLowerCase()));
            if (came && scene.places[tile].inside === holder) {
              const moving = tile;
              scene = { ...scene, places: scene.places.map((place, at) => (at === moving ? { ...place, tags: [...place.tags.filter((text) => !text.startsWith(pictures.moved)), `${pictures.moved} ${came.toLowerCase()}`] } : place)) };
            }
          } else if (label.join(' ') === 'of') {
            // What a thing is of stands inside it, the cars of a five car garage, wherever the replay had put it before.
            if (!within(tile, holder) && !scene.places[tile].named) scene = { ...scene, places: scene.places.map((place, at) => (at === tile ? { ...place, inside: holder } : place)) };
          } else {
            // A thing that stands inside its owner, or inside what its owner stands in, needs no owner line: being there says it.
            const nested = scene.places[holder].inside === tile || scene.places[tile].inside === holder || (label.join(' ') === 'owner' && within(tile, holder));
            const text = label.join(' ') === deedRelation ? deedLabel : label.join(' ');
            owned = text === 'has' && nested;
            const plainer = scene.links.findIndex((link) => link.from === holder && link.to === tile && link.label !== text && text.startsWith(`${link.label} `));
            if (plainer >= 0) scene = { ...scene, links: scene.links.map((link, at) => (at === plainer ? { ...link, label: text } : link)) };
            else if (!nested && !scene.links.some((link) => link.from === holder && link.to === tile && (link.label === text || link.label === pastForm(text)))) scene = { ...scene, links: [...scene.links, { from: holder, to: tile, label: text }] };
            lastLink = nested ? null : scene.links.findIndex((link) => link.from === holder && link.to === tile && (link.label === text || link.label === pastForm(text)));
          }
        }
        if (from === null || from === tile || label.length === 0) lastLink = null;
        from = tile;
        label = [];
      }
      // What is left after the last thing says something of it: its time, or a flag.
      const said = label.filter((one) => one !== 'is');
      // An owner told in the past, john had a box: the thing carries had, and is still drawn as it stands.
      if (from !== null && ((said[0] === 'owner' && said[1] === 'time' && said[2] === 'past') || (owned && said[0] === 'time' && said[1] === 'past'))) tag(from, `${pictures.time} had`);
      else if (from !== null && said[0] === 'time' && said[1] === 'past' && lastLink !== null && lastLink >= 0 && !label.includes('is')) {
        const told = lastLink;
        scene = { ...scene, links: scene.links.map((link, at) => (at === told && !link.past ? { ...link, label: pastForm(link.label), past: true } : link)) };
      } else if (from !== null && said[0] === 'time' && said[1] === 'past') {
        const old = from;
        if (label.includes('is')) pastIs.push(old);
        else scene = { ...scene, places: scene.places.map((place, at) => (at === old ? { ...place, past: true } : place)) };
      } else if (from !== null && said.length > 0) tag(from, said[0] === 'time' ? `${pictures.time} when: ${said.slice(1).join(' ')}` : said.join(' '));
    }
    // A thing this turn's paths open at is one the tree holds at its top, marked once the paths have made
    // their tiles, so a thing the turn itself named is marked too.
    scene = { ...scene, places: scene.places.map((place) => (freed.has(singular(place.name)) && place.rooted !== true ? { ...place, rooted: true } : place)) };
    // A past is with a line of this turn is that line, sophia was an angel; with no line it is the thing as it
    // stood then, alice was in wonderland, and the thing is drawn faded.
    for (const old of pastIs) {
      const drawn = scene.links.some((link, at) => at >= linksBefore && link.from === old && link.label === 'is');
      if (drawn) scene = { ...scene, links: scene.links.map((link, at) => (at >= linksBefore && link.from === old && link.label === 'is' ? { ...link, label: 'was', past: true } : link)) };
      else scene = { ...scene, places: scene.places.map((place, at) => (at === old ? { ...place, past: true } : place)) };
    }
    scene = { ...scene, born: null };
    track();
    if (!turn.world && turn.wrote.length > 0) {
      // He is curious about the newest thing he was told of, as he says when asked: of the things the turn's paths
      // name before any relation, a holder and what stands in it, the one that came to the map last.
      const told = turn.wrote.flatMap((path) => { const parts = partsOf(path); const upTo = parts.findIndex((part) => part.startsWith('{')); return (upTo < 0 ? parts : parts.slice(0, upTo)).filter((part) => plainName.test(part)); });
      const newest = told.map((name) => scene.places.findIndex((place) => place.name === name || singular(place.name) === singular(name))).reduce((most, at) => Math.max(most, at), -1);
      scene = { ...scene, curious: newest >= 0 ? [scene.places[newest].name] : [] };
    }
    const last = Math.max(turn.steps.length - 1, 0);
    // A statement he kept is said with no words: he puts his sunglasses on.
    const cool = !turn.said && turn.wrote.length > 0 && !turn.world;
    says = turn.said || cool || turn.world || turn.asks ? [] : [pictures.silent];
    frames.push({
      turn: turnAt, word: last, scene, stack: record, note: turn.said ? `says "${turn.said}"` : 'says nothing: a statement is kept, not answered',
      says, doing: cool ? pictures.kept : turn.said ? pictures.answer : null, answers: true, heardSoFar, movedSoFar, kept: turnAt + 1,
    });
    describing = false;
  });
  return frames;
}
