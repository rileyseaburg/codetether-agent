#!/usr/bin/env bash
# Build and push the qwen3_5-capable training image to Artifact Registry.
#
# Vertex prebuilt images stop at torch 2.4, which cannot run transformers 5.x,
# so Qwen3.6 and Qwen3.5 need this image. The build verifies the architecture
# is importable and fails early if it is not.
set -euo pipefail

project=${CODETETHER_GCP_PROJECT:-$(gcloud config get-value project)}
region=${CODETETHER_GCP_REGION:-us-central1}
repo=${CODETETHER_AR_REPO:-codetether}
tag=${CODETETHER_IMAGE_TAG:-qwen35-$(date +%Y%m%d)}
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
image="$region-docker.pkg.dev/$project/$repo/codetether-trainer:$tag"

gcloud artifacts repositories describe "$repo" \
    --location="$region" --project="$project" >/dev/null 2>&1 ||
    gcloud artifacts repositories create "$repo" \
        --repository-format=docker --location="$region" --project="$project"

# Cloud Build expects a file named Dockerfile in the context root, so the
# trainer definition is staged into a clean directory with only what it needs.
context=$(mktemp -d)
trap 'rm -rf "$context"' EXIT
cp "$here/Dockerfile.trainer" "$context/Dockerfile"
cp "$here/requirements-qwen35.txt" "$context/"
cp "$here/arch_check.py" "$context/"

gcloud builds submit "$context" \
    --project="$project" \
    --tag="$image" \
    --timeout=3600s \
    --machine-type=e2-highcpu-8

echo "{\"image\": \"$image\"}"