//! Build the signed headers for an archival PUT.

use super::{S3Sink, S3SinkError, digest::sha256_hex_bytes};
use reqwest::{
    Method,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};
impl S3Sink {
    pub(super) fn upload_headers(
        &self,
        data: &[u8],
        s3_key: &str,
    ) -> Result<HeaderMap, S3SinkError> {
        let path = format!("/{}", s3_key);

        // Generate AWS Signature V4 headers
        let now = chrono::Utc::now();
        let date = now.format("%Y%m%dT%H%M%SZ").to_string();

        let content_hash = sha256_hex_bytes(data);

        let method = Method::PUT;
        let auth_header = self.generate_aws_header(&method, &path, "", &content_hash, &date);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-amz-date",
            HeaderValue::from_str(&date).map_err(|e| S3SinkError::UploadFailed(e.to_string()))?,
        );
        headers.insert(
            "x-amz-content-sha256",
            HeaderValue::from_str(&content_hash)
                .map_err(|e| S3SinkError::UploadFailed(e.to_string()))?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_header)
                .map_err(|e| S3SinkError::UploadFailed(e.to_string()))?,
        );

        Ok(headers)
    }
}
