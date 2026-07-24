#!/usr/bin/env bash
set -euo pipefail

run_id=${CLEANUP_RUN_ID:-"cleanup-$(date -u +%Y%m%d-%H%M%S)"}
source_before=${CLEANUP_SOURCE_BEFORE:-"training/v2/$(date -u +%Y/%m/%d/%H)/"}
source_prefix=${CLEANUP_SOURCE_PREFIX:-training/v2}
pipeline_image=${PIPELINE_IMAGE:?PIPELINE_IMAGE is required}
job_args=(--run-id "$run_id" --source-prefix "$source_prefix")
job_args+=(--source-before "$source_before")
job_args+=(--catalog iceberg --namespace codetether)
job_args+=(--min-partitions "${CLEANUP_MIN_PARTITIONS:-64}")
mode=audit
if [[ ${CLEANUP_APPLY:-false} == true ]]; then
    job_args+=(--apply)
    mode=apply
fi
if [[ ${CLEANUP_REPROCESS:-false} == true ]]; then
    job_args+=(--reprocess)
fi
driver_pod="training-$mode-$run_id-driver"
secret=training-data-secrets
source /opt/spark/jobs/training_spark_secrets.sh
spark=(/opt/spark/bin/spark-submit
    --master k8s://https://kubernetes.default.svc
    --deploy-mode cluster
    --name "training-$mode-$run_id"
    --conf "spark.kubernetes.driver.pod.name=$driver_pod"
    --conf spark.kubernetes.namespace=codetether-data
    --conf spark.kubernetes.authenticate.driver.serviceAccountName=training-spark
    --conf "spark.kubernetes.container.image=$pipeline_image"
    --conf spark.kubernetes.container.image.pullSecrets=quantum-forge-registry
    --conf "spark.executor.instances=${SPARK_EXECUTOR_INSTANCES:-3}"
    --conf "spark.executor.cores=${SPARK_EXECUTOR_CORES:-2}"
    --conf "spark.executor.memory=${SPARK_EXECUTOR_MEMORY:-3g}"
    --conf "spark.executor.memoryOverhead=${SPARK_EXECUTOR_OVERHEAD:-1g}"
    --conf "spark.driver.memory=${SPARK_DRIVER_MEMORY:-2g}"
    --conf spark.eventLog.enabled=true
    --conf spark.eventLog.dir=s3a://codetether-training/spark-events/
    --conf spark.hadoop.fs.s3a.endpoint=http://192.168.50.223:9000
    --conf spark.hadoop.fs.s3a.path.style.access=true
    --conf spark.hadoop.fs.s3a.connection.ssl.enabled=false
    --conf spark.hadoop.fs.s3a.aws.credentials.provider=com.amazonaws.auth.EnvironmentVariableCredentialsProvider)
append_secret_refs "$secret"
spark+=(--conf spark.kubernetes.driverEnv.MINIO_ENDPOINT=http://192.168.50.223:9000)
spark+=(--conf spark.kubernetes.driverEnv.POLARIS_URI=http://polaris:8181/api/catalog)
spark+=(--conf spark.kubernetes.driverEnv.POLARIS_WAREHOUSE=codetether)
spark+=(--conf spark.kubernetes.driverEnv.PYTHONPATH=/opt/spark/jobs)
spark+=(--conf spark.executorEnv.PYTHONPATH=/opt/spark/jobs)
spark+=(local:///opt/spark/jobs/clean_training_data_spark.py)
/opt/spark/jobs/verify_spark_driver.sh \
    "$driver_pod" "${spark[@]}" "${job_args[@]}"
