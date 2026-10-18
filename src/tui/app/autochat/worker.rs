//! Application binding for shared autochat background launch.
use super::{events::AutochatUiEvent, host::Runtime};
/// Start a relay and return its UI event receiver.
pub fn start_autochat_relay(
    task: String,
    model: String,
) -> tokio::sync::mpsc::UnboundedReceiver<AutochatUiEvent> {
    codetether_autochat_core::interactive::worker::start_autochat_relay::<Runtime, _>(task, model)
}
