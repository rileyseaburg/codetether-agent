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
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "enter_swarm.rs"]
mod enter_swarm;
#[path = "enter_session.rs"]
mod mode;

#[path = "enter/host/mod.rs"]
mod host;

/// Dispatch Enter to the handler matching the active view.
pub(crate) async fn dispatch_enter(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    codetether_tui_navigation::enter::dispatch(&mut host::Native {
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        runtime,
    })
    .await;
}
