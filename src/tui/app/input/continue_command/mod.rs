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

mod continue_idle;
mod continue_prompt;
mod continue_stalled;

/// Handle `/continue`. Returns `true` when `prompt` was the `/continue`
/// command (and was handled); `false` otherwise so the caller keeps dispatching.
pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    prompt: &str,
) -> bool {
    if prompt.trim() != "/continue" {
        return false;
    }
    let prompt = continue_prompt::resolve(app, slot.borrow());
    app.state.clear_input();
    if app.state.processing || slot.borrow().is_none() {
        continue_stalled::resume(app, prompt, runtime).await;
    } else {
        continue_idle::resume(app, cwd, slot, registry, worker_bridge, prompt, runtime).await;
    }
    true
}
