//! enter for the planning tools.
use super::*;
pub(super) fn execute(params: Value) -> Result<ToolResult> {
    let p: EnterParams = serde_json::from_value(params).context("Invalid params")?;

    if IN_PLAN_MODE.load(Ordering::SeqCst) {
        return Ok(ToolResult::error(
            "Already in plan mode. Exit current plan first.",
        ));
    }

    if p.steps.is_empty() {
        return Ok(ToolResult::error("At least one step is required"));
    }

    let plan = Plan {
        goal: p.goal.clone(),
        steps: p
            .steps
            .iter()
            .map(|s| PlanStep {
                description: s.clone(),
                completed: false,
                notes: None,
            })
            .collect(),
        current_step: 0,
    };

    *CURRENT_PLAN.write() = Some(plan.clone());
    IN_PLAN_MODE.store(true, Ordering::SeqCst);

    let output = enter_report::render(&p);

    Ok(ToolResult::success(output)
        .with_metadata("step_count", json!(p.steps.len()))
        .with_metadata("current_step", json!(1)))
}
