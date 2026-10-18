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
            crate::tui::app::state::scroll::view(&mut app.state).scroll_up(*value);
        }
        Action::ChatDown(value) => {
            app.state.chat_viewport.scroll_down(*value);
        }
        Action::FilePageUp => {
            crate::tui::app::file_picker::file_picker_page_up(app);
        }
        Action::FilePageDown => {
            crate::tui::app::file_picker::file_picker_page_down(app);
        }
        Action::ChatPageUp => {
            crate::tui::app::state::scroll::view(&mut app.state).page_up();
        }
        Action::ChatPageDown => {
            crate::tui::app::state::scroll::view(&mut app.state).page_down();
        }
        Action::ProtocolScroll(value) => {
            app.state.protocol_registry.scroll = *value;
        }
        _ => return false,
    }
    true
}
