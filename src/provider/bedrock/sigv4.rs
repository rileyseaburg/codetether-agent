//! AWS SigV4 request signing and HTTP dispatch for Bedrock.
//!
//! This module implements the AWS Signature Version 4 algorithm needed to
//! authenticate Bedrock API calls when using IAM credentials. It also holds
//! the request dispatch helpers ([`BedrockProvider::send_request`] and
//! [`BedrockProvider::send_converse_request`]) that pick between SigV4 and
//! bearer-token auth.
//!
//! # Examples
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
//!
//! let creds = AwsCredentials::from_environment().unwrap();
//! let p = BedrockProvider::with_credentials(creds, "us-west-2".into()).unwrap();
//! let body = br#"{"messages":[]}"#;
//! let url = "https://bedrock-runtime.us-west-2.amazonaws.com/model/amazon.nova-micro-v1:0/converse";
//! let _resp = p.send_converse_request(url, body).await.unwrap();
//! # });
//! ```

use super::BedrockProvider;
use super::auth::BedrockAuth;
use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

impl BedrockProvider {
    /// Runtime Bedrock base URL (Converse / Converse-stream).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::provider::bedrock::BedrockProvider;
    /// let p = BedrockProvider::with_region("token".into(), "us-west-2".into()).unwrap();
    /// // internal: p.base_url() -> "https://bedrock-runtime.us-west-2.amazonaws.com"
    /// ```
    pub(super) fn base_url(&self) -> String {
        format!("https://bedrock-runtime.{}.amazonaws.com", self.region)
    }

    /// Management API URL (ListFoundationModels, ListInferenceProfiles).
    pub(super) fn management_url(&self) -> String {
        format!("https://bedrock.{}.amazonaws.com", self.region)
    }

    /// Send a pre-formed POST request to any Bedrock runtime URL, signed with
    /// whichever auth mode is configured. Used by the thinker backend for
    /// direct URL access.
    ///
    /// # Arguments
    ///
    /// * `url` — The full HTTPS URL to POST to.
    /// * `body` — The serialized JSON request body.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the network call fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
    /// let creds = AwsCredentials::from_environment().unwrap();
    /// let p = BedrockProvider::with_credentials(creds, "us-west-2".into()).unwrap();
    /// let url = "https://bedrock-runtime.us-west-2.amazonaws.com/model/amazon.nova-micro-v1:0/converse";
    /// let resp = p.send_converse_request(url, b"{}" ).await.unwrap();
    /// assert!(resp.status().is_client_error() || resp.status().is_success());
    /// # });
    /// ```
    pub async fn send_converse_request(&self, url: &str, body: &[u8]) -> Result<reqwest::Response> {
        self.send_request("POST", url, Some(body), "bedrock").await
    }

    /// Send an HTTP request using whichever auth mode is configured.
    pub(super) async fn send_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&[u8]>,
        service: &str,
    ) -> Result<reqwest::Response> {
        match &self.auth {
            BedrockAuth::SigV4(_) => {
                self.send_signed_request(method, url, body.unwrap_or(b""), service)
                    .await
            }
            BedrockAuth::BearerToken(token) => {
                let mut req = self
                    .client
                    .request(method.parse().unwrap_or(reqwest::Method::GET), url)
                    .bearer_auth(token)
                    .header("content-type", "application/json")
                    .header("accept", "application/json");
                if let Some(b) = body {
                    req = req.body(b.to_vec());
                }
                req.send()
                    .await
                    .context("Failed to send request to Bedrock")
            }
        }
    }

    /// Build a SigV4-signed request and send it.
    async fn send_signed_request(
        &self,
        method: &str,
        url: &str,
        body: &[u8],
        service: &str,
    ) -> Result<reqwest::Response> {
        let creds = match &self.auth {
            BedrockAuth::SigV4(c) => c,
            BedrockAuth::BearerToken(_) => {
                anyhow::bail!("send_signed_request called with bearer token auth");
            }
        };

        let now = chrono::Utc::now();
        let datestamp = now.format("%Y%m%d").to_string();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();

        let host_start = url.find("://").map(|i| i + 3).unwrap_or(0);
        let after_host = url[host_start..]
            .find('/')
            .map(|i| host_start + i)
            .unwrap_or(url.len());
        let host = url[host_start..after_host].to_string();
        let path_and_query = &url[after_host..];
        let (canonical_uri, canonical_querystring) = match path_and_query.split_once('?') {
            Some((p, q)) => (p.to_string(), q.to_string()),
            None => (path_and_query.to_string(), String::new()),
        };

        let payload_hash = sha256_hex(body);

        let mut headers_map: Vec<(&str, String)> = vec![
            ("content-type", "application/json".to_string()),
            ("host", host.clone()),
            ("x-amz-date", amz_date.clone()),
        ];
        if let Some(token) = &creds.session_token {
            headers_map.push(("x-amz-security-token", token.clone()));
        }
        headers_map.sort_by_key(|(k, _)| *k);

        let canonical_headers: String = headers_map
            .iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        let signed_headers: String = headers_map
            .iter()
            .map(|(k, _)| *k)
            .collect::<Vec<_>>()
            .join(";");

        let canonical_request = format!(
            "{method}\n{canonical_uri}\n{canonical_querystring}\n{canonical_headers}\n{signed_headers}\n{payload_hash}"
        );

        let credential_scope = format!("{datestamp}/{}/{service}/aws4_request", self.region);

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{amz_date}\n{credential_scope}\n{}",
            sha256_hex(canonical_request.as_bytes())
        );

        let k_date = hmac_sha256(
            format!("AWS4{}", creds.secret_access_key).as_bytes(),
            datestamp.as_bytes(),
        );
        let k_region = hmac_sha256(&k_date, self.region.as_bytes());
        let k_service = hmac_sha256(&k_region, service.as_bytes());
        let k_signing = hmac_sha256(&k_service, b"aws4_request");

        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{credential_scope}, SignedHeaders={signed_headers}, Signature={signature}",
            creds.access_key_id
        );

        let mut req = self
            .client
            .request(method.parse().unwrap_or(reqwest::Method::POST), url)
            .header("content-type", "application/json")
            .header("host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .header("authorization", &authorization);

        if let Some(token) = &creds.session_token {
            req = req.header("x-amz-security-token", token);
        }

        if method == "POST" || method == "PUT" {
            req = req.body(body.to_vec());
        }

        req.send()
            .await
            .context("Failed to send signed request to Bedrock")
    }
}

/// HMAC-SHA256 helper returning raw bytes.
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// SHA-256 helper returning lowercase hex.
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
