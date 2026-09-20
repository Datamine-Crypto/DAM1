// A Cloudflare Pages Function: the one same-origin path the page fetches, relayed to the
// analytics site we operate. It exists so the browser's Content Security Policy can stay locked
// to 'self' and so the analytics host need not open cross-origin reads. It is not a general
// proxy: it forwards exactly one fixed URL, accepts only GET and HEAD, sends nothing from the
// visitor's request upstream, and answers with the figure and cache headers alone.

const upstream = 'https://analytics.datamine.network/api/page/dashboard';
const edgeSeconds = 60;
const browserSeconds = 30;
const upstreamTimeoutMs = 8000;

const replyHeaders = {
  'Content-Type': 'application/json',
  'Cache-Control': `public, max-age=${browserSeconds}, s-maxage=${edgeSeconds}`,
  'X-Content-Type-Options': 'nosniff',
  'Cross-Origin-Resource-Policy': 'same-origin',
};

interface Context {
  request: Request;
  waitUntil: (promise: Promise<unknown>) => void;
}

function refused(status: number, text: string): Response {
  return new Response(JSON.stringify({ error: text }), { status, headers: { ...replyHeaders, 'Cache-Control': 'no-store' } });
}

export async function onRequest(context: Context): Promise<Response> {
  const { request } = context;
  if (request.method !== 'GET' && request.method !== 'HEAD') return refused(405, 'method not allowed');

  const cache = (caches as unknown as { default: Cache }).default;
  const cacheKey = new Request(upstream, { method: 'GET' });
  const cached = await cache.match(cacheKey);
  if (cached) return cached;

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), upstreamTimeoutMs);
  let fetched: Response;
  try {
    fetched = await fetch(upstream, { method: 'GET', headers: { Accept: 'application/json' }, signal: controller.signal });
  } catch {
    clearTimeout(timer);
    return refused(502, 'upstream unreachable');
  }
  clearTimeout(timer);
  if (!fetched.ok) return refused(502, 'upstream failed');

  const body = await fetched.text();
  const reply = new Response(body, { status: 200, headers: replyHeaders });
  context.waitUntil(cache.put(cacheKey, reply.clone()));
  return reply;
}
