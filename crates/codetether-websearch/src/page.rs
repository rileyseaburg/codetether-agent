//! HTTP retrieval shared by search backends.
use crate::{SearchFailure, WebSearchTool};

/// Raw provider response captured for parsing and evidence.
pub(crate) struct Page {
    pub(crate) status: u16,
    pub(crate) request_id: Option<String>,
    pub(crate) body: String,
}

impl WebSearchTool {
    /// GET `url`, capturing status, body, and the optional request-id header.
    pub(crate) async fn fetch(
        &self,
        url: &str,
        id_header: Option<&str>,
    ) -> Result<Page, SearchFailure> {
        let response = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.9")
            .send()
            .await
            .map_err(|error| SearchFailure::Transport(error.to_string()))?;
        let status = response.status().as_u16();
        let request_id = id_header
            .and_then(|name| response.headers().get(name))
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);
        let body = response
            .text()
            .await
            .map_err(|error| SearchFailure::Transport(format!("HTTP {status} body: {error}")))?;
        Ok(Page {
            status,
            request_id,
            body,
        })
    }
}
