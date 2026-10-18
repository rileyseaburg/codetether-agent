//! Identity and navigation header rows for agent detail panes.

use ratatui::style::Stylize;
use ratatui::text::Line;

use codetether_agent_view::AgentOrigin;

const NAV: &str = "Tab: next child · Esc: dashboard · ↑↓/PgUp/PgDn: scroll";

/// Header for a `/spawn`-managed child, which always has a parent and model.
/// Build identity and navigation rows for a managed child.
/// # Arguments
/// * `name` - Child name.
/// * `parent` - Parent name.
/// * `status` - Current activity label.
/// * `model` - Model label.
/// * `mission` - Child instructions.
/// # Returns
/// Owned header rows.
/// # Examples
/// ```
/// assert!(!codetether_agent_dashboard::metadata::lines("child", "main", "idle", "auto", "review").is_empty());
/// ```
pub fn lines(
    name: &str,
    parent: &str,
    status: &str,
    model: &str,
    mission: &str,
) -> Vec<Line<'static>> {
    header(
        name,
        status,
        format!("parent: {parent} · model: {model}"),
        mission,
    )
}

/// Header for an agent-tool child or LAN peer, described by its origin.
/// Build identity rows for a tool child or remote peer.
/// # Arguments
/// * `name` - Child or peer name.
/// * `status` - Activity label.
/// * `origin` - Local or remote provenance.
/// * `mission` - Child instructions.
/// # Returns
/// Owned identity and navigation rows.
/// # Examples
/// ```
/// let origin = codetether_agent_view::AgentOrigin::lan_peer();
/// assert!(!codetether_agent_dashboard::metadata::lines_for_origin("peer", "idle", &origin, "review").is_empty());
/// ```
pub fn lines_for_origin(
    name: &str,
    status: &str,
    origin: &AgentOrigin,
    mission: &str,
) -> Vec<Line<'static>> {
    let identity = match origin {
        AgentOrigin::Local { parent, model_id } => format!(
            "parent: {} · model: {}",
            parent.as_deref().unwrap_or("main"),
            model_id.as_deref().unwrap_or("default model")
        ),
        AgentOrigin::LanPeer { transport } => format!("transport: {transport}"),
    };
    header(name, status, identity, mission)
}

fn header(name: &str, status: &str, identity: String, mission: &str) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("@{name} · {status}").cyan().bold()),
        Line::from(identity.dim()),
        Line::from(format!("mission: {mission}")),
        Line::from(NAV.dim()),
        Line::from(""),
    ]
}
