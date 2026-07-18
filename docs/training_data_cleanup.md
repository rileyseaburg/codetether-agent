# Historical training-data cleanup

The curated table format is Apache Iceberg v2, with Parquet data files. The
existing `s3a://codetether-training/training/v2/` JSONL remains the immutable
bronze source and is never edited or deleted by this process.

## Silver tables

The default catalog namespace is `iceberg.codetether`:

| Table | Grain | Partitioning | Purpose |
|---|---|---|---|
| `training_samples` | One correlated conversation | Day of start, quality tier | Model-ready messages |
| `training_quarantine` | One rejected source line | Cleanup day, reason | Exclusion audit and diagnosis |
| `training_cleanup_manifests` | One source object per run | Cleanup day | Counts, hashes, and reproducibility |

`training_samples.messages` is a typed
`array<struct<role,content,name,tool_call_id,tool_calls>>`; it is not an opaque
JSON string. Every sample also retains its source URI, source SHA-256, source
line numbers, correlation ID, quality tier, and cleanup run ID.

The quarantine table is not training input. Rows rejected as possible secrets,
embedded images, or oversized content retain a source pointer but do not copy
the payload into the table.

## Verified initial load

Run `cleanup-v6-20260717` processed 97,888 immutable objects before the 21:00
UTC cutoff. It committed 220 samples (217 `tool_use`, 3 `complete`), 595,053
quarantine rows, and 97,888 manifests. The three initial Iceberg snapshot IDs
are `769963461985822594`, `8030728867989525809`, and
`4179817863758059339`, respectively.

The v15 scheduled-path canary then discovered 97,896 objects, anti-joined the
existing manifests, and processed only eight unseen objects. It added 12
quarantine rows and eight manifests without rewriting historical samples.

## Deployed catalog and query plane

Apache Polaris is the physical catalog. It runs as two replicas with PostgreSQL
persistence and exposes the Iceberg REST catalog as Spark catalog `iceberg` and
Trino catalog `iceberg`. Realm headers are mandatory. The pipeline principal is
granted `CATALOG_MANAGE_CONTENT` through dedicated Polaris principal and catalog
roles; root credentials are used only by the idempotent setup job.

The pinned runtime is Spark 3.5.7, Iceberg 1.10.0, Hadoop AWS 3.3.4, and Java 17.
Its published image is
`codetether-training-pipeline:20260717-v15` with digest
`sha256:c17c63bfe49d97c03cb98ee94135fc4a394d725cc0d651316a18bbb7e7e1d0e7`.
Trino 482 supplies distributed SQL and the browser query-monitoring UI.

Source discovery descends only to hour-sized shards, lists those shards
concurrently through paginated S3 APIs, and distributes object reads across
Spark executors. The historical cutoff currently resolves to 97,888 objects;
the live discovery completes in roughly 45 seconds instead of recursively
walking every hour directory on the driver.

## Operator UI

The services are intentionally cluster-internal. Use authenticated Kubernetes
access rather than exposing an unauthenticated ingress:

```bash
kubectl port-forward -n codetether-data svc/trino 8080:8080
kubectl port-forward -n codetether-data svc/spark-history 18080:18080
```

Trino is available at `http://localhost:8080`; Spark job history is available at
`http://localhost:18080`. Polaris provides the governed catalog API and metrics,
while Trino and Spark History are the operational user interfaces.

## Runbook

Deploy the catalog and suspended pipeline after exporting MinIO configuration
without placing credentials in shell history:

```bash
export MINIO_ENDPOINT=http://minio.example.internal:9000
export MINIO_ACCESS_KEY_FILE=/run/secrets/minio-access-key
export MINIO_SECRET_KEY_FILE=/run/secrets/minio-secret-key
export MINIO_ACCESS_KEY=$(<"$MINIO_ACCESS_KEY_FILE")
export MINIO_SECRET_KEY=$(<"$MINIO_SECRET_KEY_FILE")
./scripts/deploy_training_data_pipeline.sh
```

The deployment creates Kubernetes Secrets and never stores credentials in the
repository. The recurring CronJob starts suspended. Apply the audit manifest and
inspect the driver summary before allowing writes:

```bash
kubectl apply -f deploy/training-data/cleanup-audit-job.yaml
kubectl logs -n codetether-data -l spark-role=driver -f
```

Review the JSON summary, then run the same immutable ID with writes enabled:

```bash
kubectl apply -f deploy/training-data/cleanup-apply-job.yaml
```

An apply creates the namespace and tables when needed. Samples, quarantine, and
manifests are merged in that order. A partial apply can safely retry the same
run ID; once manifests exist, a repeated completed run is rejected. Stable row
identifiers are merged across run IDs, and recurring applies anti-join existing
manifest source URIs before transformation, so historical rows are not copied
again.

Every run uses an exclusive source-key cutoff. The historical deployment uses
`training/v2/2026/07/17/21/`, which includes the reported 20:07 batch and
excludes objects still arriving in the 21:00 hour. Scheduled runs derive the
current UTC hour as their cutoff, giving each run an immutable input window.

After reconciling table and manifest counts, enable the recurring schedule:

```bash
kubectl patch cronjob training-cleanup -n codetether-data \
  --type merge -p '{"spec":{"suspend":false}}'
```

## Consumption gate

Training jobs should select exactly one reviewed `run_id`, exclude quarantine,
and normally accept `quality_tier IN ('complete', 'tool_use')`. Before release,
reconcile per-source counts and hashes against `training_cleanup_manifests` and
record the chosen Iceberg snapshot ID with the model artifact.
