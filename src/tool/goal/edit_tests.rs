//! Model tools expose explicit user editing, never terminal approval.

use super::edit::EditGoalTool;
use crate::tool::Tool;
use serde_json::json;

/// The native model schema includes budget removal and excludes completion.
#[test]
fn session_goal_controls_tool_contract() {
    let tool = EditGoalTool;
    assert_eq!(tool.id(), "edit_goal");
    let schema = tool.parameters();
    assert_eq!(
        schema["properties"]["tokenBudget"]["type"],
        json!(["integer", "null"])
    );
    assert_eq!(
        schema["properties"]["action"]["enum"],
        json!(["edit", "pause", "resume", "clear"])
    );
    assert_eq!(schema["required"], json!(["goalId", "updatedAt", "action"]));
    assert!(tool.description().contains("explicitly requests"));
    assert!(tool.description().contains("without resetting usage"));
}
