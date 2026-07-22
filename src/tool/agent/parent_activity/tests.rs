use super::{Activity, mailbox, registry, steered, until};
use std::time::Duration;

#[tokio::test]
async fn activity_published_before_wait_is_not_lost() {
    let owner = "parent-activity-queued";
    registry::clear(owner);
    mailbox(owner);
    let activity = until(owner, tokio::time::Instant::now()).await;
    assert_eq!(activity, Some(Activity::Mailbox));
    registry::clear(owner);
}

#[tokio::test]
async fn steered_input_wakes_an_active_wait() {
    let owner = "parent-activity-steered";
    registry::clear(owner);
    let waiter = tokio::spawn(until(
        owner,
        tokio::time::Instant::now() + Duration::from_secs(1),
    ));
    tokio::task::yield_now().await;
    steered(owner, 7);
    assert_eq!(waiter.await.unwrap(), Some(Activity::Steered(7)));
    registry::clear(owner);
}

#[tokio::test]
async fn wait_reports_timeout_without_activity() {
    let owner = "parent-activity-timeout";
    registry::clear(owner);
    let activity = until(owner, tokio::time::Instant::now()).await;
    assert_eq!(activity, None);
    registry::clear(owner);
}
