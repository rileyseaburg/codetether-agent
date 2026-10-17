//! Project-local persistence and prompt context for beliefs.
//!
//! [load_project_beliefs] and [save_project_beliefs] use the project memory file.
//! [belief_context] renders high-confidence active claims in stored order.
mod context;
mod load;
mod save;
pub use context::belief_context;
pub use load::load_project_beliefs;
pub use save::save_project_beliefs;

/// File name for persisted project beliefs.
const MEMORY_FILE: &str = ".codetether/memory.json";
