//! Bounded interactive resume helpers, distinct from explicit full exports.
use crate::session::Session;
use anyhow::Result;
impl Session {
    /// Resume the latest workspace session without materializing its full history.
    ///
    /// # Arguments
    /// * `workspace` - Optional workspace filter; `None` uses the configured store.
    /// # Returns
    /// A bounded working tail with the original durable identity.
    /// # Errors
    /// Returns lookup, import, or database errors; absence does not erase history.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let session = codetether_agent::session::Session::resume_last(None).await?;
    /// assert!(session.messages.len() <= codetether_agent::session::store::WINDOW);
    /// # Ok(()) }
    /// ```
    pub async fn resume_last(workspace: Option<&std::path::Path>) -> Result<Self> {
        Ok(Self::last_for_directory_tail(workspace, super::WINDOW)
            .await?
            .session)
    }
}
