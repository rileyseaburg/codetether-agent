//! Thought events, phase rotation and per-tick work records.
//!
//! [ThoughtPhase] determines event classification. [ThoughtEvent] and
//! [MemorySnapshot] provide serializable output; [ThoughtWorkItem] and
//! [ThoughtResult] carry work data without starting model calls.
mod event;
mod event_type;
mod phase;
mod work;
pub use event::{MemorySnapshot, ThoughtEvent};
pub use event_type::ThoughtEventType;
pub use phase::ThoughtPhase;
pub use work::{ThoughtResult, ThoughtWorkItem};

/// Bounded event and memory-snapshot buffers.
pub mod buffers;
/// Selection of recent persona-relevant events.
pub mod context;

/// Prompt formatting and deterministic thought-output normalization.
pub mod presentation;

/// Events and snapshots constructed from completed thoughts.
pub mod build;

/// Model-backed and deterministic thought-result construction.
pub mod result;
mod tick_output;
pub use tick_output::TickOutput;

/// Model-backed generation and deterministic recovery.
pub mod generate;
