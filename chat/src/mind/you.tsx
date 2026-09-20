// The person playing as the map shows them, and the names and pictures things are shown by.
import { createContext, useContext } from 'react';
import { pictureOf } from '../emoji';
import { baseName, selfName, networkNames, networkName } from './replay';

// The person playing is the tree's user, and reads as You on the page.
// How the person playing is shown: a name and a picture they pick, kept in this browser. Clicking their tile opens the choice.
export interface You {
  name: string;
  picture: string;
  edit: () => void;
  // The labels the person put on things of this game, by name, and the asking for one.
  labels: Record<string, string>;
  label: (name: string) => void;
  // The picture of its kind for each name the network's facts give a kind, used when the name has none of its own.
  kindPictures: Map<string, string>;
}
export const plainYou: You = { name: 'You', picture: '🧑', edit: () => undefined, labels: {}, label: () => undefined, kindPictures: new Map() };
export const YouContext = createContext<You>(plainYou);
export const youKey = 'mind-you';
export const tokenPictures = ['⭐', '❤️', '🔥', '💡', '🎯', '🏆', '🎁', '🔑', '📦', '🧩', '🧪', '🛠️', '📍', '🏠', '🌍', '🌙', '☀️', '🌊', '🌳', '🌸', '🍎', '🍕', '☕', '🚗', '✈️', '🚀', '⚽', '🎵', '📚', '💰', '🐶', '🐱', '🐦', '🐟', '🧑', '👩', '👨', '👶', '🤖', '👻'];
export const youPictures = ['🧑', '👩', '👨', '🧒', '👧', '👦', '🧓', '👵', '👴', '🧙', '🦸', '🥷', '🧑‍🚀', '🧑‍🎨', '🧑‍💻', '🧑‍🍳', '🤠', '😎', '🤓', '🦊', '🐱', '🐶', '🐼', '🦉', '🐸', '🦄', '🤖', '👽', '👻', '⭐'];

export function keptYou(): { name: string; picture: string } {
  try {
    const kept: unknown = JSON.parse(window.localStorage.getItem(youKey) ?? 'null');
    const { name, picture } = (kept ?? {}) as { name?: unknown; picture?: unknown };
    return { name: typeof name === 'string' && name.trim() !== '' ? name.slice(0, 40) : plainYou.name, picture: typeof picture === 'string' && picture !== '' ? picture.slice(0, 16) : plainYou.picture };
  } catch {
    return { name: plainYou.name, picture: plainYou.picture };
  }
}

// The name and the picture a thing is shown with: the person's own choice for the user, the tables for the rest.
export function useShown(): { nameOf: (name: string) => string; pictureFor: (name: string) => string | null } {
  const you = useContext(YouContext);
  // The person playing is always You; only their picture is theirs to pick.
  const own = plainYou.name;
  // The one the person speaks to, you, is the network itself.
  return { nameOf: (name) => (name === selfName ? own : networkNames.includes(name) ? networkName : baseName(name)), pictureFor: (name) => (name === selfName ? you.picture : you.labels[baseName(name)] ?? pictureOf(baseName(name)) ?? you.kindPictures.get(baseName(name)) ?? null) };
}
