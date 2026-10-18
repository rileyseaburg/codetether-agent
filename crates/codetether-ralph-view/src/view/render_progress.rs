//! Ralph render progress presentation.
use crate::RalphViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Gauge};
pub(super) fn render_progress(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let progress = state.progress();
    let (_, _, passed, _) = state.status_counts();
    let total = state.stories.len();
    let label = format!("{}/{} stories — {:.0}%", passed, total, progress);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::DarkGray))
        .percent(progress as u16)
        .label(label);

    f.render_widget(gauge, area);
}
