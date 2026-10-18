//! Main TUI event loop orchestration.
//!
//! Drives the terminal draw → event → dispatch cycle using
//! `tokio::select!` across terminal, session, result, watchdog
//! and tick channels.

mod bus_inbox;
mod driver;
mod io;
mod redraw;
mod select_args;
mod select_loop;
mod setup;
mod shutdown;
mod smart_retry;
mod terminal;
mod tick;
mod watchdog;
mod watchdog_retry;

#[cfg(not(test))]
pub(crate) use {io::LoopIo, select_args::SelectArgs};

codetether_tui_reactor::declare_frontend_loop_entry!(
    ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    crate::tui::app::state::App,
    crate::provider::ProviderRegistry,
    crate::tui::app::session_runtime::SessionSlot,
    crate::bus::recorder::BusRecorder,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::session::SessionEvent,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::app::session_runtime::SessionNotice;
    crate::tui::app::event_loop::setup::create,
    crate::tui::app::event_loop::driver::Native,
);

#[cfg(test)]
include!("mod_test_imports.rs");
