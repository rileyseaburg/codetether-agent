//! Catalog fetch from.
use super::*;
impl ModelCatalog {
    /// Fetch models with a custom URL (for testing or alternate sources)
    #[allow(dead_code)]
    ///
    /// # Errors
    /// Returns HTTP transport or response JSON decoding failures.
    pub async fn fetch_from(url: &str) -> anyhow::Result<Self> {
        let response = reqwest::get(url).await?;
        let providers: ModelsApiResponse = response.json().await?;
        Ok(Self { providers })
    }
}
