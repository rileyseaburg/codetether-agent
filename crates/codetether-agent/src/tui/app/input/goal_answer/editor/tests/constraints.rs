//! Nonempty goal fields ensure text editing does not reset governance settings.
use crate::session::tasks::{GoalEdit, GoalEditAction, control, runtime::answer_review};

pub(super) async fn set(id: &str) {
    let goal = answer_review::read(id).unwrap().goal.unwrap();
    control::update_user(
        id,
        GoalEdit {
            goal_id: goal.id,
            updated_at: goal.last_updated_at,
            action: GoalEditAction::Edit,
            objective: None,
            success_criteria: Some(vec!["Keep API compatibility".into()]),
            forbidden: Some(vec!["No deployment".into()]),
            token_budget: Some(Some(1000)),
        },
    )
    .await
    .unwrap();
}
