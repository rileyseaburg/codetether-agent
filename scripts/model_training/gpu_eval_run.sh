#!/usr/bin/env bash
# Evaluate one checkpoint on the deployment GPU.
#
# Run detached on the GPU host: nested SSH drops backgrounded children, so
# the work is wrapped in a script that setsid can own.
set -euo pipefail

adapter=${1:?adapter directory is required}
output=${2:?output json path is required}
root=/home/ubuntu/ct-eval

cd "$root"
export HF_HOME="$root/hf-cache"
export PYTHONPATH="$root"

exec "$root/.venv/bin/python" -m model_training.eval_checkpoint \
    --adapter "$adapter" \
    --output "$output"
