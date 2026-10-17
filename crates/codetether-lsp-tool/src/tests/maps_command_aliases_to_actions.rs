//! Preserved LSP argument regression.
use crate::args::resolve_action_raw;
use serde_json::json;

#[test]
fn maps_command_aliases_to_actions() {
    let args = json!({
        "command": "textDocument/hover",
        "file_path": "src/lib.rs",
        "line": 1,
        "column": 1
    });
    let action = resolve_action_raw(&args).expect("command should map");
    assert_eq!(action, "hover");
}
