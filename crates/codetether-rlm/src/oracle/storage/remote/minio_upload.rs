//! Upload implementation for [`MinioOracleRemote`].
//!
//! Implements the [`OracleRemote`](super::OracleRemote) trait and
//! builds date-partitioned `.jsonl` object keys.
//!
//! # Examples
//!
//! ```ignore
//! let (key, url) = backend.upload_record(&record).await?;
//! ```

use super::OracleRemote;
use super::minio_backend::MinioOracleRemote;
use crate::oracle::record::OracleTraceRecord;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use minio::s3::builders::ObjectContent;

#[async_trait]
impl OracleRemote for MinioOracleRemote {
    async fn upload_record(&self, record: &OracleTraceRecord) -> Result<(String, String)> {
        let key = object_key(self, record);
        let payload = serde_json::to_vec(record).context("Serialize oracle record")?;
        self.client
            .put_object_content(&self.bucket, &key, ObjectContent::from(payload))
            .region(Some(self.region.clone()))
            .send()
            .await
            .with_context(|| format!("Upload to {}/{}", self.bucket, key))?;
        let url = format!(
            "{}/{}/{}",
            self.endpoint.trim_end_matches('/'),
            self.bucket,
            key
        );
        Ok((key, url))
    }
}

fn object_key(remote: &MinioOracleRemote, record: &OracleTraceRecord) -> String {
    let now = Utc::now();
    let trace_id = if record.trace.trace_id.is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        record.trace.trace_id.clone()
    };
    let prefix = remote.prefix.trim_end_matches('/');
    format!(
        "{prefix}/oracle/{}/{}/{}_{}.jsonl",
        record.verdict,
        now.format("%Y/%m/%d/%H"),
        now.format("%Y%m%dT%H%M%S"),
        trace_id,
    )
}
