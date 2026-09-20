// The picture a common name is drawn with on the explorer's map, the region of the map it belongs to, and the
// color a name is drawn in. A name no table knows has no picture and is drawn by its letters alone. A plural
// finds its singular.
export type Region = 'people' | 'animals' | 'food' | 'things' | 'places' | 'nature' | 'qualities' | 'numbers' | 'doing' | 'time';

const people: Record<string, string> = {
  tom: '👦', bob: '👨', sam: '🧑', ben: '👦', dan: '👨', max: '👦', john: '👨', jack: '👦', michael: '👨', peter: '👨', paul: '👨', david: '👨', greg: '👨',
  ann: '👧', anna: '👩', mary: '👩', sue: '👩', kim: '👧', emma: '👩', alice: '👧', sally: '👩', lucy: '👧', jane: '👩', kate: '👩', lisa: '👩', sara: '👩',
  he: '👨', she: '👩', they: '🧑‍🤝‍🧑', we: '🧑‍🤝‍🧑', me: '🧑', my: '🧑',
  group: '🧑‍🤝‍🧑',
  user: '🧑', i: '🧑', you: '🤖', assistant: '🤖', man: '👨', woman: '👩', boy: '👦', girl: '👧', baby: '👶', child: '🧒', king: '🤴', queen: '👸',
  mother: '👩', father: '👨', sister: '👧', brother: '👦', friend: '🧑‍🤝‍🧑', teacher: '🧑‍🏫', doctor: '🧑‍⚕️', nurse: '🧑‍⚕️', farmer: '🧑‍🌾', cook: '🧑‍🍳', chef: '🧑‍🍳',
  pilot: '🧑‍✈️', baker: '🧑‍🍳', guard: '💂', student: '🧑‍🎓', painter: '🧑‍🎨', police: '👮',
};

const animals: Record<string, string> = {
  rat: '🐀', squirrel: '🐿️', deer: '🦌', donkey: '🫏', bull: '🐂', ox: '🐂', lamb: '🐑', rooster: '🐓', turkey: '🦃', pigeon: '🕊️', dove: '🕊️', seal: '🦭', otter: '🦦',
  beaver: '🦫', hedgehog: '🦔', koala: '🐨', panda: '🐼', kangaroo: '🦘', gorilla: '🦍', hippo: '🦛', rhino: '🦏', leopard: '🐆', crocodile: '🐊', lizard: '🦎', dinosaur: '🦖',
  crab: '🦀', lobster: '🦞', shrimp: '🦐', squid: '🦑', jellyfish: '🪼', worm: '🪱', mosquito: '🦟', beetle: '🪲', ladybug: '🐞', cricket: '🦗', scorpion: '🦂',
  peacock: '🦚', flamingo: '🦩', pet: '🐾', animal: '🐾', insect: '🐞', mammal: '🐾', reptile: '🦎',
  dog: '🐶', cat: '🐱', bird: '🐦', fish: '🐟', horse: '🐴', cow: '🐮', pig: '🐷', sheep: '🐑', hen: '🐔', chicken: '🐔', duck: '🦆', goose: '🦢', swan: '🦢',
  mouse: '🐭', rabbit: '🐰', fox: '🦊', wolf: '🐺', bear: '🐻', lion: '🦁', tiger: '🐯', monkey: '🐵', elephant: '🐘', giraffe: '🦒', zebra: '🦓', camel: '🐫',
  whale: '🐳', shark: '🦈', dolphin: '🐬', octopus: '🐙', frog: '🐸', snake: '🐍', turtle: '🐢', spider: '🕷️', bee: '🐝', ant: '🐜', butterfly: '🦋', owl: '🦉',
  eagle: '🦅', penguin: '🐧', bat: '🦇', snail: '🐌', hamster: '🐹', kitten: '🐱', puppy: '🐶', parrot: '🦜', crow: '🐦‍⬛', hound: '🐕', hare: '🐇', goat: '🐐',
};

const food: Record<string, string> = {
  apple: '🍎', banana: '🍌', orange: '🍊', lemon: '🍋', grape: '🍇', pear: '🍐', cherry: '🍒', strawberry: '🍓', plum: '🍑', tomato: '🍅', carrot: '🥕',
  potato: '🥔', onion: '🧅', corn: '🌽', bread: '🍞', cheese: '🧀', egg: '🥚', milk: '🥛', tea: '🍵', coffee: '☕', water: '💧', cake: '🍰', pizza: '🍕',
  cookie: '🍪', sweet: '🍬', honey: '🍯', rice: '🍚', meat: '🍖', nut: '🥜', food: '🍽️', fruit: '🍎', juice: '🧃', soup: '🍲',
};

