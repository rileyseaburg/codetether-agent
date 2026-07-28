#!/usr/bin/env bash
# Stream checkpoints to Cloud Storage while training runs.
#
# Preemptible A100 capacity can be reclaimed mid-run, so checkpoints must
# leave the ephemeral boot disk. Runs as a background loop beside training.
#
# The interval also bounds diagnostic loss: an early failure before the
# first sync leaves nothing to inspect, and Cloud Logging is not readable by
# this project's service account.
set -euo pipefail

state=${1:?state directory is required}
bucket=${2:?bucket is required}
interval=${CODETETHER_MIRROR_INTERVAL:-60}
run_id=${CODETETHER_RUN_ID:-qwen3-coder-v4}
prefix="gs://$bucket/model-training/$run_id"

while true; do
    gsutil -q -m rsync -r "$state/output" "$prefix/output" 2>/dev/null || true
    gsutil -q -m cp "$state/logs/"*.json "$prefix/logs/" 2>/dev/null || true
    gsutil -q -m cp "$state/logs/"*.log "$prefix/logs/" 2>/dev/null || true
    sleep "$interval"
done