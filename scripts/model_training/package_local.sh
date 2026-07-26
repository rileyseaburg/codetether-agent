#!/usr/bin/env bash
# Merge a trained adapter and package it as quantized GGUF.
#
# Naming is derived from the model rather than hardcoded, so a 30B export is
# never labelled with an earlier model's identity.
set -euo pipefail

adapter=${1:?adapter directory is required}
output=${2:?output directory is required}
venv=${3:?merge virtualenv is required}
llama=${4:?llama.cpp directory is required}
name=${CODETETHER_MODEL_NAME:-codetether-qwen3-coder-30b-v4}
quant=${CODETETHER_QUANT:-Q4_K_M}
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)

test -d "$adapter"
mkdir -p "$output"
export PYTHONPATH="$root/scripts"

"$venv/bin/python" -m model_training.merge \
    --adapter "$adapter" \
    --output "$output/merged"

f16="$output/$name-f16.gguf"
quantized="$output/$name-${quant,,}.gguf"

"$venv/bin/python" "$llama/convert_hf_to_gguf.py" \
    "$output/merged" \
    --outfile "$f16" \
    --outtype f16

"$llama/build/bin/llama-quantize" "$f16" "$quantized" "$quant"

sha256sum "$f16" "$quantized" | tee "$output/$name-checksums.txt"

# The f16 intermediate is large; keep it only when explicitly requested.
if [[ "${CODETETHER_KEEP_F16:-0}" != "1" ]]; then
    rm -f "$f16"
fi

echo "{\"model\": \"$name\", \"quantized\": \"$quantized\"}"
