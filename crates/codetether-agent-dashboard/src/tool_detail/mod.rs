//! Detail-row assembly for children spawned through the agent tool.
//! [Host] supplies parent-scoped registry reads; [lines] combines identity,
//! transcript and live-trace rows for one child.

use ratatui::style::Stylize;
use ratatui::text::Line;

mod host;
mod registry;
pub use host::Host;
pub use registry::Registry;

/// Assemble one tool child's parent-scoped details.
/// # Arguments
/// * `host` - Existing owner-scoped registry reads.
/// * `parent` - Current parent session identifier, when available.
/// * `name` - Requested child name.
/// # Returns
/// Identity, transcript and live rows, or a missing-child hint.
/// # Examples
/// ```no_run
/// fn detail(host: &impl codetether_agent_dashboard::tool_detail::Host) {
///     let rows = codetether_agent_dashboard::tool_detail::lines(host, Some("parent"), "reviewer");
///     assert!(!rows.is_empty());
/// }
/// ```
pub fn lines(host: &impl Host, parent: Option<&str>, name: &str) -> Vec<Line<'static>> {
    let Some(parent_id) = parent else {
        return missing(name);
    };
    let Some(agent) = host.find(name, parent_id) else {
        return missing(name);
    };
    let messages = host.transcript(name, parent_id).unwrap_or_default();
    let status = match (agent.is_processing, agent.failed) {
        (true, _) => "working",
        (false, true) => "failed",
        (false, false) => "idle",
    };
    let remote = agent.origin.is_remote();
    let mut rows =
        crate::metadata::lines_for_origin(name, status, &agent.origin, &agent.instructions);
    if remote && agent.is_processing {
        rows.push(Line::from(
            "A2A request in flight; polling peer for completion…".yellow(),
        ));
        rows.push(Line::from(""));
    }
    let source = crate::messages::Source::for_remote(remote);
    crate::messages::append(&mut rows, &messages, source);
    if let Some(trace) = host.trace(name, parent_id) {
        crate::live::append(&mut rows, &trace);
    }
    rows
}

fn missing(name: &str) -> Vec<Line<'static>> {
    vec![Line::from(
        format!("Agent @{name} is no longer available.").red(),
    )]
}
