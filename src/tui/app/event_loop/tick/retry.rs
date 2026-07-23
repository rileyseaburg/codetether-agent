//! Per-tick execution of delayed smart-switch retries.

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use std::sync::Arc;

pub(crate) async fn check_and_retry(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
    interval: std::time::Duration,
) {
    super::check(app, runtime, interval).await;
    super::super::smart_retry::execute_smart_switch_retry(app, slot, registry, runtime).await;
}
