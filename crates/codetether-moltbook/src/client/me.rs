//! Moltbook client me.
use super::*;
impl MoltbookClient {
    // ------- profile --------------------------------------------------------

    /// Get own profile.
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
    /// let _ = client.me().await?;
    /// # Ok(()) }
    /// ```
    pub async fn me(&self) -> Result<AgentProfile> {
        let resp = self.get("/agents/me").await?;
        let wrapper: ApiResponse<serde_json::Value> = serde_json::from_str(&resp)?;
        if let Some(agent) = wrapper.data.get("agent") {
            Ok(serde_json::from_value(agent.clone())?)
        } else {
            // Try parsing the whole data as the profile
            Ok(serde_json::from_str(&resp)?)
        }
    }
}
