//! Oracle trace persistence with local spool + MinIO/S3 upload.

use super::{OracleResult, OracleTraceRecord};
use crate::bus::s3_sink::BusS3SinkConfig;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use minio::s3::builders::ObjectContent;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::{Client as MinioClient, ClientBuilder as MinioClientBuilder};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

const DEFAULT_MAX_SPOOL_BYTES: u64 = 500 * 1024 * 1024;

/// Returns the default local spool directory for oracle traces.
pub fn default_spool_dir() -> PathBuf {
    if let Ok(path) = std::env::var("CODETETHER_ORACLE_SPOOL_DIR") {
        return PathBuf::from(path);
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".codetether")
        .join("traces")
        .join("pending")
}

/// Result of persisting a single oracle trace record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleTracePersistResult {
    pub verdict: String,
    pub spooled_path: String,
    pub uploaded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    pub pending_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

/// Result of syncing pending spool files to remote storage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OracleTraceSyncStats {
    pub uploaded: usize,
    pub retained: usize,
    pub failed: usize,
    pub pending_after: usize,
}

#[async_trait]
trait OracleRemote: Send + Sync {
    async fn upload_record(&self, record: &OracleTraceRecord) -> Result<(String, String)>;
}

struct MinioOracleRemote {
    client: MinioClient,
    endpoint: String,
    bucket: String,
    prefix: String,
    region: String,
}

impl MinioOracleRemote {
    fn from_bus_config(config: BusS3SinkConfig) -> Result<Self> {
        let endpoint = normalize_endpoint(&config.endpoint, config.secure);
        let static_provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
        let base_url = BaseUrl::from_str(&endpoint)
            .with_context(|| format!("Invalid MinIO endpoint URL: {endpoint}"))?;
        let client = MinioClientBuilder::new(base_url)
            .provider(Some(Box::new(static_provider)))
            .build()
            .context("Failed to build MinIO client for oracle storage")?;
        let region = std::env::var("CODETETHER_BUS_S3_REGION")
            .or_else(|_| std::env::var("MINIO_REGION"))
            .or_else(|_| std::env::var("AWS_REGION"))
            .unwrap_or_else(|_| "us-east-1".to_string());

        Ok(Self {
            client,
            endpoint,
            bucket: config.bucket,
            prefix: config.prefix,
            region,
        })
    }

    fn object_key(&self, record: &OracleTraceRecord) -> String {
        let now = Utc::now();
        let date_path = now.format("%Y/%m/%d/%H").to_string();
        let timestamp = now.format("%Y%m%dT%H%M%S").to_string();
        let trace_id = if record.trace.trace_id.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            record.trace.trace_id.clone()
        };
        let prefix = self.prefix.trim_end_matches('/');
        format!(
            "{prefix}/oracle/{}/{date_path}/{}_{}.jsonl",
            record.verdict, timestamp, trace_id
        )
    }
}

#[async_trait]
impl OracleRemote for MinioOracleRemote {
    async fn upload_record(&self, record: &OracleTraceRecord) -> Result<(String, String)> {
        let key = self.object_key(record);
        let payload = serde_json::to_vec(record).context("Failed to serialize oracle record")?;
        let content = ObjectContent::from(payload);

        self.client
            .put_object_content(&self.bucket, &key, content)
            .region(Some(self.region.clone()))
            .send()
            .await
            .with_context(|| format!("Failed to upload oracle trace to {}/{}", self.bucket, key))?;

        let url = format!(
            "{}/{}/{}",
            self.endpoint.trim_end_matches('/'),
            self.bucket,
            key
        );
        Ok((key, url))
    }
}

/// Oracle trace storage manager.
pub struct OracleTraceStorage {
    spool_dir: PathBuf,
    max_spool_bytes: u64,
    remote: Option<Arc<dyn OracleRemote>>,
}

impl OracleTraceStorage {
    /// Build storage from environment + Vault-backed MinIO config.
    pub async fn from_env_or_vault() -> Self {
        let spool_dir = default_spool_dir();
        let max_spool_bytes = std::env::var("CODETETHER_ORACLE_SPOOL_MAX_BYTES")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_MAX_SPOOL_BYTES);

