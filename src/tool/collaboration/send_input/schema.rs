//! JSON schema for Codex-compatible structured child input.

use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({"type":"object","properties":{
        "target":{"type":"string"},
        "message":{"type":"string","description":"Use either message or items."},
        "items":{"type":"array","minItems":1,"items":{
            "type":"object","properties":{"type":{"type":"string","enum":[
                "text","image","local_image","audio","local_audio","skill","mention"
            ]}},"required":["type"]
        }},
        "interrupt":{"type":"boolean"}
    },"required":["target"]})
}
