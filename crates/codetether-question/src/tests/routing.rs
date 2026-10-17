//! routing contract tests.
use super::fixture::{request, wait_pending};
use crate::{Action, Error, runtime};

#[tokio::test]
async fn rendezvous_routes_answers_to_exact_session() {
    let a = tokio::spawn(runtime::scope(
        "roundtrip-a".into(),
        runtime::ask(request(false), Some("roundtrip-a")),
    ));
    let b = tokio::spawn(runtime::scope(
        "roundtrip-b".into(),
        runtime::ask(request(false), Some("roundtrip-b")),
    ));
    wait_pending("roundtrip-a").await;
    wait_pending("roundtrip-b").await;
    assert!(!runtime::act("unknown", Action::Submit));
    runtime::act("roundtrip-a", Action::Next);
    runtime::act("roundtrip-a", Action::Submit);
    assert_eq!(a.await.unwrap().unwrap(), vec![vec!["Two"]]);
    assert!(runtime::pending("roundtrip-a").is_none());
    assert!(runtime::pending("roundtrip-b").is_some());
    assert!(runtime::dismiss("roundtrip-b"));
    assert!(matches!(b.await.unwrap(), Err(Error::Dismissed)));
    assert!(!runtime::dismiss("roundtrip-b"));
}
