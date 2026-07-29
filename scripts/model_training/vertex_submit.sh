#!/usr/bin/env bash
# Submit the v4 QLoRA run as a Vertex AI custom training job.
#
# The project's service account holds roles/aiplatform.admin but lacks
# Compute Engine permissions, so Vertex custom training is the available
# path. Verified quota in us-central1:
#
#   restricted_image_training_nvidia_a100_80gb_gpus     limit 8
#   custom_model_training_preemptible_nvidia_a100_gpus  limit 8
#
# Vertex jobs are server-side: they survive local disconnects entirely.
set -euo pipefail

project=${CODETETHER_GCP_PROJECT:-$(gcloud config get-value project)}
region=${CODETETHER_GCP_REGION:-us-central1}
bucket=${CODETETHER_GCS_BUCKET:?CODETETHER_GCS_BUCKET is required}

# v4 pairs supervised prose only; tool calls appeared in 4,989 of 6,000
# sampled prompts and 0 completions, and the resulting adapter scored 0.125
# tool-call rate against 0.875 for the untuned base. v5 supervises emitted
# calls in 82.1 percent of completions.
repo=${CODETETHER_HF_REPO:-rileyseaburg/codetether-agent-traces-v5}

# Verified to exist on 27 July 2026. The published name carries the Python
# suffix: `pytorch-gpu.2-4` alone does not exist and a job submitted against
# it fails with "the image does not exist" after reaching PENDING.
default_image=us-docker.pkg.dev/vertex-ai/training/pytorch-gpu.2-4.py310:latest
image=${CODETETHER_TRAIN_IMAGE:-$default_image}

# Read the default from the pinned constants so the machine tier follows the
# configured base model instead of a stale literal.
default_model=$(python3 -m model_training.read_base_model)
model=${CODETETHER_BASE_MODEL:-$default_model}

# Machine follows the model: the 30B needs the 80 GB tier, the 4B does not.
selected=$(python3 -m model_training.vertex_machine_cli --model "$model")
machine=${CODETETHER_MACHINE:-${selected%% *}}
accelerator=${CODETETHER_ACCELERATOR:-${selected##* }}

name=${CODETETHER_JOB_NAME:-codetether-qwen3-v4-$(date +%Y%m%d-%H%M%S)}

: "${VAULT_ADDR:?VAULT_ADDR is required}"
: "${VAULT_TOKEN:?VAULT_TOKEN is required}"

config=$(mktemp)
trap 'rm -f "$config"' EXIT

# Non-preemptible A100 40 GB quota is zero on this project; the available
# quotas are preemptible A100 (limit 8) and A100 80 GB (limit 8). Default to
# preemptible because checkpoints stream to Cloud Storage continuously.
preempt_flag=()
if [[ "${CODETETHER_PREEMPTIBLE:-1}" == "1" ]]; then
    preempt_flag=(--preemptible)
fi

python3 -m model_training.vertex_config \
    --image "$image" \
    --machine "$machine" \
    --accelerator "$accelerator" \
    --bucket "$bucket" \
    --hf-repo "$repo" \
    "${preempt_flag[@]}" \
    --output "$config"

gcloud ai custom-jobs create \
    --region="$region" \
    --project="$project" \
    --display-name="$name" \
    --config="$config"

echo "{\"job\": \"$name\", \"region\": \"$region\"}"