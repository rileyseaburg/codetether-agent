//! Model picker footer presentation.
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub(super) fn render(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" MODEL ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Nav "),
        Span::styled("Type", Style::default().fg(Color::Yellow)),
        Span::raw(": Filter "),
        Span::styled("Backspace", Style::default().fg(Color::Yellow)),
        Span::raw(": Edit "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": Apply "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Cancel | "),
        Span::raw("Codex/Bedrock effort: configure in /settings").dim(),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Controls "));
    f.render_widget(footer, area);
}
