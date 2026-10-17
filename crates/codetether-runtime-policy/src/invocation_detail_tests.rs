use serde_json::json;

#[test]
fn generic_detail_hides_runtime_injected_keys() {
    let args = json!({"action": "delete", "__ct_session_id": "s", "_tool_call_id": "c"});
    let detail = super::render("memory", &args).expect("detail");
    assert!(detail.contains("\"action\": \"delete\""));
    assert!(!detail.contains("__ct_"));
    assert!(!detail.contains("_tool_call_id"));
}

#[test]
fn patch_detail_is_not_truncated() {
    let patch = format!("--- a/file\n+++ b/file\n{}", "+full line\n".repeat(100));
    assert_eq!(
        super::render("apply_patch", &json!({"patch": patch})),
        Some(patch)
    );
}
