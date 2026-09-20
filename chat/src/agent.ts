// The agent is the word network, compiled to WebAssembly and running in this page, in a worker
// thread of its own so a long turn never stalls the page. Every turn is read on the tree the chat has
// built so far, and the reply is what the network says. Each chat has its own tree: the engine goes back
// to the permanent state and reads the chat's earlier prompts again when the chat changes. Nothing leaves the
// browser.
import type { BootReport, Progress, SavedContext, TurnResult, WorkerReply, WorkerRequestBody, WorldReport, Said } from './reader.worker';
import { loadContext, removeContext, saveContext } from './storage';
import type { Message } from './store';

export type { Progress } from './reader.worker';

// Each chat's context, the engine's mind after its latest turn as a delta on the state every chat starts
// from, kept here once read or saved, and in the browser's database across visits, so a reopened chat goes
// on where it was without reading its prompts again.
const contexts = new Map<string, SavedContext>();

async function contextOf(chatId: string): Promise<SavedContext | null> {
  const held = contexts.get(chatId);
  if (held) return held;
  const found = await loadContext(chatId);
  if (found) contexts.set(chatId, found);
  return found;
}

export function forgetContext(chatId: string): void {
  contexts.delete(chatId);
  removeContext(chatId);
}

// A reply that says nothing is marked, so the page can show it as one.
export type ReplyKind = 'nothing' | 'noted';

export interface Reply {
  text: string;
  detail?: string[];
  kind?: ReplyKind;
  // The moves the network took at each word of the turn, kept with the chat so the turn can be played again.
  steps?: Steps;
}

export type Steps = [string, string[]][];

// One turn as the network read it: the steps it took at each input item by their classes, what it
// said, and whether every item continued within the step limit.
export interface Turn {
  told?: string[];
  steps: Steps;
  output: string[];
  ended: boolean;
}

// The engine's own values, as it reports them once loaded.
export interface Settings {
  classes: string[];
  steps: number;
  slots: number;
  hidden: number;
  voters?: number;
}

// One node of the mind's tree as the engine writes it: its name, whether this chat told it, the thing a
// mention names, and the nodes inside it.
export interface WorldNode {
  name: string;
  told: boolean;
  names: string | null;
  children: WorldNode[];
}

// How long one request to the worker may run before it is given up and the worker is replaced, so a
// hung worker cannot leave the page thinking forever. Generous: the first boot downloads the engine
// and its files on a slow connection, and a long turn on a slow device takes many seconds.
const requestTimeout = 5 * 60 * 1000;

export interface AgentStatus {
  ready: boolean;
  weightBytes: number;
  weightCount: number;
  modelBytes: number;
  // The permanent state every chat starts from: the nodes of its tree and the size of the file it is loaded from.
  nodes: number;
  stateBytes: number;
  settings: Settings | null;
  failure: string;
}

const notReady: AgentStatus = { ready: false, weightBytes: 0, weightCount: 0, modelBytes: 0, nodes: 0, stateBytes: 0, settings: null, failure: '' };

let worker: Worker | null = null;
let status: AgentStatus = notReady;
let nextId = 1;
const waiting = new Map<number, { resolve: (result: BootReport | TurnResult | WorldReport) => void; reject: (error: Error) => void; progress?: (progress: Progress) => void }>();

function readerWorker(): Worker {
  if (worker) return worker;
  worker = new Worker(new URL('./reader.worker.ts', import.meta.url), { type: 'module' });
  worker.onmessage = (event: MessageEvent<WorkerReply>) => {
    const reply = event.data;
    const pending = waiting.get(reply.id);
    if (!pending) return;
    // Progress arrives while the turn runs and leaves the request waiting for its result.
    if ('progress' in reply) {
      pending.progress?.(reply.progress);
      return;
    }
    waiting.delete(reply.id);
    if (reply.ok) {
      pending.resolve(reply.result);
      return;
    }
    pending.reject(new Error(reply.error));
    // A trap leaves the engine's instance unusable: drop this worker, and the next turn boots a new one
    // and reads the chat into it again.
    if (reply.trapped) replaceWorker();
  };
  // A worker whose script fails to load, or that throws outside a turn, is dead: drop it, and the next
  // turn boots a new one.
  worker.onerror = (event) => {
    replaceWorker(event.message || 'The engine stopped and is being restarted.');
  };
  return worker;
}

