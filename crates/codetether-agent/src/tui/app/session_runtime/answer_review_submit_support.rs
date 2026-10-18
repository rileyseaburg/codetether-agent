//! Fixtures for a held runtime handoff and explicit answer acceptance.

use crate::provider::ProviderRegistry;
use crate::session::{
    Session,
    tasks::{AnswerReviewAction, runtime::answer_review},
};
use crate::tui::app::session_runtime::PromptRequest;
use std::sync::Arc;

pub(super) fn request(session: Session) -> PromptRequest {
    PromptRequest::new(
        session,
        "continue".into(),
        Vec::new(),
        Arc::new(ProviderRegistry::new()),
        None,
        None,
    )
}

pub(super) async fn accept(session_id: &str) {
    let review = answer_review::read(session_id)
        .unwrap()
        .answer_review
        .unwrap();
    for decision in [AnswerReviewAction::Answered, AnswerReviewAction::Satisfied] {
        answer_review::record(session_id, &review.goal_id, &review.id, decision)
            .await
            .unwrap();
    }
    assert!(!answer_review::held(session_id));
    assert!(crate::session::tasks::runtime::resume_prompt(session_id).is_some());
}
