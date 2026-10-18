//! Subsequent provider calls must contain steering or the unchanged goal.

use super::{Mock, Scenario};
use crate::{
    provider::{CompletionRequest, ContentPart},
    session::{runtime::goals, tasks::GoalStatus},
};
use std::sync::atomic::Ordering;

impl Mock {
    pub(super) async fn observe(&self, request: &CompletionRequest) -> anyhow::Result<()> {
        let expected = if matches!(self.scenario, Scenario::Goal) {
            "Continue working toward the active thread goal."
        } else {
            "use the cloned repos"
        };
        self.saw_input.store(
            request
                .messages
                .iter()
                .flat_map(|m| &m.content)
                .any(|p| matches!(p, ContentPart::Text {text} if text.contains(expected))),
            Ordering::SeqCst,
        );
        if matches!(self.scenario, Scenario::Goal) {
            goals::set_status(&self.id, GoalStatus::Complete).await?;
        }
        Ok(())
    }
}
