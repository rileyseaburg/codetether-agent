use super::{activate, current_name, deactivate};

#[test]
fn stale_handle_cannot_clear_newer_identity() {
    activate("first-peer");
    assert_eq!(current_name().as_deref(), Some("first-peer"));

    activate("replacement-peer");
    deactivate("first-peer");
    assert_eq!(current_name().as_deref(), Some("replacement-peer"));

    deactivate("replacement-peer");
    assert_eq!(current_name(), None);
}
