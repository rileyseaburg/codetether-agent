//! Shared editor language-server request and navigation behavior.
//!
//! [EditorLspState] holds a host-specific manager without creating it.
//! [cursor_target], [resolve_definition], [hover] and [navigate] implement
//! coordinate conversion, retries and buffer navigation. The host applies
//! returned status/content, while [interactive] owns approval-aware key dispatch.
mod cursor;
mod definition;
mod hover;
mod navigate;
mod outcomes;
mod state;
mod state_traits;
pub use cursor::cursor_target;
pub use definition::resolve_definition;
pub use hover::hover;
pub use navigate::navigate;
pub use outcomes::{DefinitionOutcome, HoverOutcome};
pub use state::EditorLspState;

/// Borrowed frontend controls for editor clicks, shortcuts and LSP actions.
pub mod interactive;
