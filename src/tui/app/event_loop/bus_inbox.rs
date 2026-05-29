use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn trigger_next(
    app: &mut App,
    cwd: &std::path::Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    crate::tui::app::inbox::trigger_next(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        event_tx,
        result_tx,
    )
    .await;
}
