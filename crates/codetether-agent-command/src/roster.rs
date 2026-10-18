//! Legacy and parent-scoped managed-agent roster formatting.
use crate::{Listing, State};
/// A legacy child row projected without retaining its session.
/// # Examples
/// ```
/// let row = codetether_agent_command::roster::LegacyAgent {
///     key: "worker".into(), name: "worker".into(), message_count: 0, model: None };
/// assert_eq!(row.message_count, 0);
/// ```
pub struct LegacyAgent {
    /// Registry key used for focus lookup.
    pub key: String,
    /// Display name.
    pub name: String,
    /// Number of history messages.
    pub message_count: usize,
    /// Configured model, or none for the default model.
    pub model: Option<String>,
}
/// Build display lines combining legacy and managed registries.
/// # Arguments
/// `app` provides focus and ordered, parent-scoped agent snapshots.
/// # Returns
/// Legacy rows followed by managed rows whose names are absent from legacy rows.
/// # Examples
/// ```no_run
/// fn rows<H: codetether_agent_command::Host>(host: &H) -> Vec<String> {
///     codetether_agent_command::roster::build_agent_list(host)
/// }
/// ```
pub fn build_agent_list<H: Listing + State>(app: &H) -> Vec<String> {
    let mut lines = Vec::new();
    for agent in app.legacy_rows() {
        let msg_count = agent.message_count;
        let model = agent.model.as_deref().unwrap_or("default");
        let active = if app.active() == Some(agent.key.as_str()) {
            " [active]"
        } else {
            ""
        };
        lines.push(format!(
            "  {}{} — {} messages — model: {}",
            agent.name, active, msg_count, model
        ));
    }
    let snapshots = app.managed_rows();
    let existing: std::collections::HashSet<String> =
        lines.iter().filter_map(|l| name_from_line(l)).collect();
    for snap in snapshots {
        if existing.contains(&snap.name) {
            continue;
        }
        lines.push(format!(
            "  {} — {} messages — {} ({})",
            snap.name,
            snap.message_count,
            snap.origin.lineage(),
            snap.origin.kind()
        ));
    }
    lines
}
fn name_from_line(line: &str) -> Option<String> {
    line.trim_start()
        .split_whitespace()
        .next()
        .map(str::to_string)
}
