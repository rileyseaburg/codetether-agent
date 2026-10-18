use super::super::{Capability, with_defaults};
use serde_json::json;

#[tokio::test]
async fn omitted_bash_and_git_cwd_use_assigned_workspace() {
    let root = tempfile::tempdir().unwrap();
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(root.path())
        .status()
        .unwrap();
    std::fs::write(root.path().join("assigned-only.txt"), "marker").unwrap();
    let registry = with_defaults(root.path(), Capability::Verification);

    let bash = registry.get("bash").unwrap();
    let pwd = bash.execute(json!({"command":"pwd -P"})).await.unwrap();
    assert_eq!(
        std::path::Path::new(pwd.output.trim()),
        root.path().canonicalize().unwrap()
    );

    let git = registry.get("git").unwrap();
    let status = git.execute(json!({"op":"status"})).await.unwrap();
    assert!(status.success);
    assert!(status.output.contains("assigned-only.txt"));
}

#[tokio::test]
async fn apply_patch_changes_only_the_assigned_workspace() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("file.txt"), "old\n").unwrap();
    let registry = with_defaults(root.path(), Capability::Mutating);
    let patch = "--- a/file.txt\n+++ b/file.txt\n@@ -1 +1 @@\n-old\n+new";
    let result = registry
        .get("apply_patch")
        .unwrap()
        .execute(json!({"patch":patch}))
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert_eq!(
        std::fs::read_to_string(root.path().join("file.txt")).unwrap(),
        "new"
    );
}
