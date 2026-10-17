use super::Params;

#[test]
fn batch_call_accepts_name_arguments_aliases() {
    let params: Params = serde_json::from_value(serde_json::json!({
        "calls": [
            {
                "name": "read",
                "arguments": { "path": "src/main.rs" }
            }
        ]
    }))
    .expect("should parse alias form");

    assert_eq!(params.calls.len(), 1);
    assert_eq!(params.calls[0].tool, "read");
    assert_eq!(params.calls[0].args["path"], "src/main.rs");
}

#[test]
fn batch_call_accepts_tool_args_primary_form() {
    let params: Params = serde_json::from_value(serde_json::json!({
        "calls": [
            {
                "tool": "read",
                "args": { "path": "src/main.rs" }
            }
        ]
    }))
    .expect("should parse primary form");

    assert_eq!(params.calls.len(), 1);
    assert_eq!(params.calls[0].tool, "read");
    assert_eq!(params.calls[0].args["path"], "src/main.rs");
}
