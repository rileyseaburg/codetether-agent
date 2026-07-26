#!/usr/bin/env bash
set -euo pipefail

model=${CODETETHER_LOCAL_GGUF:?CODETETHER_LOCAL_GGUF is required}
llama=${CODETETHER_LLAMA_DIR:?CODETETHER_LLAMA_DIR is required}
port=${CODETETHER_LOCAL_PORT:-8080}
alias=${CODETETHER_LOCAL_MODEL_ALIAS:-codetether-local}
log=${CODETETHER_LOCAL_LOG:?CODETETHER_LOCAL_LOG is required}
threads=${CODETETHER_LOCAL_THREADS:-8}
ctx=${CODETETHER_LOCAL_CTX:-32768}
template=${CODETETHER_LOCAL_TEMPLATE:-chatml}

mkdir -p "$(dirname "$log")"
setsid nohup "$llama/build/bin/llama-server" \
    --model "$model" \
    --alias "$alias" \
    --host 127.0.0.1 \
    --port "$port" \
    --ctx-size "$ctx" \
    --threads "$threads" \
    --chat-template "$template" \
    --no-webui \
    </dev/null >>"$log" 2>&1 &

echo "{\"pid\": $!, \"port\": $port, \"alias\": \"$alias\", \"log\": \"$log\"}"