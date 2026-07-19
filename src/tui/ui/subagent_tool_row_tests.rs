use crate::tool::agent::bridge::AgentSnapshot;

#[test]
fn remote_peer_row_exposes_transport_and_live_state() {
    let peer = AgentSnapshot {
        id: "peer-id".into(),
        name: "ci-inspector".into(),
        instructions: "Inspect CI".into(),
        message_count: 1,
        model_id: Some("a2a-mdns".into()),
        parent: Some("main".into()),
        depth: 0,
        is_processing: true,
        is_remote: true,
        failed: false,
    };
    let rendered = super::line(&peer, true)
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>();
    assert!(rendered.contains("[LAN peer]"));
    assert!(rendered.contains("[working]"));
}
