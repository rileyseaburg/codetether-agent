use crate::tool::agent::bridge::{AgentOrigin, AgentSnapshot};

fn peer() -> AgentSnapshot {
    AgentSnapshot {
        id: "peer-id".into(),
        name: "ci-inspector".into(),
        instructions: "Inspect CI".into(),
        message_count: 1,
        depth: 0,
        is_processing: true,
        origin: AgentOrigin::lan_peer(),
        failed: false,
    }
}

fn text(agent: &AgentSnapshot) -> String {
    super::line(agent, true)
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect()
}

#[test]
fn remote_peer_row_exposes_transport_and_live_state() {
    let rendered = text(&peer());
    assert!(rendered.contains("[LAN peer]"));
    assert!(rendered.contains("via a2a-mdns"));
    assert!(rendered.contains("[working]"));
}

#[test]
fn remote_peer_row_never_claims_a_parent_or_model() {
    let rendered = text(&peer());
    assert!(!rendered.contains("← main"), "{rendered}");
    assert!(!rendered.contains("default model"), "{rendered}");
}

#[test]
fn local_child_row_shows_parent_and_model() {
    let mut local = peer();
    local.origin = AgentOrigin::Local {
        parent: Some("main".into()),
        model_id: Some("openai-codex/x".into()),
    };
    let rendered = text(&local);
    assert!(rendered.contains("← main · openai-codex/x"));
    assert!(rendered.contains("[tool-agent]"));
}
