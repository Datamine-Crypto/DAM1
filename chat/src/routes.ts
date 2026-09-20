// The page's routes and their paths. The store reads its first route from the URL so the page
// opens where the address says, and the shell keeps the two in step from then on.
import type { Route } from './store';

const chatPrefix = '/chat/';
const privacyPath = '/privacy';
const mindPath = '/mind';
const mindPrefix = '/mind/';

// An id as the address writes it; a malformed escape such as %E0 reads as no id, so a bad link
// opens a page instead of stopping the app before it draws.
function idFrom(written: string): string {
  try {
    return decodeURIComponent(written);
  } catch {
    return '';
  }
}

export function routeFromPath(pathname: string): Route {
  if (pathname.startsWith(chatPrefix)) {
    const id = idFrom(pathname.slice(chatPrefix.length));
    return id ? { kind: 'chat', id } : { kind: 'home' };
  }
  if (pathname === privacyPath) return { kind: 'privacy' };
  if (pathname === mindPath) return { kind: 'mind' };
  if (pathname.startsWith(mindPrefix)) {
    const id = idFrom(pathname.slice(mindPrefix.length));
    return id ? { kind: 'mind', id } : { kind: 'mind' };
  }
  return { kind: 'home' };
}

export function pathFor(route: Route): string {
  switch (route.kind) {
    case 'chat': return `${chatPrefix}${encodeURIComponent(route.id)}`;
    case 'privacy': return privacyPath;
    case 'mind': return route.id ? `${mindPrefix}${encodeURIComponent(route.id)}` : mindPath;
    default: return '/';
  }
}

export function currentRoute(): Route {
  return typeof window === 'undefined' ? { kind: 'home' } : routeFromPath(window.location.pathname);
}
