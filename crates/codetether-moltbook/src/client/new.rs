//! Moltbook client new.
use super::*;
impl MoltbookClient {
    // ------- construction ---------------------------------------------------

    /// Build a client from a known API key.
    ///
    /// # Arguments
    /// * `api_key` - Credential used by authenticated requests.
    /// # Returns
    /// A client using the fixed Moltbook API base.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// let client = MoltbookClient::new("example".into());
    /// let _ = client;
    /// ```
    pub fn new(api_key: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
        }
    }
}
