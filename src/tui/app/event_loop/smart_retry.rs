//! Smart-switch retry execution after a provider failure.
//!
//! When a prompt fails on one model the TUI can schedule a
//! retry on a fallback model.  This module executes that
//! pending retry after the failed result is applied.
//!
//! # Examples
//!
//! ```ignore
//! execute_smart_switch_retry(
//!     &mut app, session, &registry, &event_tx, &result_tx,
//! ).await;
//! ```

use std::sync::Arc;
use std::time::Instant;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::smart_switch::should_execute_smart_switch;
use crate::tui::app::state::App;

/// Execute a pending smart-switch retry if scheduled.
///
/// Checks whether a retry was queued by the result handler
/// and, if valid, switches the model and spawns a new
/// prompt task.
///
/// # Examples
///
/// ```ignore
/// execute_smart_switch_retry(
///     &mut app, session, &registry, &tx, &rtx,
/// ).await;
/// ```
pub(super) async fn execute_smart_switch_retry(
    app: &mut App,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    let Some(pending) = app.state.pending_smart_switch_retry.take() else {
        return;
    };
    if !should_execute_smart_switch(session.metadata.model.as_deref(), Some(&pending)) {
        return;
    }

    session.metadata.model = Some(pending.target_model.clone());
    let _ = session.save().await;

    app.state.processing = true;
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(pending.prompt.clone());
    app.state.main_last_event_at = Some(Instant::now());
    app.state.status = format!("Retrying with {}…", pending.target_model);

    if let Some(registry) = registry.as_ref() {
        spawn_retry(session, &pending.prompt, event_tx, result_tx, registry);
    }
}

/// Spawn the retry task.
fn spawn_retry(
    session: &Session,
    prompt: &str,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    registry: &Arc<ProviderRegistry>,
) {
    let mut s = session.clone();
    let tx = event_tx.clone();
    let rtx = result_tx.clone();
    let reg = Arc::clone(registry);
    let p = prompt.to_string();
    tokio::spawn(async move {
        let r = s.prompt_with_events(&p, tx, reg).await.map(|_| s);
        let _ = rtx.send(r).await;
    });
}
