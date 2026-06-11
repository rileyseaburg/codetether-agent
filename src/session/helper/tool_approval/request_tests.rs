use crate::approval::ReviewDecision;
use crate::tool::ToolResult;
use serde_json::json;

#[test]
fn live_request_carries_execpolicy_amendment() {
    let result = ToolResult::structured_error(
        "TOOL_APPROVAL_REQUIRED",
        "bash",
        "approval required",
        None,
        None,
    )
    .with_metadata("approval_request_id", json!("approval-1"))
    .with_metadata("proposed_execpolicy_amendment", json!(["cargo", "test"]));
    let request = super::request::from_result(&result, "call-1", "bash").expect("request");
    let amendment = request.proposed_execpolicy_amendment.expect("amendment");

    assert_eq!(amendment.command, vec!["cargo", "test"]);
    assert!(request.available_decisions.iter().any(|decision| {
        matches!(decision, ReviewDecision::ApprovedExecpolicyAmendment { .. })
    }));
}
