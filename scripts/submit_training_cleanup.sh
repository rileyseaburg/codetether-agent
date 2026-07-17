#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
artifact_root=${CODETETHER_TRAINING_CLEANUP_ARTIFACT_DIR:-"$repo_root/artifacts/training-cleanup"}
stamp=$(date -u +%Y%m%dT%H%M%SZ)
package="$artifact_root/training_cleanup_$stamp.zip"
mkdir -p "$artifact_root"
(
    cd "$repo_root/scripts"
    python3 -m zipfile -c "$package" training_cleanup
)
spark_submit=${SPARK_SUBMIT_BIN:-spark-submit}
spark_args=(--py-files "$package")
if [[ -n ${SPARK_S3_PACKAGES:-} ]]; then
    spark_args+=(--packages "$SPARK_S3_PACKAGES")
fi
exec "$spark_submit" "${spark_args[@]}" \
    "$repo_root/scripts/clean_training_data_spark.py" "$@"
