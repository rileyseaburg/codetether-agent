use std::sync::Arc;

use crate::{provider::ProviderRegistry, session::Session};

use super::{PromptRequest, SessionNotice};

fn request(session: Session) -> PromptRequest {
    PromptRequest::new(
        session,
        "hello".into(),
        Vec::new(),
        Arc::new(ProviderRegistry::new()),
        None,
        None,
    )
}

#[tokio::test]
async fn completion_notice_is_published_after_active_owner_clears() {
    let session = Session::new().await.expect("session");
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(4);
    let (notice_tx, mut notice_rx) = tokio::sync::mpsc::channel(4);
    let runtime = super::spawn(event_tx, notice_tx);
    runtime.submit(request(session)).await.unwrap();
    assert!(matches!(
        notice_rx.recv().await,
        Some(SessionNotice::Started)
    ));
    let Some(SessionNotice::Failed { session, .. }) = notice_rx.recv().await else {
        panic!("expected first failure notice");
    };

    runtime.submit(request(session)).await.unwrap();
    assert!(matches!(
        notice_rx.recv().await,
        Some(SessionNotice::Started)
    ));
    runtime.shutdown().await;
}
