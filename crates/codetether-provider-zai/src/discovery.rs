//! Best-effort API discovery merged with the static catalog.
use crate::{ModelInfo, ZaiProvider};
use anyhow::Result;
use serde_json::Value;
impl ZaiProvider {
    /// Return the provider identifier.
    ///
    /// # Examples
    /// ```rust
    /// use codetether_provider_zai::ZaiProvider;
    /// let provider = ZaiProvider::with_client(reqwest::Client::new(), String::new(), String::new());
    /// assert_eq!(provider.name(), "zai");
    /// ```
    pub fn name(&self) -> &str {
        "zai"
    }

    /// List discovered models, filling omissions from the known catalog.
    ///
    /// # Returns
    /// The merged catalog; discovery failures return the known models.
    /// # Errors
    /// Discovery errors are logged and absorbed rather than returned.
    /// # Examples
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_provider_zai::ZaiProvider;
    /// let provider = ZaiProvider::with_client(reqwest::Client::new(),
    ///     String::new(), "http://localhost:1234/api/coding/paas/v4".into());
    /// assert!(provider.list_models().await?.iter().any(|m| m.id == "glm-5.2"));
    /// # Ok::<(), anyhow::Error>(()) }).unwrap();
    /// ```
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(crate::catalog::merge_known(self.discover().await))
    }
    async fn discover(&self) -> Vec<ModelInfo> {
        let url = format!("{}/models", self.base_url.replace("/coding/", "/"));
        let payload = async {
            self.client
                .get(&url)
                .bearer_auth(&self.api_key)
                .send()
                .await?
                .error_for_status()?
                .json::<Value>()
                .await
        }
        .await;
        match payload {
            Ok(payload) => crate::model_wire::parse(&payload),
            Err(error) => {
                tracing::debug!(url = %url, error = %error, "Z.AI model discovery failed");
                Vec::new()
            }
        }
    }
}
