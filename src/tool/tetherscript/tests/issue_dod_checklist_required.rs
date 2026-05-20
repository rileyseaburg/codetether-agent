use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn issue_dod_review_requires_explicit_issue_dod_checklist() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/issue_dod_review.tether",
            "hook": "validate",
            "args": [
                "## Acceptance criteria\n- Preserve the source issue checklist.",
                "Reviewed tests and docs. Looks good."
            ]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({
            "ok": {
                "approved": false,
                "decision": "reject",
                "expected_dod_items": 1,
                "has_issue_dod_checklist": false,
                "has_missing_or_unproven_item": false,
                "reason": "review output must include an Issue DoD checklist",
                "valid": false
            }
        }))
    );
}
