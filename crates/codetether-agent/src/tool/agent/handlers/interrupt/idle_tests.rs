use super::super::super::{persistence, store};
use super::test_support as support;

#[tokio::test]
async fn idle_interrupt_does_not_append_a_marker() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let entry = support::child(&owner).await;
    let result = super::handle(entry.id(), Some(&owner), true).await.unwrap();
    assert!(result.success);
    assert!(store::get(entry.id()).unwrap().session.messages.is_empty());
    support::cleanup(entry.id());
}
