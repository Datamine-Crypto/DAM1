import { execSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { defineConfig, type Plugin } from 'vite';
import react from '@vitejs/plugin-react';

// The reader is one large wasm asset handed to `init` by URL; inlining it would bloat the entry
// chunk. The reader's weights are a public file fetched at startup so a different one can be dropped in.
// The one outside request, the liquidity figure, goes to /api and is proxied to the analytics
// host, which answers no cross-origin reads; in production a Pages Function relays it.

// Every file the build writes, as the paths the browser asks for.
function built(dir: string, root: string): string[] {
  const paths: string[] = [];
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) paths.push(...built(full, root));
    else paths.push(`/${relative(root, full).split('\\').join('/')}`);
  }
  return paths;
}

// Writes the service worker after the build, with the list of files to keep offline and a
// version that changes when their contents do. The _headers file, the worker itself, the manifest
// and the files for crawlers and link previews are not kept, and neither are the model's files,
// which the engine's worker downloads once and keeps in its own cache, so a first visit downloads
// the network one time.
function offlinePlugin(): Plugin {
  let outDir = 'dist';
  return {
    name: 'datamine-offline',
    apply: 'build',
    configResolved(config) { outDir = resolve(config.root, config.build.outDir); },
    closeBundle() {
      // Pages redirects /index.html to /, so the page is kept as / alone. The files for crawlers and
      // link previews are never needed offline.
      const skip = new Set(['/_headers', '/sw.js', '/manifest.webmanifest', '/index.html', '/robots.txt', '/sitemap.xml', '/images/og.png', '/network.bin', '/network.json', '/state.bin']);
      const list = ['/', ...built(outDir, outDir).filter((file) => !skip.has(file))];
      // The version follows the contents: a retrained reader keeps its file name, and the stored
      // copy must still be replaced on the next visit.
      const digest = createHash('sha256');
      for (const file of list) digest.update(readFileSync(join(outDir, file === '/' ? 'index.html' : file)));
      const version = digest.digest('hex').slice(0, 12);
      const template = readFileSync(resolve(import.meta.dirname, 'src/sw.template.js'), 'utf8');
      const worker = template.replace('__VERSION__', version).replace('__PRECACHE__', JSON.stringify(list));
      writeFileSync(join(outDir, 'sw.js'), worker);
    },
  };
}

// The page links to its own source at the commit it was built from, as its licence asks; a build
// outside a checkout links to the repository as a whole.
function builtFrom(): string {
  try {
    return execSync('git rev-parse HEAD', { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
  } catch {
    return '';
  }
}

export default defineConfig({
  define: { __COMMIT__: JSON.stringify(builtFrom()) },
  plugins: [react(), offlinePlugin()],
  server: {
    proxy: {
      '/api': {
        target: 'https://analytics.datamine.network',
        changeOrigin: true,
      },
    },
  },
  build: {
    target: 'es2022',
    assetsInlineLimit: 0,
  },
});
