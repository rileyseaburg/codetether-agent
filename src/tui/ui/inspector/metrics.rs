use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::state::AppState;

pub fn render_metrics(f: &mut ratatui::Frame, state: &AppState, area: Rect) {
    let label = Style::default().fg(Color::DarkGray);
    let mut lines = Vec::new();
    lines.push(Line::styled("Metrics", Style::default().add_modifier(Modifier::BOLD)));

    let model = state.last_completion_model.as_deref().unwrap_or("auto");
    lines.push(metric_line("Model:", model, Color::Green, label));

    let prompt_t = fmt_opt(state.last_completion_prompt_tokens.map(|t| t.to_string()));
    lines.push(metric_line("Prompt tokens:", &prompt_t, Color::White, label));

    let output_t = fmt_opt(state.last_completion_output_tokens.map(|t| t.to_string()));
    lines.push(metric_line("Output tokens:", &output_t, Color::White, label));

    let latency = fmt_opt(state.last_completion_latency_ms.map(|ms| format!("{ms}ms")));
    lines.push(metric_line("Latency:", &latency, Color::Cyan, label));

    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Token & Cost ")),
        area,
    );
}

fn metric_line(key: &str, val: &str, color: Color, label: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key} "), label),
        Span::styled(val.to_string(), Style::default().fg(color)),
    ])
}

fn fmt_opt(opt: Option<String>) -> String {
    opt.unwrap_or_else(|| "—".into())
}
