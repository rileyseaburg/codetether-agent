//! Unified diff patch execution with shared approval records.
//!
//! [ApplyPatchTool] previews or applies patches beneath its workspace root.
//! [proposed] reconstructs file contents for approval-time diagnostics.
//! Parsing and hunk application use the shared tool-core patch algorithms.
//!
//! ```rust
//! let _tool = codetether_patch::ApplyPatchTool::with_root("workspace".into());
//! ```
mod apply;
mod approval;
mod approval_flow;
mod args;
mod file_io;
mod group;
mod hunk_apply;
mod metadata;
mod parser;
mod path_guard;
mod pipeline;
pub mod proposed;
mod result;
mod schema;
mod success;
mod tool;
mod types;
/// Return the approval resource string for a unified diff patch.
pub use codetether_tool_core::patch::approval_resource_from_patch;
pub use tool::ApplyPatchTool;
