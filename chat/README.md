# 💬 Datamine chat

[![Deploy Web](https://github.com/Datamine-Crypto/DAM1/actions/workflows/deploy-web.yml/badge.svg)](https://github.com/Datamine-Crypto/DAM1/actions/workflows/deploy-web.yml)
[![Discord](https://img.shields.io/badge/Discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/2dQ7XAB22u)

A chat with the DAM1 cursor network. The engine is `model/crates/dam-web` compiled to WebAssembly and
running in the page, with the trained network loaded from `public/network.bin` and the permanent state
every chat starts from loaded whole from `public/state.bin`. Nothing is sent anywhere: the model, the
weights and every chat stay in the browser.

```sh
NETWORK=path/to/network.bin sh scripts/build-chat.sh   # from the repository root, after the network changes
cd chat
npm install
npm run dev
```

React 19, Material UI 9, zustand and Vite, in TypeScript. The palette and the rail are taken from
[analytics.datamine.network](https://analytics.datamine.network/), which is the same network this
page belongs to.

## 🧩 What it needs

- The engine (`src/wasm/`), the network (`public/network.bin`, `public/network.json`) and the state
  (`public/state.bin`) are built by `scripts/build-chat.sh` from the repository root. It needs Rust,
  wasm-pack and a trained cursor network with its classes file beside it. They are not committed.
- Deploys run in GitHub Actions (`.github/workflows/deploy-web.yml`): the workflow runs
  build-chat.sh, the typecheck and the Vite build, then deploys `dist` to Cloudflare Pages.

## 🚀 Deploy

Each push to `main` deploys the page. You can also start the Deploy Web workflow by hand
(workflow_dispatch). The workflow builds everything and runs `wrangler pages deploy dist` with
`--branch` set to the pushed branch. It uploads `dist` and `functions/` together.

To set it up once:

1. In Cloudflare, create a Pages project with Direct Upload. If the project is connected to Git,
   turn off its automatic deployments in the project's build settings, so Pages does not also
   build each push. The workflow is the only build.
2. Set the project's production branch to `main`. A deploy from `main` then goes to production.
3. In the GitHub repository settings, add two secrets under Actions: `CLOUDFLARE_API_TOKEN`, an
   API token with the Cloudflare Pages Edit permission, and `CLOUDFLARE_ACCOUNT_ID`, the account
   that owns the project.
4. Add one variable under Actions: `CLOUDFLARE_PAGES_PROJECT`, the name of the Pages project.
5. In the Pages project, add the custom domain under Custom domains. Cloudflare adds the DNS
   record when the zone is on the same account; otherwise point a CNAME at
   `<project>.pages.dev`.

## 📄 What is in the page

- **New.** The landing page: a greeting and a prompt box. The first prompt starts a chat and names
  it.
- **A chat.** Your prompts on the right, the network's replies on the left. Every prompt is read
  by the cursor network on the tree the chat has built so far, closed with the end word `{eom}`,
  and the reply is the output the network says. A reply with no output is shown as nothing. While
  a turn runs, the bar above the composer shows the words read, the steps the network takes at
  each word by their classes, and the words of the reply.
- **Chats.** The rail lists every chat, most recent first. Each chat has its own tree: opening one
  starts the engine again from the seeds and reads that chat's prompts before it answers. Chats
  live in IndexedDB.
- **Chat menu.** Each chat in the rail has a menu: pin, rename, delete. Pinned chats sit in their
  own section. Above the game the same chats stand in one dialog, with a new one, the chat to read,
  a rename, a copy and a delete.
- **Rail width.** Drag the rail's right edge to resize it, between 220 and 480 pixels. The width
  is remembered with the chats.
- **Privacy Policy.** A page at `/privacy` for Datamine Network Inc., Ontario, Canada. It states
  what this page does: the model runs in the browser, the chats stay in IndexedDB, and
  the one outside request is the liquidity figure. A change that moves data anywhere is a change
  to it.
- **Offline, and as an app.** A service worker (`src/sw.template.js`, filled in by the build with
  the list of files) stores the page, the engine and the model files on first visit, so the page
  opens and answers with no connection; `public/manifest.webmanifest` makes it installable. The
  one relay path is never stored. The worker's version is a hash of the stored files' contents, so a
  retrained network under the same name replaces the stored copy on the next visit. `scripts/build-chat.sh`
  gzips the network and the engine's worker inflates it, since Cloudflare does not compress binary
  files; the wasm compiles while it downloads.
- **DAM1 | Max.** The dropdown in the prompt box names the model and lists what it runs with: the
  weights, the model size, the seeds, the step classes, and the cursor settings (steps per input
  item, reach, stack size and hidden units). There is one model and one effort, so nothing in it
  is a choice.
- **We are raising liquidity for further training.** The card above the greeting shows available
  liquidity across the network against the campaign's milestones, from $300,000 by steps of ten
  up to $1,000,000,000,000. Clicking the card opens the liquidity pools: each token with the trade
  link the Datamine dashboard sends a buyer to (`src/pools.ts`). The figure comes from the
  analytics site's dashboard page, fetched at `/api/page/dashboard`. That host answers no
  cross-origin reads, so the path stays same-origin: the dev server proxies it to
  `analytics.datamine.network`, and in production the Pages Function in `functions/api/` relays it
  (see The relay below). This is the page's one outside request; the engine never calls out.

## 🔒 Security

- **Headers.** `public/_headers` is read by Cloudflare Pages. It sends a Content Security Policy
  locked to `'self'` (plus `'wasm-unsafe-eval'` for the engine and `'unsafe-inline'` for the
  styles Material UI injects), `frame-ancestors 'none'`, `nosniff`, `no-referrer`, a
  Permissions-Policy that denies camera, microphone, geolocation and payment,
  `Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Resource-Policy: same-origin` and HSTS.
  It sends no `Cross-Origin-Embedder-Policy`, so the page is not cross-origin isolated. Pages get
  `Cache-Control: no-cache`. Hashed assets are cached for a year; the model files for a minute.
  Those paths detach the `/*` value with `! Cache-Control` first, because Pages joins the values of
  several matching rules with a comma.
- **The relay.** `functions/api/page/dashboard.ts` is a Cloudflare Pages Function: the one path
  the page fetches, forwarded to `analytics.datamine.network`. It forwards one fixed URL only,
  GET and HEAD only, sends nothing from the visitor upstream, and caches the figure at the edge
  for a minute. It exists so `connect-src` can stay `'self'`.
- **Stored state.** The chats persist in IndexedDB (`src/storage.ts`). The page reads the
  database once when it opens and works from memory after that; a refresh reads it again. Each
  change to a saved field is queued to a storage worker (`src/storage.worker.ts`), which writes one
  change at a time on its own thread and keeps only the latest state waiting per record. What comes
  back from storage is validated field by field before the store trusts it: ids must match a
  pattern, texts have a length cap, unknown kinds are dropped, and records are looked up with
  own-key checks so a key like `__proto__` cannot reach a lookup.
- **Fetched data.** The liquidity payload is checked shape by shape; a wrong shape is a failure,
  never a number on screen. Same-origin fetches send no credentials and no referrer.
- **Links.** Every outside link opens with `noopener noreferrer`. No script, style, font or image
  is loaded from another origin. No HTML is ever injected: React escapes every string, and the
  engine's output is plain text.
- **Dependencies.** Run `npm audit --omit=dev` after any dependency change.

## 🗂️ Layout

| File | What it holds |
| --- | --- |
| `src/agent.ts` | The agent: asks the worker for a turn and makes the reply from the network's output |
| `src/reader.worker.ts` | The engine in its own thread: loads the wasm, the network and the seeds, and takes every turn |
| `src/store.ts` | The zustand store: the chats, the route, the thinking flag, persistence |
| `src/storage.ts`, `src/storage.worker.ts` | The one read of IndexedDB at page open, and the worker that queues and writes every change |
| `src/routes.ts` | Routes and their paths; the store reads its first route from the URL |
| `src/liquidity.ts` | The liquidity store: the dashboard fetch, the milestones, the progress |
| `src/pools.ts` | The pools by chain, with each token's icon and trade link |
| `src/install.ts` | Registers the service worker, in a production build only |
| `src/sw.template.js` | The service worker, filled in by the build with the files to keep offline |
| `src/App.tsx` | The shell: rail, top bar, route sync with the URL |
| `src/components/` | Sidebar, ChatsMenu, RailBanner, Landing, Campaign, PoolsDialog, Conversation, Composer, ModelMenu, ConfirmDialog, PrivacyPage, MindGame, MindPage, Mark |
| `src/theme.ts` | The palette and spacing from the analytics site |
| `functions/api/page/dashboard.ts` | The Pages Function that relays the liquidity figure |
| `public/_headers` | The headers Cloudflare Pages sends |

`src/wasm/`, `public/*.bin` and `public/network.json` are written by
`scripts/build-chat.sh` and ignored by git.

## ⚖️ License

The page, the engine it loads and the weights it serves are released under the GNU Affero General
Public License, version 3 or later, by Datamine Network Inc.: see [LICENSE](../LICENSE) and
[NOTICE.md](../NOTICE.md) at the root. A site serving this page must make its source available to
the people who use it; the unmodified page does so with the Source link under the composer and at
the bottom right, which opens this repository at the commit the page was built from (`__COMMIT__`,
set in `vite.config.ts` from `git rev-parse HEAD`).
