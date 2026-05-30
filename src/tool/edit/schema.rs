use serde_json::{Value, json};

pub fn parameters() -> Value {
    json!({
        "type":"object",
        "properties":{
            "path":{"type":"string","description":"The path to the file to edit"},
            "old_string":{"type":"string","description":"Text to replace; exact first, then whitespace-tolerant matching"},
            "new_string":{"type":"string","description":"Replacement text"},
            "replace_all":{"type":"boolean","description":"Replace every exact occurrence instead of returning AMBIGUOUS_MATCH"},
            "instruction":{"type":"string","description":"Optional Morph instruction."},
            "update":{"type":"string","description":"Optional Morph update snippet."}
        },
        "required":["path"],
        "example":{"path":"src/main.rs","old_string":"old","new_string":"new","replace_all":false}
    })
}
