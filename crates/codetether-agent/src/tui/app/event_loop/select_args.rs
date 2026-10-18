//! Frontend resource specialization for one reactor selection cycle.
pub(crate) type SelectArgs<'a> = codetether_tui_reactor::Selection<
    'a,
    crossterm::event::EventStream,
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::SessionEvent,
    crate::tui::app::session_runtime::SessionNotice,
    crate::bus::recorder::BusRecorder,
>;
