//! Optional bus + trace_id handle for [`super::produce_summary`].

use uuid::Uuid;

use crate::session::SessionBus;

/// Bus + trace id passed to [`super::produce_summary`].
///
/// Defaults to "no bus, no trace id" so existing call sites don't need
/// to change. The production caller is `derive_incremental`, which
/// constructs one wrapping its parent `event_tx` so the resulting
/// `RlmProgress`/`RlmComplete` events share a single trace id across
/// every summary range produced for one derivation.
#[derive(Default)]
pub struct SummaryObservability {
    pub bus: Option<SessionBus>,
    pub trace_id: Option<Uuid>,
}
