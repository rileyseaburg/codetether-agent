//! Apply explicit human goal edits through the native controller, not model tools.
use crate::Host;

use anyhow::{Result, anyhow};
use codetether_goal_core::{GoalEdit, GoalEditAction};

pub(super) async fn run<H: Host>(
    session_id: &str,
    action: GoalEditAction,
    objective: Option<String>,
    token_budget: Option<Option<i64>>,
) -> Result<String> {
    let goal = H::goal(session_id)?.ok_or_else(|| anyhow!("no goal exists"))?;
    let state = H::update_user(
        session_id,
        GoalEdit {
            goal_id: goal.id,
            updated_at: goal.last_updated_at,
            action,
            objective,
            token_budget,
            success_criteria: None,
            forbidden: None,
        },
    )
    .await?;
    let goal = &state["goal"];
    Ok(format!(
        "Session goal updated ({}) — {}. Session tasks are unchanged.",
        goal["status"].as_str().unwrap_or("cleared"),
        goal["objective"].as_str().unwrap_or("no goal")
    ))
}
