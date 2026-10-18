//! Terminal RLM overview with workspace identity and a session summary.
//!
//! Call [`render_rlm`] with the current workspace and session-selection values.
//! This crate renders presentation only; model execution belongs to the runtime.
mod render;
pub use render::render_rlm;
