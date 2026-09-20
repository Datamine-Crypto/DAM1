// The one thread that touches IndexedDB. The page reads the chats from here once, when it opens, and
// keeps them in memory; every change after that is queued here and written one at a time, off the
// page's thread, over one connection kept open. A change to a name that is still waiting replaces
// the waiting one, since only the latest state is worth writing. A second store keeps each chat's
// context, the engine's mind after its latest turn, as bytes. Nothing leaves the browser.
const database = 'datamine-chats';
const store = 'state';
const contextStore = 'contexts';
const version = 4;

export type StorageRequest =
  | { kind: 'read'; id: number; name: string }
  | { kind: 'write'; name: string; value: unknown }
  | { kind: 'remove'; name: string }
  | { kind: 'readContext'; id: number; name: string }
  | { kind: 'writeContext'; name: string; value: { key: string; bytes: Uint8Array } }
  | { kind: 'removeContext'; name: string };

export type StorageReply =
  | { id: number; ok: true; value: unknown }
  | { id: number; ok: false; error: string };

let connection: Promise<IDBDatabase> | null = null;

function opened(): Promise<IDBDatabase> {
  if (connection) return connection;
  connection = new Promise<IDBDatabase>((resolve, reject) => {
    const request = indexedDB.open(database, version);
    // The chats live in one store and their contexts in another; an older database that kept the earlier
    // reader's saved memories has that store dropped.
    request.onupgradeneeded = () => {
      if (!request.result.objectStoreNames.contains(store)) request.result.createObjectStore(store);
      if (!request.result.objectStoreNames.contains(contextStore)) request.result.createObjectStore(contextStore);
      if (request.result.objectStoreNames.contains('snapshots')) request.result.deleteObjectStore('snapshots');
    };
    request.onsuccess = () => {
      const db = request.result;
      // A newer page in another tab may need to upgrade the database: let it, and reopen on the next write.
      db.onversionchange = () => {
        db.close();
        connection = null;
      };
      resolve(db);
    };
    request.onerror = () => reject(request.error ?? new Error('IndexedDB refused to open'));
    request.onblocked = () => reject(new Error('IndexedDB is blocked by another tab'));
  });
  connection.catch(() => { connection = null; });
  return connection;
}

async function found(table: string, name: string): Promise<unknown> {
  const db = await opened();
  return new Promise<unknown>((resolve, reject) => {
    const request = db.transaction(table, 'readonly').objectStore(table).get(name);
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error('IndexedDB read failed'));
  });
}

async function read(name: string): Promise<unknown> {
  const value = await found(store, name);
  if (typeof value !== 'string') return null;
  try {
    return JSON.parse(value);
  } catch {
    return null;
  }
}

// A write counts as done when its transaction commits, not when the request succeeds.
async function write(table: string, name: string, value: unknown): Promise<void> {
  const db = await opened();
  await new Promise<void>((resolve, reject) => {
    const transaction = db.transaction(table, 'readwrite');
    const records = transaction.objectStore(table);
    if (value === null) records.delete(name);
    else records.put(value, name);
    transaction.oncomplete = () => resolve();
    transaction.onerror = () => reject(transaction.error ?? new Error('IndexedDB write failed'));
    transaction.onabort = () => reject(transaction.error ?? new Error('IndexedDB write aborted'));
  });
}

// The writes waiting their turn, the latest per record, in the order the records were first queued;
// null removes the record.
interface Waiting {
  table: string;
  name: string;
  value: unknown;
}

const pending = new Map<string, Waiting>();
let draining = false;

function place(table: string, name: string): string {
  return `${table}\n${name}`;
}

async function drain(): Promise<void> {
  if (draining) return;
  draining = true;
  try {
    for (let next = pending.entries().next(); !next.done; next = pending.entries().next()) {
      const [at, { table, name, value }] = next.value;
      pending.delete(at);
      try {
        await write(table, name, value);
      } catch {
        // A blocked or full database loses persistence, not the page.
      }
    }
  } finally {
    draining = false;
  }
}

function queue(table: string, name: string, value: unknown): void {
  const at = place(table, name);
  pending.delete(at);
  pending.set(at, { table, name, value });
  void drain();
}

async function answer(request: { id: number }, value: () => Promise<unknown>): Promise<void> {
  let reply: StorageReply;
  try {
    reply = { id: request.id, ok: true, value: await value() };
  } catch (failure) {
    reply = { id: request.id, ok: false, error: failure instanceof Error ? failure.message : String(failure) };
  }
  self.postMessage(reply);
}

self.onmessage = async (event: MessageEvent<StorageRequest>) => {
  const request = event.data;
  if (request.kind === 'read') {
    // A write still waiting is newer than what the database holds.
    await answer(request, async () => {
      const waiting = pending.get(place(store, request.name));
      if (waiting === undefined) return read(request.name);
      return typeof waiting.value === 'string' ? JSON.parse(waiting.value) : null;
    });
    return;
  }
  if (request.kind === 'readContext') {
    await answer(request, async () => {
      const waiting = pending.get(place(contextStore, request.name));
      if (waiting === undefined) return found(contextStore, request.name);
      return waiting.value;
    });
    return;
  }
  if (request.kind === 'writeContext' || request.kind === 'removeContext') {
    queue(contextStore, request.name, request.kind === 'writeContext' ? request.value : null);
    return;
  }
  // The state is turned into text here, so the page's thread never does it.
  queue(store, request.name, request.kind === 'write' ? JSON.stringify(request.value) : null);
};
