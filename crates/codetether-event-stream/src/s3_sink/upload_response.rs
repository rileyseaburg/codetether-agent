//! Interpret upload responses and report archival URLs.

use super::{S3Sink, S3SinkError};
impl S3Sink {
    pub(super) async fn upload_response(
        &self,
        response: reqwest::Response,
        s3_key: &str,
    ) -> Result<String, S3SinkError> {
        if response.status().is_success() {
            let url = if let Some(ref endpoint) = self.config.endpoint {
                format!("{}/{}", endpoint.trim_end_matches('/'), s3_key)
            } else {
                format!(
                    "https://{}.s3.{}.amazonaws.com/{}",
                    self.config.bucket, self.config.region, s3_key
                )
            };

            tracing::info!(url = %url, "Uploaded event stream to S3");
            Ok(url)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(S3SinkError::S3Error(format!("{}: {}", status, body)))
        }
    }
}