const things: Record<string, string> = {
  car: '🚗', bus: '🚌', truck: '🚚', van: '🚐', bike: '🚲', train: '🚆', plane: '✈️', jet: '✈️', boat: '⛵', ship: '🚢', raft: '🛶', tractor: '🚜', rocket: '🚀',
  ball: '⚽', kite: '🪁', book: '📕', pen: '🖊️', pencil: '✏️', key: '🔑', lamp: '💡', clock: '🕰️', watch: '⌚', phone: '📱', bag: '👜', box: '📦', cup: '☕',
  bottle: '🍾', hat: '🎩', shirt: '👕', shoe: '👟', sock: '🧦', coat: '🧥', ring: '💍', crown: '👑', coin: '🪙', dollar: '💵', money: '💰', card: '🃏',
  stamp: '📮', letter: '✉️', bell: '🔔', drum: '🥁', guitar: '🎸', piano: '🎹', hammer: '🔨', saw: '🪚', knife: '🔪', brush: '🖌️', broom: '🧹', bed: '🛏️',
  chair: '🪑', table: '🪑', sofa: '🛋️', door: '🚪', window: '🪟', fridge: '🧊', stove: '🍳', bath: '🛁', mug: '☕', vase: '🏺', flag: '🚩', rope: '🪢',
  umbrella: '☂️', scarf: '🧣', doll: '🪆', toy: '🧸', marble: '🔮', sticker: '🏷️', shell: '🐚', stone: '🪨', rock: '🪨', net: '🥅', wheel: '🛞', ticket: '🎫',
  wallet: '👛', drill: '🛠️', pump: '⛽', hose: '🚿', heater: '🔥', fan: '🌀', scale: '⚖️', ruler: '📏', thermometer: '🌡️', computer: '💻', robot: '🤖',
};

const places: Record<string, string> = {
  house: '🏠', home: '🏠', school: '🏫', hospital: '🏥', shop: '🏪', store: '🏪', bank: '🏦', library: '📚', office: '🏢', kitchen: '🍳', bedroom: '🛏️',
  bathroom: '🛁', garden: '🌷', park: '🌳', farm: '🚜', zoo: '🦁', forest: '🌲', jungle: '🌴', desert: '🏜️', mountain: '⛰️', beach: '🏖️', sea: '🌊', ocean: '🌊',
  lake: '🏞️', river: '🏞️', pond: '🏞️', island: '🏝️', city: '🏙️', town: '🏘️', street: '🛣️', road: '🛣️', bridge: '🌉', castle: '🏰', station: '🚉',
  airport: '🛫', harbour: '⚓', garage: '🅿️', hall: '🚪', yard: '🏡', cave: '🕳️', field: '🌾', meadow: '🌾', world: '🌍', earth: '🌍', moon: '🌙', sun: '☀️',
  france: '🇫🇷', paris: '🗼', england: '🇬🇧', london: '🎡', italy: '🇮🇹', rome: '🏛️', spain: '🇪🇸', germany: '🇩🇪', japan: '🇯🇵', tokyo: '🗾', china: '🇨🇳',
  america: '🇺🇸', canada: '🇨🇦', egypt: '🇪🇬',
};

const nature: Record<string, string> = {
  star: '⭐', sky: '🌌', cloud: '☁️', rain: '🌧️', snow: '❄️', ice: '🧊', fire: '🔥', rainbow: '🌈', tree: '🌳', flower: '🌸', rose: '🌹', leaf: '🍃', grass: '🌱',
};

