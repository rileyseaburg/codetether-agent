
#[path = "network_allowed_tests.rs"]
mod allowed;
use super::{Registry, network_approval_support as approval};
use super::{network_env_support as env, network_test_support as tcp};
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn approved_sandboxed_exec_cannot_reach_loopback_and_rejects_replay() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = env::DisabledNetwork::set();
    if let Some(reason) = crate::tool::sandbox::unavailable_reason() {
        panic!("mandatory sandbox unavailable: {reason}");
    }
    eprintln!("executing network denial through the available OS sandbox");
    let listener = tcp::listener();
    let mut args = json!({
        "cmd": tcp::command(&listener),
        "workdir": std::env::current_dir().expect("workspace"),
        "yield_time_ms": 1_000,
    });
    let request_id = approval::grant(&args);
    args["approval_id"] = json!(&request_id);
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);

    let result = tool.execute(args.clone()).await.expect("execution result");
    assert!(!result.success, "network probe unexpectedly succeeded");
    assert_eq!(result.metadata["sandboxed"], true);
    assert!(!tcp::received(&listener).await, "sandbox reached loopback");

    let replay = tool.execute(args).await.expect("replay result");
    assert!(!replay.success);
    assert!(replay.output.contains("approval"));
}