//! Typed frontend entry points for relay execution and events.
/// Implementation detail of [crate::declare_frontend_autochat].
#[doc(hidden)]
#[macro_export]
macro_rules! __autochat_entrypoints {
    ($state:path) => {
        /// Frontend relay event handling.
        pub mod handler {
            use super::events::AutochatUiEvent;
            use $state as AppState;
            /// Apply a relay event to frontend state.
            pub fn handle_autochat_event(app: &mut AppState, event: AutochatUiEvent) {
                $crate::interactive::handler::handle_autochat_event(app, event);
            }
        }
        /// Relay execution with the frontend provider registry.
        pub mod run {
            use super::{events::AutochatUiEvent, host::Runtime};
            /// Run the relay and publish its events.
            pub async fn run_relay(
                task: String,
                model: String,
                ui_tx: ::tokio::sync::mpsc::UnboundedSender<AutochatUiEvent>,
            ) {
                $crate::interactive::run::run_relay::<Runtime, _>(task, model, ui_tx).await;
            }
        }
        /// Background launch with the frontend provider registry.
        pub mod worker {
            use super::{events::AutochatUiEvent, host::Runtime};
            /// Start a relay and return its UI event receiver.
            pub fn start_autochat_relay(
                task: String,
                model: String,
            ) -> ::tokio::sync::mpsc::UnboundedReceiver<AutochatUiEvent> {
                $crate::interactive::worker::start_autochat_relay::<Runtime, _>(task, model)
            }
        }
    };
}
