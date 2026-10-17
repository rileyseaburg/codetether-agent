//! Catalog fetch.
use super::*;
impl ModelCatalog {
    /// Fetch models from the CodeTether API
    ///
    /// # Errors
    /// Returns HTTP transport or response JSON decoding failures.
    pub async fn fetch() -> anyhow::Result<Self> {
        const MODELS_URL: &str = "https://models.dev/api.json";
        tracing::info!("Fetching models from {}", MODELS_URL);
        let response = reqwest::get(MODELS_URL).await?;
        let providers: ModelsApiResponse = response.json().await?;
        tracing::info!("Loaded {} providers", providers.len());
        Ok(Self { providers })
    }
}
