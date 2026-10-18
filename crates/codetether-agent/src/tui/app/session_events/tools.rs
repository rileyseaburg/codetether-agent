//! Frontend callbacks for shared tool-event presentation.
#[path = "approval.rs"]
mod approval;
codetether_chat_events::declare_tool_events!(
    crate::tui::app::state::App,
    crate::session::SessionEvent,
    crate::tui::worker_bridge::TuiWorkerBridge;
    crate::tui::app::session_events::tools::approval::request,
    crate::tui::ui::trust_status::approval_guidance,
    crate::tui::app::worker_bridge::handle_processing_started,
);
