//! Generic HTTP request tool.
//!
//! Sends an arbitrary HTTP request (method, URL, headers, body) and returns
//! the response. Designed for the common case where an agent has watched a
//! form submit in browserctl, captured the `fetch` call (method, headers,
//! body), and wants to replay it directly against the server without going
//! through the UI.
//!
//! The tool intentionally supports headers the browser would normally fill
//! in (Authorization, Cookie, X-CSRF-Token, etc.) so an authenticated
//! session can drive the backend API once the agent has lifted credentials
//! from the page's request.

mod execute;
mod params;
mod response;

use async_trait::async_trait;
use serde_json::{Value, json};

use super::{Tool, ToolResult};

/// HTTP tool. Owns a single reqwest client reused across calls.
pub struct HttpTool {
    client: reqwest::Client,
}

impl Default for HttpTool {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }
}

#[async_trait]
impl Tool for HttpTool {
    fn id(&self) -> &str {
        "http"
    }
    fn name(&self) -> &str {
        "HTTP Request"
    }
    fn description(&self) -> &str {
        "Send an arbitrary HTTP request (method, URL, headers, body). \
         Use this to replay API calls captured from browserctl network \
         inspection, bypassing the UI entirely. Supports GET, POST, PUT, \
         PATCH, DELETE, HEAD, OPTIONS. Body can be raw string or JSON."
    }
    fn parameters(&self) -> Value {
        params::schema()
    }
    async fn execute(&self, value: Value) -> anyhow::Result<ToolResult> {
        let params: params::HttpParams = serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("invalid params: {e}"))?;
        crate::tls::ensure_rustls_crypto_provider();
        execute::run(&self.client, params).await
    }
}

/// JSON metadata attached to successful responses (exposed for tests).
#[cfg(test)]
pub(crate) fn _status_metadata_key() -> &'static str {
    "status"
}
