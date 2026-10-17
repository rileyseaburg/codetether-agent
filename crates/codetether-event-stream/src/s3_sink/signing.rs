//! Produce the existing authorization format for archival requests.

use super::{
    S3Sink,
    digest::{hmac_sha256, sha256_hex},
};
use reqwest::Method;
impl S3Sink {
    /// Generate AWS Authorization header
    pub(super) fn generate_aws_header(
        &self,
        method: &Method,
        path: &str,
        query: &str,
        content_hash: &str,
        date: &str,
    ) -> String {
        let access_key = self.config.access_key.as_deref().unwrap_or("");
        let secret_key = self.config.secret_key.as_deref().unwrap_or("");

        let canonical_request = format!(
            "{}\n{}\n{}\n\ncontent-type:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n\ncontent-type;x-amz-content-sha256;x-amz-date",
            method, path, query, "application/octet-stream", content_hash, date
        );

        // String to sign
        let canonical_request_hash = sha256_hex(&canonical_request);
        let credential_scope = format!("{}/{}/s3/aws4_request", &date[..8], self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            date, credential_scope, canonical_request_hash
        );

        // Signing key
        let k_date = hmac_sha256(
            format!("AWS4{}", secret_key).as_bytes(),
            date[..8].as_bytes(),
        );
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");

        // Signature
        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders=content-type;x-amz-content-sha256;x-amz-date, Signature={}",
            access_key, credential_scope, signature
        )
    }
}
