//! Top-level TUI view dispatcher.
//!
//! Routes the current `ViewMode` to the appropriate renderer.
//! All per-view rendering lives in dedicated sub-modules
//! (see `chat_view`, `sessions`, `inspector`, `webview`, etc.)
//! to satisfy SRP and the 50-line file limit for new modules.

use ratatui::Frame;

use crate::tui::app::state::App;
use crate::tui::audit_view::render_audit_view;
use crate::tui::bus_log::{ProtocolSummary, render_bus_log_with_summary};
use crate::tui::latency::render_latency;
use crate::tui::lsp::render_lsp;
use crate::tui::models::ViewMode;
use crate::tui::ralph_view::render_ralph_view;
use crate::tui::rlm::render_rlm;
use crate::tui::settings::render_settings;
use crate::tui::swarm_view::render_swarm_view;
use crate::tui::symbol_search::render_symbol_search;

use super::chat_view::render_chat_view;
use super::sessions::render_sessions_view;

/// Top-level entry point called by the TUI event loop on every frame.
///
/// Dispatches to the active [`ViewMode`] renderer and renders overlays.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::main::ui;
/// # fn demo(f: &mut ratatui::Frame, app: &mut codetether_agent::tui::app::state::App, sess: &codetether_agent::session::Session) {
/// ui(f, app, sess);
/// # }
/// ```
pub fn ui(f: &mut Frame, app: &mut App, session: &crate::session::Session) {
    dispatch_view(f, app, session);
    render_overlays(f, app);
}

fn dispatch_view(f: &mut Frame, app: &mut App, session: &crate::session::Session) {
    match app.state.view_mode {
        ViewMode::Chat => render_chat_or_webview(f, app, session),
        ViewMode::Sessions => render_sessions_view(f, app),
        ViewMode::Swarm => render_swarm_view(f, &mut app.state.swarm, f.area()),
        ViewMode::Ralph => render_ralph_view(f, &mut app.state.ralph, f.area()),
        ViewMode::Bus => render_bus_view(f, app),
        ViewMode::Model => crate::tui::model_picker::render_model_picker(f, f.area(), app, session),
        ViewMode::Settings => render_settings(f, f.area(), &app.state),
        ViewMode::Lsp => render_lsp(f, f.area(), &app.state.cwd_display, &app.state.status),
        ViewMode::Rlm => render_rlm(
            f,
            f.area(),
            &app.state.cwd_display,
            &app.state.status,
            app.state.sessions.len(),
            app.state.selected_session,
        ),
        ViewMode::Latency => render_latency(f, f.area(), app),
        ViewMode::Protocol => {
            crate::tui::protocol_registry_view::render_protocol_registry(f, app, f.area())
        }
        ViewMode::FilePicker => crate::tui::app::file_picker::render_file_picker(f, f.area(), app),
        ViewMode::Inspector => super::inspector::render_inspector_view(f, app),
        ViewMode::Audit => render_audit_view(f, &mut app.state.audit, f.area()),
        ViewMode::Git => {
            crate::tui::git_view::render_git_view(f, f.area(), &app.state.git, &app.state.status)
        }
    }
}

fn render_chat_or_webview(f: &mut Frame, app: &mut App, session: &crate::session::Session) {
    if super::webview::layout::is_webview(app.state.chat_layout_mode) {
        super::webview::render(f, app);
    } else {
        render_chat_view(f, app, session);
    }
}

fn render_bus_view(f: &mut Frame, app: &mut App) {
    let mut registered_agents = app
        .state
        .worker_bridge_registered_agents
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    registered_agents.sort();
    let summary = ProtocolSummary {
        cwd_display: app.state.cwd_display.clone(),
        worker_id: app.state.worker_id.clone(),
        worker_name: app.state.worker_name.clone(),
        a2a_connected: app.state.a2a_connected,
        processing: app.state.worker_bridge_processing_state,
        registered_agents,
        queued_tasks: app.state.worker_task_queue.len(),
        recent_task: app.state.recent_tasks.last().cloned(),
        peer_endpoint_ready: app.state.peer_endpoint_ready,
    };
    render_bus_log_with_summary(f, &mut app.state.bus_log, f.area(), Some(summary))
}

fn render_overlays(f: &mut Frame, app: &mut App) {
    if app.state.symbol_search.loading
        || !app.state.symbol_search.query.is_empty()
        || !app.state.symbol_search.results.is_empty()
        || app.state.symbol_search.error.is_some()
    {
        render_symbol_search(f, &mut app.state.symbol_search, f.area());
    }

    if app.state.watchdog_notification.is_some() {
        crate::tui::app::watchdog::render_watchdog_notification(f, f.area(), &app.state);
    }
}
