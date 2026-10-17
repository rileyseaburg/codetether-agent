//! Tool result data and construction, error, and truncation operations.
//!
//! [ToolResult] carries text and structured metadata independently of any registry.
mod construct;
mod error;
mod state;
mod truncate;
pub use state::ToolResult;
