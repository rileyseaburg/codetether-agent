//! Model-visible contract for explicit user goal edits.

use serde_json::{Value, json};

/// Expose revision-bound edits without human-only budget controls.
/// # Returns
/// JSON schema for non-budget, revision-bound model edits.
/// # Examples
/// ```
/// assert_eq!(codetether_goal_tools::edit_parameters()["type"], "object");
/// ```
pub fn parameters() -> Value {
    json!({"type":"object","additionalProperties":false,"properties":{
        "goalId":{"type":"string","description":"Current get_goal ID"},
        "updatedAt":{"type":"string","description":"Current get_goal revision"},
        "action":{"type":"string","enum":["edit","pause","resume","clear"]},
        "objective":{"type":"string","minLength":1,"maxLength":10000},
        "successCriteria":{"type":"array","maxItems":100,
            "items":{"type":"string","maxLength":2000}},
        "forbidden":{"type":"array","maxItems":100,
            "items":{"type":"string","maxLength":2000}}
    },"required":["goalId","updatedAt","action"]})
}
