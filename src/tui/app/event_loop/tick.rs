#[path = "tick/approval.rs"]
mod approval;
#[path = "dirty.rs"]
mod dirty;
#[path = "tick/retry.rs"]
mod retry;
#[path = "tick_watchdog.rs"]
mod tick_watchdog;
#[path = "tick/worker_sync.rs"]
mod worker_sync;

pub(super) use worker_sync::before_draw;
pub(super) use {retry::check_and_retry, tick_watchdog::check};

codetether_tui_tick::declare_frontend_tick!(
    crate::tui::app::state::App,
    crate::tui::models::ViewMode;
    crate::tui::app::watchdog::question_wait::tick,
    crate::tui::app::event_loop::tick::dirty::capture,
    codetether_autochat_core::interactive::drain::drain_autochat,
    crate::tui::app::event_handlers::drain_voice_transcription,
    crate::tool::swarm_execute::tui_bridge::drain,
    crate::tui::forage_run::drain_forage_updates,
    crate::tui::app::input::shell_bg::drain_shell_events,
    crate::tui::app::state::history_page::drain,
    crate::tui::app::symbols::drain_refresh,
    crate::tui::app::event_loop::tick::approval::reconcile;
    crate::tui::audit_view::refresh_audit_snapshot,
);
