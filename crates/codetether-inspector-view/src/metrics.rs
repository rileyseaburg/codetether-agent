//! Completion model, token counts, and latency panel.
use crate::metric_line::{fmt_opt, metric_line};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

/// Render completion metrics in a supplied rectangle.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `state` - Completion values.
/// * `area` - Panel rectangle.
///
/// # Returns
/// Draws model, token counts and latency.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let view = codetether_inspector_view::InspectorView::default();
/// let area = f.area();
/// codetether_inspector_view::render_metrics(f, &view, area);
/// # }
/// ```
pub fn render_metrics(f: &mut ratatui::Frame, state: &crate::InspectorView<'_>, area: Rect) {
    let label = Style::default().fg(Color::DarkGray);
    let mut lines = Vec::new();
    lines.push(Line::styled(
        "Metrics",
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let model = state.last_completion_model.as_deref().unwrap_or("auto");
    lines.push(metric_line("Model:", model, Color::Green, label));

    let prompt_t = fmt_opt(state.last_completion_prompt_tokens.map(|t| t.to_string()));
    lines.push(metric_line(
        "Prompt tokens:",
        &prompt_t,
        Color::White,
        label,
    ));

    let output_t = fmt_opt(state.last_completion_output_tokens.map(|t| t.to_string()));
    lines.push(metric_line(
        "Output tokens:",
        &output_t,
        Color::White,
        label,
    ));

    let latency = fmt_opt(state.last_completion_latency_ms.map(|ms| format!("{ms}ms")));
    lines.push(metric_line("Latency:", &latency, Color::Cyan, label));

    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Token & Cost "),
        ),
        area,
    );
}
