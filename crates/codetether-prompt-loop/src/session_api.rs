//! Inherent prompt entry points using application-selected runner functions.
pub use anyhow;
pub use codetether_image::ImageAttachment;
pub use codetether_session_events::{SessionEvent, SessionResult};
pub use tokio::sync::mpsc::Sender;
/// Bind the public session prompt API to the existing plain and streaming runners.
///
/// The type, registry and two runner paths keep application composition explicit.
/// # Usage
/// ```text
/// codetether_prompt_loop::impl_session_prompt!(Session, Registry, plain_runner, stream_runner);
/// ```
#[macro_export]
macro_rules! impl_session_prompt {
    ($session:ty, $registry:ty, $plain:path, $stream:path) => {
        impl $session {
            /// Execute a prompt and return the final text answer (non-streaming).
            ///
            /// Provider configuration is loaded lazily and shared for the process.
            ///
            /// # Errors
            ///
            /// Returns an error if no providers are configured, the model cannot
            /// be resolved, or the agentic loop exhausts its retry budgets.
            pub async fn prompt(
                &mut self,
                message: &str,
            ) -> $crate::session_api::anyhow::Result<$crate::session_api::SessionResult> {
                $plain(self, message).await
            }

            /// Process a user message with real-time event streaming for UI
            /// updates.
            ///
            /// Accepts a pre-loaded
            /// [`ProviderRegistry`](crate::provider::ProviderRegistry) to avoid
            /// re-fetching secrets from Vault on every message.
            pub async fn prompt_with_events(
                &mut self,
                message: &str,
                event_tx: $crate::session_api::Sender<$crate::session_api::SessionEvent<$session>>,
                registry: ::std::sync::Arc<$registry>,
            ) -> $crate::session_api::anyhow::Result<$crate::session_api::SessionResult> {
                $stream(self, message, ::std::vec::Vec::new(), event_tx, registry).await
            }

            /// Execute a prompt with optional image attachments and stream events.
            ///
            /// Images must be base64-encoded data URLs.
            pub async fn prompt_with_events_and_images(
                &mut self,
                message: &str,
                images: ::std::vec::Vec<$crate::session_api::ImageAttachment>,
                event_tx: $crate::session_api::Sender<$crate::session_api::SessionEvent<$session>>,
                registry: ::std::sync::Arc<$registry>,
            ) -> $crate::session_api::anyhow::Result<$crate::session_api::SessionResult> {
                $stream(self, message, images, event_tx, registry).await
            }
        }
    };
}
