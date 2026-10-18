use super::fixture::text;
use crate::History;
use codetether_provider_core::{Message, Role};

#[test]
fn len_tracks_underlying_vec() {
    let mut buf: Vec<Message> = vec![text(Role::User, "seed")];
    let history = History::new(&mut buf);
    assert_eq!(history.len(), 1);
    assert!(!history.is_empty());
}
