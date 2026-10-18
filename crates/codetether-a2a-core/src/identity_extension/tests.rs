//! Preserve persona and SPIFFE claims in the identity extension.
use super::attach_with_claims;
#[test]
fn card_carries_persona_and_spiffe_claims() {
    let mut card = crate::application_card::default_card("http://127.0.0.1:8000", "test");
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
