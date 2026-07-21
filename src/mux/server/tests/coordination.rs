//! Authenticated end-to-end lease conflict proof.

use crate::mux::client::MuxConnection;
use crate::mux::lease::{CoordinationReply, CoordinationRequest};
use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn mux_server_blocks_overlapping_authenticated_claims() {
    let root = tempfile::tempdir().unwrap();
    let (record, _, server) = super::pty_support::server(root.path().into()).await;
    let mut first = MuxConnection::connect(&record).await.unwrap();
    let mut second = MuxConnection::connect(&record).await.unwrap();

    let one = claim("one", "agent-one", root.path(), "src");
    let two = claim("two", "agent-two", root.path(), "src/lib.rs");
    assert!(matches!(
        first.request(one).await.unwrap(),
        ServerResponse::Coordination {
            reply: CoordinationReply::Acquired { .. }
        }
    ));
    assert!(matches!(
        second.request(two).await.unwrap(),
        ServerResponse::Coordination {
            reply: CoordinationReply::Blocked { .. }
        }
    ));
    first.request(ClientRequest::Detach).await.unwrap();
    second.request(ClientRequest::Detach).await.unwrap();
    server.await.unwrap();
}

fn claim(owner: &str, agent: &str, root: &std::path::Path, path: &str) -> ClientRequest {
    ClientRequest::Coordinate {
        request: CoordinationRequest::Acquire {
            owner: owner.into(),
            agent: agent.into(),
            workspace: root.into(),
            paths: vec![path.into()],
            wait_ms: 0,
        },
    }
}
