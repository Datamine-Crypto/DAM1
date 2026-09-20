// Where the chats live: an IndexedDB database in this browser, one object store, one record, with a
// second store beside it for each chat's memory as the reader saved it. The
// database is read once, when the page opens, into the store's memory; the page works from memory
// after that, and a refresh drops it and reads the database again. Every change is handed to a
// storage worker (src/storage.worker.ts), which writes the changes one at a time on its own thread.
// A copy left by the earlier localStorage version is carried over on the first read and then
// removed. Nothing leaves the browser.
import type { PersistStorage, StorageValue } from 'zustand/middleware';
import type { SavedContext } from './reader.worker';
import type { StorageReply, StorageRequest } from './storage.worker';

let worker: Worker | null = null;
let unavailable = false;
let nextId = 1;
const waiting = new Map<number, { resolve: (value: unknown) => void; reject: (error: Error) => void }>();

function storageWorker(): Worker | null {
  if (worker || unavailable) return worker;
  if (typeof indexedDB === 'undefined' || typeof Worker === 'undefined') {
    unavailable = true;
    return null;
  }
  try {
    worker = new Worker(new URL('./storage.worker.ts', import.meta.url), { type: 'module' });
  } catch {
    unavailable = true;
    return null;
  }
  worker.onmessage = (event: MessageEvent<StorageReply>) => {
    const reply = event.data;
    const pending = waiting.get(reply.id);
    if (!pending) return;
    waiting.delete(reply.id);
    if (reply.ok) pending.resolve(reply.value);
    else pending.reject(new Error(reply.error));
  };
  // A worker that fails to start loses persistence, not the page.
  worker.onerror = () => {
    for (const pending of waiting.values()) pending.reject(new Error('The storage worker failed'));
    waiting.clear();
    worker?.terminate();
    worker = null;
    unavailable = true;
  };
  return worker;
}

function post(request: StorageRequest, transfer: Transferable[] = []): void {
  storageWorker()?.postMessage(request, transfer);
}

function asked(request: (id: number) => StorageRequest): Promise<unknown> {
  const target = storageWorker();
  if (!target) return Promise.resolve(null);
  const id = nextId;
  nextId += 1;
  return new Promise((resolve, reject) => {
    waiting.set(id, { resolve, reject });
    target.postMessage(request(id));
  });
}

function stored(name: string): Promise<unknown> {
  return asked((id) => ({ kind: 'read', id, name }));
}

// A chat's context, the engine's mind after its latest turn, is kept as bytes beside the chats.
export async function loadContext(chatId: string): Promise<SavedContext | null> {
  const found = await asked((id) => ({ kind: 'readContext', id, name: chatId })).catch(() => null);
  if (!found || typeof found !== 'object') return null;
  const { key, bytes } = found as { key?: unknown; bytes?: unknown };
  if (typeof key !== 'string' || !(bytes instanceof Uint8Array)) return null;
  return { key, bytes: bytes as Uint8Array<ArrayBuffer> };
}

export function saveContext(chatId: string, context: SavedContext): void {
  const copy = new Uint8Array(context.bytes);
  post({ kind: 'writeContext', name: chatId, value: { key: context.key, bytes: copy } }, [copy.buffer]);
}

export function removeContext(chatId: string): void {
  post({ kind: 'removeContext', name: chatId });
}

// The earlier version kept the same JSON under the same name in localStorage.
function inherited(name: string): unknown {
  try {
    const value = localStorage.getItem(name);
    if (value === null) return null;
    localStorage.removeItem(name);
    return JSON.parse(value);
  } catch {
    return null;
  }
}

// Whether two saved states hold the same fields by reference: the store replaces a field whenever it
// changes it, so a state whose fields are all the same objects is the state already queued.
function sameFields(a: object, b: object): boolean {
  const left = Object.entries(a);
  return left.length === Object.keys(b).length && left.every(([key, value]) => Object.hasOwn(b, key) && Object.is(value, (b as Record<string, unknown>)[key]));
}

// The store saves on every change to any of its fields, a streamed word included; only a change to a
// saved field is queued. What the read returns is untrusted: the store checks it before using it.
export function queuedStorage<S extends object>(): PersistStorage<S> {
  let last: StorageValue<S> | null = null;
  return {
    getItem: async (name) => {
      const found = await stored(name).catch(() => null);
      if (found !== null) return found as StorageValue<S>;
      const carried = inherited(name);
      if (carried !== null) post({ kind: 'write', name, value: carried });
      return carried as StorageValue<S> | null;
    },
    setItem: (name, value) => {
      if (last && last.version === value.version && sameFields(value.state, last.state)) return;
      last = value;
      post({ kind: 'write', name, value });
    },
    removeItem: (name) => {
      last = null;
      post({ kind: 'remove', name });
    },
  };
}
