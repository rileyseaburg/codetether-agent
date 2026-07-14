use crate::tui::app::state::App;

pub fn handle(app: &mut App) {
    if crate::tui::app::symbols::mention::confirm(app) {
        return;
    }
    if let Some(symbol) = app.state.symbol_search.selected_symbol() {
        let location = symbol
            .line
            .map(|line| format!("at line {line}"))
            .unwrap_or_default();
        app.state.status = format!("Selected symbol {} {location}", symbol.name);
    }
    app.state.symbol_search.close();
}
