//! Search tool schema retaining the existing advertised defaults.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "query": {"type": "string", "description": "Natural-language search query"},
            "top_n": {"type": "integer", "description": "Max backends to run (default 1)"},
            "router_model": {"type": "string", "description": "Override router model (default zai/glm-5.1)"}
        },
        "required": ["query"]
    })
}
