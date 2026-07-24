# Historical training-data cleanup

The curated table format is Apache Iceberg v2, with Parquet data files. The
existing `s3a://codetether-training/training/v2/` JSONL remains the immutable
bronze source and is never edited or deleted by this process.

## Silver tables

The default catalog namespace is `iceberg.codetether`:

| Table | Grain | Partitioning | Purpose |
|---|---|---|---|
| `training_samples` | One bounded conversation chunk | Day of start, quality tier | Model-ready messages |
| `training_quarantine` | One rejected source line | Cleanup day, reason | Exclusion audit and diagnosis |
| `training_cleanup_manifests` | One source object per run | Cleanup day | Counts, hashes, and reproducibility |

`training_samples.messages` is a typed
`array<struct<role,content,name,tool_call_id,tool_calls>>`; it is not an opaque
JSON string. Every v2 sample retains every source object, SHA-256, and line in
`source_records`, plus its correlation ID, quality tier, cleanup run ID, chunk
index/count, and serialized message size. Tool calls and their results remain
in the same chunk. Chunks are capped at 96 messages and 65,536 characters.

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

Cleanup v2 reconstructs turns globally by sender and correlation ID before
validation, so conversations spanning multiple JSONL batch objects are no
longer discarded. Reprocessing upgrades manifests to cleanup version 2 while
stable sample and quarantine identifiers keep retries idempotent.

The v17 write canary processed 399 July 23 objects and committed 51 unique
samples (38 `complete`, 13 `tool_use`), 90 unique quarantine rows, and 399 v2
manifests. Its largest sample has 89 messages and 65,443 characters. All 51
samples retain multi-record provenance, 46 span multiple source objects (up to
21 objects), and sensitive quarantine rows retain no payload.

Run `cleanup-v2-reprocess-v19-20260723` then processed 103,284 objects and
635,296 raw lines before the exclusive 21:00 cutoff. The reconciled v2 tables
contain 698 unique samples (584 `complete`, 114 `tool_use`), 626,667 unique
quarantine rows, and 103,286 unique manifests including two later canary
objects. Every manifest balances; 473 samples span multiple objects, the
largest spans 27 objects, and tool-call/result validation has zero mismatches.
The resulting snapshot IDs are `6814887792823430908`,
`7306953001138880240`, and `8580768907438163835`.

## VM history coverage

The live AgentBus sink does not crawl project directories. Historical VM
coverage is supplied by `scripts/training_import`, which recursively discovers
`.codetether-agent/sessions/*.json`, selects the newest and fullest snapshot
for each session ID, and converts native message parts into `training/v2`
records. Developer messages become system context; tool calls and results keep
their original identifiers. Objects are bounded at 8 MiB and uploaded under
deterministic, immutable `vm_session_*` keys.

The initial VM import collapsed 61,386 observed snapshots to 29,343 canonical
sessions and uploaded 29,007 objects containing 1,013,656 normalized records.
The 43 durable batches previously retained in
`~/.codetether/training/pending` were also uploaded without deleting the local
copies.

Project `.codetether/session-ledgers` and `memory-writeback` files are scope,
evidence, and memory metadata rather than chat transcripts. They remain local
audit inputs and are not mislabeled as training dialogue. Conversation history
comes from the canonical session snapshots and the live AgentBus stream.

## Deployed catalog and query plane

Apache Polaris is the physical catalog. It runs as two replicas with PostgreSQL
persistence on the `nfs-storage` StorageClass and exposes the Iceberg REST
catalog as Spark catalog `iceberg` and Trino catalog `iceberg`. Realm headers
are mandatory. The pipeline principal is granted `CATALOG_MANAGE_CONTENT`
through dedicated Polaris principal and catalog roles; root credentials are
used only by the idempotent setup job.

The pinned runtime is Spark 3.5.7, Iceberg 1.10.0, Hadoop AWS 3.3.4, and Java 17.
Its published image is
`codetether-training-pipeline:20260724-v20` with digest
`sha256:dc80b8009df1b0c1f7089e0ca038ff663f12a219d4e4378ada13c2964f754804`.
Historical jobs use 256 source partitions and disk-backed intermediates with
one Python task per executor to bound heap use; recurring jobs retain smaller
defaults. System context preceding the first user prompt is retained.
Trino 482 supplies distributed SQL and the browser query-monitoring UI.

Source discovery descends only to hour-sized shards, lists those shards
concurrently through paginated S3 APIs, and distributes object reads across
Spark executors. The MinIO client uses adaptive retries and a two-minute read
timeout so historical hours can tolerate slow paginated responses.

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
repository. The recurring CronJob starts suspended. Apply the hour and day audit
manifests and inspect their driver summaries before allowing writes:

```bash
kubectl apply -f deploy/training-data/cleanup-audit-v2-job.yaml
kubectl apply -f deploy/training-data/cleanup-audit-v2-day-job.yaml
kubectl logs -n codetether-data -l spark-role=driver -f
```

Review the JSON summary, then apply a bounded write canary:

```bash
kubectl apply -f deploy/training-data/cleanup-apply-v2-canary-v17-job.yaml
```

After the canary reconciles, run the resource-bounded historical reprocess:

```bash
kubectl apply -f deploy/training-data/cleanup-reprocess-v19-job.yaml
```

An apply creates the namespace and tables when needed. Samples, quarantine, and
manifests are merged in that order. A partial apply can safely retry the same
run ID; once manifests exist, a repeated completed run is rejected. Stable row
identifiers are merged across run IDs, and recurring applies anti-join existing
manifest source URIs before transformation, so historical rows are not copied
again.

Every run uses an exclusive source-key cutoff. The cleanup-v2 backlog uses
`training/v2/2026/07/23/21/`, excluding objects still arriving in the 21:00
hour. Scheduled runs derive the current UTC hour as their cutoff, giving each
run an immutable input window.

After reconciling table and manifest counts, enable the recurring schedule:

```bash
kubectl patch cronjob training-cleanup -n codetether-data \
  --type merge -p '{"spec":{"suspend":false}}'
```

## Consumption gate

Training jobs should require `cleanup_version = 2`, select reviewed run IDs,
exclude quarantine, and normally accept
`quality_tier IN ('complete', 'tool_use')`. They must also enforce
`cardinality(messages) <= 96` and `message_chars <= 65536`. Before release,
reconcile per-source counts and hashes against `training_cleanup_manifests` and
record the chosen Iceberg snapshot ID with the model artifact.