const qualities: Record<string, string> = {
  sunny: '☀️', rainy: '🌧️', cloudy: '☁️', windy: '🌬️', snowy: '🌨️', stormy: '⛈️', foggy: '🌫️', warm: '🌡️', cool: '🧊', dark: '🌑', bright: '🔆', wet: '💦', dry: '🏜️',
  old: '👴', young: '🧒', new: '✨', tall: '📏', short: '📐', heavy: '🏋️', light: '🪶', strong: '💪', weak: '🥀', rich: '💰', poor: '🪙', kind: '💗', nice: '💗', mean: '😈',
  good: '👍', bad: '👎', clean: '🧼', dirty: '🧹', full: '🈵', empty: '🫙', open: '🔓', closed: '🔒', loud: '📢', quiet: '🤫', soft: '🧸', hard: '🪨', sweet: '🍬', sour: '🍋',
  red: '🔴', blue: '🔵', green: '🟢', yellow: '🟡', purple: '🟣', black: '⚫', white: '⚪', brown: '🟤', big: '🐘', small: '🐜',
  hot: '🥵', cold: '🥶', happy: '😊', sad: '😢', angry: '😠', tired: '😴', hungry: '🍽️', fast: '⚡', slow: '🐌', gold: '🥇', silver: '🥈',
};

// Words of doing, by their plain form: a picture of the deed, so a card that reads measures is a ruler at a glance.
const doing: Record<string, string> = {
  measure: '📏', display: '🖥️', show: '👁️', tell: '🗣️', say: '🗣️', ask: '❓', read: '📖', write: '✍️', draw: '🖍️', paint: '🎨', sing: '🎤', play: '🎮', dance: '💃',
  dancing: '💃', run: '🏃', running: '🏃', walk: '🚶', walking: '🚶', jump: '🤸', swim: '🏊', swimming: '🏊', fly: '🪽', drive: '🚘', ride: '🚴', climb: '🧗', sleep: '😴',
  sleeping: '😴', eat: '🍴', eating: '🍴', drink: '🥤', cook: '🍳', cooking: '🍳', bake: '🥧', buy: '🛒', bought: '🛒', sell: '🏷️', sold: '🏷️', pay: '💳', cost: '💲',
  give: '🎁', gave: '🎁', take: '🫳', took: '🫳', find: '🔎', found: '🔎', lose: '🕳️', lost: '🕳️', make: '🛠️', made: '🛠️', build: '🏗️', built: '🏗️', fix: '🔧', cut: '✂️',
  open: '🔓', close: '🔒', push: '👉', pull: '🫷', carry: '🎒', hold: '🤲', throw: '🤾', catch: '🧤', kick: '🦵', hit: '👊', help: '🤝', love: '❤️', like: '👍',
  hate: '💔', want: '🙏', need: '❗', know: '🧠', think: '💭', learn: '🎓', teach: '🧑‍🏫', work: '💼', live: '🏡', go: '➡️', went: '➡️', come: '⬅️', came: '⬅️',
  see: '👀', saw: '👀', look: '👀', hear: '👂', heard: '👂', call: '📞', meet: '🤝', met: '🤝', wait: '⏳', start: '▶️', stop: '⏹️', grow: '🌱', hunt: '🏹',
  chase: '🏃', follow: '👣', orbit: '🪐', rise: '🌅', set: '🌇', rain: '🌧️', raining: '🌧️', laugh: '😂', cry: '😭', smile: '😊', wash: '🧼', clean: '🧽', travel: '🧳',
};

// Words of when: days, parts of the day, months and seasons.
const time: Record<string, string> = {
  time: '⏳', always: '♾️', never: '🚫', often: '🔁', sometimes: '🔁', soon: '⏩', later: '⏩', early: '🌅', late: '🌙', before: '⏪', after: '⏩', today: '📅', tomorrow: '📅', yesterday: '📅', now: '⏱️', morning: '🌅', noon: '🕛', afternoon: '🌤️', evening: '🌆', night: '🌙', midnight: '🕛', week: '🗓️', weekend: '🗓️',
  month: '🗓️', year: '🗓️', hour: '🕐', minute: '⏱️', second: '⏱️', monday: '📅', tuesday: '📅', wednesday: '📅', thursday: '📅', friday: '📅', saturday: '📅', sunday: '📅',
  january: '🗓️', february: '🗓️', march: '🗓️', april: '🗓️', may: '🗓️', june: '🗓️', july: '🗓️', august: '🗓️', september: '🗓️', october: '🗓️', november: '🗓️', december: '🗓️',
  spring: '🌷', summer: '🏖️', autumn: '🍂', fall: '🍂', winter: '⛄', birthday: '🎂', christmas: '🎄',
};

const tables: [Region, Record<string, string>][] = [['time', time], ['people', people], ['animals', animals], ['food', food], ['things', things], ['places', places], ['nature', nature], ['qualities', qualities], ['doing', doing]];

