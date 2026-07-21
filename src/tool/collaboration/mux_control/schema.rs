//! JSON schema for the mux-only manager tool.

pub(super) fn parameters() -> serde_json::Value {
    serde_json::json!({
        "type":"object",
        "properties":{
            "action":{"type":"string","enum":["list","read","status","steer","interact","watch","start","roll","stop"]},
            "name":{"type":"string","description":"Exact mux route name"},
            "message":{"type":"string","description":"Steering text"},
            "workspace":{"type":"string","description":"Workspace for start"},
            "session_id":{"type":"string","description":"Durable session to resume during start or roll"},
            "timeout_ms":{"type":"integer","minimum":250,"maximum":30000,"description":"Mux watch timeout"},
            "force":{"type":"boolean","description":"Permit stop or roll when semantic state is unavailable"}
        },
        "required":["action"]
    })
}
