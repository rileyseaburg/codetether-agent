//! Watchdog retry spawn helper.
//!
//! Spawns a new prompt task when the watchdog detects a
//! stall, reusing the original prompt text.
//!
//! # Examples
//!
//! ```ignore
//! spawn_watchdog_retry(&mut app, session, &reg, &tx, &rtx, "prompt");
//! ```

use std::sync::Arc;
use std::time::Instant;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;

/// Spawn a retry task for the watchdog restart.
///
/// Updates app state to reflect the in-flight retry and
/// spawns a Tokio task that re-submits the prompt.
///
/// # Examples
///
/// ```ignore
/// spawn_watchdog_retry(&mut app, session, &reg, &tx, &rtx, "prompt");
/// ```
pub(super) fn spawn_watchdog_retry(
    app: &mut App,
    session: &Session,
    registry: &Option<Arc<ProviderRegistry>>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    prompt: &str,
) {
    let Some(registry) = registry.as_ref() else {
        return;
    };

    app.state.main_inflight_prompt = Some(prompt.to_string());
    app.state.processing = true;
    app.state.begin_request_timing();
    app.state.main_last_event_at = Some(Instant::now());

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
