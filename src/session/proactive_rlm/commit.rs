//! Commit prepared indexes while respecting session deletion races.

use crate::session::index::SummaryIndex;

use super::types::Snapshot;
use super::{fingerprint, registry_status, store};

pub(super) async fn write(snapshot: &Snapshot, index: SummaryIndex) {
    if !registry_status::exists(&snapshot.session_id) {
        return;
    }
    let prepared = store::Prepared {
        schema_version: 1,
        message_count: snapshot.messages.len(),
        fingerprint: fingerprint::messages(&snapshot.messages),
        generation: snapshot.generation,
        index,
    };
    if let Err(error) = store::write(&snapshot.session_id, &prepared).await {
        tracing::warn!(session_id = %snapshot.session_id, %error, "Proactive RLM persistence failed");
    }
    if !registry_status::exists(&snapshot.session_id) {
        store::remove(&snapshot.session_id).await;
    }
}
