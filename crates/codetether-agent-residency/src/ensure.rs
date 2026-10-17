//! Canonical ensure-open operation shared by collaboration tools.

use crate::{EnsureOpen, Host, ResumeConfig, capacity};
use codetether_collaboration_state::residency as state;

#[path = "ensure/refresh.rs"]
mod refresh;
#[path = "ensure/resident.rs"]
mod resident;

/// Find a resident child or reload its durable identity under one activation lock.
/// # Arguments
/// * `target` — Scoped path, ID, or display name.
/// * `owner` — Optional owning session ID.
/// * `config` — Current parent settings for activation or stale refresh.
/// # Returns
/// Readiness, missing-target, or capacity-rejection outcome.
/// # Errors
/// Propagates durable lookup, overlay, and activation errors.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_residency::{open, Host};
/// async fn example<H: Host>() -> anyhow::Result<()> {
///     let _ = open::<H>("child", Some("owner"), Default::default()).await?;
///     Ok(())
/// }
/// ```
pub async fn open<H: Host>(
    target: &str,
    owner: Option<&str>,
    config: ResumeConfig,
) -> anyhow::Result<EnsureOpen> {
    if let Some(outcome) = resident::find::<H>(target, owner, &config).await? {
        return Ok(outcome);
    }
    let Some(agent_id) = H::durable_id(owner, target).await? else {
        return Ok(EnsureOpen::Missing);
    };
    let _load = state::acquire(&agent_id).await;
    if let Some(outcome) = resident::find::<H>(&agent_id, owner, &config).await? {
        return Ok(outcome);
    }
    let slot = match capacity::reserve::<H>(Some(&agent_id)).await {
        Ok(slot) => slot,
        Err(result) => return Ok(EnsureOpen::Rejected(result)),
    };
    let entry = super::activate::closed::<H>(&agent_id, owner, &config).await?;
    let agent_id = entry.id().to_owned();
    slot.commit(&agent_id);
    Ok(EnsureOpen::Ready {
        agent_id,
        name: entry.name,
        resumed: true,
    })
}
