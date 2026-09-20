// Every chat lives in this browser, in IndexedDB through zustand's persist
// middleware: read once when the page opens, then kept in memory, with each change queued to the
// storage worker. Nothing is sent anywhere.
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { agentStatus, bootAgent, forgetContext, mindQuery, reply, replyOfTurn, type AgentStatus, type Progress, type ReplyKind, type Steps } from './agent';
import { currentRoute } from './routes';
import { queuedStorage } from './storage';
import type { Said } from './reader.worker';

export type Role = 'user' | 'agent';

export interface Message {
  id: string;
  role: Role;
  text: string;
  at: number;
  detail?: string[];
  kind?: ReplyKind;
  // The moves of the turn this reply ends, so the chat can be played again as a game.
  steps?: Steps;
  // True for a message that described the world: the engine wrote it into the tree and the network never heard it.
  world?: boolean;
}

export interface Chat {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
  messages: Message[];
  pinned?: boolean;
}


// What a turn shows while it runs: the prompt's words as they are read, the action taken on each,
// and the reply's words as they are said. It is never saved with the chats.
// The counts say how far it has come: how many words the prompt has, and how many of the sentence
// being said are out; a reply of several sentences says each one in turn.
export interface Live {
  input: string[];
  actions: { action: string; word: string }[];
  output: string[];
  inputOf: number;
  outputAt: number;
  outputOf: number;
}

function emptyLive(): Live {
  return { input: [], actions: [], output: [], inputOf: 0, outputAt: 0, outputOf: 0 };
}

function withProgress(live: Live, progress: Progress): Live {
  if (progress.kind === 'input') return { ...live, input: [...live.input, progress.token], inputOf: progress.of };
  if (progress.kind === 'output') return { ...live, output: [...live.output, progress.token], outputAt: progress.at + 1, outputOf: progress.of };
  return { ...live, actions: [...live.actions, { action: progress.action, word: progress.word }] };
}

export type Route =
  | { kind: 'home' }
  | { kind: 'chat'; id: string }
  | { kind: 'privacy' }
  // The explorer; the id is the game an address names, and the game being played is kept apart, in game.
  | { kind: 'mind'; id?: string };


interface ChatState {
  chats: Record<string, Chat>;
  route: Route;
  // The chat the mind explorer plays, or none for a new game.
  game: string | null;
  // True while a chat read before its moves were kept is read again for them.
  recalling: boolean;
  railWidth: number;
  // True while the side bar is folded away on a wide window.
  railHidden: boolean;
  toggleRail: () => void;
  thinking: boolean;
  live: Live | null;
  agent: AgentStatus;
  // The reader loads on the first message, not at page open, so a visit that only reads costs
  // nothing; booting is true while that first load runs.
  booting: boolean;
  ensureAgent: () => Promise<AgentStatus>;
  newChat: () => void;
  openChat: (id: string) => void;
  openPrivacy: () => void;
  openMind: (chatId?: string) => void;
  pickGame: (chatId: string | null) => void;
  recallGame: (chatId: string) => Promise<void>;
  // Takes one turn out of a game, what was said and what it answered, and reads the rest again without it.
  undoTurn: (chatId: string, messageId: string) => Promise<void>;
  deleteChat: (id: string) => void;
  togglePin: (id: string) => void;
  renameChat: (id: string, title: string) => void;
  // Saves a chat as another under a new title, and plays the copy.
  copyChat: (id: string, title: string) => void;
  setRailWidth: (width: number) => void;
  send: (prompt: string, world?: boolean) => Promise<void>;
}

// One boot at a time: a second message while the first load runs waits on the same promise.
let booting: Promise<AgentStatus> | null = null;

const storageKey = 'datamine-chats';
const titleLength = 48;

// The reader posts a turn's progress as fast as it works. The page queues it and draws what has
// arrived once every drawMs, so the words and actions can still be followed while the turn itself is
// never held back.
const drawMs = 35;

// The rail can be dragged between these, in pixels, and the width is remembered.
export const rail = { min: 220, max: 480, initial: 248 } as const;

export function clampRail(width: number): number {
  return Math.min(rail.max, Math.max(rail.min, Math.round(width)));
}

