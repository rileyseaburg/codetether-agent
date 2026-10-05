//! Runtime queue rejection returns ownership without starting provider work.

use super::{ActiveCancel, PromptRequest, SessionNotice};
use crate::provider::ProviderRegistry;
use crate::session::tasks::runtime::answer_review;
use std::sync::Arc;

#[tokio::test]
async fn answer_review_runtime_rejects_submission_before_started() {
    let session = crate::session::tasks::answer_review_test_support::session().await;
    let id = session.id.clone();
    answer_review::begin(&id, "Why?").await.unwrap();
    let request = PromptRequest::new(
        session,
        "continue".into(),
        Vec::new(),
        Arc::new(ProviderRegistry::new()),
        None,
        None,
    );
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(4);
    let (notice_tx, mut notice_rx) = tokio::sync::mpsc::channel(4);
    assert!(!super::submit(request, &ActiveCancel::default(), &event_tx, &notice_tx).await);
    let Some(SessionNotice::Failed { session, error }) = notice_rx.recv().await else {
        panic!("expected hold notice");
    };
    assert_eq!(session.id, id);
    assert!(error.contains("Goal paused"));
    assert!(notice_rx.try_recv().is_err());
    assert!(event_rx.try_recv().is_err());
}
