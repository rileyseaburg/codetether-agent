//! Human continuation releases holds without synthesizing satisfaction or approval.
use crate::session::tasks::{AnswerReviewAction as Action, GoalStatus, runtime::answer_review};
#[path = "commands/tests/fixture.rs"]
mod fixture;

#[tokio::test]
async fn answer_review_explicit_continuation_releases_each_hold_stage() {
    let cases = [
        ("continue", None),
        ("coninue", Some(Action::Unsatisfied)),
        ("/continue", Some(Action::Answered)),
        ("resume", Some(Action::Answered)),
    ];
    for (input, action) in cases {
        let (mut app, mut slot, runtime) = fixture::setup().await;
        let id = slot.view().id().to_string();
        answer_review::begin(&id, "Why?").await.unwrap();
        let review = answer_review::read(&id).unwrap().answer_review.unwrap();
        if let Some(action) = action {
            answer_review::record(&id, &review.goal_id, &review.id, action)
                .await
                .unwrap();
        }
        app.state.approval_waiting = true;
        fixture::submit(&mut app, &mut slot, &runtime, input).await;
        let state = answer_review::read(&id).unwrap();
        assert!(state.answer_review.is_none());
        assert_eq!(state.goal.unwrap().status, GoalStatus::Active);
        assert!(app.state.approval_waiting);
        assert!(app.state.input.is_empty());
        assert!(!app.state.answer_review_yes);
        runtime.shutdown().await;
    }
}

#[test]
fn answer_review_continuation_requires_a_whole_explicit_command() {
    for input in [" continue ", "CONTINUE", "coninue", "resume", "/continue"] {
        assert!(super::requested(input));
    }
    for input in [
        "",
        "Why?",
        "do not continue",
        "continue?",
        "/ask continue",
        "/continue later",
    ] {
        assert!(!super::requested(input));
    }
}
