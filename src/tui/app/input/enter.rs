//! Enter-key dispatch across all TUI view modes.
//!
//! Examines the active view mode and routes the Enter
//! press to the appropriate handler — sessions, chat,
//! bus filter, model, settings, etc.
//!
//! # Examples
//!
//! ```ignore
//! dispatch_enter(&mut app, cwd, &mut session, &reg,
//!     &bridge, &tx, &rtx).await;
//! ```

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::{models::ViewMode, worker_bridge::TuiWorkerBridge};

#[path = "enter_swarm.rs"]
mod enter_swarm;
#[path = "enter_session.rs"]
mod mode;

/// Dispatch Enter to the handler matching the active view.
pub(crate) async fn dispatch_enter(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    match app.state.view_mode {
        ViewMode::Sessions => {
            mode::sessions(app, cwd, slot, registry, worker_bridge, runtime).await
        }
        ViewMode::FilePicker => crate::tui::app::file_picker::file_picker_enter(app, cwd),
        ViewMode::Swarm => {
            enter_swarm::dispatch_swarm_enter(app, cwd, slot, registry, worker_bridge, runtime)
                .await
        }
        ViewMode::Ralph => app.state.ralph.enter_detail(),
        ViewMode::Bus if app.state.bus_log.filter_input_mode => {
            super::bus::handle_enter_bus_filter(app)
        }
        ViewMode::Bus => app.state.bus_log.enter_detail(),
        ViewMode::Chat => {
            super::chat_submit::handle_enter_chat(app, cwd, slot, registry, worker_bridge, runtime)
                .await
        }
        ViewMode::Model => super::model_apply::selected(app, slot),
        ViewMode::Settings => mode::settings(app, slot).await,
        ViewMode::Subagents => super::enter_subagents::dispatch(app),
        ViewMode::Lsp
        | ViewMode::Rlm
        | ViewMode::Latency
        | ViewMode::Transport
        | ViewMode::Protocol
        | ViewMode::Inspector
        | ViewMode::Audit
        | ViewMode::Git
        | ViewMode::AuditLoop
        | ViewMode::Editor => {}
    }
}
