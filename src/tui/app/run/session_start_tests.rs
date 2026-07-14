use std::path::Path;

use crate::bus::AgentBus;

#[tokio::test]
async fn startup_resolves_distinct_fresh_sessions() {
    let bus = AgentBus::new().into_arc();
    let first_scan = super::session_scan::load(Path::new(".")).await;
    let first = super::session_resolve::resolve(Some(first_scan), &bus)
        .await
        .expect("first session");
    let second_scan = super::session_scan::load(Path::new(".")).await;
    let second = super::session_resolve::resolve(Some(second_scan), &bus)
        .await
        .expect("second session");

    assert_ne!(first.session.id, second.session.id);
    assert!(first.session.messages.is_empty());
    assert!(second.session.messages.is_empty());
}
