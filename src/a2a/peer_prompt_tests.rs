use super::{append, append_identity};
use crate::a2a::{peer_route, server::A2AServer};

#[test]
fn names_live_peer_and_forbids_manual_rediscovery() {
    let mut card = A2AServer::default_card("http://192.0.2.10:4000");
    card.name = "voice-api-owner".to_string();
    card.description = "Owns the typed Voice API".to_string();
    peer_route::register(&card, &card.url, Some("capability".to_string()));
    let prompt = append("base".to_string());
    peer_route::remove(&card.name);
    assert!(prompt.contains("@voice-api-owner: Owns the typed Voice API"));
    assert!(prompt.contains("Do not run dns-sd"));
    assert!(!prompt.contains("capability"));
    assert!(!prompt.contains("192.0.2.10"));
}

#[test]
fn names_this_process_separately_from_discovered_peers() {
    let mut prompt = String::new();
    append_identity(&mut prompt, Some("workspace-agent-ab12"));

    assert!(prompt.contains("Your LAN peer name is @workspace-agent-ab12"));
    assert!(prompt.contains("this process"));
}