// Places of the wild and the sky belong with nature, though their pictures are listed with the places.
const wild = ['garden', 'park', 'forest', 'jungle', 'desert', 'mountain', 'beach', 'sea', 'ocean', 'lake', 'river', 'pond', 'island', 'cave', 'field', 'meadow', 'world', 'earth', 'moon', 'sun'];
const numberWords = ['all', 'every', 'each', 'some', 'many', 'much', 'few', 'most', 'none', 'both', 'several', 'more', 'less', 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'ten', 'eleven', 'twelve', 'twenty', 'thirty', 'forty', 'fifty', 'hundred', 'thousand', 'million', 'half', 'quarter', 'dozen', 'pair'];

function known(name: string): [Region, string] | null {
  const plain = name.toLowerCase();
  const forms = [plain, plain.replace(/ies$/, 'y'), plain.replace(/es$/, ''), plain.replace(/s$/, '')];
  for (const form of forms) {
    for (const [region, table] of tables) if (Object.hasOwn(table, form)) return [wild.includes(form) ? 'nature' : region, table[form]];
  }
  return null;
}

export function pictureOf(name: string): string | null {
  return known(name)?.[1] ?? null;
}

// The region a name belongs to by what it is: a number or a name no table knows is left to the caller, which
// sees what the moves did with it.
const superlatives = ['best', 'worst', 'most', 'least'];
const notSuperlative = ['forest', 'harvest', 'contest', 'interest', 'priest', 'request', 'protest', 'suggest', 'digest', 'invest', 'arrest'];

export function regionOf(name: string): Region | null {
  if (/^[0-9]/.test(name) || numberWords.includes(name.toLowerCase())) return 'numbers';
  const region = known(name)?.[0] ?? null;
  if (region) return region;
  // A word that names a kind stands with its kind: a person with the people, an animal with the animals.
  const asKind = regionOfKinds([name.toLowerCase()]);
  if (asKind) return asKind;
  // A superlative tells how a thing is among others and is no thing: the best, the tallest.
  const word = name.toLowerCase();
  if (superlatives.includes(word) || (word.length > 5 && word.endsWith('est') && !notSuperlative.includes(word))) return 'qualities';
  return null;
}

// A color of its own for every name, the same each time, from the letters of the name.
export function colorOf(name: string): string {
  let sum = 7;
  for (const letter of name) sum = (sum * 31 + letter.charCodeAt(0)) % 360;
  return `hsl(${sum} 80% 68%)`;
}

// The group a kind belongs to, for the kinds the network's facts give a thing: the page asks the engine what a
// thing is and groups it by the first kind it knows here, so a rat stands with the animals because the facts
// say a rat is a rodent and a rodent is a mammal.
const kindRegions: [Region, string[]][] = [
  ['people', ['person', 'human', 'people', 'man', 'woman', 'job', 'worker', 'kin', 'relative', 'scientist', 'writer', 'artist', 'king', 'queen', 'leader', 'athlete', 'composer', 'painter', 'explorer', 'inventor']],
  ['animals', ['animal', 'mammal', 'bird', 'fish', 'insect', 'reptile', 'rodent', 'amphibian', 'pet', 'primate', 'marsupial', 'dog', 'cat', 'snake', 'lizard', 'seabird']],
  ['food', ['food', 'fruit', 'vegetable', 'drink', 'meal', 'grain', 'dessert', 'meat', 'spice', 'nut', 'dairy']],
  ['places', ['place', 'city', 'country', 'continent', 'building', 'room', 'capital', 'town', 'state', 'village', 'landmark', 'museum', 'island', 'region']],
  ['nature', ['planet', 'star', 'plant', 'tree', 'flower', 'weather', 'river', 'mountain', 'ocean', 'sea', 'lake', 'desert', 'satellite', 'element', 'metal', 'gas', 'liquid', 'solid', 'mineral', 'nature', 'water', 'matter', 'energy']],
  ['time', ['time', 'day', 'month', 'season', 'year', 'holiday']],
  ['qualities', ['color', 'size', 'feeling', 'speed', 'temperature', 'age', 'quality', 'shape', 'material', 'trait']],
  ['numbers', ['number']],
  ['doing', ['sport', 'game', 'activity']],
];

export function regionOfKinds(kinds: string[]): Region | null {
  for (const kind of kinds) {
    for (const [region, names] of kindRegions) if (names.includes(kind)) return region;
  }
  return null;
}

