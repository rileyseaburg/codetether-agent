//! Add isolation metadata without changing a tool result's success flag.
use crate::Handoff;
use codetether_tool_core::ToolResult;
impl Handoff {
    /// Attach checkout identity to JSON objects or append it to other output.
    /// # Arguments
    /// * `result` — Existing tool result whose success flag is retained.
    /// # Returns
    /// The result with its isolation metadata added.
    /// # Examples
    /// Use a [Handoff] from [crate::allocate] and pass a [ToolResult].

    pub fn attach(&self, mut result: ToolResult) -> ToolResult {
        let isolation = serde_json::json!({"mode": "worktree", "checkout": self,
            "integration": "review_then_cherry_pick", "auto_merge": false});
        if let Ok(serde_json::Value::Object(mut object)) = serde_json::from_str(&result.output) {
            object.insert("isolation".into(), isolation);
            result.output = serde_json::Value::Object(object).to_string();
        } else {
            result.output = format!("{}\n\nChild isolation: {isolation}", result.output);
        }
        result
    }
}
