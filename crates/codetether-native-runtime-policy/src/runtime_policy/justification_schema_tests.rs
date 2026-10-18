use super::{apply, note_access_mode};
use crate::config::AccessMode;
use crate::provider::ToolDefinition;
use serde_json::json;

fn tool(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.into(),
        description: String::new(),
        parameters: json!({
            "type": "object",
            "properties": {"command": {}, "justification": {}},
            "required": ["command"]
        }),
    }
}

fn required(def: &ToolDefinition) -> Vec<String> {
    serde_json::from_value(def.parameters["required"].clone()).unwrap()
}

#[test]
fn ask_mode_requires_justification_only_on_mutating_tools() {
    let _lock = crate::approval::test_env::lock_env();
    note_access_mode(Some(AccessMode::Ask));
    let defs = apply(vec![tool("bash"), tool("read")]);
    note_access_mode(None);
    assert_eq!(required(&defs[0]), ["command", "justification"]);
    assert_eq!(required(&defs[1]), ["command"]);
    let again = apply(vec![tool("bash")]);
    assert_eq!(required(&again[0]), ["command"]);
}

#[test]
fn other_modes_leave_schemas_untouched() {
    let _lock = crate::approval::test_env::lock_env();
    note_access_mode(Some(AccessMode::Approve));
    let defs = apply(vec![tool("bash")]);
    note_access_mode(None);
    assert_eq!(required(&defs[0]), ["command"]);
}
