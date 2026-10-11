//! Only deliberate override bypasses a stale revision, never a replaced identity.

use super::{fixture, user_fixture};
use chrono::{Duration, Utc};

#[tokio::test]
async fn session_goal_user_override_rejects_replacement_not_stale_revision() {
    let (_dir, log) = user_fixture::held().await;
    let before = user_fixture::state(&log).await;
    let mut request = fixture::request(&before, "override");
    request.updated_at = Utc::now() - Duration::days(1);
    request.goal_id = "other-goal".into();
    assert!(
        super::super::edit::apply_user(&log, request.clone())
            .await
            .is_err()
    );
    request.goal_id = "goal".into();
    assert!(super::super::edit::apply_user(&log, request).await.is_ok());
}

#[tokio::test]
async fn session_goal_user_edit_keeps_review_and_rejects_stale_revision() {
    let (_dir, log) = user_fixture::held().await;
    let before = user_fixture::state(&log).await;
    let mut request = fixture::request(&before, "edit");
    request.objective = Some("Updated while answering".into());
    assert!(
        super::super::edit::apply(&log, request.clone())
            .await
            .is_err()
    );
    let after = super::super::edit::apply_user(&log, request.clone())
        .await
        .unwrap();
    assert!(after.answer_review.is_some());
    assert_eq!(after.goal.unwrap().objective, "Updated while answering");
    assert!(super::super::edit::apply_user(&log, request).await.is_err());
}
