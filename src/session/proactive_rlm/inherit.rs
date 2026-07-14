//! Reuse prefix-compatible summaries from the previous prepared sidecar.

use crate::session::index::SummaryIndex;

use super::types::Snapshot;
use super::{freshness, store};

pub(super) async fn index(snapshot: &Snapshot) -> SummaryIndex {
    let Some(previous) = store::read_id(&snapshot.session_id).await else {
        return snapshot.index.clone();
    };
    if freshness::matches(
        &snapshot.messages,
        previous.message_count,
        previous.fingerprint,
    ) {
        previous.index
    } else {
        snapshot.index.clone()
    }
}
