//! Helper for the Ctrl+m model-picker keybind.

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

/// Open the model picker if a session is currently active.
pub(super) async fn open_model_picker_if_session(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
) {
    if let Some(session) = slot.borrow() {
        crate::tui::app::model_picker::open_model_picker(app, session, registry.as_ref()).await;
    }
}
