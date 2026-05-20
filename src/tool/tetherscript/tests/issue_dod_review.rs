use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn issue_dod_review_rejects_approval_with_unproven_items() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/issue_dod_review.tether",
            "hook": "validate",
            "args": [
                "## Acceptance criteria\n- A repo-local test captures the Issue DoD reviewer expectation.\n- Run validation and include evidence.",
                "## Issue DoD checklist\n- [x] A repo-local test captures the Issue DoD reviewer expectation.\n- [ ] Run validation and include evidence. not-run\n\nApproved."
            ]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({
            "ok": {
                "approved": true,
                "decision": "reject",
                "expected_dod_items": 2,
                "has_issue_dod_checklist": true,
                "has_missing_or_unproven_item": true,
                "reason": "review must not approve when any Issue DoD item is missing or unproven",
                "valid": false
            }
        }))
    );
}

#[tokio::test]
async fn issue_dod_review_accepts_proven_checklist_without_false_approval() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/issue_dod_review.tether",
            "hook": "validate",
            "args": [
                "## Acceptance criteria\n- A repo-local test captures the Issue DoD reviewer expectation.\n- Run validation and include evidence.",
                "## Issue DoD checklist\n- [x] A repo-local test captures the Issue DoD reviewer expectation — static/local test added.\n- [x] Run validation and include evidence — focused CI-like cargo test passed."
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
