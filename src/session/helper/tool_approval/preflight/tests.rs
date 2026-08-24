//! Side-effect-free pre-approval validation tests.

use serde_json::json;

#[tokio::test]
async fn path_escape_blocks_without_writing_or_spawning() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let path = outside.path().join("outside.ts");
    let original = "export const valid: string = 'yes';\n";
    std::fs::write(&path, original).unwrap();
    let args = json!({"path": path, "content": "changed\n"});

    let result = super::blocked(dir.path(), "write", &args)
        .await
        .expect("path escape must block approval");

    assert_eq!(result.metadata["error_code"], "PREAPPROVAL_INVALID_INPUT");
    assert_eq!(result.metadata["approval_suppressed"], true);
    assert_eq!(std::fs::read_to_string(path).unwrap(), original);
}
