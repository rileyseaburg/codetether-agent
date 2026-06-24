//! `/continue` command — manual resume for stalled or idle turns.
//!
//! Sometimes a provider socket silently dies: the TUI shows `processing` but no
//! events ever arrive, and the user does not want to wait for the watchdog
//! timeout. `/continue` is a manual trigger for the same recovery path.
//!
//! Two cases:
//! * **Stalled** — a turn is in flight (session checked out of the slot). We
//!   cancel the dead request and arm the watchdog-retry machinery, which
//!   resubmits the in-flight prompt once the runtime returns the session.
//! * **Idle** — no turn in flight. We resubmit the resolved prompt directly.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::continue_prompt;

/// Handle `/continue`. Returns `true` when the command was recognized.
pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) -> bool {
    let prompt = continue_prompt::resolve(app, slot.borrow());
    app.state.clear_input();
    if app.state.processing || slot.borrow().is_none() {
        super::continue_stalled::resume(app, prompt, runtime).await;
    } else {
        super::continue_idle::resume(app, cwd, slot, registry, worker_bridge, prompt, runtime)
            .await;
    }
    true
}
