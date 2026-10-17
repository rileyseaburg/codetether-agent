//! Resolve the configured service endpoint.

use super::S3Sink;
impl S3Sink {
    /// Build the S3 endpoint URL
    pub(super) fn endpoint_url(&self) -> String {
        if let Some(ref endpoint) = self.config.endpoint {
            // Custom endpoint (R2, MinIO, etc.)
            format!("{}/{}", endpoint.trim_end_matches('/'), self.config.bucket)
        } else {
            // AWS S3
            format!(
                "https://{}.s3.{}.amazonaws.com",
                self.config.bucket, self.config.region
            )
        }
    }
}
