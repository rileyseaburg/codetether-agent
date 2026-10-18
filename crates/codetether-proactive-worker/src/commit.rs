//! Commit prepared indexes while respecting session deletion races.

use codetether_context_policy::summary_index::SummaryIndex;

use codetether_prepared_context::{Prepared, fingerprint};
use codetether_proactive_runtime as registry_status;
use codetether_proactive_runtime::Snapshot;

pub(super) async fn write<H: crate::Host>(snapshot: &Snapshot, index: SummaryIndex, host: &H) {
    if !registry_status::exists(&snapshot.session_id) {
        return;
    }
    let prepared = Prepared {
        schema_version: 1,
        message_count: snapshot.messages.len(),
        fingerprint: fingerprint(&snapshot.messages),
        generation: snapshot.generation,
        index,
    };
    if let Err(error) = host.write_prepared(&snapshot.session_id, &prepared).await {
        tracing::warn!(target: "codetether_agent::session::index_produce::proactive::commit", session_id = %snapshot.session_id, %error, "Proactive RLM persistence failed");
    }
    if !registry_status::exists(&snapshot.session_id) {
        host.remove_prepared(&snapshot.session_id).await;
    }
}
