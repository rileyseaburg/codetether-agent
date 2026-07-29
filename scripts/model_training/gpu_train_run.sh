#!/usr/bin/env bash
# Run local fine-tuning on the deployment GPU.
#
# Nested SSH drops backgrounded children, so the work is wrapped in a script
# that setsid can own. Uses a dedicated HF cache because the shared one has
# root-owned lock directories.
set -euo pipefail

root=/home/ubuntu/ct-eval
cd "$root"

export HF_HOME="$root/hf-cache"
export PYTHONPATH="$root"
export PYTORCH_CUDA_ALLOC_CONF=expandable_segments:True
export TOKENIZERS_PARALLELISM=false

exec "$root/.venv/bin/python" -m model_training.train_local \
    --train "$root/data/local-train.jsonl" \
    --validation "$root/data/local-val.jsonl" \
    --output "$root/out-local" \
    --epochs "${CODETETHER_EPOCHS:-1}"
