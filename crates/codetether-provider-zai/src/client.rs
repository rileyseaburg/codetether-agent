//! HTTP client configuration with externally supplied credentials and endpoint.
use reqwest::Client;
/// Default endpoint when no base URL was configured by the secrets layer.
pub const DEFAULT_BASE_URL: &str = "https://api.z.ai/api/paas/v4";
/// Z.AI completion client, independent of the agent's provider registry.
///
/// # Examples
/// ```rust
/// use codetether_provider_zai::ZaiProvider;
/// let provider = ZaiProvider::with_client(reqwest::Client::new(),
///     "test-key".into(), "http://localhost:1234/api/coding/paas/v4".into());
/// assert!(format!("{provider:?}").contains("localhost"));
/// assert!(!format!("{provider:?}").contains("test-key"));
/// ```
pub struct ZaiProvider {
    pub(crate) client: Client,
    pub(crate) api_key: String,
    pub(crate) base_url: String,
}
impl ZaiProvider {
    /// Construct with a pooled client, credential, and configured base URL.
    ///
    /// # Examples
    /// ```rust
    /// use codetether_provider_zai::ZaiProvider;
    /// let provider = ZaiProvider::with_client(reqwest::Client::new(),
    ///     "test".into(), "http://localhost/api/coding/paas/v4".into());
    /// assert!(format!("{provider:?}").contains("coding/"));
    /// ```
    pub fn with_client(client: Client, api_key: String, base_url: String) -> Self {
        Self {
            client,
            api_key,
            base_url: base_url.trim_end_matches('/').into(),
        }
    }
}
impl std::fmt::Debug for ZaiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZaiProvider")
            .field("api_key", &"<REDACTED>")
            .field("base_url", &self.base_url)
            .finish()
    }
}
