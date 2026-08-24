use super::sanitized;
use serde_json::json;

#[test]
fn scope_sanitization_removes_only_approval_ids_recursively() {
    let task = json!({
        "id": "task-1",
        "approval_id": "top",
        "metadata": {
            "approval_id": "nested",
            "repository": "https://example.com/repo.git"
        }
    });
    let value = sanitized(&task);
    assert!(value.get("approval_id").is_none());
    assert!(value["metadata"].get("approval_id").is_none());
    assert_eq!(
        value["metadata"]["repository"],
        "https://example.com/repo.git"
    );
}
