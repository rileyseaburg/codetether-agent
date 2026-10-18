//! Native registry binding for the shared header visibility policy.
use crate::tui::app::state::App;
pub(in crate::tui::ui::chat_view) fn visible(app: &App) -> bool {
    codetether_agent_dashboard::bar::visible(app)
}
