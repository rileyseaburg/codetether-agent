//! Runtime queue rejection returns ownership without starting provider work.

use super::{ActiveCancel, SessionNotice};
use crate::session::tasks::runtime::answer_review;

#[path = "answer_review_submit_support.rs"]
mod support;

#[tokio::test]
async fn answer_review_runtime_rejects_submission_before_started() {
    let session = crate::session::tasks::answer_review_test_support::session().await;
    let id = session.id.clone();
    let active = ActiveCancel::default();
    assert!(active.prepare(&id));
    answer_review::begin(&id, "Why?").await.unwrap();
    let request = support::request(session);
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(4);
    let (notice_tx, mut notice_rx) = tokio::sync::mpsc::channel(4);
    assert!(!super::submit(request, &active, &event_tx, &notice_tx).await);
    let Some(SessionNotice::Failed { session, error }) = notice_rx.recv().await else {
        panic!("expected hold notice");
    };
    assert_eq!(session.id, id);
    assert!(error.contains("Goal paused"));
    assert!(notice_rx.try_recv().is_err());
    assert!(event_rx.try_recv().is_err());
    assert!(
        active.prepare(&id),
        "held submission leaked its reservation"
    );
    support::accept(&id).await;
    super::submit(support::request(session), &active, &event_tx, &notice_tx).await;
    assert!(matches!(
        notice_rx.recv().await,
        Some(SessionNotice::Started)
    ));
    // The empty mock registry fails execution, but the handoff must start first.
    assert!(matches!(
        notice_rx.recv().await,
        Some(SessionNotice::Failed { .. })
    ));
    assert!(active.prepare(&id), "finished execution retained ownership");
    active.clear();
}
