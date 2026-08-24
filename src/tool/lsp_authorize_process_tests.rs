//! Exact approval through a real sandboxed rust-analyzer request.

use crate::approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use crate::config::AccessMode;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
#[ignore = "requires rust-analyzer and enforced OS sandbox"]
async fn approved_lsp_starts_sandboxed_server_once_and_rejects_replay() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    std::fs::create_dir(project.path().join("src")).unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname='approval_lsp'\nversion='0.1.0'\nedition='2024'\n",
    )
    .unwrap();
    let source = project.path().join("src/lib.rs");
    std::fs::write(&source, "pub fn valid() {}\n").unwrap();
    let tool = super::super::LspTool::with_root(format!("file://{}", project.path().display()));
    let mut args = json!({
        "action":"diagnostics", "file_path":source,
        "__ct_session_id":"lsp-approval-test",
        "__ct_parent_workspace":project.path(),
    });
    let blocked = tool.execute(args.clone()).await.unwrap();
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    ApprovalStore::open_default()
        .unwrap()
        .approve(id, "test", "lsp")
        .unwrap();
    args["approval_id"] = json!(id);

    let result = tool.execute(args.clone()).await.unwrap();
    assert!(result.success, "{}", result.output);
    let replay = tool
        .execute(args)
        .await
        .expect("structured replay rejection");
    assert!(!replay.success);
    assert_eq!(replay.metadata["error_code"], "APPROVAL_RECEIPT_REJECTED");
}
