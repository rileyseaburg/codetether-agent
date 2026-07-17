#!/usr/bin/env bash
set -euo pipefail

: "${MINIO_ENDPOINT:?MINIO_ENDPOINT is required}"
: "${MINIO_ACCESS_KEY:?MINIO_ACCESS_KEY is required}"
: "${MINIO_SECRET_KEY:?MINIO_SECRET_KEY is required}"
namespace=codetether-data
if kubectl get secret training-data-secrets -n "$namespace" >/dev/null 2>&1; then
    echo "Reusing existing training-data-secrets"
    exit 0
fi
postgres_password=$(openssl rand -hex 32)
root_secret=$(openssl rand -hex 32)
pipeline_id=$(openssl rand -hex 8)
pipeline_secret=$(openssl rand -hex 16)
token_key=$(openssl rand -base64 64 | tr -d '\n')
kubectl create secret generic training-data-secrets -n "$namespace" \
    --from-literal=postgres-user=polaris \
    --from-literal=postgres-password="$postgres_password" \
    --from-literal=postgres-jdbc-url=jdbc:postgresql://polaris-postgres:5432/polaris \
    --from-literal=polaris-root-id=root \
    --from-literal=polaris-root-secret="$root_secret" \
    --from-literal=polaris-bootstrap-credentials="POLARIS,root,$root_secret" \
    --from-literal=polaris-pipeline-id="$pipeline_id" \
    --from-literal=polaris-pipeline-secret="$pipeline_secret" \
    --from-literal=polaris-pipeline-credential="$pipeline_id:$pipeline_secret" \
    --from-literal=polaris-token-key="$token_key" \
    --from-literal=minio-endpoint="$MINIO_ENDPOINT" \
    --from-literal=minio-access-key="$MINIO_ACCESS_KEY" \
    --from-literal=minio-secret-key="$MINIO_SECRET_KEY"
