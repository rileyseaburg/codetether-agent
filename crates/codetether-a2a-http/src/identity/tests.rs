//! Runtime identity extension round-trip regression.
use super::{attach, from_card};
#[test]
fn card_round_trips_routable_identity() {
    let mut card =
        codetether_a2a_core::application_card::default_card("http://127.0.0.1:8000", "test");
    attach(&mut card, "author-agent");
    assert_eq!(from_card(&card).as_deref(), Some("author-agent"));
}