// The picture of a kind, for a thing that has no picture of its own: every word the network's facts give a kind
// is drawn with that kind's picture, so rome is a city and mozart a composer at a glance.
const kindPictures: Record<string, string> = {
  person: '🧑', human: '🧑', man: '👨', woman: '👩', king: '🤴', queen: '👸', emperor: '👑', president: '🎖️', leader: '🎖️', general: '🎖️', explorer: '🧭', sailor: '⚓',
  scientist: '🔬', physicist: '⚛️', chemist: '⚗️', biologist: '🧬', mathematician: '📐', astronomer: '🔭', philosopher: '🤔', inventor: '💡', doctor: '🧑‍⚕️',
  writer: '✍️', poet: '🪶', author: '✍️', novelist: '📚', playwright: '🎭', painter: '🎨', sculptor: '🗿', artist: '🎨', composer: '🎼', musician: '🎵', singer: '🎤',
  actor: '🎭', actress: '🎭', director: '🎬', athlete: '🏅', player: '🏅', footballer: '⚽', boxer: '🥊', runner: '🏃', swimmer: '🏊', astronaut: '🧑‍🚀', pilot: '🧑‍✈️',
  god: '⚡', goddess: '⚡', hero: '🦸', prophet: '📜', saint: '😇', job: '💼',
  animal: '🐾', mammal: '🐾', bird: '🐦', fish: '🐟', insect: '🐞', reptile: '🦎', amphibian: '🐸', rodent: '🐭', primate: '🐒', marsupial: '🦘', seabird: '🕊️', dinosaur: '🦖', pet: '🐾',
  food: '🍽️', fruit: '🍎', vegetable: '🥕', drink: '🥤', grain: '🌾', meal: '🍽️', dessert: '🍰', meat: '🍖', spice: '🌶️', nut: '🥜', dairy: '🥛',
  place: '📍', city: '🏙️', capital: '🏛️', country: '🗺️', continent: '🌍', state: '🗺️', region: '🗺️', town: '🏘️', village: '🏡', island: '🏝️', building: '🏢', room: '🚪',
  landmark: '🗽', museum: '🏛️', tower: '🗼', bridge: '🌉', palace: '🏰', castle: '🏰', temple: '🛕', church: '⛪', university: '🎓', company: '🏢', website: '🌐',
  planet: '🪐', star: '⭐', moon: '🌙', satellite: '🛰️', galaxy: '🌌', comet: '☄️', constellation: '✨', river: '🏞️', mountain: '⛰️', volcano: '🌋', ocean: '🌊', sea: '🌊',
  lake: '🏞️', desert: '🏜️', forest: '🌲', plant: '🌿', tree: '🌳', flower: '🌸', weather: '⛅', element: '⚗️', metal: '⛏️', gas: '💨', liquid: '💧', solid: '🧱', mineral: '💎',
  matter: '⚛️', energy: '⚡', water: '💧', nature: '🍃', organ: '🫀', bone: '🦴',
  time: '⏳', day: '📅', month: '🗓️', season: '🍂', year: '🗓️', holiday: '🎉', festival: '🎉', event: '📜', battle: '⚔️', war: '⚔️', revolution: '✊', treaty: '📜',
  color: '🎨', size: '📏', feeling: '💭', speed: '💨', temperature: '🌡️', age: '⌛', quality: '✨', shape: '🔷', material: '🧵',
  number: '🔢', letter: '🔤', word: '🔤', language: '🗣️', currency: '💱', money: '💰', sport: '⚽', game: '🎲', activity: '🎯', instrument: '🎻', tool: '🛠️', vehicle: '🚗',
  machine: '⚙️', device: '📟', invention: '💡', book: '📕', novel: '📕', play: '🎭', poem: '📜', epic: '📜', painting: '🖼️', opera: '🎭', symphony: '🎼', song: '🎵', film: '🎬',
  team: '🏟️', club: '🏟️', party: '🏛️', empire: '👑', kingdom: '👑', republic: '🏛️', dynasty: '👑', tribe: '🛖', people: '👥', nationality: '🪪', religion: '🕊️', myth: '🐉',
  program: '💾', computer: '💻', unit: '📐', measure: '📐', science: '🔬', subject: '📚',
};

export function pictureOfKinds(kinds: string[]): string | null {
  for (const kind of kinds) if (Object.hasOwn(kindPictures, kind)) return kindPictures[kind];
  return null;
}
