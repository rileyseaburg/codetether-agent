#!/usr/bin/env bash

append_secret_refs() {
    local secret_name=$1
    local role
    for role in driver executor; do
        spark+=(--conf "spark.kubernetes.$role.secretKeyRef.MINIO_ACCESS_KEY=$secret_name:minio-access-key")
        spark+=(--conf "spark.kubernetes.$role.secretKeyRef.MINIO_SECRET_KEY=$secret_name:minio-secret-key")
        spark+=(--conf "spark.kubernetes.$role.secretKeyRef.AWS_ACCESS_KEY_ID=$secret_name:minio-access-key")
        spark+=(--conf "spark.kubernetes.$role.secretKeyRef.AWS_SECRET_ACCESS_KEY=$secret_name:minio-secret-key")
        spark+=(--conf "spark.kubernetes.$role.secretKeyRef.POLARIS_PIPELINE_ID=$secret_name:polaris-pipeline-id")
        spark+=(--conf "spark.kubernetes.$role.secretKeyRef.POLARIS_PIPELINE_SECRET=$secret_name:polaris-pipeline-secret")
    done
}
