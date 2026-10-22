//! First-party Rust invocation planning, without spawning processes.
//!
//! [`RustBuildPlanTool`] returns a command for the existing `exec_command` tool.
//! That second call owns approval, sandboxing, streaming and `write_stdin` IDs.
//! Planning never grants approval or substitutes for compiler verification.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
mod input;
mod metadata;
#[cfg(test)]
mod metadata_tests;
mod output;
mod schema;
#[cfg(test)]
mod tests;

/// Plan an explicit CodeTether Rust command without executing workspace code.
///
/// # Examples
/// ```
/// use codetether_tool_core::Tool;
/// let tool = codetether_rust_build_tool::RustBuildPlanTool;
/// assert_eq!(tool.id(), "rust_build_plan");
/// ```
pub struct RustBuildPlanTool;

#[async_trait]
impl Tool for RustBuildPlanTool {
    fn id(&self) -> &str {
        "rust_build_plan"
    }
    fn name(&self) -> &str {
        "Rust Build Plan"
    }
    fn description(&self) -> &str {
        "Plan ra-jit build/run/test, ordinary Cargo check, or repository cache-build/cache-publish. Cache actions use scripts/dev/release-build-loop.sh; cache-build builds then attempts guarded publication, cache-publish retries publication only. Does NOT execute: submit exec_command through normal approval, then poll with write_stdin."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        match input::parse(args).and_then(output::render) {
            Ok(value) => Ok(ToolResult::success(serde_json::to_string_pretty(&value)?)),
            Err(error) => Ok(ToolResult::error(format!("Rust build plan: {error:#}"))),
        }
    }
}
