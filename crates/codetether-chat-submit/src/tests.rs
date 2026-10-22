//! Regression coverage for expanded paste publication on both submission paths.
mod finish_host;
mod steering_host;

const COMPACT: &str = "Review [Pasted text #1: 6 lines] please";
const EXPANDED: &str = "Review one\ntwo\nthree\nfour\nfive\nsix please";

#[test]
fn submitted_paste_is_readable_in_chat() {
    use std::{future::Future, task::Context, task::Poll, task::Waker};
    let mut host = finish_host::Host::default();
    {
        let mut task = std::pin::pin!(crate::finish::finish(&mut host, COMPACT));
        assert_eq!(
            task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(())
        );
    }
    assert_eq!(host.published, EXPANDED);
    assert_eq!(host.dispatched, host.published);
}

#[test]
fn steered_paste_is_readable_in_chat() {
    let mut host = steering_host::Host {
        accept: true,
        ..Default::default()
    };
    crate::steering::submit(&mut host, COMPACT);
    assert_eq!(host.messages.len(), 1);
    assert_eq!(host.messages[0].content, EXPANDED);
    assert!(host.cleared);
}

#[test]
fn rejected_steering_retains_compact_input_and_attachments() {
    let mut host = steering_host::Host::default();
    crate::steering::submit(&mut host, COMPACT);
    assert!(host.messages.is_empty());
    assert!(!host.cleared);
    assert_eq!(host.queued, COMPACT);
}
