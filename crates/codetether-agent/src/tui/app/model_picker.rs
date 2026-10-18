//! Native model-picker entry points over shared interaction policy.
mod host;
use crate::{provider::ProviderRegistry, session::Session, tui::app::state::App};
use std::sync::Arc;
/// Open and target the current session model.
pub async fn open_model_picker(
    app: &mut App,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
) {
    codetether_model_picker::interactive::open_for_session(
        &mut host::Picker(app),
        session.metadata.model.clone(),
        registry,
    );
}
/// Open the picker while the session is checked out by an active turn.
pub(crate) fn open_without_session(app: &mut App, registry: Option<&Arc<ProviderRegistry>>) {
    codetether_model_picker::interactive::open_without_session(&mut host::Picker(app), registry);
}
/// Close the picker through the existing chat-return behavior.
pub fn close_model_picker(app: &mut App) {
    codetether_model_picker::interactive::close(&mut host::Picker(app));
}
/// Apply the selected model to the active session.
pub fn apply_selected_model(app: &mut App, session: &mut Session) {
    codetether_model_picker::interactive::apply(&mut host::Picker(app), session);
}

mod slot;
/// Apply the selected session or verifier model using native ownership bindings.
pub(crate) fn apply_selection(
    app: &mut App,
    session_slot: &mut crate::tui::app::session_runtime::SessionSlot,
) {
    codetether_model_picker::interactive::selected(
        &mut host::Picker(app),
        &mut slot::SelectionSlot(session_slot),
        crate::tool::goal::verify::set_verifier_model,
    );
}

/// Handle verifier selection with the application's preference storage.
pub(crate) fn verifier_command(
    app: &mut App,
    registry: Option<&Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    codetether_model_picker::interactive::verifier_command::run(
        &mut host::Picker(app),
        registry,
        prompt,
    )
}
