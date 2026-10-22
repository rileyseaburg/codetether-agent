//! Executable tool contracts and reusable result/feedback adapters.
//!
//! [Tool] defines execution; [ToolResult] carries output and metadata. Aliases
//! and unknown-tool fallback are registry-independent. Concrete tools, plugin
//! registries and access policy live in higher layers. Tool profiles select
//! which registered contracts are advertised to each provider.
mod alias;
pub mod budget;
pub mod context_edit;
mod contract;
pub mod feedback;
pub mod images;
mod invalid;
mod result;
pub use alias::AliasTool;
pub use budget::{DEFAULT_TOOL_OUTPUT_MAX_BYTES, tool_output_budget};
pub use contract::Tool;
pub use invalid::InvalidTool;
pub use result::ToolResult;

#[cfg(test)]
mod tests;

pub mod messages;

/// Workspace-root validation and normalization of tool arguments.
pub mod path_guard;

/// Read-only tool classification shared by dispatch and permission policy.
pub mod readonly;

/// File-mutation preview classification shared by approval and tool dispatch.
pub mod proposed;

/// Pure patch parsing and transformation shared by tool adapters and policy.
pub mod patch;

/// Deterministic advertised-tool profiles and provider defaults.
pub mod profile;

/// Hard-denied temporary-directory writes across tool boundaries.
pub mod temp_write_guard;
