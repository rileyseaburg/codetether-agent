#[path = "subprocess_policy_process_connection.rs"]
mod connection;
#[path = "subprocess_policy_process_fixture.rs"]
mod fixture;

use super::{ApprovalStore, EnvGuard, lock_env, policy_args};

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn alias_approval_starts_one_real_mcp_process() {
    let _lock = lock_env();
    let workspace = std::env::current_dir().expect("workspace");
    let data = tempfile::Builder::new()
        .prefix(".mcp-test-")
        .tempdir_in(workspace)
        .expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    if let Some(reason) = crate::tool::sandbox::unavailable_reason() {
        panic!("mandatory sandbox unavailable: {reason}");
    }
    let (marker, server) = fixture::create(data.path());
    let command = server.to_str().expect("server path");
    let reviewed = policy_args(command, &[], None);
    let blocked = crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", &reviewed)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "fixture MCP process")
        .expect("approve");

    let client = connection::approved(command, request_id).await;
    drop(client);
    assert_eq!(
        std::fs::read_to_string(&marker).expect("marker"),
        "started\n"
    );

    let replay = connection::replay(command, request_id).await;
    assert!(replay.is_err());
    assert_eq!(
        std::fs::read_to_string(marker).expect("marker"),
        "started\n"
    );
}
