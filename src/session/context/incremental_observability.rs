//! Build a per-derivation bus + trace id for incremental summary production.

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::session::SessionEvent;
use crate::session::index_produce::SummaryObservability;
use crate::session::{SessionBus};

const BUS_CAPACITY: usize = 16;

/// One trace id and one [`SessionBus`] cover every summary range
/// produced for a single `derive_incremental` call, so TUI / audit
/// consumers can correlate them. Built once outside the per-range
/// loop so we don't churn forwarder tasks.
pub(super) struct DerivationObservability {
    bus: Option<SessionBus>,
    trace_id: Uuid,
}

impl DerivationObservability {
    pub(super) fn new(event_tx: Option<&mpsc::Sender<SessionEvent>>) -> Self {
        let bus = event_tx
            .cloned()
            .map(|tx| SessionBus::new(BUS_CAPACITY).with_legacy_mpsc(tx));
        Self { bus, trace_id: Uuid::new_v4() }
    }

    /// Fresh [`SummaryObservability`] for one call site.
    pub(super) fn template(&self) -> SummaryObservability {
        SummaryObservability { bus: self.bus.clone(), trace_id: Some(self.trace_id) }
    }
}
