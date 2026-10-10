use super::{WorktreeInfo, WorktreeManager};

#[tokio::test]
async fn writes_multi_root_workspace_with_subfolder_detection() {
    let dir = tempfile::tempdir().unwrap();
    let mgr = WorktreeManager::for_repo(dir.path());
    let worktrees = vec![WorktreeInfo {
        name: "feature-auth".to_string(),
        path: dir.path().join("feature-auth"),
        branch: "codetether/feature-auth".to_string(),
        active: true,
    }];
    let dest = dir.path().join("project.code-workspace");
    let written = mgr.write_code_workspace(&worktrees, &dest).await.unwrap();
    assert_eq!(written, dest);

    let body = tokio::fs::read_to_string(&dest).await.unwrap();
    let doc: serde_json::Value = serde_json::from_str(&body).unwrap();
    let folders = doc["folders"].as_array().unwrap();
    assert_eq!(folders.len(), 2);
    assert_eq!(folders[0]["name"], "main");
    assert_eq!(folders[1]["name"], "feature-auth");
    assert_eq!(doc["settings"]["git.autoRepositoryDetection"], "subFolders");
}
