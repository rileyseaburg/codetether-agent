use super::{attach, attach_with_claims, from_card};
use crate::a2a::server::A2AServer;

#[test]
fn card_round_trips_routable_identity() {
    let mut card = A2AServer::default_card("http://127.0.0.1:8000");
    attach(&mut card, "author-agent");
    assert_eq!(from_card(&card).as_deref(), Some("author-agent"));
}

#[test]
fn card_carries_persona_and_spiffe_claims() {
    let mut card = A2AServer::default_card("http://127.0.0.1:8000");
    attach_with_claims(
        &mut card,
        "author-agent",
        Some("engineering-manager"),
        Some("spiffe://codetether.run/agent"),
    );
    let params = card.capabilities.extensions[0].params.as_ref().unwrap();
    assert_eq!(params["personaId"], "engineering-manager");
    assert_eq!(params["spiffeId"], "spiffe://codetether.run/agent");
}
