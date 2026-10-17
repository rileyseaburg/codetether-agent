//! Owner-scoped tool activity as fallback child liveness.
use crate::status_source::AgentStatus;
use crate::{status_tool_filter, tool_events::tool_activity, tool_merge::merge};
use std::collections::HashMap;

/// Insert a `Working` status for agents with tool activity but no `TaskUpdate`.
/// Add recent tool activity for children owned by the current parent.
pub(super) fn backfill<'a>(
    out: &mut HashMap<String, AgentStatus>,
    allowed: impl Iterator<Item = &'a String>,
) {
    merge(out, status_tool_filter::owned(tool_activity(), allowed));
}
#[cfg(test)]
#[path = "status_tool_activity_tests.rs"]
mod tests;
