//! RLM observability for compaction (issue #231 item 3).
//!
//! Holds the optional bus + trace id that the compression pass
//! threads into the RLM router so its progress/completion events ride
//! the same trace as the surrounding `CompactionStarted`/`Completed`
//! pair.

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::session::{SessionBus, SessionEvent};

use super::compression::CompressContext;

const BUS_CAPACITY: usize = 16;

/// Bus + trace id forwarded into [`crate::rlm::router::AutoProcessContext`].
#[derive(Clone, Default)]
pub(crate) struct Observability {
    pub bus: Option<SessionBus>,
    pub trace_id: Option<Uuid>,
}

/// Clone `base` and attach a fresh [`SessionBus`] bridging `event_tx`,
/// stamped with the supplied `trace_id`. When `event_tx` is `None` the
/// bus stays empty and only the trace id is carried.
pub(super) fn observability_ctx(
    base: &CompressContext,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    trace_id: Uuid,
) -> CompressContext {
    let bus = event_tx
        .cloned()
        .map(|tx| SessionBus::new(BUS_CAPACITY).with_legacy_mpsc(tx));
    let mut out = base.clone();
    out.observability = Observability {
        bus,
        trace_id: Some(trace_id),
    };
    out
}
