//! In-memory registration of one restored durable child.

use super::super::super::{
    collaboration_runtime::{message_queue, thread_status},
    store::{self, AgentEntry},
};
use super::super::manifest::Manifest;
use anyhow::Result;

/// Register a child restored without a pending residency reservation.
pub(in crate::tool::agent::persistence) async fn resident(
    manifest: Manifest,
    entry: AgentEntry,
) -> Result<()> {
    let child_id = manifest.child_session_id;
    store::insert(entry);
    finish(child_id).await
}

/// Register a child whose pending reservation will be committed by its caller.
pub(in crate::tool::agent::persistence) async fn reserved(
    manifest: Manifest,
    entry: AgentEntry,
) -> Result<()> {
    let child_id = manifest.child_session_id;
    store::insert_reserved(entry);
    finish(child_id).await
}

async fn finish(child_id: String) -> Result<()> {
    thread_status::restored(&child_id);
    let queued = match message_queue::hydrate(&child_id).await {
        Ok(queued) => queued,
        Err(error) => {
            store::remove(&child_id);
            return Err(error);
        }
    };
    if queued {
        message_queue::dispatch_next(child_id);
    }
    Ok(())
}
