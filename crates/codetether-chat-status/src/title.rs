//! Neon chat-panel title assembly.
use codetether_tui_style::spinner::spinner_color;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Build the neon title and lazily resolve model/session labels.
///
/// # Arguments
/// * `model_label` - Called when the model label is rendered.
/// * `session_label` - Called when the session label is rendered.
///
/// # Returns
/// Owned title spans.
///
/// # Examples
/// ```
/// let _title = codetether_chat_status::build_title(|| "auto".into(), || "new".into());
/// ```
pub fn build_title(
    model_label: impl FnOnce() -> String,
    session_label: impl FnOnce() -> String,
) -> Line<'static> {
    let neon = spinner_color();
    let mut spans = vec![Span::styled(
        " ⬡ ",
        Style::default().fg(neon).add_modifier(Modifier::BOLD),
    )];
    spans.extend(crate::title_brand::brand_spans());
    spans.extend([
        Span::styled(" ▸ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("model:{}", model_label()),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("  session:", Style::default().fg(Color::DarkGray)),
        Span::styled(session_label(), Style::default().fg(Color::Yellow)),
        Span::raw(" "),
    ]);
    Line::from(spans)
}
