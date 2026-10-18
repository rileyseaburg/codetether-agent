//! Frontend selection for relay provider loading, state and entry points.
mod entrypoints;
mod runtime;
mod view;

/// Bind the relay to frontend state, session events and provider types.
///
/// Declares the `host`, `events`, `state`, `handler`, `run` and `worker` modules
/// used by the frontend. Provider loading retains the registry's shared Vault
/// loader. The frontend owns the concrete state and message types.
///
/// # Examples
/// ```text
/// codetether_autochat_core::declare_frontend_autochat!(
///     AppState, SessionEvent, ProviderRegistry, Provider, ChatMessage, MessageType,
/// );
/// ```
#[macro_export]
macro_rules! declare_frontend_autochat {
    ($state:path, $event:ty, $registry:path, $provider:path, $message:path, $kind:path $(,)?) => {
        /// Session-specific relay events.
        pub mod events {
            /// Relay event carrying the frontend's session event type.
            pub type AutochatUiEvent = $crate::interactive::events::AutochatUiEvent<$event>;
        }
        /// Session-specific relay state.
        pub mod state {
            /// Relay state carrying the frontend's session event type.
            pub type AutochatState = $crate::interactive::state::AutochatState<$event>;
        }
        pub(crate) mod host {
            $crate::__autochat_runtime!($registry, $provider);
            $crate::__autochat_view!($state, $event, $message, $kind);
        }
        $crate::__autochat_entrypoints!($state);
    };
}
