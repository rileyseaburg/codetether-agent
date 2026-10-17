//! Moltbook client feed.
use super::*;
impl MoltbookClient {
    /// Get the hot feed.
    ///
    /// # Arguments
    /// * `sort` - API sort selector.
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
    /// let _ = client.feed("hot", 10).await?;
    /// # Ok(()) }
    /// ```
    pub async fn feed(&self, sort: &str, limit: usize) -> Result<Vec<Post>> {
        let resp = self
            .get(&format!("/posts?sort={}&limit={}", sort, limit))
            .await?;
        // Try structured deserialization first
        if let Ok(feed) = serde_json::from_str::<FeedResponse>(&resp)
            && feed.success
        {
            return Ok(feed.posts);
        }
        // Fallback: The API may return posts under different keys
        let val: serde_json::Value = serde_json::from_str(&resp)?;
        if let Some(posts) = val.get("posts") {
            Ok(serde_json::from_value(posts.clone()).unwrap_or_default())
        } else if let Some(data) = val.get("data") {
            Ok(serde_json::from_value(data.clone()).unwrap_or_default())
        } else {
            Ok(Vec::new())
        }
    }
}
