//! Provider new.
use crate::*;
impl GeminiWebProvider {
    /// Create a new provider from Netscape cookies.txt content.
    ///
    /// # Arguments
    /// * `cookies` - Netscape cookies.txt or supported simple cookie lines.
    /// # Returns
    /// A provider with an empty token cache and the existing HTTP defaults.
    /// # Errors
    /// Returns HTTP client construction failures.
    pub fn new(cookies: String) -> Result<Self> {
        let client = codetether_provider_core::socket_opts::apply_socket_opts(Client::builder())
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/131.0.0.0 Safari/537.36",
            )
            .connect_timeout(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .context("Failed to build reqwest client for GeminiWebProvider")?;
        Ok(Self {
            client,
            cookies,
            token_cache: Mutex::new(None),
        })
    }
}
