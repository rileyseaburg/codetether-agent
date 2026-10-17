//! Moltbook client patch.
use super::*;
impl MoltbookClient {
    pub(super) async fn patch(&self, path: &str, payload: &serde_json::Value) -> Result<String> {
        let url = Self::validate_url(path);
        let resp = self
            .http
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .with_context(|| format!("PATCH {}", url))?;
        let status = resp.status();
        let body = resp.text().await?;
        if !status.is_success() {
            anyhow::bail!("Moltbook API error {} on PATCH {}: {}", status, path, body);
        }
        Ok(body)
    }
}
