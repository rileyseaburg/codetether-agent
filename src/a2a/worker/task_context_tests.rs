use super::build_task_context;
use serde_json::json;

#[test]
fn reads_reusable_a2a_context_from_task_metadata() {
    let task = json!({
        "metadata": {
            "context_id": "forgejo_pr_42_context",
            "resume_session_id": "author-session",
            "provenance_verified": true,
            "preserve_session_workspace": true
        },
        "prompt": "continue review"
    });
    let context = build_task_context(&task, "fallback");
    assert_eq!(context.context_id.as_deref(), Some("forgejo_pr_42_context"));
    assert!(context.preserve_session_workspace);
}

#[test]
fn accepts_legacy_conversation_id_alias() {
    let task = json!({
        "metadata": {"conversation_id": "forgejo_pr_42_legacy"}
    });
    let context = build_task_context(&task, "fallback");
    assert_eq!(context.context_id.as_deref(), Some("forgejo_pr_42_legacy"));
    assert!(!context.preserve_session_workspace);
}
