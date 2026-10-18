use super::{get, register, remove};
use crate::a2a::{agent_identity, server::A2AServer};

#[test]
fn resolves_card_route_by_provenance_identity() {
    let route_name = "route-for-identity-test";
    let identity = "identity-for-route-test";
    let endpoint = "http://127.0.0.1:48123";
    let mut card = A2AServer::default_card(endpoint);
    card.name = route_name.to_string();
    agent_identity::attach(&mut card, identity);

    register(&card, endpoint, Some("test-token".to_string()));
    let route = get(identity).expect("identity alias should resolve");
    assert_eq!(route.endpoint, endpoint);
    assert_eq!(route.agent_identity_id.as_deref(), Some(identity));
    remove(route_name);
    assert!(get(identity).is_none());
}

#[test]
fn replacing_route_removes_stale_identity_alias() {
    let name = "route-identity-replacement-test";
    let endpoint = "http://127.0.0.1:48124";
    let mut first = A2AServer::default_card(endpoint);
    first.name = name.to_string();
    agent_identity::attach(&mut first, "retired-route-identity");
    register(&first, endpoint, None);

    let mut replacement = A2AServer::default_card(endpoint);
    replacement.name = name.to_string();
    agent_identity::attach(&mut replacement, "current-route-identity");
    register(&replacement, endpoint, None);

    assert!(get("retired-route-identity").is_none());
    assert!(get("current-route-identity").is_some());
    remove(name);
}
