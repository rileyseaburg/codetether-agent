use super::{attach, from_card};
use crate::a2a::server::A2AServer;

#[test]
fn card_round_trips_routable_identity() {
    let mut card = A2AServer::default_card("http://127.0.0.1:8000");
    attach(&mut card, "author-agent");
    assert_eq!(from_card(&card).as_deref(), Some("author-agent"));
}
