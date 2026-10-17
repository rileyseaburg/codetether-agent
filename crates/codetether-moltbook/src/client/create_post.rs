//! Moltbook client create post.
use super::*;
impl MoltbookClient {
    // ------- posts ----------------------------------------------------------

    /// Create a post in a submolt.
    ///
    /// # Arguments
    /// * `submolt` - Destination community.
    /// * `title` - Post title.
    /// * `content` - Post body.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = client.create_post("community", "title", "content").await?;
    /// # Ok(()) }
    /// ```
    pub async fn create_post(&self, submolt: &str, title: &str, content: &str) -> Result<String> {
        let resp = self
            .post_json(
                "/posts",
                &serde_json::json!({
                    "submolt": submolt,
                    "title": title,
                    "content": content,
                }),
            )
            .await?;
        Ok(resp)
    }
}
