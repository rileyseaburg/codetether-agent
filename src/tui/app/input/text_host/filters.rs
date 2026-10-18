use super::{Action, App};
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::BusPush(c) => {
            app.state.bus_log.push_filter_char(*c);
        }
        Action::BusPop => {
            app.state.bus_log.pop_filter_char();
        }
        Action::ModelPush(c) => {
            app.state.model_filter_push(*c);
        }
        Action::ModelBackspace => {
            app.state.model_filter_backspace();
        }
        Action::FilePush(c) => {
            crate::tui::app::file_picker::file_picker_filter_push(app, *c);
        }
        Action::FileBackspace => {
            crate::tui::app::file_picker::file_picker_filter_backspace(app);
        }
        _ => return false,
    }
    true
}
