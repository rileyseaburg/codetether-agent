#!/usr/bin/env bash
# Verify bootstrap path wiring without a GPU or network.
#
# Confirms state lives outside the clone, so deleting the clone cannot
# destroy a running job, and that launch reporting requires a live process.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"; pkill -f "$work" 2>/dev/null || true' EXIT

clone="$work/clone"
state="$work/state"
mkdir -p "$clone/scripts"
cp -r "$root/scripts/model_training" "$clone/scripts/"
cp "$clone/scripts/model_training/tests/bootstrap_stubs/"*.py \
    "$clone/scripts/model_training/"

export CODETETHER_HF_REPO=stub/repo
export CODETETHER_BUNDLE="$clone"
export CODETETHER_STATE="$state"
export VAULT_ADDR=https://vault.invalid
export VAULT_TOKEN=stub
export CODETETHER_SKIP_INSTALL=1
export HF_TOKEN=stub-token

bash "$clone/scripts/model_training/colab_bootstrap.sh" \
    >"$work/bootstrap.out" 2>"$work/bootstrap.err" || true

if [[ ! -f "$state/logs/train.log" ]]; then
    echo 'FAIL: no train log'
    echo '--- stderr ---'; tail -20 "$work/bootstrap.err"
    echo '--- stdout ---'; tail -20 "$work/bootstrap.out"
    exit 1
fi
test -f "$state/data/train-pairs.jsonl" || { echo 'FAIL: no data'; exit 1; }
grep -q '"running": true' "$work/bootstrap.out" \
    || { echo 'FAIL: launch not verified'; cat "$work/bootstrap.err"; exit 1; }

# Deleting the clone must not remove training state.
rm -rf "$clone"
test -f "$state/logs/train.log" || { echo 'FAIL: state lost'; exit 1; }

echo 'PASS: state survives clone deletion and launch is verified'