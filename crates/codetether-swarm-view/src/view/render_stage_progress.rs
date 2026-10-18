//! Swarm render stage progress presentation.
use crate::SwarmViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Gauge};
pub(super) fn render_stage_progress(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let progress = state.progress();
    let label = format!(
        "Stage {}/{} - {:.0}%",
        state.current_stage.min(state.total_stages),
        state.total_stages,
        progress
    );

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(progress as u16)
        .label(label);

    f.render_widget(gauge, area);
}
