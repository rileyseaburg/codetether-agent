//! Moltbook client heartbeat.
use super::*;
impl MoltbookClient {
    // ------- heartbeat ------------------------------------------------------

    /// Run a heartbeat: check feed, optionally engage.
    ///
    /// Returns a summary of what was seen.
    ///
    /// # Arguments
    /// Uses the client credential and the documented endpoint.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = client.heartbeat().await?;
    /// # Ok(()) }
    /// ```
    pub async fn heartbeat(&self) -> Result<String> {
        let posts = self.feed("hot", 10).await?;

        let mut summary = format!("Moltbook heartbeat — {} hot posts\n", posts.len());
        for (i, p) in posts.iter().enumerate().take(5) {
            let title = p.title.as_deref().unwrap_or("(untitled)");
            let author = p
                .author
                .as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("unknown");
            let votes = p.upvotes.unwrap_or(0) - p.downvotes.unwrap_or(0);
            summary.push_str(&format!(
                "  {}. [{}] {} by {} ({} votes)\n",
                i + 1,
                &p.id[..8.min(p.id.len())],
                title,
                author,
                votes,
            ));
        }

        // Engage with top post if available
        if let Some(top_post) = posts.first()
            && let Ok(resp) = self.upvote(&top_post.id).await
        {
            summary.push_str(&format!("  Upvoted top post: {}\n", resp));
        }

        Ok(summary)
    }
}
