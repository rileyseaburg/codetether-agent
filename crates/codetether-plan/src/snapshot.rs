//! snapshot for the planning tools.
use super::*;
pub(super) fn snapshot(p: &ExitParams) -> Result<(String, usize, usize, bool)> {
    let mut plan_guard = CURRENT_PLAN.write();
    let plan = plan_guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("No active plan"))?;

    // Mark step complete if specified
    if let Some(step_num) = p.step_complete
        && step_num > 0
        && step_num <= plan.steps.len()
    {
        let step = &mut plan.steps[step_num - 1];
        step.completed = true;
        step.notes = p.notes.clone();
        plan.current_step = step_num;
    }

    // Build status report
    let completed_count = plan.steps.iter().filter(|s| s.completed).count();
    let total_steps = plan.steps.len();
    let status = step_report::render(plan);

    let output = format!(
        "📋 Plan Status\n\nGoal: {}\n\nProgress: {}/{} steps\n\n{}\n\n{}",
        plan.goal,
        completed_count,
        total_steps,
        status,
        p.summary
            .as_ref()
            .map(|s| format!("Summary: {}", s))
            .unwrap_or_default()
    );

    let should_exit = completed_count == total_steps || p.summary.is_some();
    Ok((output, completed_count, total_steps, should_exit))
}
