//! Frontend selection of shared reactor dispatch callbacks.
#[path = "dirty.rs"]
mod dirty;
#[path = "queued_prompt.rs"]
mod queued_prompt;
codetether_tui_reactor::declare_frontend_selection!(
    crate::tui::app::event_loop::SelectArgs,
    codetether_tui_tick::Snapshot,
    std::io::Result<crossterm::event::Event>,
    crate::session::SessionEvent,
    crate::tui::app::session_runtime::SessionNotice;
    crate::tui::app::event_loop::select_loop::dirty::capture;
    crate::tui::app::event_loop::terminal::handle_terminal_event,
    crate::tui::app::session_events::handle_session_event,
    crate::tui::app::session_event_drain::drain_batch;
    crate::tui::app::background::drain_background_updates,
    crate::tui::app::event_loop::select_loop::queued_prompt::drain,
    crate::tui::app::event_loop::bus_inbox::trigger_next;
    crate::tui::app::event_loop::watchdog::maybe_watchdog_restart,
    crate::tui::app::event_loop::tick::refresh_audit,
    crate::tui::app::event_loop::tick::run,
    crate::tui::app::event_loop::tick::check_and_retry;
    crate::tui::app::background::apply_single_notice,
    crate::tui::app::background::stream_reconnect_execute::execute,
    crate::tui::app::event_loop::smart_retry::execute_smart_switch_retry,
    crate::tui::app::event_loop::watchdog_retry::execute,
);
