//! Structured Git input validation tests.

use super::GitTool;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
async fn unknown_op_and_missing_message_error() {
    let tool = GitTool::new();
    let bad = tool
        .execute(json!({ "op": "frobnicate" }))
        .await
        .expect("unknown op result");
    assert!(!bad.success);
    let no_msg = tool
        .execute(json!({ "op": "commit" }))
        .await
        .expect("missing message result");
    assert!(!no_msg.success);
}
