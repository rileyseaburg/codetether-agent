//! Dispatch byte uploads, including explicit stub mode.

use super::{S3Sink, S3SinkError};
impl S3Sink {
    /// Upload bytes directly to S3/R2
    ///
    /// # Arguments
    /// Uploads the supplied bytes to the object key; content type is retained for compatibility and requests use application/json.
    ///
    /// # Returns
    /// The archival URL on success.
    ///
    /// # Errors
    /// Returns an error for request preparation, transport, or non-success responses.
    ///
    /// # Examples
    /// ```no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_event_stream::s3_sink::{S3Sink, S3SinkConfig};
    /// let config = serde_json::from_str::<S3SinkConfig>(r#"{"bucket":"logs","prefix":"events/","stub_mode":true}"#).unwrap();
    /// let sink = S3Sink::from_config(config).await.unwrap();
    /// assert_eq!(sink.upload_bytes(b"{}", "event.json", "application/json").await.unwrap(), "s3://logs/event.json");
    /// # });
    /// ```
    pub async fn upload_bytes(
        &self,
        data: &[u8],
        s3_key: &str,
        _content_type: &str,
    ) -> Result<String, S3SinkError> {
        // Stub mode: skip actual HTTP request
        if self.config.stub_mode {
            let url = if let Some(ref endpoint) = self.config.endpoint {
                format!("{}/{}", endpoint.trim_end_matches('/'), s3_key)
            } else {
                format!("s3://{}/{}", self.config.bucket, s3_key)
            };

            tracing::debug!(
                key = %s3_key,
                bytes = data.len(),
                "[STUB MODE] Skipping S3 upload"
            );

            return Ok(url);
        }

        let headers = self.upload_headers(data, s3_key)?;
        let url = format!("{}/{}", self.endpoint_url(), s3_key);

        let response = self
            .client
            .put(&url)
            .headers(headers)
            .body(data.to_vec())
            .send()
            .await?;

        self.upload_response(response, s3_key).await
    }
}
