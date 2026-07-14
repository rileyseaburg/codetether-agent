//! Required final status contract for each delegated deliverable.

pub(super) fn render() -> &'static str {
    "DELIVERABLE STATUS CONTRACT:\n- Treat this delegated task as a deliverable independent of agent attempts or implementation methods.\n- End the summary with exactly `STATUS: completed`, `STATUS: blocked`, or `STATUS: pending`.\n- Completed requires a concrete evidence line.\n- Blocked requires concrete blocker evidence, not merely a failed agent/model/tool attempt.\n- Pending requires the next action that should continue the deliverable.\n- Never report a failed implementation method as task completion."
}