        let remote = match BusS3SinkConfig::from_env_or_vault().await {
            Ok(cfg) => match MinioOracleRemote::from_bus_config(cfg) {
                Ok(remote) => Some(Arc::new(remote) as Arc<dyn OracleRemote>),
                Err(e) => {
                    tracing::warn!(error = %e, "Oracle storage remote init failed; using local spool");
                    None
                }
            },
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    "Oracle storage remote not configured; using local spool only"
                );
                None
            }
        };

        Self {
            spool_dir,
            max_spool_bytes,
            remote,
        }
    }

    #[cfg(test)]
    fn new_for_test(
        spool_dir: PathBuf,
        max_spool_bytes: u64,
        remote: Option<Arc<dyn OracleRemote>>,
    ) -> Self {
        Self {
            spool_dir,
            max_spool_bytes,
            remote,
        }
    }

    /// Persist an oracle result by first writing to local spool, then uploading.
    pub async fn persist_result(&self, result: &OracleResult) -> Result<OracleTracePersistResult> {
        let record = result.to_record();
        self.persist_record(&record).await
    }

    /// Persist a canonical oracle trace record.
    pub async fn persist_record(
        &self,
        record: &OracleTraceRecord,
    ) -> Result<OracleTracePersistResult> {
        tokio::fs::create_dir_all(&self.spool_dir)
            .await
            .with_context(|| format!("Failed to create spool dir {}", self.spool_dir.display()))?;
        self.warn_if_spool_large().await;

        let spool_path = self
            .spool_dir
            .join(spool_filename(record, Utc::now().timestamp_millis()));
        write_json_atomic(&spool_path, record).await?;

        let mut uploaded = false;
        let mut remote_key = None;
        let mut remote_url = None;
        let mut warning = None;

        if let Some(remote) = &self.remote {
            match remote.upload_record(record).await {
                Ok((key, url)) => {
                    remote_key = Some(key);
                    remote_url = Some(url);
                    uploaded = true;
                    if let Err(e) = tokio::fs::remove_file(&spool_path).await {
                        warning = Some(format!(
                            "Uploaded but failed to remove spool file {}: {}",
                            spool_path.display(),
                            e
                        ));
                    }
                }
                Err(e) => {
                    warning = Some(format!("Upload failed, kept local spool: {e}"));
                }
            }
        } else {
            warning = Some("Remote MinIO not configured, kept local spool".to_string());
        }

        let pending_count = self.pending_count().await?;
        Ok(OracleTracePersistResult {
            verdict: record.verdict.clone(),
            spooled_path: spool_path.display().to_string(),
            uploaded,
            remote_key,
            remote_url,
            pending_count,
            warning,
        })
    }

    /// Sync all pending spool records to remote MinIO, retaining failures locally.
    pub async fn sync_pending(&self) -> Result<OracleTraceSyncStats> {
        tokio::fs::create_dir_all(&self.spool_dir)
            .await
            .with_context(|| format!("Failed to create spool dir {}", self.spool_dir.display()))?;

        let Some(remote) = &self.remote else {
            let pending_after = self.pending_count().await?;
            return Ok(OracleTraceSyncStats {
                uploaded: 0,
                retained: pending_after,
                failed: 0,
                pending_after,
            });
        };

        let mut stats = OracleTraceSyncStats::default();
        let mut dir = tokio::fs::read_dir(&self.spool_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }

            let data = tokio::fs::read_to_string(&path).await?;
            let record: OracleTraceRecord = match serde_json::from_str(&data) {
                Ok(r) => r,
                Err(e) => {
                    stats.failed += 1;
                    stats.retained += 1;
                    tracing::warn!(
                        path = %path.display(),
                        error = %e,
                        "Skipping invalid oracle spool file"
                    );
                    continue;
                }
            };

            match remote.upload_record(&record).await {
                Ok(_) => {
                    stats.uploaded += 1;
                    if let Err(e) = tokio::fs::remove_file(&path).await {
                        stats.failed += 1;
                        stats.retained += 1;
                        tracing::warn!(
                            path = %path.display(),
                            error = %e,
                            "Uploaded oracle spool file but failed to delete local copy"
                        );
                    }
                }
                Err(e) => {
                    stats.failed += 1;
                    stats.retained += 1;
                    tracing::warn!(
                        path = %path.display(),
                        error = %e,
                        "Failed to sync oracle spool file"
                    );
                }
            }
        }

        stats.pending_after = self.pending_count().await?;
        Ok(stats)
    }

    async fn pending_count(&self) -> Result<usize> {
        if !self.spool_dir.exists() {
            return Ok(0);
        }
        let mut count = 0usize;
        let mut dir = tokio::fs::read_dir(&self.spool_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("jsonl") {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn warn_if_spool_large(&self) {
        match spool_usage_bytes(&self.spool_dir).await {
            Ok(bytes) if bytes > self.max_spool_bytes => {
                tracing::warn!(
                    usage_bytes = bytes,
                    max_bytes = self.max_spool_bytes,
                    dir = %self.spool_dir.display(),
                    "Oracle spool usage exceeds configured limit"
                );
            }
            Ok(_) => {}
            Err(e) => {
                tracing::debug!(
                    error = %e,
                    dir = %self.spool_dir.display(),
                    "Failed to compute oracle spool usage"
                );
            }
        }
    }
}

fn normalize_endpoint(endpoint: &str, secure: bool) -> String {
    let endpoint = endpoint.trim_end_matches('/');
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.to_string()
    } else if secure {
        format!("https://{endpoint}")
    } else {
        format!("http://{endpoint}")
    }
}

fn spool_filename(record: &OracleTraceRecord, ts_ms: i64) -> String {
    let trace_id = if record.trace.trace_id.is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        record.trace.trace_id.clone()
    };
    format!("{ts_ms}_{trace_id}_{}.jsonl", record.verdict)
}

