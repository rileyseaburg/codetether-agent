//! Default in-process plugins claim exact approval once.

use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, tetherscript::TetherScriptPluginTool};
use serde_json::json;

#[tokio::test]
async fn default_plugin_execution_is_blocked_then_runs_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let mut args = json!({
        "source": "fn run(value) { return Ok(value) }",
        "hook": "run",
        "args": ["once"]
    });
    let tool = TetherScriptPluginTool::with_root(data.path().to_path_buf());
    let first = tool.execute(args.clone()).await.expect("blocked");
    assert!(!first.success);
    let id = first.metadata["approval_request_id"]
        .as_str()
        .expect("approval id")
        .to_string();
    ApprovalStore::open_default()
        .expect("store")
        .approve(&id, "test", "allow")
        .expect("approve");
    args["approval_id"] = json!(id);

    let mut changed = args.clone();
    changed["source"] = json!("fn run(value) { return Ok(\"changed\") }");
    changed["__ct_tetherscript_source_sha256"] = json!("caller-spoof");
    let mismatched = tool.execute(changed).await.expect("mismatch");
    assert!(!mismatched.success);

    let approved = tool.execute(args.clone()).await.expect("approved");
    assert!(approved.success, "{}", approved.output);
    assert_eq!(approved.metadata["value"], json!({"ok": "once"}));
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
}
