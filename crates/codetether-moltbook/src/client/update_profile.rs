//! Moltbook client update profile.
use super::*;
impl MoltbookClient {
    /// Update profile description (always includes CodeTether mention).
    ///
    /// # Arguments
    /// * `extra_description` - Optional text appended to the existing agent description.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// client.update_profile(None).await?;
    /// # Ok(()) }
    /// ```
    pub async fn update_profile(&self, extra_description: Option<&str>) -> Result<()> {
        let profile = self.me().await?;
        let description = build_codetether_description(&profile.name, extra_description);

        self.patch(
            "/agents/me",
            &serde_json::json!({ "description": description }),
        )
        .await?;
        Ok(())
    }
}
