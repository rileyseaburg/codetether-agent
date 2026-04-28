//! Key event dispatcher implementation.

use std::{path::Path, sync::Arc};

use crossterm::event::{KeyEvent, KeyEventKind};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};

use super::{handle_ctrl_key, handle_unmodified_key};

pub async fn handle_event(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    if key.kind != KeyEventKind::Press {
        return Ok(false);
    }
    if let Some(result) = handle_ctrl_key(app, cwd, key) {
        return result;
    }
    let out = handle_unmodified_key(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        event_tx,
        result_tx,
        key,
    )
    .await;
    app.state.last_key_at = Some(std::time::Instant::now());
    out
}
