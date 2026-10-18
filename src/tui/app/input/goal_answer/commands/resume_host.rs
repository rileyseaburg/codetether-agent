//! Native ownership checks and freshly persisted goal continuation.
use crate::provider::ProviderRegistry;
use crate::tui::{
    app::{
        session_runtime::{SessionSlot, TuiSessionHandle},
        state::App,
    },
    worker_bridge::TuiWorkerBridge,
};
use std::{path::Path, sync::Arc};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
    pub bridge: &'a Option<TuiWorkerBridge>,
    pub runtime: &'a TuiSessionHandle,
}
impl codetether_goal_command::input::Resume for Native<'_> {
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    fn available(&self) -> bool {
        self.slot.borrow().is_some()
    }
    fn providers(&self) -> bool {
        self.registry.is_some()
    }
    fn session_id(&self) -> String {
        self.slot.view().id().to_string()
    }
    async fn resume(&mut self) {
        crate::tui::app::input::sessions::goal_autostart::resume(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.bridge,
            self.runtime,
        )
        .await;
    }
}
