use serde_json::{Value, json};

pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "The path to the file to edit"},
            "old_string": {"type": "string", "description": "The string to replace; exact, whitespace-tolerant, then fuzzy matching are tried"},
            "new_string": {"type": "string", "description": "The string to replace old_string with"},
            "replace_all": {"type": "boolean", "description": "Replace every exact occurrence when old_string appears multiple times", "default": false},
            "instruction": {"type": "string", "description": "Optional Morph instruction."},
            "update": {"type": "string", "description": "Optional Morph update snippet."}
        },
        "required": ["path"],
        "example": {
            "path": "src/main.rs",
            "old_string": "fn old() {\n    println!(\"old\");\n}",
            "new_string": "fn new() {\n    println!(\"new\");\n}",
            "replace_all": false
        }
    })
}
