pub mod layout;
pub mod metrics;
pub mod tool_history;

use ratatui::Frame;

use crate::tui::app::state::App;

use self::layout::render_inspector;

pub fn render_inspector_view(f: &mut Frame, app: &App) {
    render_inspector(f, &app.state);
}
