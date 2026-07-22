#!/usr/bin/env bash
set -euo pipefail

if [[ ${1:-} == "--help" ]]; then
    echo "usage: scripts/open_trace_ui.sh [network.strace.PID.jsonl]"
    exit 0
fi

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/.." && pwd)
viewer_root="$repo_root/tools/trace-viewer"
trace_file=${1:-}
tetherscript_bin=${TETHERSCRIPT_BIN:-tetherscript}

if [[ -z $trace_file ]]; then
    trace_file=$(find "$repo_root/artifacts/openai-codex-trace" -type f -name 'network.strace.*.jsonl' -printf '%T@ %p\n' | sort -nr | head -n 1 | cut -d' ' -f2-)
fi
if [[ ! -f $trace_file ]]; then
    echo "Trace JSONL not found: $trace_file" >&2
    exit 1
fi

trace_hash=$(sha256sum "$trace_file" | cut -d' ' -f1)
chunk_root="$(dirname -- "$trace_file")/.trace-viewer/$trace_hash"
mkdir -p "$chunk_root"
if [[ ! -f $chunk_root/chunks.txt ]]; then
    split -C 900K -d -a 4 --additional-suffix=.jsonl \
        "$trace_file" "$chunk_root/chunk-"
    find "$chunk_root" -maxdepth 1 -type f -name 'chunk-*.jsonl' \
        -printf '%f\n' | sort > "$chunk_root/chunks.txt"
fi

tsc -p "$viewer_root/tsconfig.json"
npx --yes esbuild@0.27.7 "$viewer_root/src/app.ts" --bundle --format=iife --minify \
    --outfile="$viewer_root/dist/app.bundle.js" >/dev/null
export TRACE_UI_ROOT="$repo_root"
export TRACE_JSONL=$(realpath "$trace_file")
export TRACE_CHUNK_ROOT=$(realpath "$chunk_root")
export TRACE_UI_PORT=${TRACE_UI_PORT:-8789}
export TRACE_UI_TOKEN=$(openssl rand -hex 24)
viewer_host=127.0.0.1
if [[ -n ${SSH_CONNECTION:-} ]]; then
    read -r _ _ viewer_host _ <<< "$SSH_CONNECTION"
fi
url="http://$viewer_host:$TRACE_UI_PORT/?token=$TRACE_UI_TOKEN"

if [[ ${TRACE_UI_NO_OPEN:-0} != 1 ]] && command -v xdg-open >/dev/null 2>&1; then
    (sleep 0.7; xdg-open "$url" >/dev/null 2>&1 || true) &
fi
echo "Opening $url"
echo "Press Ctrl+C to stop the TetherScript server."
exec "$tetherscript_bin" run --interp --grant-fs "$repo_root" \
    "$repo_root/examples/tetherscript/trace_viewer_server.tether"
