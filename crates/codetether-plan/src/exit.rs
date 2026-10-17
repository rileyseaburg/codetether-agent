//! exit for the planning tools.
use super::*;
pub(super) fn execute(params: Value) -> Result<ToolResult> {
    let p: ExitParams = serde_json::from_value(params).unwrap_or(ExitParams {
        summary: None,
        step_complete: None,
        notes: None,
    });

    if !IN_PLAN_MODE.load(Ordering::SeqCst) {
        return Ok(ToolResult::error("Not in plan mode"));
    }

    let (output, completed_count, total_steps, should_exit) = snapshot::snapshot(&p)?;

    // If all steps complete or explicit exit, leave plan mode
    if should_exit {
        IN_PLAN_MODE.store(false, Ordering::SeqCst);
        *CURRENT_PLAN.write() = None;
    }

    Ok(ToolResult::success(output)
        .with_metadata("completed", json!(completed_count))
        .with_metadata("total", json!(total_steps)))
}
