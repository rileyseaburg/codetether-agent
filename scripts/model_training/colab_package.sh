#!/usr/bin/env bash
# Merge, convert, and quantize the trained adapter inside the GPU runtime.
#
# Merging a 30B model dequantizes it on the CPU and needs about 67 GB of
# RAM, so it must run where that memory exists rather than on a workstation.
set -euo pipefail

bundle=${CODETETHER_BUNDLE:-/content/ct}
adapter=${CODETETHER_ADAPTER:-$bundle/output/final-adapter}
output=${CODETETHER_PACKAGE:-$bundle/package}
llama=${CODETETHER_LLAMA_DIR:-$bundle/llama.cpp}

test -d "$adapter"
mkdir -p "$output"
export PYTHONPATH="$bundle/scripts"

if [[ ! -f "$llama/convert_hf_to_gguf.py" ]]; then
    git clone --depth 1 https://github.com/ggml-org/llama.cpp "$llama"
fi
if [[ ! -x "$llama/build/bin/llama-quantize" ]]; then
    cmake -S "$llama" -B "$llama/build" -DLLAMA_CURL=OFF >/dev/null
    cmake --build "$llama/build" --target llama-quantize -j "$(nproc)" >/dev/null
fi

python3 -m pip install --quiet -r "$llama/requirements.txt"

python3 -m model_training.merge \
    --adapter "$adapter" \
    --output "$output/merged"

name=${CODETETHER_MODEL_NAME:-codetether-qwen3-coder-30b-v4}
f16="$output/$name-f16.gguf"
quantized="$output/$name-q4_k_m.gguf"

python3 "$llama/convert_hf_to_gguf.py" "$output/merged" \
    --outfile "$f16" --outtype f16
"$llama/build/bin/llama-quantize" "$f16" "$quantized" Q4_K_M

sha256sum "$quantized" | tee "$output/$name-checksums.txt"
rm -f "$f16"
echo "{\"quantized\": \"$quantized\"}"