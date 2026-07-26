#!/usr/bin/env bash
# Bootstrap the v4 QLoRA run on a Colab GPU runtime.
#
# Pulls the dataset from HuggingFace, installs CodeTether, loads provider
# credentials from public Vault, then trains with assistant-only loss
# masking. Requires VAULT_ADDR and VAULT_TOKEN in the environment; no
# credential is ever written into this script or the notebook.
set -euo pipefail

repo=${CODETETHER_HF_REPO:?CODETETHER_HF_REPO is required}
bundle=${CODETETHER_BUNDLE:-/content/ct}
# Keep state outside the clone so re-cloning cannot destroy a running job.
state=${CODETETHER_STATE:-/content/codetether-state}
output=${CODETETHER_OUTPUT:-$state/output}
log=${CODETETHER_LOG:-$state/logs/train.log}
resume=${CODETETHER_RESUME:-}
cli=${CODETETHER_INSTALL_CLI:-0}

: "${VAULT_ADDR:?VAULT_ADDR is required}"
: "${VAULT_TOKEN:?VAULT_TOKEN is required}"

mkdir -p "$state/data" "$output" "$(dirname "$log")"

# Refuse to start a second trainer, but only when the existing one is
# healthy. A process whose code directory was deleted keeps running while
# writing to deleted inodes, so it must not block a relaunch.
for existing in $(pgrep -f 'model_training.train' 2>/dev/null); do
    if python3 -m model_training.proc_health --pid "$existing"; then
        echo "a healthy training process is already running: $existing" >&2
        exit 1
    fi
    echo "reaping wedged trainer $existing (code directory deleted)" >&2
    kill -9 "$existing" 2>/dev/null || true
done

# Dependency installation is skippable so path wiring can be verified on a
# host with an externally managed Python.
if [[ "${CODETETHER_SKIP_INSTALL:-0}" != "1" ]]; then
    python3 -m pip install --quiet --upgrade pip
    python3 -m pip install --quiet huggingface_hub
    python3 -m pip install --quiet -r \
        "$bundle/scripts/model_training/requirements-gpu.txt"
fi

export PYTHONPATH="$bundle/scripts"
export PYTORCH_CUDA_ALLOC_CONF=expandable_segments:True
export TOKENIZERS_PARALLELISM=false

# Base-model weights are a ~60 GB anonymous pull otherwise, which the Hub
# rate-limits. Export the Vault token so downloads are authenticated.
HF_TOKEN=$(python3 -m model_training.hf_export_token)
export HF_TOKEN
export HF_HUB_ENABLE_HF_TRANSFER=0

python3 -m model_training.hf_fetch \
    --repo "$repo" \
    --output "$state/data"

# Building the Rust CLI takes tens of minutes and is not needed to train,
# so it is opt-in. Vault reachability is still verified either way.
setup_args=(--output "$state/codetether-setup.json")
if [[ "$cli" != "1" ]]; then
    setup_args+=(--skip-install)
fi
python3 -m model_training.codetether_setup "${setup_args[@]}"

python3 -m model_training.gpu_probe | tee "$state/logs/gpu-probe.json"

probe="$state/logs/gpu-probe.json"
model=$(python3 -m model_training.read_plan --path "$probe")
python3 -m model_training.disk_guard --path "$state" --model "$model" \
    | tee "$state/logs/disk-guard.json"

args=(
    --train "$state/data/train-pairs.jsonl"
    --validation "$state/data/validation-pairs.jsonl"
    --output "$output"
    --epochs "${CODETETHER_EPOCHS:-1}"
    --masked
)
if [[ -n "$resume" ]]; then
    args+=(--resume "$resume")
fi

setsid nohup python3 -m model_training.train "${args[@]}" \
    </dev/null >>"$log" 2>&1 &
child=$!

# Report only a launch that survived startup. A stale PID with no log is
# indistinguishable from success otherwise.
sleep 5
if ! kill -0 "$child" 2>/dev/null; then
    echo "training exited immediately; last log lines:" >&2
    tail -20 "$log" >&2 || true
    exit 1
fi

python3 -m model_training.launch_report \
    --pid "$child" \
    --log "$log" \
    --output "$output"

# Mirror checkpoints to Drive when it is mounted, so a lost runtime does not
# discard hours of training. Runs detached alongside the trainer.
mirror=${CODETETHER_MIRROR:-/content/drive/MyDrive/codetether-v4}
if [[ -d "$(dirname "$mirror")" ]]; then
    mkdir -p "$mirror"
    setsid nohup bash -c \
        "while kill -0 $child 2>/dev/null; do \
            rsync -a --delete '$output/' '$mirror/' 2>/dev/null; \
            sleep 300; \
        done" </dev/null >>"$state/logs/mirror.log" 2>&1 &
    echo "{\"mirror\": \"$mirror\", \"interval_seconds\": 300}"
fi