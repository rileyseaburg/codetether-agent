//! Canonical ensure-open operation shared by collaboration tools.

use super::{EnsureOpen, ResumeConfig, capacity};

#[path = "ensure/refresh.rs"]
mod refresh;
#[path = "ensure/resident.rs"]
mod resident;

pub(in crate::tool) async fn open(
    target: &str,
    owner: Option<&str>,
    config: ResumeConfig,
) -> anyhow::Result<EnsureOpen> {
    if let Some(outcome) = resident::find(target, owner, &config).await? {
        return Ok(outcome);
    }
    let Some(agent_id) = super::super::persistence::durable_id(owner, target).await? else {
        return Ok(EnsureOpen::Missing);
    };
    let _load = super::singleflight::acquire(&agent_id).await;
    if let Some(outcome) = resident::find(&agent_id, owner, &config).await? {
        return Ok(outcome);
    }
    let slot = match capacity::reserve(Some(&agent_id)).await {
        Ok(slot) => slot,
        Err(result) => return Ok(EnsureOpen::Rejected(result)),
    };
    let entry = super::activate::closed(&agent_id, owner, &config).await?;
    slot.commit(&entry.session.id);
    Ok(EnsureOpen::Ready {
        agent_id: entry.session.id,
        name: entry.name,
        resumed: true,
    })
}
