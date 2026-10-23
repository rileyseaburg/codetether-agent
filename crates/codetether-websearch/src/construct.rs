//! construct for web search.
use super::*;
/// Search the web through Bing HTML, then DuckDuckGo HTML if Bing fails.
///
/// # Examples
/// ```rust
/// let _tool = codetether_websearch::WebSearchTool::new();
/// ```
pub struct WebSearchTool {
    pub(crate) client: reqwest::Client,
}

impl Default for WebSearchTool {
    fn default() -> Self {
        Self::new()
    }
}
impl WebSearchTool {
    /// Construct the search HTTP client.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_websearch::WebSearchTool::new();
    /// ```
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }
}
