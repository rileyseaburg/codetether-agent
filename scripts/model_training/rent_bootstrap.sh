#!/usr/bin/env bash
# Provision any rented GPU host and run the v4 QLoRA training to completion.
#
# Colab recycles runtimes long before a multi-hour epoch finishes, so long
# runs belong on a persistent box. Works on RunPod, Paperspace, Lambda,
# Crusoe, or any Ubuntu host with an NVIDIA driver: the workspace path is
# the only provider-specific detail.
#
# State lives outside the clone and training is detached, so an SSH
# disconnect cannot end the run.
set -euo pipefail

workspace=${CODETETHER_WORKSPACE:-$(pwd)}
bundle=${CODETETHER_BUNDLE:-$workspace/ct}
state=${CODETETHER_STATE:-$workspace/codetether-state}
repo=${CODETETHER_HF_REPO:?CODETETHER_HF_REPO is required}
origin=${CODETETHER_GIT_REPO:-https://github.com/rileyseaburg/codetether-training}

: "${VAULT_ADDR:?VAULT_ADDR is required}"
: "${VAULT_TOKEN:?VAULT_TOKEN is required}"

export CODETETHER_BUNDLE="$bundle"
export CODETETHER_STATE="$state"
export CODETETHER_HF_REPO="$repo"
export CODETETHER_MIRROR=${CODETETHER_MIRROR:-$state/mirror}

if [[ ! -d "$bundle/.git" ]]; then
    git clone --depth 1 "$origin" "$bundle"
fi

if ! command -v rsync >/dev/null 2>&1; then
    apt-get update -qq && apt-get install -y -qq rsync
fi

bash "$bundle/scripts/model_training/colab_bootstrap.sh"

cat <<INFO

Training is detached and survives disconnects.

  tail -f $state/logs/train.log
  PYTHONPATH=$bundle/scripts python3 -m model_training.run_status

When the final adapter appears:

  CODETETHER_BUNDLE=$bundle CODETETHER_STATE=$state \\
    bash $bundle/scripts/model_training/colab_package.sh
INFO