async fn write_json_atomic(path: &Path, value: &OracleTraceRecord) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid spool path {}", path.display()))?;
    tokio::fs::create_dir_all(parent).await?;

    let tmp_path = parent.join(format!(
        ".{}.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("oracle_record")
    ));
    let json = serde_json::to_string(value).context("Failed to serialize oracle record")?;
    tokio::fs::write(&tmp_path, json).await?;
    tokio::fs::rename(&tmp_path, path).await?;
    Ok(())
}

async fn spool_usage_bytes(dir: &Path) -> Result<u64> {
    if !dir.exists() {
        return Ok(0);
    }
    let mut total = 0u64;
    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("jsonl")
            && let Ok(meta) = entry.metadata().await
        {
            total = total.saturating_add(meta.len());
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlm::oracle::{FinalPayload, VerificationMethod};

    struct MockRemoteOk;

    #[async_trait]
    impl OracleRemote for MockRemoteOk {
        async fn upload_record(&self, _record: &OracleTraceRecord) -> Result<(String, String)> {
            Ok((
                "oracle/key.jsonl".to_string(),
                "http://example/oracle/key.jsonl".to_string(),
            ))
        }
    }

    struct MockRemoteErr;

    #[async_trait]
    impl OracleRemote for MockRemoteErr {
        async fn upload_record(&self, _record: &OracleTraceRecord) -> Result<(String, String)> {
            anyhow::bail!("simulated upload failure")
        }
    }

    fn sample_record(verdict: &str) -> OracleTraceRecord {
        OracleTraceRecord {
            verdict: verdict.to_string(),
            reason: Some("reason".to_string()),
            agreement_ratio: None,
            trace: crate::rlm::oracle::ValidatedTrace {
                prompt: "Find async fns".to_string(),
                trace: vec![],
                final_payload: Some(FinalPayload::Semantic(
                    crate::rlm::oracle::SemanticPayload {
                        file: "src/main.rs".to_string(),
                        answer: "answer".to_string(),
                    },
                )),
                verdict: verdict.to_string(),
                oracle_diff: None,
                repo_revision: "abc123".to_string(),
                timestamp: Utc::now().to_rfc3339(),
                answer: "answer".to_string(),
                iterations: 1,
                subcalls: 0,
                input_tokens: 1,
                output_tokens: 1,
                elapsed_ms: 1,
                source_path: Some("src/main.rs".to_string()),
                verification_method: VerificationMethod::None,
                trace_id: uuid::Uuid::new_v4().to_string(),
            },
        }
    }

    #[tokio::test]
    async fn persist_keeps_spool_when_remote_fails() {
        let temp = tempfile::tempdir().expect("tempdir");
        let storage = OracleTraceStorage::new_for_test(
            temp.path().to_path_buf(),
            DEFAULT_MAX_SPOOL_BYTES,
            Some(Arc::new(MockRemoteErr)),
        );

        let result = storage
            .persist_record(&sample_record("failed"))
            .await
            .expect("persist");

        assert!(!result.uploaded);
        assert_eq!(result.pending_count, 1);
        assert!(Path::new(&result.spooled_path).exists());
    }

    #[tokio::test]
    async fn persist_removes_spool_when_remote_succeeds() {
        let temp = tempfile::tempdir().expect("tempdir");
        let storage = OracleTraceStorage::new_for_test(
            temp.path().to_path_buf(),
            DEFAULT_MAX_SPOOL_BYTES,
            Some(Arc::new(MockRemoteOk)),
        );

        let result = storage
            .persist_record(&sample_record("golden"))
            .await
            .expect("persist");

        assert!(result.uploaded);
        assert_eq!(result.pending_count, 0);
        assert!(!Path::new(&result.spooled_path).exists());
    }

    #[tokio::test]
    async fn sync_pending_uploads_and_clears_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let storage = OracleTraceStorage::new_for_test(
            temp.path().to_path_buf(),
            DEFAULT_MAX_SPOOL_BYTES,
            Some(Arc::new(MockRemoteOk)),
        );

        let _ = storage
            .persist_record(&sample_record("unverified"))
            .await
            .expect("persist");
        // Persist with failing remote first to leave one pending file.
        let failing = OracleTraceStorage::new_for_test(
            temp.path().to_path_buf(),
            DEFAULT_MAX_SPOOL_BYTES,
            Some(Arc::new(MockRemoteErr)),
        );
        let _ = failing
            .persist_record(&sample_record("failed"))
            .await
            .expect("persist fail-mode");

        let stats = storage.sync_pending().await.expect("sync");
        assert!(stats.uploaded >= 1);
        assert_eq!(stats.pending_after, 0);
    }
}
