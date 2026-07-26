#!/usr/bin/env bash
set -euo pipefail

root=${1:?bundle root is required}
output=${2:?output directory is required}
venv=${CODETETHER_TRAIN_VENV:-$root/.venv}
python=${PYTHON:-python3}
train=${CODETETHER_TRAIN_FILE:-$root/data/train-pairs.jsonl}
validation=${CODETETHER_VALIDATION_FILE:-$root/data/validation-pairs.jsonl}
epochs=${CODETETHER_EPOCHS:-1}
resume=${CODETETHER_RESUME:-}

test -f "$train"
test -f "$validation"

if [[ ! -x "$venv/bin/python" ]]; then
    "$python" -m venv "$venv"
    "$venv/bin/pip" install --upgrade pip
    "$venv/bin/pip" install \
        torch==2.6.0 torchvision==0.21.0 \
        --index-url https://download.pytorch.org/whl/cu124
    "$venv/bin/pip" install -r \
        "$root/scripts/model_training/requirements-gpu.txt"
fi

export PYTHONPATH="$root/scripts"
export PYTORCH_CUDA_ALLOC_CONF=expandable_segments:True
export TOKENIZERS_PARALLELISM=false

"$venv/bin/python" -m model_training.gpu_probe

args=(
    --train "$train"
    --validation "$validation"
    --output "$output"
    --epochs "$epochs"
    --masked
)
if [[ -n "$resume" ]]; then
    args+=(--resume "$resume")
fi

exec "$venv/bin/python" -m model_training.train "${args[@]}"
