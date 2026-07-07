//! Sub-module declarations for the bubble family.
//!
//! Pulled out of `bubble.rs` to keep that file within the 50-line budget.

#[path = "bubble_assistant.rs"]
pub mod bubble_assistant;
#[path = "timeline.rs"]
pub mod timeline;
pub use bubble_assistant::assistant_bubble_lines;
