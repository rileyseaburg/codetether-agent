use super::WriteArgs;
use serde_json::json;

#[test]
fn accepts_a_workspace_target() {
    let args = json!({"path": "src/lib.rs", "content": "// x"});
    let parsed = WriteArgs::parse(&args).expect("workspace write is allowed");
    assert_eq!(parsed.path, "src/lib.rs");
    assert_eq!(parsed.content, "// x");
}

#[test]
fn rejects_temp_directory_targets() {
    let args = json!({"path": "/tmp/scratch.txt", "content": "x"});
    let error = WriteArgs::parse(&args).expect_err("temp write must be refused");
    assert!(!error.success);
    assert_eq!(
        error.metadata.get("error_code").and_then(|v| v.as_str()),
        Some("TEMP_DIR_WRITE_BLOCKED")
    );
}

#[test]
fn reports_missing_fields() {
    let error = WriteArgs::parse(&json!({"content": "x"})).expect_err("path required");
    assert!(error.output.contains("path is required"));
    let error = WriteArgs::parse(&json!({"path": "a.rs"})).expect_err("content required");
    assert!(error.output.contains("content is required"));
}
