//! Identity and navigation header rows for agent detail panes.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::tool::agent::bridge::AgentOrigin;

const NAV: &str = "Tab: next child · Esc: dashboard · ↑↓/PgUp/PgDn: scroll";

/// Header for a `/spawn`-managed child, which always has a parent and model.
pub(super) fn lines(
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
pub(super) fn lines_for_origin(
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
