//! Model-visible contract for explicit user goal edits.

use serde_json::{Value, json};

/// Keep model tools and the authenticated UI on the same native controller.
pub(super) fn parameters() -> Value {
    json!({"type":"object","additionalProperties":false,"properties":{
        "goalId":{"type":"string","description":"Current get_goal ID"},
        "updatedAt":{"type":"string","description":"Current get_goal revision"},
        "action":{"type":"string","enum":["edit","pause","resume","clear"]},
        "objective":{"type":"string","minLength":1,"maxLength":10000},
        "successCriteria":{"type":"array","maxItems":100,
            "items":{"type":"string","maxLength":2000}},
        "forbidden":{"type":"array","maxItems":100,
            "items":{"type":"string","maxLength":2000}},
        "tokenBudget":{"type":["integer","null"],"minimum":1,
            "description":"Omit to keep the cap; null explicitly removes it"}
    },"required":["goalId","updatedAt","action"]})
}
