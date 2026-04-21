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
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::{settings::toggle_selected_setting, state::App};
use crate::tui::{models::ViewMode, worker_bridge::TuiWorkerBridge};

/// Dispatch Enter to the handler matching the active view.
///
/// # Examples
///
/// ```ignore
/// dispatch_enter(&mut app, cwd, &mut session, &reg,
///     &bridge, &tx, &rtx).await;
/// ```
pub async fn dispatch_enter(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    match app.state.view_mode {
        ViewMode::Sessions => super::sessions::handle_enter_sessions(app, cwd, session).await,
        ViewMode::FilePicker => crate::tui::app::file_picker::file_picker_enter(app, cwd),
        ViewMode::Swarm => app.state.swarm.enter_detail(),
        ViewMode::Ralph => app.state.ralph.enter_detail(),
        ViewMode::Bus if app.state.bus_log.filter_input_mode => {
            super::bus::handle_enter_bus_filter(app)
        }
        ViewMode::Bus => app.state.bus_log.enter_detail(),
        ViewMode::Chat => {
            super::chat_submit::handle_enter_chat(
                app,
                cwd,
                session,
                registry,
                worker_bridge,
                event_tx,
                result_tx,
            )
            .await
        }
        ViewMode::Model => crate::tui::app::model_picker::apply_selected_model(app, session),
        ViewMode::Settings => toggle_selected_setting(app, session).await,
        ViewMode::Lsp
        | ViewMode::Rlm
        | ViewMode::Latency
        | ViewMode::Protocol
        | ViewMode::Inspector
        | ViewMode::Audit => {}
    }
}
