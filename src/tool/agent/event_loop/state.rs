//! Mutable state for streamed sub-agent events.
//!
//! This module keeps the event-loop state shape separate from the collection
//! and finalization logic.
//!
//! # Examples
//!
//! ```ignore
//! let state = EventLoopState::default();
//! assert!(state.response.is_empty());
//! ```

use serde_json::Value;

/// Mutable accumulator for streamed session output.
///
/// # Examples
///
/// ```ignore
/// let state = EventLoopState::default();
/// ```
#[derive(Default)]
pub(super) struct EventLoopState {
    pub(super) response: String,
    pub(super) thinking: String,
    pub(super) tools: Vec<Value>,
    pub(super) error: Option<String>,
    pub(super) done: bool,
}
