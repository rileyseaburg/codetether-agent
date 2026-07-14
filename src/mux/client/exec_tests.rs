use tempfile::tempdir;

#[tokio::test]
async fn command_runs_in_active_workspace() {
    let workspace = tempdir().unwrap();
    super::run("pwd > mux-pwd.txt", workspace.path())
        .await
        .unwrap();
    let observed = tokio::fs::read_to_string(workspace.path().join("mux-pwd.txt"))
        .await
        .unwrap();
    assert_eq!(observed.trim(), workspace.path().to_str().unwrap());
}
