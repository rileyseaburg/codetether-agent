//! Prepared durable child identity and live entry.
use crate::Manifest;
use codetether_agent_registry::AgentEntry;
/// Prepared child whose private manifest is retained for transactional activation.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::{Host, prepare_resume};
/// # async fn example<H: Host>() -> anyhow::Result<()> {
/// let child = prepare_resume::<H>(Some("parent"), "reviewer").await?;
/// assert_eq!(child.name, "reviewer");
/// # Ok(()) }
/// ```
pub struct ClosedAgent<S> {
    /// Display name from the durable manifest.
    pub name: String,
    /// Live session and child metadata, editable before activation.
    pub entry: AgentEntry<S>,
    pub(crate) manifest: Manifest,
}
