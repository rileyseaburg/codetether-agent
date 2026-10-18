//! Multi-process proof: sessions on one checkout share one server and one lease table.

use crate::mux::client::MuxConnection;
use crate::mux::lease::{CoordinationReply, CoordinationRequest};
use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn second_session_on_a_served_checkout_joins_and_shares_leases() {
    let _env = crate::approval::test_env::lock_env();
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", root.path()) };
    let workspace = tokio::fs::canonicalize(root.path()).await.unwrap();
    let mut first = super::process::start("shared-one", &workspace, root.path()).await;

    let record = crate::mux::control::start_join::live_server(&workspace)
        .await
        .expect("first server is live for the workspace");
    let second =
        crate::mux::control::start_join::create_session(record, "shared-two", workspace.clone())
            .await
            .unwrap();
    assert_eq!(second.record.pid, first.target.record.pid);
    assert_eq!(second.record.state.sessions.len(), 2);

    let mut one = MuxConnection::connect(&first.target).await.unwrap();
    let mut two = MuxConnection::connect(&second).await.unwrap();
    let claim = |owner: &str, path: &str| ClientRequest::Coordinate {
        request: CoordinationRequest::Acquire {
            owner: owner.into(),
            agent: owner.into(),
            workspace: workspace.clone(),
            paths: vec![path.into()],
            wait_ms: 0,
        },
    };
    let acquired = one.request(claim("shared-one", "src")).await.unwrap();
    assert!(matches!(
        acquired,
        ServerResponse::Coordination {
            reply: CoordinationReply::Acquired { .. }
        }
    ));
    let blocked = two
        .request(claim("shared-two", "src/lib.rs"))
        .await
        .unwrap();
    assert!(matches!(
        blocked,
        ServerResponse::Coordination {
            reply: CoordinationReply::Blocked { .. }
        }
    ));

    super::verify::shutdown(&mut one, &mut first.child).await;
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
