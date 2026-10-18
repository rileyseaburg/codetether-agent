//! Native lists operations requested by navigation policy.
use crate::tui::app::state::App;
use codetether_tui_navigation::Action;
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::SymbolPrevious => {
            app.state.symbol_search.select_prev();
        }
        Action::SymbolNext => {
            app.state.symbol_search.select_next();
        }
        Action::SessionPrevious => {
            app.state.session_picker.sessions_select_prev();
        }
        Action::SessionNext => {
            app.state.session_picker.sessions_select_next();
        }
        Action::ModelPrevious => {
            app.state.model_picker.model_select_prev();
        }
        Action::ModelNext => {
            app.state.model_picker.model_select_next();
        }
        Action::SettingsPrevious => {
            app.state.settings_panel.select_previous();
        }
        Action::SettingsNext => {
            app.state.settings_panel.select_next();
        }
        Action::SlashPrevious => {
            app.state.composer.select_prev_slash_suggestion();
        }
        Action::SlashNext => {
            app.state.composer.select_next_slash_suggestion();
        }
        Action::FilePrevious => {
            crate::tui::app::file_picker::file_picker_select_prev(app);
        }
        Action::FileNext => {
            crate::tui::app::file_picker::file_picker_select_next(app);
        }
        Action::FileFirst => {
            crate::tui::app::file_picker::file_picker_select_first(app);
        }
        Action::FileLast => {
            crate::tui::app::file_picker::file_picker_select_last(app);
        }
        _ => return false,
    }
    true
}
