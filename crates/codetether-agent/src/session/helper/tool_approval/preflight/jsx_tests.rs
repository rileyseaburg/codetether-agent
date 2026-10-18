//! Real-LSP preapproval permits valid JSX but still rejects actual JSX/type errors.

use super::jsx_fixture::{ORIGINAL, PROPOSED, available, project};
use serde_json::json;

#[tokio::test]
async fn jsx_preapproval_accepts_valid_write_and_patch_without_disk_mutation() {
    if !available() {
        return;
    }
    for (extension, tool) in [
        ("tsx", "write"),
        ("jsx", "write"),
        ("tsx", "apply_patch"),
        ("jsx", "apply_patch"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let path = project(dir.path(), extension);
        let args = if tool == "write" {
            json!({"path": path, "content": PROPOSED})
        } else {
            let relative = format!("src/MetadataInspector.{extension}");
            let patch =
                format!("--- a/{relative}\n+++ b/{relative}\n@@ -1 +1 @@\n-{ORIGINAL}+{PROPOSED}");
            json!({"patch": patch})
        };
        let blocked = super::blocked(dir.path(), tool, &args).await;
        assert!(blocked.is_none(), "{extension} {tool}: {blocked:?}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), ORIGINAL);
    }
}
