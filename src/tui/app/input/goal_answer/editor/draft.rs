//! Compare semantic goal fields without treating live usage accounting as an edit.

use crate::session::tasks::{Goal, GoalEdit, GoalEditAction, TaskState};
use anyhow::{Result, bail};

/// Original goal shown to the user, used to avoid overwriting concurrent edits.
pub(crate) struct Draft {
    pub(super) session_id: String,
    pub(super) original: Goal,
}

impl Draft {
    pub(super) fn request(&self, state: &TaskState, text: String) -> Result<GoalEdit> {
        let Some(goal) = &state.goal else {
            bail!("Goal was cleared; draft retained. Esc to close.");
        };
        let base = &self.original;
        if goal.id != base.id
            || goal.objective != base.objective
            || goal.success_criteria != base.success_criteria
            || goal.forbidden != base.forbidden
            || goal.token_budget != base.token_budget
            || goal.status != base.status
        {
            bail!("Goal changed while editing; draft retained. Esc, then /goal edit to reload.");
        }
        Ok(GoalEdit {
            goal_id: goal.id.clone(),
            updated_at: goal.last_updated_at,
            action: GoalEditAction::Edit,
            objective: Some(text),
            success_criteria: None,
            forbidden: None,
            token_budget: None,
        })
    }
}
