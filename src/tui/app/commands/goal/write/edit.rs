//! `/goal edit` persistence.

use crate::session::tasks::GoalEditAction;
use anyhow::{Result, anyhow};

pub(super) async fn run(session_id: &str, objective: &str) -> Result<String> {
    if objective.is_empty() {
        return Err(anyhow!("usage: /goal edit <objective>"));
    }
    super::change(
        session_id,
        GoalEditAction::Edit,
        Some(objective.into()),
        None,
    )
    .await
}
