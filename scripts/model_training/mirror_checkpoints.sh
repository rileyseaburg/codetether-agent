#!/usr/bin/env bash
# Mirror training checkpoints to Drive independently of the bootstrap.
#
# The bootstrap only starts mirroring when Drive is already mounted. Mount
# order is easy to get wrong, so this can be started at any time against a
# run that is already in progress.
set -euo pipefail

state=${CODETETHER_STATE:-/content/codetether-state}
output=${CODETETHER_OUTPUT:-$state/output}
mirror=${CODETETHER_MIRROR:-/content/drive/MyDrive/codetether-v4}
interval=${CODETETHER_MIRROR_INTERVAL:-300}

test -d "$(dirname "$mirror")" || {
    echo 'Drive is not mounted; mount it before mirroring' >&2
    exit 1
}
mkdir -p "$mirror" "$output"

setsid nohup bash -c \
    "while pgrep -f 'model_training.train' >/dev/null 2>&1; do \
        rsync -a --delete '$output/' '$mirror/' 2>/dev/null; \
        sleep $interval; \
    done; \
    rsync -a '$output/' '$mirror/' 2>/dev/null" \
    </dev/null >>"$state/logs/mirror.log" 2>&1 &

echo "{\"mirror\": \"$mirror\", \"pid\": $!, \"interval_seconds\": $interval}"