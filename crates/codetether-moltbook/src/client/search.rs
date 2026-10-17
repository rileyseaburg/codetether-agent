//! Moltbook client search.
use super::*;
impl MoltbookClient {
    // ------- search ---------------------------------------------------------

    /// Semantic search across Moltbook.
    ///
    /// # Arguments
    /// * `query` - Search text, URL-encoded before the request.
    /// * `limit` - Requested result count.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = client.search("query", 10).await?;
    /// # Ok(()) }
    /// ```
    pub async fn search(&self, query: &str, limit: usize) -> Result<serde_json::Value> {
        let encoded_query = urlencoding::encode(query);
        let resp = self
            .get(&format!("/search?q={}&limit={}", encoded_query, limit))
            .await?;
        Ok(serde_json::from_str(&resp)?)
    }
}
