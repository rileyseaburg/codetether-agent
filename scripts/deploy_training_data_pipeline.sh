#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifests="$repo_root/deploy/training-data"
image=${PIPELINE_IMAGE:-us-central1-docker.pkg.dev/spotlessbinco/codetether/codetether-training-pipeline:20260723-v19}
chart=https://github.com/apache/polaris/releases/download/apache-polaris-1.3.0-incubating/polaris-1.3.0-incubating.tgz
if [[ ${SKIP_PIPELINE_BUILD:-false} != true ]]; then
    docker build -f "$repo_root/docker/training-data-pipeline.Dockerfile" -t "$image" "$repo_root"
    docker push "$image"
fi
kubectl apply -f "$manifests/namespace.yaml"
"$repo_root/scripts/deploy_training_data_secrets.sh"
kubectl get secret gcp-artifact-registry -n a2a-server -o json | jq \
    '.metadata = {name:"quantum-forge-registry",namespace:"codetether-data"}' | \
    kubectl apply -f -
kubectl apply -f "$manifests/postgres-service.yaml"
kubectl apply -f "$manifests/postgres-statefulset.yaml"
kubectl rollout status statefulset/polaris-postgres-nfs -n codetether-data --timeout=5m
kubectl apply -f "$manifests/polaris-bootstrap-job.yaml"
kubectl wait job/polaris-bootstrap-v1 -n codetether-data --for=condition=complete --timeout=5m
helm upgrade --install polaris "$chart" -n codetether-data \
    -f "$manifests/polaris-values.yaml" --wait --timeout 10m
kubectl apply -f "$manifests/spark-rbac.yaml"
kubectl apply -f "$manifests/polaris-setup-job-v2.yaml"
kubectl wait job/polaris-catalog-setup-v2 -n codetether-data --for=condition=complete --timeout=5m
kubectl apply -f "$manifests/spark-history-config.yaml"
kubectl apply -f "$manifests/spark-history-deployment.yaml"
kubectl apply -f "$manifests/spark-history-service.yaml"
kubectl apply -f "$manifests/trino-config.yaml"
kubectl apply -f "$manifests/trino-deployment.yaml"
kubectl apply -f "$manifests/trino-service.yaml"
kubectl apply -f "$manifests/cleanup-cronjob.yaml"
kubectl rollout status deployment/spark-history -n codetether-data --timeout=5m
kubectl rollout status deployment/trino -n codetether-data --timeout=5m
