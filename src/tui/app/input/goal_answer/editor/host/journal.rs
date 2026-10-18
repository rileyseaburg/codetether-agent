//! Preserve native human-only goal journal reads and updates.
use crate::session::tasks::{control, runtime::answer_review};
pub(crate) struct Native;
impl codetether_goal_editor::Journal for Native {
    fn read(id: &str) -> anyhow::Result<codetether_goal_core::TaskState> {
        answer_review::read(id)
    }
    async fn update(id: &str, edit: codetether_goal_core::GoalEdit) -> anyhow::Result<()> {
        control::update_user(id, edit).await?;
        Ok(())
    }
}
