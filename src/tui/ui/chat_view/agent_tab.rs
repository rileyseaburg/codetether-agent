//! Formatting for one spawned-agent header tab.

use ratatui::{style::Style, text::Span};

#[path = "agent_tab_style.rs"]
pub mod agent_tab_style;
use agent_tab_style::tab_style;

/// Data needed to render a compact agent tab.
pub struct AgentTabMeta<'a> {
    pub name: &'a str,
    pub model_id: Option<&'a str>,
    pub session_id: Option<&'a str>,
    pub indent: u8,
    pub selected: bool,
    pub processing: bool,
}

/// Build a single tab span with name, model id, session id, and busy marker.
///
/// Each agent gets a stable identity color derived from its name: selected
/// tabs use it as the background, unselected tabs tint the label with it.
pub fn agent_tab(meta: AgentTabMeta<'_>) -> Span<'static> {
    let dots = "·".repeat(meta.indent as usize);
    let model = meta
        .model_id
        .map(compact_model)
        .unwrap_or_else(|| "auto".into());
    let session = meta
        .session_id
        .map(short_id)
        .unwrap_or_else(|| "new".into());
    let marker = if meta.processing { " ⋯" } else { "" };
    let label = format!(" {dots}{} {model} {session}{marker} ", meta.name);
    Span::styled(label, tab_style(meta.name, meta.selected))
}

fn short_id(value: &str) -> String {
    value.chars().take(8).collect()
}

fn compact_model(value: &str) -> String {
    let model = value.rsplit('/').next().unwrap_or(value);
    let mut chars = model.chars();
    let short: String = chars.by_ref().take(18).collect();
    if chars.next().is_some() {
        format!("{short}…")
    } else {
        short
    }
}
