//! Coalescing proactive-RLM worker loop.

use super::types::Snapshot;
use super::{capacity, commit, inherit, ranges, registry, registry_status, summarize};

pub(super) fn spawn(session_id: String) {
    tokio::spawn(async move {
        loop {
            if let Some(snapshot) = registry::take(&session_id) {
                process(snapshot).await;
                continue;
            }
            if registry::settle(&session_id) {
                break;
            }
        }
    });
}

async fn process(snapshot: Snapshot) {
    let Some(_permit) = capacity::acquire().await else {
        return;
    };
    let mut index = inherit::index(&snapshot).await;
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
                tracing::warn!(session_id = %snapshot.session_id, %error, "Proactive RLM summary failed");
                return;
            }
        }
    }
    commit::write(&snapshot, index).await;
}
