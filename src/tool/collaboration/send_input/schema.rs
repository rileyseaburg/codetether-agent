//! JSON schema for Codex-compatible structured child input.

use serde_json::{Value, json};

#[cfg(test)]
#[path = "tests/schema.rs"]
mod schema_tests;

pub(super) fn parameters() -> Value {
    json!({"type":"object","properties":{
        "target":{"type":"string"},
        "message":{"type":"string","description":"Use either message or items."},
        "items":{"type":"array","minItems":1,"items":item_schema()},
        "interrupt":{"type":"boolean"}
    },"required":["target"]})
}
fn item_schema() -> Value {
    json!({"anyOf":[
        variant("text", &["text"], "Text to send to the child."),
        variant("image", &["image_url"], "Base64 image data URL or absolute HTTP(S) image URL; remote references are forwarded without fetching."),
        variant("local_image", &["path"], "Image path, relative to the parent workspace or absolute."),
        variant("audio", &["audio_url"], "Audio input (currently unsupported; returns an error)."),
        variant("local_audio", &["path"], "Local audio (currently unsupported; returns an error)."),
        variant("skill", &["name", "path"], "Named skill reference."),
        variant("mention", &["name", "path"], "Named mention reference.")
    ]})
}

fn variant(kind: &str, fields: &[&str], description: &str) -> Value {
    let mut properties = serde_json::Map::new();
    properties.insert("type".into(), json!({"type":"string","enum":[kind]}));
    for field in fields {
        properties.insert((*field).into(), json!({"type":"string"}));
    }
    let mut required = vec!["type"];
    required.extend_from_slice(fields);
    json!({"type":"object", "description":description,
        "properties":properties, "required":required, "additionalProperties":false})
}
