//! Application provider binding for the shared persona relay.
use super::{events::AutochatUiEvent, host::Runtime};
/// Run the relay and publish its events.
pub async fn run_relay(
    task: String,
    model: String,
    ui_tx: tokio::sync::mpsc::UnboundedSender<AutochatUiEvent>,
) {
    codetether_autochat_core::interactive::run::run_relay::<Runtime, _>(task, model, ui_tx).await;
}
