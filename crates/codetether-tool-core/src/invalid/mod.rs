//! Unknown-tool fallback with deterministic formatting and suggestions.
//!
//! [InvalidTool] stores the requested name and available catalog, then returns
//! corrective feedback through the shared tool interface.
mod distance;
mod message;
mod state;
mod suggestions;
mod tool;
pub use state::InvalidTool;
