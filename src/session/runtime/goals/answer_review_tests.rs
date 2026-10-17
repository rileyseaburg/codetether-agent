//! Durable gate APIs must not offer tools a way to release a user hold.

use super::*;
use crate::session::tasks::answer_review_test_support::session;
use crate::session::tasks::{AnswerReviewAction as Action, GoalStatus};

async fn decide(id: &str, action: Action) {
    let review = read(id).unwrap().answer_review.unwrap();
    record(id, &review.goal_id, &review.id, action)
        .await
        .unwrap();
}

#[tokio::test]
async fn answer_review_public_status_updates_cannot_resume() {
    let session = session().await;
    assert!(begin(&session.id, "Why?").await.unwrap());
    assert!(held(&session.id));
    assert!(!ready(&session.id));
    assert!(
        !super::super::set_status(&session.id, GoalStatus::Active)
            .await
            .unwrap()
    );
    decide(&session.id, Action::Answered).await;
    assert!(ready(&session.id));
    decide(&session.id, Action::Unsatisfied).await;
    assert!(held(&session.id));
    assert!(!ready(&session.id));
    assert!(begin(&session.id, "More detail?").await.unwrap());
    decide(&session.id, Action::Answered).await;
    decide(&session.id, Action::Satisfied).await;
    assert!(!held(&session.id));
}

#[tokio::test]
async fn answer_review_absent_goal_uses_normal_input_path() {
    let session = crate::session::Session::new().await.unwrap();
    assert!(!begin(&session.id, "Why?").await.unwrap());
    assert!(!held(&session.id));
}
