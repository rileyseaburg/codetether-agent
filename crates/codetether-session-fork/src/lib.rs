//! Non-destructive edited-conversation prefix branching.
//!
//! ForkSession supplies native fields and storage. create validates the exact
//! source user message, creates a fresh identity and copies only prior context.
mod error;
mod host;
pub mod interactive;
mod metadata;
mod repository;
mod service;
pub use error::ForkError;
pub use host::ForkSession;
pub use repository::create as create_persisted;
pub use service::create;

/// Persisted snapshots for detached background conversations.
pub mod detached;

mod application_binding;
/// Types used by frontend command declarations.
#[doc(hidden)]
pub mod __private {
    pub use codetether_chat_model::ChatMessage;
    pub use codetether_context_policy::pages::PageKind;
    pub use codetether_provider_core::Message;
    pub use codetether_session_metadata::SessionMetadata;
}
