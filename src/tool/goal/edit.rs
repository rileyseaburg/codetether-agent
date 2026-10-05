//! User-requested goal changes through the native goal controller.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub(super) struct EditGoalTool;

#[async_trait]
impl Tool for EditGoalTool {
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
         Use tokenBudget:null to remove a cap without resetting usage. \
         Resume is separate. Never clear a goal or raise/remove its budget \
         to bypass a limit on your own. Completion still requires update_goal."
    }
    /// Use the same native edit schema as the HTTP controller.
    fn parameters(&self) -> Value {
        super::edit_schema::parameters()
    }
    /// Apply through native revision and accounting checks.
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::edit_run::run(input).await
    }
}
