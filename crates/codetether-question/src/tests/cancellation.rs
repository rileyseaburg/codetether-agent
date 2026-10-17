//! cancellation contract tests.
use super::fixture::{request, wait_pending};
use crate::{Error, runtime};
#[tokio::test]
async fn cancellation_cleans_up_and_duplicate_does_not_replace_request() {
    let task = tokio::spawn(runtime::scope(
        "cancel-test".into(),
        runtime::ask(request(false), None),
    ));
    wait_pending("cancel-test").await;
    let second = runtime::scope("cancel-test".into(), runtime::ask(request(true), None)).await;
    assert!(matches!(second, Err(Error::Busy)));
    assert!(!runtime::pending("cancel-test").unwrap().question().multiple);
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert!(runtime::pending("cancel-test").is_none());
}
