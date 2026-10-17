//! Moltbook client get.
use super::*;
impl MoltbookClient {
    pub(super) async fn get(&self, path: &str) -> Result<String> {
        let url = Self::validate_url(path);
        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .with_context(|| format!("GET {}", url))?;
        let status = resp.status();
        let body = resp.text().await?;
        if !status.is_success() {
            anyhow::bail!("Moltbook API error {} on GET {}: {}", status, path, body);
        }
        Ok(body)
    }
}
