// The layout a person gives the map, kept for each game in the browser: what was dragged, sized, bent and zoomed.
import type { Region } from '../emoji';

export type Register = (nth: number, element: HTMLElement | null) => void;
// How far a person dragged each tile from where the map put it, by the tile's name.
export type Moved = Record<string, { x: number; y: number }>;
export type OnMove = (name: string, x: number, y: number) => void;
export const groupKey = (region: Region): string => `group:${region}`;
// The bends a person put into a relation's line, in order from the tile it leaves, kept with the layout under one key a bend.
const bendKey = (line: string, nth: number): string => `bend:${line}:${nth}`;
export function bendsOf(moved: Moved, line: string): { x: number; y: number }[] {
  const bends: { x: number; y: number }[] = [];
  while (Object.hasOwn(moved, bendKey(line, bends.length))) bends.push(moved[bendKey(line, bends.length)]);
  return bends;
}
export function withBends(moved: Moved, line: string, bends: { x: number; y: number }[]): Moved {
  const next: Moved = {};
  for (const [key, at] of Object.entries(moved)) if (!key.startsWith(`bend:${line}:`)) next[key] = at;
  bends.forEach((bend, nth) => { next[bendKey(line, nth)] = bend; });
  return next;
}
// How near a dragged bend must come to the line through a neighbour, up or across, to be pulled onto it, so right angles are easy.
export const bendSnap = 10;
// The least a tile or a group can be sized to, so its corner can still be taken hold of.
export const sizeLeast = 12;
// The event the overview sends to the map with the name of a thing to show.
export const focusEvent = 'mind-focus';
export const labelKey = (line: string): string => `label:${line}`;
export const tileSizeKey = (name: string): string => `tilesize:${name}`;
export interface Line { key: string; x1: number; y1: number; x2: number; y2: number; cx: number; cy: number; ax: number; ay: number; bx: number; by: number; bends: { x: number; y: number }[]; label: string; past: boolean }
export const layoutPrefix = 'mind-layout:';
export const plainView = { zoom: 1, x: 0, y: 0 };

// The layout kept for a game, read with care: what storage holds may be anything.
export function layoutOf(key: string): { moved: Moved; view: { zoom: number; x: number; y: number } } {
  try {
    const kept: unknown = JSON.parse(window.localStorage.getItem(layoutPrefix + key) ?? 'null');
    if (typeof kept !== 'object' || kept === null) return { moved: {}, view: plainView };
    const { moved, view } = kept as { moved?: unknown; view?: unknown };
    const safeMoved: Moved = {};
    if (typeof moved === 'object' && moved !== null) {
      for (const [name, at] of Object.entries(moved)) {
        const spot = at as { x?: unknown; y?: unknown } | null;
        if (spot && typeof spot.x === 'number' && typeof spot.y === 'number' && Number.isFinite(spot.x) && Number.isFinite(spot.y)) safeMoved[name] = { x: spot.x, y: spot.y };
      }
    }
    const seen = view as { zoom?: unknown; x?: unknown; y?: unknown } | null;
    const safeView = seen && typeof seen.zoom === 'number' && typeof seen.x === 'number' && typeof seen.y === 'number' && [seen.zoom, seen.x, seen.y].every(Number.isFinite)
      ? { zoom: Math.min(Math.max(seen.zoom, zoomLeast), zoomMost), x: seen.x, y: seen.y }
      : plainView;
    return { moved: safeMoved, view: safeView };
  } catch {
    return { moved: {}, view: plainView };
  }
}

// How near its holder's edge a dragged tile may go, and the room a group's name takes at its top, in pixels.
export const tileEdge = 6;
export const groupHead = 26;
export const sizeKey = (region: Region): string => `size:${region}`;
export const gridStep = 40;
export const zoomStep = 1.25;
export const wheelStep = 1.1;
export const zoomLeast = 0.4;
export const zoomMost = 3;
