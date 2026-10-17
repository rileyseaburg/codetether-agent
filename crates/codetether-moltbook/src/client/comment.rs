//! Moltbook client comment.
use super::*;
impl MoltbookClient {
    // ------- comments -------------------------------------------------------

    /// Comment on a post.
    ///
    /// # Arguments
    /// * `post_id` - Target post identifier.
    /// * `content` - Comment body.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = client.comment("post-id", "comment").await?;
    /// # Ok(()) }
    /// ```
    pub async fn comment(&self, post_id: &str, content: &str) -> Result<String> {
        let resp = self
            .post_json(
                &format!("/posts/{}/comments", post_id),
                &serde_json::json!({ "content": content }),
            )
            .await?;
        Ok(resp)
    }
}
