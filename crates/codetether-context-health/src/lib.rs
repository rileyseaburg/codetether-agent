//! Recent context-compaction, truncation and RLM progress summaries.
//!
//! [ContextHealthState] records shared session events as display text.
//! [render] combines those summaries with current token usage for status output.
mod compaction;
mod lines;
mod render;
mod rlm;
mod state;
pub use render::render;
pub use state::ContextHealthState;
