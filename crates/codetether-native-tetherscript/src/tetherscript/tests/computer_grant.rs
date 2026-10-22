//! Test computer capability grant fields and snapshot dispatch.

use serde_json::json;

use crate::tool::Tool;
use codetether_tetherscript_tool::input::TetherScriptPluginInput;

#[test]
fn parses_computer_grant_fields() {
    let input: TetherScriptPluginInput = serde_json::from_value(json!({
        "hook": "main",
        "grant_computer": true,
        "computer_origin": ["agent://desktop-script"],
        "computer_scope": ["computer.inspect"]
    }))
    .unwrap();

    assert!(input.grant_computer);
    assert!(input.wants_computer());
    assert_eq!(input.computer_origin, ["agent://desktop-script"]);
    assert_eq!(input.computer_scope, ["computer.inspect"]);
}

#[tokio::test]
async fn grant_computer_runs_status_from_tetherscript() {
    let tool = crate::project_tool();
    let result = tool
        .execute(json!({
            "source": "fn main() { return computer.status() }",
            "hook": "main",
            "grant_computer": true,
            "computer_scope": ["computer.inspect"]
        }))
        .await
        .unwrap();

    assert!(result.output.contains("success"), "{}", result.output);
}
