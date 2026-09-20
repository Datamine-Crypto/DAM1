// The engine, in its own thread. This worker loads the WebAssembly build, the network and the state,
// and takes every turn, so the page never waits on the main thread. The page speaks to it in
// messages; nothing here touches the document.
import init, { context, describe, forget, kinds, load, read, restore, settings, state, world } from './wasm/dam_web.js';
import wasmUrl from './wasm/dam_web_bg.wasm?url';

export interface BootReport {
  weightBytes: number;
  weightCount: number;
  modelBytes: number;
  nodes: number;
  stateBytes: number;
  settings: string;
}

// A chat's context as the engine saves it after a turn: the chat's mind as a delta on the state every chat
// starts from, and the key of the chat, preface and history it belongs to.
export interface SavedContext {
  key: string;
  bytes: Uint8Array<ArrayBuffer>;
}

export interface TurnRequest {
  chatId: string;
  preface: string[];
  // What was said before, in order; a described world stands among it as an object, since the engine writes it without the network hearing it.
  history: Said[];
  prompt: string;
  // True when the prompt describes the world and is not said to the network.
  world?: boolean;
  context: SavedContext | null;
}

export type Said = string | { world: string };

// A turn's reading as the engine returns it, as JSON, with the chat's context after it.
export interface TurnResult {
  result: string;
  context: SavedContext | null;
}

// The whole tree of the mind the engine holds, as JSON, for the explorer page.
export interface WorldReport {
  world: string;
}

export type WorkerRequestBody =
  | { kind: 'boot' }
  | { kind: 'world' }
  // The kinds the mind's tree gives some names, asked by the explorer to group what it draws; the answer comes back as a world report's JSON.
  | { kind: 'kinds'; names: string }
  | ({ kind: 'turn' } & TurnRequest);

export type WorkerRequest = WorkerRequestBody & { id: number };

// What a turn sends while it runs: each word of the prompt, the steps the network took at it, and each
// word of the reply. A word carries its place and how many there are, so the page can show how far the
// turn has come.
export type Progress =
  | { kind: 'input'; token: string; at: number; of: number }
  | { kind: 'action'; action: string; word: string }
  | { kind: 'output'; token: string; at: number; of: number };

// A failure that trapped inside the engine leaves its WebAssembly instance unusable, so the reply says
// so and the page replaces the worker rather than asking this one again.
export type WorkerReply =
  | { id: number; ok: true; result: BootReport | TurnResult | WorldReport }
  | { id: number; ok: false; error: string; trapped: boolean }
  | { id: number; progress: Progress };

interface Reading {
  steps: [string, string[]][];
  output: string[];
  ended: boolean;
}

const continueClass = '{continue}';
const inputStart = '{input start}';
const inputEnd = '{input end}';

let chatOf = '';

// The model's files are kept in the browser's cache storage by this worker, so the network is downloaded
// once and served from the device after that, offline too. Each boot asks the server whether the stored
// copy is still current, by its entity tag, so a retrained network replaces it on the next visit; when the
// server cannot be reached, the stored copy is used.
const modelCache = 'datamine-model';
const notModified = 304;

async function cachedResponse(path: string): Promise<Response> {
  const store = 'caches' in self ? await caches.open(modelCache).catch(() => null) : null;
  const stored = store ? await store.match(path) : undefined;
  const headers: Record<string, string> = {};
  const tag = stored?.headers.get('etag');
  if (tag) headers['If-None-Match'] = tag;
  let fresh: Response | null = null;
  try {
    fresh = await fetch(path, { credentials: 'omit', referrerPolicy: 'no-referrer', cache: 'no-cache', headers });
  } catch (failure) {
    if (!stored) throw failure;
  }
  if (fresh && fresh.status === notModified && stored) return stored;
  if (fresh && fresh.ok) {
    if (store) await store.put(path, fresh.clone()).catch(() => undefined);
    return fresh;
  }
  if (stored) return stored;
  throw new Error(`${path}: ${fresh?.status ?? 'unreachable'}`);
}

async function fetchedBytes(path: string): Promise<Uint8Array<ArrayBuffer>> {
  const response = await cachedResponse(path);
  return new Uint8Array(await response.arrayBuffer());
}

