//! Native review persistence and UI bindings for shared interaction policy.
use super::{App, SessionSlot};
use crate::session::tasks::{AnswerReview, AnswerReviewAction, runtime::answer_review};
use anyhow::Result;

pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a SessionSlot,
}
impl codetether_answer_review_view::interactive::Host for Native<'_> {
    fn read_review(&self) -> Result<Option<AnswerReview>> {
        Ok(answer_review::read(self.slot.view().id())?.answer_review)
    }
    fn input(&self) -> &str {
        &self.app.state.composer.input
    }
    fn selected_yes(&mut self) -> &mut bool {
        &mut self.app.state.answer_review_yes
    }
    async fn record(&mut self, review: &AnswerReview, action: AnswerReviewAction) -> Result<()> {
        answer_review::record(self.slot.view().id(), &review.goal_id, &review.id, action).await
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
    fn redraw(&mut self) {
        self.app.state.needs_redraw = true;
    }
}
