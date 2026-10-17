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
pub struct EventLoopState {
    pub response: String,
    pub thinking: String,
    pub tools: Vec<Value>,
    pub error: Option<String>,
    pub done: bool,
    pub timed_out: bool,
}
