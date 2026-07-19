use crate::a2a::peer_route::PeerRoute;
use std::time::Duration;

#[tokio::test]
async fn detached_remote_turn_returns_immediately_and_records_transport_failure() {
    let suffix = uuid::Uuid::new_v4();
    let name = format!("detached-{suffix}");
    let parent = format!("parent-{suffix}");
    let turn = super::super::observation::begin(&name, Some(&parent), "inspect CI");
    let result = super::detached::spawn(
        &name,
        "inspect CI",
        Some(&parent),
        PeerRoute {
            endpoint: "not-a-url".into(),
            token: None,
            description: String::new(),
            skills: Vec::new(),
            agent_identity_id: None,
        },
        turn,
    );
    assert!(result.success);
    assert!(result.output.contains("/agents"));

    let failed = tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            let peers = super::super::observation::snapshots(Some(&parent));
            if let Some(peer) = peers.into_iter().find(|peer| !peer.is_processing) {
                break peer;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("detached transport should fail promptly");
    assert!(failed.failed);
}
