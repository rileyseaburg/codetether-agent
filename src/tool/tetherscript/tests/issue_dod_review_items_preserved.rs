use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn issue_dod_review_rejects_when_dod_items_omitted() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/issue_dod_review.tether",
            "hook": "validate",
            "args": [
                "## Acceptance criteria\n- Item A.\n- Item B.\n- Item C.",
                "## Issue DoD checklist\n- [x] Item A. Verified.\n\nApproved."
            ]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({
            "ok": {
                "actual_dod_items": 1,
                "all_dod_items_present": false,
                "approved": true,
                "decision": "reject",
                "expected_dod_items": 3,
                "has_issue_dod_checklist": true,
                "has_missing_or_unproven_item": false,
                "reason": "review output must preserve each source issue DoD item",
                "valid": false
            }
        }))
    );
}
