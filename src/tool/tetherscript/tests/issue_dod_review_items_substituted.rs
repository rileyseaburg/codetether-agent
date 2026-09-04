use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn issue_dod_review_rejects_substituted_dod_items() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/issue_dod_review.tether",
            "hook": "validate",
            "args": [
                "## Acceptance criteria\n- Preserve source issue DoD checklist.\n- Run local validation.",
                "## Issue DoD checklist\n- [x] Preserve source issue DoD checklist. Verified.\n- [x] Added unrelated bullet with evidence.\n\nApproved."
            ]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({
            "ok": {
                "actual_dod_items": 2,
                "all_dod_items_present": false,
                "approved": true,
                "decision": "reject",
                "expected_dod_items": 2,
                "has_issue_dod_checklist": true,
                "has_missing_or_unproven_item": false,
                "reason": "review output must preserve each source issue DoD item",
                "valid": false
            }
        }))
    );
}
