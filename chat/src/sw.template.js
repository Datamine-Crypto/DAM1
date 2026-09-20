// The service worker. The build fills in the list of files to keep and a version that changes
// whenever that list does, so a new build replaces the old cache on the next visit.
//
// What it does: on install it stores every file of the page, the reader and the model, so the
// page opens and answers with no network. A page navigation is answered from the network when
// there is one and from the stored page when there is not. The one relay path is never stored:
// it is always fetched, and fails quietly when offline.
const version = '__VERSION__';
const files = __PRECACHE__;
const cacheName = `datamine-chat-${version}`;
const relayPrefix = '/api/';
const page = '/';

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(cacheName)
      .then((cache) => cache.addAll(files))
      .then(() => self.skipWaiting()),
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys()
      .then((names) => Promise.all(names.filter((name) => name !== cacheName).map((name) => caches.delete(name))))
      .then(() => self.clients.claim()),
  );
});

self.addEventListener('fetch', (event) => {
  const { request } = event;
  if (request.method !== 'GET') return;
  const url = new URL(request.url);
  if (url.origin !== self.location.origin) return;
  if (url.pathname.startsWith(relayPrefix)) return;

  if (request.mode === 'navigate') {
    event.respondWith(
      fetch(request).catch(() => caches.match(page).then((stored) => stored ?? Response.error())),
    );
    return;
  }

  event.respondWith(
    caches.match(request).then((stored) => stored ?? fetch(request).then((fresh) => {
      if (fresh.ok && files.includes(url.pathname)) {
        const copy = fresh.clone();
        caches.open(cacheName).then((cache) => cache.put(request, copy));
      }
      return fresh;
    })),
  );
});
