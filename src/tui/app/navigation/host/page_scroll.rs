//! Native page scroll operations requested by navigation policy.
use crate::tui::app::state::App;
use codetether_tui_navigation::Action;
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::HelpUp(value) => {
            app.state.help.scroll.scroll_up(*value);
        }
        Action::HelpDown(value, limit) => {
            app.state.help.scroll.scroll_down(*value, *limit);
        }
        Action::ChatUp(value) => {
            app.state.scroll_up(*value);
        }
        Action::ChatDown(value) => {
            app.state.scroll_down(*value);
        }
        Action::FilePageUp => {
            crate::tui::app::file_picker::file_picker_page_up(app);
        }
        Action::FilePageDown => {
            crate::tui::app::file_picker::file_picker_page_down(app);
        }
        Action::ChatPageUp => {
            app.state.page_up();
        }
        Action::ChatPageDown => {
            app.state.page_down();
        }
        Action::ProtocolScroll(value) => {
            app.state.protocol_scroll = *value;
        }
        _ => return false,
    }
    true
}