export function newId(): string {
  return typeof crypto.randomUUID === 'function'
    ? crypto.randomUUID()
    : `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

function flat(text: string): string {
  return text.replace(/\s+/g, ' ').trim();
}

export function titleFrom(prompt: string): string {
  const line = flat(prompt);
  if (line.length <= titleLength) return line;
  const cut = line.slice(0, titleLength);
  const space = cut.lastIndexOf(' ');
  return `${space > titleLength / 2 ? cut.slice(0, space) : cut}...`;
}

function message(role: Role, text: string, detail?: string[], kind?: ReplyKind, steps?: Steps): Message {
  return { id: newId(), role, text, at: Date.now(), detail, kind, steps };
}

// What comes back from local storage is checked before it is trusted: only same-origin script can
// write it, but a corrupted or hand-edited entry must not crash the page or smuggle a key such as
// __proto__ into a record. Anything that is not the expected shape is dropped.
const idPattern = /^[A-Za-z0-9-]{1,64}$/;
const textLimit = 200_000;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function text(value: unknown, limit = textLimit): string | null {
  return typeof value === 'string' && value.length <= limit ? value : null;
}

function stamp(value: unknown): number {
  return typeof value === 'number' && Number.isFinite(value) && value >= 0 ? value : 0;
}

// The kept moves of a turn: a list of a word and the moves taken at it, every one a short string.
const stepLimit = 4000;

function cleanSteps(value: unknown): Steps | undefined {
  if (!Array.isArray(value) || value.length > stepLimit) return undefined;
  const steps: Steps = [];
  for (const item of value) {
    if (!Array.isArray(item) || item.length !== 2 || typeof item[0] !== 'string' || !Array.isArray(item[1])) return undefined;
    const moves = item[1].filter((move): move is string => typeof move === 'string' && move.length <= stepLimit);
    steps.push([item[0].slice(0, stepLimit), moves]);
  }
  return steps;
}

function cleanMessage(value: unknown): Message | null {
  if (!isRecord(value)) return null;
  const id = text(value.id, 64);
  const body = text(value.text);
  if (id === null || body === null || !idPattern.test(id)) return null;
  if (value.role !== 'user' && value.role !== 'agent') return null;
  const detail = Array.isArray(value.detail) ? value.detail.filter((line): line is string => typeof line === 'string' && line.length <= textLimit) : undefined;
  const kind = value.kind === 'nothing' || value.kind === 'noted' ? value.kind : undefined;
  return { id, role: value.role, text: body, at: stamp(value.at), detail, kind, steps: cleanSteps(value.steps), world: value.world === true ? true : undefined };
}
function cleanChat(value: unknown): Chat | null {
  if (!isRecord(value)) return null;
  const id = text(value.id, 64);
  const title = text(value.title, 400);
  if (id === null || title === null || !idPattern.test(id) || !Array.isArray(value.messages)) return null;
  const messages = value.messages.map(cleanMessage).filter((message): message is Message => message !== null);
  return {
    id, title, messages,
    createdAt: stamp(value.createdAt), updatedAt: stamp(value.updatedAt),
    pinned: value.pinned === true,
  };
}


function cleanTable<T extends { id: string }>(value: unknown, clean: (item: unknown) => T | null): Record<string, T> {
  const table: Record<string, T> = {};
  if (!isRecord(value)) return table;
  for (const item of Object.values(value)) {
    const cleaned = clean(item);
    if (cleaned && !Object.hasOwn(table, cleaned.id)) table[cleaned.id] = cleaned;
  }
  return table;
}


type Persisted = Pick<ChatState, 'chats' | 'railWidth' | 'railHidden'>;

function cleanPersisted(value: unknown): Persisted {
  const raw = isRecord(value) ? value : {};
  const railWidth = typeof raw.railWidth === 'number' && Number.isFinite(raw.railWidth) ? clampRail(raw.railWidth) : rail.initial;
  const chats = cleanTable(raw.chats, cleanChat);
  return { chats, railWidth, railHidden: raw.railHidden === true };
}

// The day a chat is read on, told to the network as a statement before anything else, so a question
// about tomorrow or yesterday reads today from the tree the way the lessons state it.
export function dayPreface(now: Date = new Date()): string {
  const weekday = now.toLocaleDateString('en-US', { weekday: 'long' }).toLowerCase();
  return `today is ${weekday}.`;
}


export const useChatStore = create<ChatState>()(
  persist(
    (set, get) => ({
      chats: {},
      route: currentRoute(),
      game: null,
      recalling: false,
      railWidth: rail.initial,
      railHidden: false,
      toggleRail: () => set((state) => ({ railHidden: !state.railHidden })),
      thinking: false,
      live: null,
      booting: false,
      agent: agentStatus(),

      ensureAgent: async () => {
        if (get().agent.ready) return get().agent;
        if (!booting) {
          set({ booting: true });
          booting = bootAgent().finally(() => { booting = null; set({ booting: false }); });
        }
        const agent = await booting;
        set({ agent });
        return agent;
      },

      // New opens the mind explorer, the default view, on a game not yet begun.
      newChat: () => set({ route: { kind: 'home' }, game: null }),

      copyChat: (id, title) => {
        const chat = get().chats[id];
        const line = flat(title);
        if (!chat || line === '') return;
        const now = Date.now();
        const copy: Chat = { ...chat, id: newId(), title: line, createdAt: now, updatedAt: now, pinned: false, messages: chat.messages.map((said) => ({ ...said, id: newId() })) };
        set((state) => ({ chats: { ...state.chats, [copy.id]: copy }, game: copy.id }));
      },

      openChat: (id) => set({ route: Object.hasOwn(get().chats, id) ? { kind: 'chat', id } : { kind: 'home' } }),



      openPrivacy: () => set({ route: { kind: 'privacy' } }),
      openMind: (chatId) => set((state) => ({ route: { kind: 'mind' }, game: chatId && Object.hasOwn(state.chats, chatId) ? chatId : state.game })),

      pickGame: (chatId) => set((state) => ({ game: chatId && Object.hasOwn(state.chats, chatId) ? chatId : null })),

      // A chat read before its moves were kept is read once more, on the explorer's own mind, and every
      // reply takes the moves of its turn. The replies themselves stay as they were said.
      recallGame: async (chatId) => {
        const chat = get().chats[chatId];
        if (!chat || get().recalling || get().thinking) return;
        set({ recalling: true });
        try {
          await get().ensureAgent();
          const told: Said[] = [];
          const kept = new Map<string, Steps>();
          for (const [at, said] of chat.messages.entries()) {
            if (said.role !== 'user') continue;
            const turn = await mindQuery(told, said.text, said.world === true);
            told.push(said.world ? { world: said.text } : said.text);
            const next = chat.messages[at + 1];
            if (next && next.role === 'agent') kept.set(next.id, turn.steps);
          }
          set((state) => {
            const current = state.chats[chatId];
            if (!current) return {};
            const messages = current.messages.map((said) => (kept.has(said.id) ? { ...said, steps: kept.get(said.id) } : said));
            return { chats: { ...state.chats, [chatId]: { ...current, messages } } };
          });
        } finally {
          set({ recalling: false });
        }
      },

      undoTurn: async (chatId, messageId) => {
        const chat = get().chats[chatId];
        if (!chat || get().recalling || get().thinking) return;
        const at = chat.messages.findIndex((said) => said.id === messageId && said.role === 'user');
        if (at < 0) return;
        const answered = chat.messages[at + 1]?.role === 'agent' ? 2 : 1;
        const left = [...chat.messages.slice(0, at), ...chat.messages.slice(at + answered)];
        // The saved context holds the turn taken out, so the chat is read from its start the next time it is spoken in.
        forgetContext(chatId);
        set((state) => ({ recalling: true, chats: { ...state.chats, [chatId]: { ...chat, messages: left } } }));
        try {
          await get().ensureAgent();
          const told: Said[] = [];
          const again = new Map<string, ReturnType<typeof replyOfTurn>>();
          for (const [nth, said] of left.entries()) {
            if (said.role !== 'user') continue;
            const turn = await mindQuery(told, said.text, said.world === true);
            told.push(said.world ? { world: said.text } : said.text);
            const next = left[nth + 1];
            if (next && next.role === 'agent') again.set(next.id, replyOfTurn(turn, said.world === true));
          }
          set((state) => {
            const current = state.chats[chatId];
            if (!current) return {};
            const messages = current.messages.map((said) => { const fresh = again.get(said.id); return fresh ? { ...said, text: fresh.text, kind: fresh.kind, detail: fresh.detail, steps: fresh.steps } : said; });
            return { chats: { ...state.chats, [chatId]: { ...current, messages } } };
          });
        } finally {
          set({ recalling: false });
        }
      },

      setRailWidth: (width) => set({ railWidth: clampRail(width) }),


      // A deleted chat's saved context goes with it.
      deleteChat: (id) => {
        forgetContext(id);
        set((state) => {
          const chats = { ...state.chats };
          delete chats[id];
          const route = state.route.kind === 'chat' && state.route.id === id ? { kind: 'home' as const } : state.route;
          return { chats, route, game: state.game === id ? null : state.game };
        });
      },

      togglePin: (id) => set((state) => {
        const chat = state.chats[id];
        if (!chat) return {};
        return { chats: { ...state.chats, [id]: { ...chat, pinned: !chat.pinned } } };
      }),

      renameChat: (id, title) => set((state) => {
        const chat = state.chats[id];
        const line = flat(title);
        if (!chat || line === '') return {};
        return { chats: { ...state.chats, [id]: { ...chat, title: line } } };
      }),







      send: async (prompt, world = false) => {
        if (get().thinking) return;
        const now = Date.now();
        const route = get().route;
        const inGame = route.kind === 'mind' || route.kind === 'home';
        const played = get().game;
        const existing = route.kind === 'chat' ? get().chats[route.id] : inGame && played ? get().chats[played] : undefined;
        const chat: Chat = existing
          ? { ...existing, updatedAt: now, messages: [...existing.messages, { ...message('user', prompt), world: world || undefined }] }
          : { id: newId(), title: titleFrom(prompt), createdAt: now, updatedAt: now, messages: [{ ...message('user', prompt), world: world || undefined }] };
        // A message sent from the mind explorer stays on the explorer: the chat it made or went into is the game played.
        set((state) => ({ chats: { ...state.chats, [chat.id]: chat }, route: inGame ? state.route : { kind: 'chat', id: chat.id }, game: inGame ? chat.id : state.game, thinking: true, live: emptyLive() }));
        const history = chat.messages.slice(0, -1);
        const queued: Progress[] = [];
        const draw = (): void => {
          if (queued.length === 0) return;
          const arrived = queued.splice(0);
          set((state) => (state.live ? { live: arrived.reduce(withProgress, state.live) } : {}));
        };
        const drawing = setInterval(draw, drawMs);
        try {
          await get().ensureAgent();
          const answer = await reply(chat.id, [], history, prompt, (progress) => {
            queued.push(progress);
          }, world);
          clearInterval(drawing);
          set((state) => {
            const current = state.chats[chat.id];
            if (!current) return { thinking: false, live: null };
            const updated = { ...current, updatedAt: Date.now(), messages: [...current.messages, message('agent', answer.text, answer.detail, answer.kind, answer.steps)] };
            return { chats: { ...state.chats, [chat.id]: updated }, thinking: false, live: null };
          });
        } catch (failure) {
          clearInterval(drawing);
          const text = failure instanceof Error ? failure.message : String(failure);
          set((state) => {
            const current = state.chats[chat.id];
            if (!current) return { thinking: false, live: null };
            const updated = { ...current, messages: [...current.messages, message('agent', 'The reader failed on that.', [text])] };
            return { chats: { ...state.chats, [chat.id]: updated }, thinking: false, live: null };
          });
        }
      },
    }),
    {
      partialize: (state): Persisted => ({ chats: state.chats, railWidth: state.railWidth, railHidden: state.railHidden }),
      name: storageKey,
      storage: queuedStorage<Persisted>(),
      version: 1,
      migrate: (persisted) => cleanPersisted(persisted),
      merge: (persisted, current) => ({ ...current, ...cleanPersisted(persisted) }),
    },
  ),
);

export const selectActiveChat = (state: ChatState): Chat | null =>
  (state.route.kind === 'chat' && Object.hasOwn(state.chats, state.route.id) ? state.chats[state.route.id] : null);


export function byRecent<T extends { updatedAt: number }>(items: T[]): T[] {
  return [...items].sort((a, b) => b.updatedAt - a.updatedAt);
}
