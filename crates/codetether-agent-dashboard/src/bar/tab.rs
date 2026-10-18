//! Formatting for one spawned-agent header tab.

use ratatui::text::Span;

use super::style::tab_style;

/// Data needed to render a compact agent tab.
/// # Examples
/// ```
/// let tab = codetether_agent_dashboard::bar::tab::AgentTabMeta {
///     name: "main", model_id: None, session_id: None, identity: None,
///     indent: 0, selected: true, processing: false,
/// };
/// assert_eq!(tab.name, "main");
/// ```
pub struct AgentTabMeta<'a> {
    pub name: &'a str,
    /// Model label; `None` renders as `auto`.
    pub model_id: Option<&'a str>,
    /// Session id, shortened; `None` renders as `new`.
    pub session_id: Option<&'a str>,
    /// When set, replaces the model/session pair (LAN peers have neither).
    pub identity: Option<&'a str>,
    pub indent: u8,
    pub selected: bool,
    pub processing: bool,
}

/// Build a single tab span with name, model id, session id, and busy marker.
///
/// Each agent gets a stable identity color derived from its name: selected
/// tabs use it as the background, unselected tabs tint the label with it.
/// # Arguments
/// Receives tab identity, display labels, indentation and activity flags.
/// # Returns
/// An owned span with identity color and a compact label.
/// # Examples
/// ```
/// let span = codetether_agent_dashboard::bar::tab::agent_tab(codetether_agent_dashboard::bar::tab::AgentTabMeta {
///     name: "main", model_id: None, session_id: None, identity: None,
///     indent: 0, selected: true, processing: false,
/// });
/// assert!(span.content.contains("main auto new"));
/// ```
pub fn agent_tab(meta: AgentTabMeta<'_>) -> Span<'static> {
    let dots = "·".repeat(meta.indent as usize);
    let identity = match meta.identity {
        Some(identity) => identity.to_string(),
        None => {
            let model = meta
                .model_id
                .map(compact_model)
                .unwrap_or_else(|| "auto".into());
            let session = meta
                .session_id
                .map(short_id)
                .unwrap_or_else(|| "new".into());
            format!("{model} {session}")
        }
    };
    let marker = if meta.processing { " ⋯" } else { "" };
    let label = format!(" {dots}{} {identity}{marker} ", meta.name);
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
