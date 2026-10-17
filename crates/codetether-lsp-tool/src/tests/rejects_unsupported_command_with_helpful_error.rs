//! Preserved LSP argument regression.
use crate::args::resolve_action_raw;
use serde_json::json;

#[test]
fn rejects_unsupported_command_with_helpful_error() {
    let args = json!({
        "command": "workspace/diagnostics"
    });
    let err = resolve_action_raw(&args).expect_err("unsupported command should error");
    assert!(err.to_string().contains("Unsupported lsp command"));
}
