//! Compare semantic goal fields without treating live usage accounting as an edit.

use anyhow::{Result, bail};
use codetether_goal_core::{Goal, GoalEdit, GoalEditAction, TaskState};

/// Original goal shown to the user, used to avoid overwriting concurrent edits.
/// # Examples
/// ```no_run
/// fn draft(goal: codetether_goal_core::Goal) -> codetether_goal_editor::Draft {
///     codetether_goal_editor::Draft::new("session".into(), goal)
/// }
/// ```
pub struct Draft {
    pub(crate) session_id: String,
    pub(crate) original: Goal,
}

impl Draft {
    /// Retain the session identity and original goal for conflict detection.
    pub fn new(session_id: String, original: Goal) -> Self {
        Self {
            session_id,
            original,
        }
    }

    /// Construct an edit only when semantic goal fields match the original.
    /// # Arguments
    /// Receives current journal state and edited text.
    /// # Returns
    /// A goal edit retaining criteria, budget and forbidden behaviors.
    /// # Errors
    /// Returns a conflict when the goal was cleared or semantically changed.
    /// # Examples
    /// ```no_run
    /// fn edit(draft: &codetether_goal_editor::Draft, state: &codetether_goal_core::TaskState) {
    ///     let _ = draft.request(state, "new objective".into());
    /// }
    /// ```
    pub fn request(&self, state: &TaskState, text: String) -> Result<GoalEdit> {
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
