//! Mocked local answer delivery cannot release or repeatedly answer a hold.

use crate::session::tasks::runtime::answer_review;
#[path = "answer_test_provider.rs"]
mod provider;
#[path = "answer_test_reply.rs"]
mod reply;
#[path = "answer_test_setup.rs"]
mod setup;

#[tokio::test]
async fn answer_review_delivery_waits_for_goal_and_failure_keeps_hold() {
    for fail in [false, true] {
        let (mut app, slot, registry) = setup::setup(fail).await;
        let history_len = slot.borrow().unwrap().messages.len();
        app.state.processing = true;
        assert!(!super::deliver(&mut app, &slot, &registry).await);
        assert!(!answer_review::ready(slot.view().id()));
        app.state.processing = false;
        assert!(super::deliver(&mut app, &slot, &registry).await);
        assert!(answer_review::held(slot.view().id()));
        assert_eq!(answer_review::ready(slot.view().id()), !fail);
        assert!(!super::deliver(&mut app, &slot, &registry).await);
        let session = slot.borrow().unwrap();
        assert_eq!(session.messages.len(), history_len);
    }
}
