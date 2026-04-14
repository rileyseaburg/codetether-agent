//! Worker-local tool registry construction.
//!
//! This module keeps worker approval policy and workspace-aware tool wiring
//! out of the main worker loop.

mod build;
mod mutating;
mod policy;
mod safe;

pub use build::create_filtered_registry;
pub use mutating::{
    register_edit_tools, register_go_tool, register_model_tools, register_mutating_tools,
    register_task_tools,
};
pub use policy::{is_safe_tool, is_tool_allowed};
pub use safe::register_safe_tools;
