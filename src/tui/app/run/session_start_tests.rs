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

#[tokio::test]
async fn truncated_startup_preserves_full_history_source() {
    let bus = AgentBus::new().into_arc();
    let mut session = crate::session::Session::new().await.unwrap();
    session.id = "full-history".into();
    let load = crate::session::TailLoad {
        session,
        dropped: 25,
        file_bytes: 100,
    };
    let resolved = super::session_resolve::resolve(Some(Ok(load)), &bus)
        .await
        .unwrap();
    let (source, has_older) = resolved.outcome.history_source(&resolved.session.id);
    assert_eq!(source, "full-history");
    assert_ne!(resolved.session.id, source);
    assert!(has_older);
}
