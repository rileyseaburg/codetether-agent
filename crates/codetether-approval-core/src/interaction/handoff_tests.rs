//! Cancellation and failure coverage without a runtime or external delivery.
use super::*;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

struct Noop;
impl Wake for Noop {
    fn wake(self: Arc<Self>) {}
}
fn poll<F: Future>(future: Pin<&mut F>) -> Poll<F::Output> {
    let waker = Waker::from(Arc::new(Noop));
    future.poll(&mut Context::from_waker(&waker))
}
fn details() -> LiveApprovalRequest {
    LiveApprovalRequest::new(
        uuid::Uuid::new_v4().to_string(),
        "call".into(),
        "write".into(),
        "write".into(),
        "note.txt".into(),
        "save notes".into(),
    )
}

#[test]
fn cancellation_during_delivery_removes_pending_entry() {
    let details = details();
    let id = details.approval_id.clone();
    let mut future = Box::pin(request(details, |_| std::future::pending::<bool>()));
    assert!(poll(future.as_mut()).is_pending());
    assert!(super::super::pending_query::is_pending(&id));
    drop(future);
    assert!(!super::super::pending_query::is_pending(&id));
}

#[test]
fn failed_delivery_denies_and_removes_pending_entry() {
    let details = details();
    let id = details.approval_id.clone();
    let mut future = Box::pin(request(details, |_| std::future::ready(false)));
    assert_eq!(
        poll(future.as_mut()),
        Poll::Ready(LiveApprovalDecision::denied())
    );
    assert!(!super::super::pending_query::is_pending(&id));
}

#[test]
fn decision_is_returned_and_removes_pending_entry() {
    let details = details();
    let id = details.approval_id.clone();
    let mut future = Box::pin(request(details, |_| std::future::ready(true)));
    assert!(poll(future.as_mut()).is_pending());
    assert!(pending::decide(&id, LiveApprovalDecision::Approved));
    assert_eq!(
        poll(future.as_mut()),
        Poll::Ready(LiveApprovalDecision::Approved)
    );
    assert!(!super::super::pending_query::is_pending(&id));
}
