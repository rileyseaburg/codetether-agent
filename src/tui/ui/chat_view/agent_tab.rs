//! Formatting for one spawned-agent header tab.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

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
    Span::styled(label, style(meta.selected))
}

fn style(selected: bool) -> Style {
    if selected {
        return Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(Color::Gray)
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
