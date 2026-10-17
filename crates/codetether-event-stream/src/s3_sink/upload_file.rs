//! Read an event file and derive its archival object key.

use super::{S3Sink, S3SinkError};
use std::path::PathBuf;
impl S3Sink {
    /// Upload a local file to S3/R2
    ///
    /// # Arguments
    /// Reads the supplied path and uploads it under the prefix and session ID.
    ///
    /// # Returns
    /// The archival URL on success.
    ///
    /// # Errors
    /// Returns an error for invalid filenames, filesystem reads, request preparation, transport, or non-success responses.
    ///
    /// # Examples
    /// ```no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_event_stream::s3_sink::{S3Sink, S3SinkConfig};
    /// let config = serde_json::from_str::<S3SinkConfig>(r#"{"bucket":"logs","prefix":"events/","stub_mode":true}"#).unwrap();
    /// let sink = S3Sink::from_config(config).await.unwrap();
    /// let url = sink.upload_file(&"events.jsonl".into(), "run").await.unwrap();
    /// assert!(url.contains("events/run/events.jsonl"));
    /// # });
    /// ```
    pub async fn upload_file(
        &self,
        local_path: &PathBuf,
        session_id: &str,
    ) -> Result<String, S3SinkError> {
        let filename = local_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| S3SinkError::UploadFailed("Invalid filename".to_string()))?;

        // Read file contents
        let data = tokio::fs::read(local_path).await?;

        // S3 key: prefix/session_id/timestamp-filename
        let s3_key = format!("{}{}/{}", self.config.prefix, session_id, filename);

        // Upload to S3
        self.upload_bytes(&data, &s3_key, "application/json").await
    }
}
