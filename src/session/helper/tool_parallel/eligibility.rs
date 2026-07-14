//! Read-only batch eligibility.

use std::path::Path;

use crate::session::Session;
use crate::session::helper::edit::{detect_stub_in_tool_input, normalize_tool_call_for_execution};
use crate::session::helper::runtime::{enrich_tool_input_for_session, is_interactive_tool};

use super::job::Job;

pub(super) fn prepare(
    calls: &[(String, String, serde_json::Value)],
    cwd: &Path,
    session: &Session,
) -> Option<Vec<Job>> {
    if calls.len() < 2 {
        return None;
    }
    let mut jobs = Vec::with_capacity(calls.len());
    for (tool_id, raw_name, raw_input) in calls {
        let (tool_name, tool_input) = normalize_tool_call_for_execution(raw_name, raw_input);
        if !crate::tool::readonly::is_read_only(&tool_name)
            || is_interactive_tool(&tool_name)
            || detect_stub_in_tool_input(&tool_name, &tool_input).is_some()
        {
            return None;
        }
        let exec_input = enrich_tool_input_for_session(&tool_input, cwd, session);
        jobs.push(Job {
            tool_id: tool_id.clone(),
            tool_name,
            tool_input,
            exec_input,
        });
    }
    Some(jobs)
}
