//! Session picker layout orchestration.
//!
//! Header, visible-list, and help-bar rendering live in focused child modules.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::tui::app::state::App;

mod header;
mod help;
mod item;
mod list;
pub(super) mod window;

/// Render the session picker into the current terminal frame.
///
/// # Arguments
///
/// * `f` - Terminal frame receiving the picker widgets.
/// * `app` - Application state containing session summaries and selection.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::sessions::render_sessions_view;
/// # fn render(f: &mut ratatui::Frame, app: &mut codetether_agent::tui::app::state::App) {
/// render_sessions_view(f, app);
/// # }
/// ```
pub fn render_sessions_view(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());
    header::render(f, app, chunks[0]);
    list::render(f, app, chunks[1]);
    help::render(f, app, chunks[2]);
}
