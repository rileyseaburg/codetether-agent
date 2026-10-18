//! Retained legacy full-window proactive summary processing.
use crate::{commit, inherit, summarize};
use codetether_proactive_planning as ranges;
use codetether_proactive_runtime as registry_status;
use codetether_proactive_runtime::{self as capacity, Snapshot};

pub(super) async fn process<H: crate::Host>(mut snapshot: Snapshot, host: &H) {
    let Ok(messages) = host.load_tail(&snapshot.session_id).await else {
        return;
    };
    snapshot.messages = messages;
    let Some(_permit) = capacity::acquire().await else {
        return;
    };
    let mut index = inherit::index(&snapshot, host).await;
    for plan in ranges::planned(snapshot.messages.len()) {
        if !registry_status::is_current(&snapshot.session_id, snapshot.generation) {
            break;
        }
        if index.get(plan.range).is_some() {
            continue;
        }
        match summarize::range(&snapshot, plan).await {
            Ok(node) => {
                index.insert(plan.range, node);
            }
            Err(error) => {
                tracing::warn!(target: "codetether_agent::session::index_produce::proactive::worker", session_id = %snapshot.session_id, %error, "Proactive RLM summary failed");
                return;
            }
        }
    }
    commit::write(&snapshot, index, host).await;
}
