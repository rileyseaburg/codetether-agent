//! Build normalized jobs with application-enriched execution arguments.
use crate::{Host, eligibility::is_eligible, job::Job};
use codetether_edit_policy::normalize_tool_call_for_execution;

pub(super) fn prepare(
    calls: &[(String, String, serde_json::Value)],
    session: &impl Host,
) -> Option<Vec<Job>> {
    if calls.len() < 2 {
        return None;
    }
    let mut jobs = Vec::with_capacity(calls.len());
    for (tool_id, raw_name, raw_input) in calls {
        if !is_eligible(raw_name, raw_input) {
            return None;
        }
        let (tool_name, tool_input) = normalize_tool_call_for_execution(raw_name, raw_input);
        let exec_input = session.enrich(&tool_input);
        jobs.push(Job {
            tool_id: tool_id.clone(),
            tool_name,
            tool_input,
            exec_input,
        });
    }
    Some(jobs)
}
