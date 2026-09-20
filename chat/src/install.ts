// Registers the service worker that keeps the page on the device, as soon as the page starts rather
// than once it has loaded, so the page's files are stored while the engine's worker downloads the
// model. Only in a production build: the dev server serves modules one by one and has no worker to
// offer.
export function registerOffline(): void {
  if (!import.meta.env.PROD || typeof navigator === 'undefined' || !('serviceWorker' in navigator)) return;
  navigator.serviceWorker.register('/sw.js', { scope: '/' }).catch(() => undefined);
}
