//! construct for web content fetching.
use super::*;
/// Fetch web content with bounded output and repository-scoped GitHub auth.
///
/// # Examples
/// ```rust
/// let _tool = codetether_webfetch::WebFetchTool::new();
/// ```

pub struct WebFetchTool {
    pub(crate) client: reqwest::Client,
}

impl Default for WebFetchTool {
    fn default() -> Self {
        Self::new()
    }
}
impl WebFetchTool {
    /// Construct an HTTP client using the standard timeout and redirect policy.
    /// # Panics
    /// Panics if the HTTP client cannot be built.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_webfetch::WebFetchTool::new();
    /// ```

    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }
}
