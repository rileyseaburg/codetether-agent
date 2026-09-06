//! Correct JSX mode must not suppress real syntax or TypeScript errors.

use super::jsx_fixture::{ORIGINAL, PROPOSED, available, project};
use serde_json::json;

#[tokio::test]
async fn jsx_preapproval_still_blocks_real_errors_without_disk_mutation() {
    if !available() {
        return;
    }
    for extension in ["tsx", "jsx"] {
        let dir = tempfile::tempdir().unwrap();
        let path = project(dir.path(), extension);
        let broken = "export const MetadataInspector = () => <section><span></section>;\n";
        let blocked = super::blocked(
            dir.path(),
            "write",
            &json!({"path": path, "content": broken}),
        )
        .await
        .expect("mismatched JSX must block approval");
        assert_eq!(blocked.metadata["error_code"], "LSP_PREAPPROVAL_FAILED");
        assert_eq!(blocked.metadata["approval_suppressed"], true);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), ORIGINAL);
    }
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path(), "tsx");
    let content = format!("{PROPOSED}export const label: string = 42;\n");
    let blocked = super::blocked(
        dir.path(),
        "write",
        &json!({"path": path, "content": content}),
    )
    .await
    .expect("TypeScript errors must still block in TSX");
    assert_eq!(blocked.metadata["error_code"], "LSP_PREAPPROVAL_FAILED");
    assert!(blocked.output.contains("2322"), "{}", blocked.output);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), ORIGINAL);
}
