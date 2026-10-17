//! admission contract tests.
use super::fixture::request;
use crate::{Error, runtime};
#[tokio::test]
async fn headless_and_wrong_session_fail_without_waiting() {
    assert!(matches!(
        runtime::ask(request(false), None).await,
        Err(Error::Unavailable)
    ));
    runtime::scope("owner".into(), async {
        assert!(runtime::available());
        assert!(matches!(
            runtime::ask(request(false), Some("other")).await,
            Err(Error::Unavailable)
        ));
        assert!(!tokio::spawn(async { runtime::available() }).await.unwrap());
    })
    .await;
    assert!(!runtime::available());
}
