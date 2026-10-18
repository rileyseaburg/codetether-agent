//! Original mux wire-roundtrip regression.
use crate::{ClientRequest, read_frame, write_frame};
use codetether_worktree_lease::CoordinationRequest;
#[tokio::test]
async fn coordination_request_round_trips_over_json_frame() {
    let request = ClientRequest::Coordinate {
        request: CoordinationRequest::Acquire {
            owner: "turn-1".into(),
            agent: "agent-a".into(),
            workspace: "/workspace".into(),
            paths: vec!["src/lib.rs".into()],
            wait_ms: 60_000,
        },
    };
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &request).await.unwrap();
    let decoded: ClientRequest = read_frame(&mut reader).await.unwrap().unwrap();
    assert!(matches!(decoded, ClientRequest::Coordinate { .. }));
}
