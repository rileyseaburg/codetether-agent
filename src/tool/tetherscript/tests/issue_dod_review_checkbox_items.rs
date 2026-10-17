use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn issue_dod_review_accepts_preserved_checkbox_source_items() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/issue_dod_review.tether",
            "hook": "validate",
            "args": [
                "## Issue DoD checklist\n- [ ] Preserve source checklist.\n- [ ] Run validation.",
                "## Issue DoD checklist\n- [x] Preserve source checklist — test added.\n- [x] Run validation — cargo test passed."
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
                "all_dod_items_present": true,
                "approved": false,
                "decision": "accept",
                "expected_dod_items": 2,
                "has_issue_dod_checklist": true,
                "has_missing_or_unproven_item": false,
                "reason": "review output preserves the Issue DoD gate",
                "valid": true
            }
        }))
    );
}
