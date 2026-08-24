//! Real loopback HTTP proof for direct network approval and replay.

#[path = "webfetch_preflight_approval_tests.rs"]
mod preflight;
#[path = "webfetch_test_server.rs"]
mod server;

use super::WebFetchTool;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
async fn direct_fetch_is_blocked_then_requests_exactly_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let network = crate::tool::network_access::test_env::Network::set("1");
    let (url, hits, server) = server::start().await;
    let tool = WebFetchTool::new();
    let mut args = json!({"url": url, "format": "text"});
    let first = tool.execute(args.clone()).await.expect("blocked");
    assert!(!first.success);
    assert_eq!(server::count(&hits), 0);
    let id = first.metadata["approval_request_id"]
        .as_str()
        .expect("id")
        .to_string();
    ApprovalStore::open_default()
        .expect("store")
        .approve(&id, "test", "allow")
        .expect("approve");
    args["approval_id"] = json!(id);

    let approved = tool.execute(args.clone()).await.expect("approved");
    assert!(approved.success, "{}", approved.output);
    server.await.expect("server");
    assert_eq!(server::count(&hits), 1);
    let replay = tool.execute(args.clone()).await.expect("replay");
    assert!(!replay.success);
    assert_eq!(server::count(&hits), 1);
    drop(network);
    let disabled = tool
        .execute(json!({"url": args["url"], "__ct_effective_network_allowed": true}))
        .await
        .expect("disabled");
    assert!(!disabled.success);
    assert_eq!(server::count(&hits), 1);
}
