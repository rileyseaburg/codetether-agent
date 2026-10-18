//! End-to-end pre-approval diagnostics tests.

use serde_json::json;

#[tokio::test]
async fn typescript_error_blocks_without_writing() {
    if !typescript_server_available() {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    std::fs::create_dir(&src).unwrap();
    std::fs::write(
        dir.path().join("tsconfig.json"),
        r#"{"compilerOptions":{"strict":true}}"#,
    )
    .unwrap();
    let path = src.join("broken.ts");
    let original = "export const valid: string = 'yes';\n";
    std::fs::write(&path, original).unwrap();
    let args = json!({
        "path": path,
        "content": "export const broken: string = 42;\n"
    });

    let result = super::blocked(dir.path(), "write", &args)
        .await
        .expect("type error must block approval");

    assert_eq!(result.metadata["error_code"], "LSP_PREAPPROVAL_FAILED");
    assert_eq!(result.metadata["approval_suppressed"], true);
    assert!(result.output.contains("broken.ts:1"));
    assert_eq!(std::fs::read_to_string(path).unwrap(), original);
}

fn typescript_server_available() -> bool {
    std::process::Command::new("typescript-language-server")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}