let replaced = false;

function replaceWorker(reason = 'The engine stopped and is being restarted.'): void {
  worker?.terminate();
  worker = null;
  for (const pending of waiting.values()) pending.reject(new Error(reason));
  waiting.clear();
  status = { ...status, ready: false };
  replaced = true;
}

function ask(request: WorkerRequestBody, progress?: (progress: Progress) => void): Promise<BootReport | TurnResult | WorldReport> {
  const id = nextId;
  nextId += 1;
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      if (waiting.has(id)) replaceWorker('The engine took too long on that and is being restarted.');
    }, requestTimeout);
    waiting.set(id, {
      resolve: (result) => { clearTimeout(timer); resolve(result); },
      reject: (error) => { clearTimeout(timer); reject(error); },
      progress,
    });
    readerWorker().postMessage({ ...request, id });
  });
}

export async function bootAgent(): Promise<AgentStatus> {
  try {
    const report = (await ask({ kind: 'boot' })) as BootReport;
    status = {
      ready: true, weightBytes: report.weightBytes, weightCount: report.weightCount, modelBytes: report.modelBytes,
      nodes: report.nodes, stateBytes: report.stateBytes, settings: JSON.parse(report.settings) as Settings, failure: '',
    };
  } catch (failure) {
    const text = failure instanceof Error ? failure.message : String(failure);
    status = { ...notReady, failure: text };
  }
  return status;
}

// A query read for the explorer page, on a mind of its own that holds what the page's earlier queries told:
// the reading whole, every word with the moves the network took at it, what it said and what it wrote.
const explorerChat = 'mind-explorer';

export async function mindQuery(history: Said[], prompt: string, world = false): Promise<Turn> {
  const answer = (await ask({ kind: 'turn', chatId: explorerChat, preface: [], history, prompt, world, context: null })) as TurnResult;
  return JSON.parse(answer.result) as Turn;
}

// The tree of the mind the engine holds now: the chat it read last, or the state every chat starts from.
export async function mindWorld(): Promise<WorldNode[]> {
  const report = (await ask({ kind: 'world' })) as WorldReport;
  return JSON.parse(report.world) as WorldNode[];
}

// The kinds the engine's tree gives each name, one above another: a rat is a rodent, a mammal, an animal. The
// explorer groups its tiles by them, so the groups are the network's own facts.
export async function mindKinds(names: string[]): Promise<Record<string, string[]>> {
  if (!status.ready || names.length === 0) return {};
  const report = (await ask({ kind: 'kinds', names: names.join(' ') })) as WorldReport;
  return JSON.parse(report.world) as Record<string, string[]>;
}

export function agentStatus(): AgentStatus {
  return status;
}

// The preface is what a chat is told before its own history.
export async function reply(chatId: string, preface: string[], history: Message[], prompt: string, progress?: (progress: Progress) => void, world = false): Promise<Reply> {
  if (!status.ready && replaced) {
    replaced = false;
    await bootAgent();
  }
  if (!status.ready) {
    return {
      text: 'The engine could not load. Reload the page to try again.',
      detail: status.failure ? [status.failure] : undefined,
    };
  }
  const told: Said[] = history.filter((message) => message.role === 'user').map((message) => (message.world ? { world: message.text } : message.text));
  const context = await contextOf(chatId);
  const answer = (await ask({ kind: 'turn', chatId, preface, history: told, prompt, world, context }, progress)) as TurnResult;
  if (answer.context) {
    contexts.set(chatId, answer.context);
    saveContext(chatId, answer.context);
  }
  return replyOfTurn(JSON.parse(answer.result) as Turn, world);
}

// What a turn says back, as the chat keeps it.
export function replyOfTurn(turn: Turn, world: boolean): Reply {
  const text = turn.output.join(' ');
  if (text) return { text, steps: turn.steps };
  // A described world is kept by the tree and not by him: what it wrote is shown, as the world's, not as his memory.
  if (world) return { text: '', kind: 'noted', detail: turn.told ?? [], steps: turn.steps };
  // A statement says nothing back: what it wrote into the tree is shown under the reply instead.
  return turn.told && turn.told.length > 0 ? { text: '', kind: 'noted', detail: turn.told, steps: turn.steps } : { text: '', kind: 'nothing', steps: turn.steps };
}
