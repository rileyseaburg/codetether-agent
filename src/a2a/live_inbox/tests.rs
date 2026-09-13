//! Hand-off contract: enqueue → dequeue → resolve → pending resolves.

use super::{attach, dequeue, detach, enqueue, is_attached};

#[tokio::test]
async fn parked_turn_resolves_with_the_tui_reply() {
    let pending = enqueue("task-1", Some("ctx"), "peer-a", "help me");
    let inbound = dequeue().expect("turn was parked");
    assert_eq!(inbound.task_id, "task-1");
    assert_eq!(inbound.context_id.as_deref(), Some("ctx"));
    assert_eq!(inbound.from, "peer-a");
    assert_eq!(inbound.prompt, "help me");
    inbound.responder.resolve(Ok("done: use fs_read".into()));
    assert_eq!(pending.await, Ok("done: use fs_read".into()));
}

#[tokio::test]
async fn dropped_responder_yields_an_error_not_a_hang() {
    let pending = enqueue("task-2", None, "peer-b", "x");
    drop(dequeue());
    let outcome = pending.await;
    assert!(outcome.unwrap_err().contains("dropped"));
}

#[test]
fn attach_is_explicit_and_reversible() {
    detach();
    assert!(!is_attached());
    attach();
    assert!(is_attached());
    detach();
    assert!(!is_attached());
}
