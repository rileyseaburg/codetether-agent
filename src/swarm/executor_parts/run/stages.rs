//! Ordered traversal of dependency stages.

use super::state::Run;
use anyhow::Result;

pub(super) async fn execute(run: &mut Run<'_>) -> Result<()> {
    let maximum = run
        .subtasks
        .iter()
        .map(|task| task.stage)
        .max()
        .unwrap_or(0);
    for stage in 0..=maximum {
        if !super::control::may_continue(run).await {
            break;
        }
        super::stage::execute(run, stage).await?;
    }
    Ok(())
}
