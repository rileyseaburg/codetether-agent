//! Transactional activation with manifest rollback.
use crate::{ClosedAgent, Host, Lifecycle, manifest_io, restore};
use anyhow::Result;
use codetether_agent_registry::AgentEntry;
/// Persist an open lifecycle and register a prepared child.
///
/// # Arguments
/// `closed` is the identity returned by `prepare_resume`, with any session updates.
/// # Returns
/// Returns the registered live entry after mailbox hydration.
/// # Errors
/// Persistence, registration, mailbox hydration, or rollback persistence fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::{Host, prepare_resume, activate_resume};
/// # async fn example<H: Host>() -> anyhow::Result<()> {
/// let prepared = prepare_resume::<H>(Some("parent"), "reviewer").await?;
/// let entry = activate_resume::<H>(prepared).await?;
/// # Ok(()) }
/// ```
pub async fn activate_resume<H: Host>(
    closed: ClosedAgent<H::Session>,
) -> Result<AgentEntry<H::Session>> {
    manifest_io::save_lifecycle(&closed.name, &closed.entry, Lifecycle::Open).await?;
    if let Err(error) =
        restore::register_reserved::<H>(closed.manifest.clone(), closed.entry.clone()).await
    {
        manifest_io::save_lifecycle(&closed.name, &closed.entry, Lifecycle::Closed).await?;
        return Err(error);
    }
    Ok(closed.entry)
}
