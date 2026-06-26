//! Rendering for the spawn form modal overlay.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::state::AppState;
use crate::tui::app::spawn_form::state::{SpawnField, SpawnFormState};
use crate::tui::help::centered_rect;

/// Render the spawn form if it is active.
pub fn render_spawn_form_if_needed(f: &mut Frame, app_state: &mut AppState) {
    let Some(ref form) = app_state.spawn_form else {
        return;
    };
    let area = centered_rect(50, 40, f.area());
    f.render_widget(Clear, area);
    let form = form.clone();
    let widget = build_widget(&form);
    f.render_widget(widget, area);
}

fn build_widget(form: &SpawnFormState) -> Paragraph<'static> {
    let area_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1); 5])
        .split(Rect::ZERO);
    let _ = area_split; // layout used only for constraint count documentation

    let lines = vec![
        Line::from(Span::styled(
            "  Tab: next field  ·  Enter: spawn  ·  Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
        field_row("Name", &form.name, form.active == SpawnField::Name),
        field_row(
            "Parent",
            &form.parent,
            form.active == SpawnField::Parent,
        ),
        field_row(
            "Instructions",
            &form.instructions,
            form.active == SpawnField::Instructions,
        ),
        Line::from(""),
    ];
    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Spawn Agent ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
}

fn field_row(label: &str, value: &str, active: bool) -> Line<'static> {
    let style = if active {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let cursor = if active { "█" } else { "" };
    Line::from(vec![
        Span::styled(
            format!("  {label:<14}: "),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(format!("{value}{cursor}"), style),
    ])
}
