use crate::session::SessionEvent;
use tokio::sync::mpsc;

#[tokio::test]
async fn background_notifier_does_not_hold_event_channel_open() {
    let (tx, mut rx) = mpsc::channel::<SessionEvent>(1);
    let notify = super::notifier(&tx);
    drop(tx);
    assert!(rx.recv().await.is_none());
    drop(notify);
}
