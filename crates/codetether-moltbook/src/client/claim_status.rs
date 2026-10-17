//! Moltbook client claim status.
use super::*;
impl MoltbookClient {
    /// Check claim status.
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
    /// let _ = client.claim_status().await?;
    /// # Ok(()) }
    /// ```
    pub async fn claim_status(&self) -> Result<ClaimStatus> {
        let resp = self.get("/agents/status").await?;
        Ok(serde_json::from_str(&resp)?)
    }
}
