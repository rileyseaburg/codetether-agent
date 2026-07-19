use super::TurnStateStore;

#[test]
fn begin_replaces_prior_turn_state() {
    let states = TurnStateStore::default();
    states.begin("session");
    states.capture("session", "first");
    assert_eq!(
        states.current("session").get().map(String::as_str),
        Some("first")
    );
    states.begin("session");
    assert!(states.current("session").get().is_none());
}

#[test]
fn capture_keeps_the_first_server_value() {
    let states = TurnStateStore::default();
    states.capture("session", "first");
    states.capture("session", "second");
    assert_eq!(
        states.current("session").get().map(String::as_str),
        Some("first")
    );
}
