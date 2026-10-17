//! First-class `spawn_agent` tool.

use crate::Host;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;

#[path = "spawn/args.rs"]
mod args;
#[path = "spawn/dispatch.rs"]
mod dispatch;
#[path = "spawn_name.rs"]
mod name;
#[path = "spawn/result.rs"]
mod result;

/// Spawn a named child through the existing durable agent engine.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::Host>()->codetether_collaboration_tools::SpawnAgentTool<H> {
///     codetether_collaboration_tools::SpawnAgentTool::new()
/// }
/// ```
pub struct SpawnAgentTool<H: Host>(PhantomData<fn() -> H>);
impl<H: Host> SpawnAgentTool<H> {
    /// Construct the spawn tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: Host> Tool for SpawnAgentTool<H> {
    fn id(&self) -> &str {
        "spawn_agent"
    }
    fn name(&self) -> &str {
        "Spawn Agent"
    }
    fn description(&self) -> &str {
        "Spawn a named background sub-agent in its own managed Git worktree at parent HEAD (uncommitted edits are not copied). Review/cherry-pick returned child commits explicitly; no auto-merge. The child inherits parent context policy."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "task_name":{"type":"string","pattern":"^[a-z0-9_]+$"},
            "message":{"type":"string"},
            "fork_turns":{"type":"string","description":"Compatibility: none, all, or a count"}
        },"required":["task_name","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        dispatch::execute::<H>(serde_json::from_value(input)?).await
    }
}