// The network file may be gzip, written so by scripts/build-chat.sh to cross the wire small; the first
// two bytes say so, and the browser inflates it.
const gzipFirst = 0x1f;
const gzipSecond = 0x8b;

async function inflated(bytes: Uint8Array<ArrayBuffer>): Promise<Uint8Array<ArrayBuffer>> {
  if (bytes.length < 2 || bytes[0] !== gzipFirst || bytes[1] !== gzipSecond) return bytes;
  const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream('gzip'));
  return new Uint8Array(await new Response(stream).arrayBuffer());
}

async function fetchedText(path: string): Promise<string> {
  const response = await cachedResponse(path);
  return response.text();
}

async function boot(): Promise<BootReport> {
  const response = await fetch(wasmUrl, { credentials: 'omit', referrerPolicy: 'no-referrer' });
  if (!response.ok) throw new Error(`${wasmUrl}: ${response.status}`);
  const counted = response.clone().arrayBuffer();
  const [, networkBytes, classes, stateBytes] = await Promise.all([
    init({ module_or_path: response }),
    fetchedBytes('/network.bin').then(inflated),
    fetchedText('/network.json'),
    fetchedBytes('/state.bin'),
  ]);
  const modelBytes = (await counted).byteLength;
  const weightCount = load(networkBytes, classes);
  const nodes = state(stateBytes);
  chatOf = '';
  return { weightBytes: networkBytes.length, weightCount, modelBytes, nodes, stateBytes: stateBytes.length, settings: settings() };
}

function keyOf(chatId: string, preface: string[], length: number): string {
  return `${chatId}\n${preface.join('\n')}\n${length}`;
}

// A chat's mind is put back from its saved context whenever the turn belongs to another chat or preface
// than the mind holds, and read again from the state when there is no context for it, so every turn reads
// on the tree its own chat built.
function retell(chatId: string, preface: string[], history: Said[], saved: SavedContext | null): void {
  const key = keyOf(chatId, preface, history.length);
  if (chatOf === key) return;
  if (saved && saved.key === key) {
    try {
      restore(saved.bytes);
      chatOf = key;
      return;
    } catch {
      // A context saved on another state, or one that does not read back, is replaced by reading again.
    }
  }
  forget();
  for (const said of [...preface, ...history]) {
    if (typeof said === 'string') read(said); else describe(said.world);
  }
  chatOf = key;
}

function savedContext(): SavedContext | null {
  try {
    // The engine hands its bytes back in a buffer of its own; a copy owns one the page may take over.
    return { key: chatOf, bytes: new Uint8Array(context()) };
  } catch {
    return null;
  }
}

async function turn(request: TurnRequest, id: number): Promise<TurnResult> {
  retell(request.chatId, request.preface, request.history, request.context);
  const post = (progress: Progress): void => {
    const reply: WorkerReply = { id, progress };
    self.postMessage(reply);
  };
  const result = request.world ? describe(request.prompt) : read(request.prompt);
  chatOf = keyOf(request.chatId, request.preface, request.history.length + 1);
  const reading = JSON.parse(result) as Reading;
  const words = reading.steps.filter(([word]) => word !== inputStart && word !== inputEnd);
  for (const [at, [word]] of words.entries()) {
    post({ kind: 'input', token: word, at, of: words.length });
  }
  for (const [word, classes] of reading.steps) {
    for (const action of classes.filter((c) => c !== continueClass)) {
      post({ kind: 'action', action, word });
    }
  }
  for (const [at, token] of reading.output.entries()) {
    post({ kind: 'output', token, at, of: reading.output.length });
  }
  return { result, context: savedContext() };
}

self.onmessage = async (event: MessageEvent<WorkerRequest>) => {
  const request = event.data;
  try {
    const result = request.kind === 'boot' ? await boot() : request.kind === 'world' ? { world: world() } : request.kind === 'kinds' ? { world: kinds(request.names) } : await turn(request, request.id);
    const reply: WorkerReply = { id: request.id, ok: true, result };
    const carried = 'context' in result && result.context ? [result.context.bytes.buffer] : [];
    self.postMessage(reply, { transfer: carried });
  } catch (failure) {
    const reply: WorkerReply = {
      id: request.id,
      ok: false,
      error: failure instanceof Error ? failure.message : String(failure),
      trapped: failure instanceof WebAssembly.RuntimeError,
    };
    self.postMessage(reply);
  }
};
