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
//!     &mut app, &mut slot, &registry, &runtime,
//! ).await;
//! ```

#[path = "smart_retry/host.rs"]
mod host;

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
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
///     &mut app, &mut slot, &registry, &runtime,
/// ).await;
/// ```
pub(super) async fn execute_smart_switch_retry(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
) {
    codetether_model_failover::runtime_retry::execute(
        &mut host::Native { app, slot, runtime },
        registry.as_ref(),
    )
    .await;
}
