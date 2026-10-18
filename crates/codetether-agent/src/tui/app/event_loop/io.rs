//! Frontend payload specialization for borrowed reactor channels.
pub(crate) type LoopIo<'a> = codetether_tui_reactor::LoopIo<
    'a,
    crate::session::SessionEvent,
    crate::tui::app::session_runtime::SessionNotice,
>;
