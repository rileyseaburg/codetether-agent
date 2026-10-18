use super::{Action, App};
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::DismissMention => {
            crate::tui::app::symbols::mention::dismiss(app);
        }
        Action::OpenMention => {
            crate::tui::app::symbols::mention::open(app);
        }
        Action::SymbolChar(c) => {
            app.state.symbol_search.handle_char(*c);
        }
        Action::SymbolBackspace => {
            app.state.symbol_search.handle_backspace();
        }
        Action::RefreshSymbols => {
            crate::tui::app::symbols::schedule_refresh(app);
        }
        _ => return false,
    }
    true
}
