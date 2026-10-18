//! Register restored live entries and hydrate their durable mailboxes.
use crate::{Host, Manifest};
use anyhow::Result;
use codetether_agent_registry::AgentEntry;
use codetether_collaboration_state::thread_status;
/// Register a child restored without a pending residency reservation.
pub(crate) async fn resident<H: Host>(
    manifest: Manifest,
    entry: AgentEntry<H::Session>,
) -> Result<()> {
    let child_id = manifest.child_session_id;
    H::insert(entry);
    finish::<H>(child_id).await
}

/// Register a child whose pending reservation will be committed by its caller.
pub(crate) async fn reserved<H: Host>(
    manifest: Manifest,
    entry: AgentEntry<H::Session>,
) -> Result<()> {
    let child_id = manifest.child_session_id;
    H::insert_reserved(entry);
    finish::<H>(child_id).await
}

async fn finish<H: Host>(child_id: String) -> Result<()> {
    thread_status::restored(&child_id);
    let queued = match H::hydrate_mailbox(&child_id).await {
        Ok(queued) => queued,
        Err(error) => {
            H::remove(&child_id);
            return Err(error);
        }
    };
    if queued {
        H::dispatch_next(child_id);
    }
    Ok(())
}
