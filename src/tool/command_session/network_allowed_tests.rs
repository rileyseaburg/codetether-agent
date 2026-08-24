use super::super::{Registry, network_approval_support as approval};
use super::super::{network_env_support as env, network_test_support as tcp};
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn approved_exec_with_network_authority_can_reach_loopback() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = env::DisabledNetwork::set();
    env::DisabledNetwork::allow(true);
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
    assert!(result.success, "{}", result.output);
    assert_eq!(result.metadata["sandboxed"], true);
    assert!(tcp::received(&listener).await, "sandbox missed loopback");
    let replay = tool.execute(args).await.expect("replay result");
    assert!(!replay.success);
    assert!(replay.output.contains("approval"));
}