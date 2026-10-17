//! Moltbook client upvote.
use super::*;
impl MoltbookClient {
    // ------- voting ---------------------------------------------------------

    /// Upvote a post.
    ///
    /// # Arguments
    /// * `post_id` - Target post identifier.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = client.upvote("post-id").await?;
    /// # Ok(()) }
    /// ```
    pub async fn upvote(&self, post_id: &str) -> Result<String> {
        self.post_json(
            &format!("/posts/{}/upvote", post_id),
            &serde_json::json!({}),
        )
        .await
    }
}
