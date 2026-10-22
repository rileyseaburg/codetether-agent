//! User-requested goal changes through the native goal controller.

use crate::Host;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::marker::PhantomData;

/// Model-facing edit goal tool using an application storage host.
/// # Examples
/// ```no_run
/// fn tool<H: codetether_goal_tools::Host>() {
///     let _tool = codetether_goal_tools::EditGoalTool::<H>::new();
/// }
/// ```
pub struct EditGoalTool<H>(PhantomData<fn() -> H>);
impl<H> EditGoalTool<H> {
    /// Construct the tool without allocating or touching storage.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<H> Default for EditGoalTool<H> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<H: Host + 'static> Tool for EditGoalTool<H> {
    /// Expose a stable model-tool identifier for explicit goal edits.
    fn id(&self) -> &str {
        "edit_goal"
    }
    /// Name the goal control without conflating it with completion.
    fn name(&self) -> &str {
        "Edit Goal"
    }
    /// Require user intent instead of automatic limit bypass.
    fn description(&self) -> &str {
        "Edit an existing goal only when the user explicitly requests it. \
         Read get_goal first and supply its goalId and updatedAt. \
         Budget controls require the human TUI or authenticated API and are unavailable through model tools. \
         Resume is separate. Never clear a goal to bypass a limit on your own. \
         Completion still requires update_goal."
    }
    /// Expose revision-bound edits without human-only budget controls.
    fn parameters(&self) -> Value {
        super::edit_schema::parameters()
    }
    /// Apply through native revision and accounting checks.
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::edit_run::run::<H>(input).await
    }
}
