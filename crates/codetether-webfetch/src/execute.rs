//! execute for web content fetching.
use super::*;
impl WebFetchTool {
    pub(crate) async fn run(&self, params: Value) -> Result<ToolResult> {
        let p: params::Params = serde_json::from_value(params).context("Invalid params")?;
        let url = p.url.parse::<reqwest::Url>().context("Invalid URL")?;
        if url.scheme() != "http" && url.scheme() != "https" {
            return Ok(ToolResult::error("Only HTTP/HTTPS supported"));
        }

        codetether_tls::ensure_rustls_crypto_provider();

        let github_auth = github::auth_for(&url, p.parent_workspace.as_deref()).await;
        let resp = github::apply(self.client.get(url), github_auth.as_ref())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        if !resp.status().is_success() {
            return Ok(ToolResult::error(format!("HTTP {}", resp.status())));
        }
        let ct = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        let bytes = resp.bytes().await.context("Failed to read body")?;
        if bytes.len() > MAX_CONTENT_LENGTH {
            return Ok(ToolResult::error(format!(
                "Content too large ({} bytes > {} max)",
                bytes.len(),
                MAX_CONTENT_LENGTH
            )));
        }
        let body = github::redact(
            String::from_utf8_lossy(&bytes).to_string(),
            github_auth.as_ref(),
        );
        let content = self.convert(&ct, body, &p.format);
        result::render(content, &p, github_auth.is_some())
    }
}
