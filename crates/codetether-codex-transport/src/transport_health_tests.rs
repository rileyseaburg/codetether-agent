use super::TransportHealth;

#[test]
fn interruption_only_disables_its_own_session() {
    let health = TransportHealth::default();
    health.mark_interrupted("session-a");
    assert!(health.requires_http("session-a"));
    assert!(!health.requires_http("session-b"));
}

#[test]
fn concurrent_session_updates_remain_isolated() {
    let health = TransportHealth::default();
    std::thread::scope(|scope| {
        for id in ["a", "b", "c"] {
            let health = health.clone();
            scope.spawn(move || health.mark_interrupted(id));
        }
    });
    assert!(
        ["a", "b", "c"]
            .into_iter()
            .all(|id| health.requires_http(id))
    );
    assert!(!health.requires_http("unaffected"));
}

#[test]
fn interrupted_session_tracking_is_bounded() {
    let health = TransportHealth::default();
    for index in 0..=super::MAX_TRACKED_SESSIONS {
        health.mark_interrupted(&format!("session-{index}"));
    }
    assert!(health.state().interrupted.len() <= super::MAX_TRACKED_SESSIONS);
}
