#!/bin/sh
# Build the chat page's engine and its files from the model:
#   NETWORK=path/to/network.bin[,path/to/second.bin,...] [SEEDS=all] [HALF=0] sh scripts/build-chat.sh
# The weights are written half as wide unless HALF=0, which halves what the page downloads and loads.
# It writes chat/src/wasm (the engine of the word network, model/crates/dam-web compiled to WebAssembly),
# chat/public/network.bin (the network's weights, gzipped), chat/public/network.json (its step classes
# and shape), chat/public/build.json (the commit, the voters and the time of the build) and chat/public/state.bin (the permanent state every chat starts from, built by dam word state
# from the seeds SEEDS names: the word all for every seeds file, which is the default since the network is
# taught on the state every seed makes, the word none for an empty tree, or seeds file names joined by commas).
# Several networks joined by commas read by vote, and a network file may hold several voters itself
# (scripts/join-networks.mjs): their weights are written one after another into network.bin and
# network.json counts the voters. They are not committed: run
# this before npm run dev or npm run build in chat/. It needs wasm-pack.
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
MODEL=$ROOT/model
CHAT=$ROOT/chat
NETWORK=${NETWORK:?name the trained network, its classes beside it with .json added, or several joined by commas}
SEEDS=${SEEDS:-all}
NETWORKS=$(echo "$NETWORK" | tr ',' ' ')
FIRST=${NETWORKS%% *}
VOTERS=0
for one in $NETWORKS; do
  [ -f "$one" ] && [ -f "$one.json" ] || { echo "no network at $one with its classes at $one.json" >&2; exit 1; }
  VOTERS=$((VOTERS + 1))
done
(cd "$MODEL" && wasm-pack build crates/dam-web --target web --release --out-dir pkg)
mkdir -p "$CHAT/src/wasm" "$CHAT/public"
cp "$MODEL/crates/dam-web/pkg/dam_web.js" "$MODEL/crates/dam-web/pkg/dam_web_bg.wasm" "$MODEL/crates/dam-web/pkg/dam_web.d.ts" "$CHAT/src/wasm/"
JOINED=$(mktemp)
HALF=${HALF:-1} node "$ROOT/scripts/join-networks.mjs" "$JOINED" $NETWORKS
gzip -9 -c "$JOINED" > "$CHAT/public/network.bin"
cp "$JOINED.json" "$CHAT/public/network.json"
# A network file may hold several voters itself, so the count is read from the joined classes file.
VOTERS=$(node -e "console.log(JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')).voters)" "$CHAT/public/network.json")
rm -f "$JOINED" "$JOINED.json"
(cd "$MODEL" && cargo run --release -p dam-cli -- word state --out "$CHAT/public/state.bin" --seeds "$SEEDS")
rm -f "$CHAT/public/seeds.txt"
# What the page was built from, shown small on the explorer: the commit, the networks that vote, when it was built
# and how many bytes the network, the state and the engine take as the page loads them.
COMMIT=$(cd "$ROOT" && git rev-parse --short HEAD 2>/dev/null || echo unknown)
DIRTY=$(cd "$ROOT" && git status --porcelain -- model 2>/dev/null | grep -q . && echo "+" || echo "")
NAMES=$(for one in $NETWORKS; do basename "$(dirname "$one")"/"$(basename "$one" .bin)"; done | tr "\n" " " | sed "s/ $//")
BYTES=$(cat "$CHAT/public/network.bin" "$CHAT/public/state.bin" "$CHAT/src/wasm/dam_web_bg.wasm" | wc -c | tr -d ' ')
printf '{"commit":"%s%s","voters":%s,"networks":"%s","built":"%s","bytes":%s}\n' "$COMMIT" "$DIRTY" "$VOTERS" "$NAMES" "$(date -u +%Y-%m-%dT%H:%MZ)" "$BYTES" > "$CHAT/public/build.json"
echo "built the engine, the network and the state into chat/"
