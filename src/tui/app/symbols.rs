mod languages;
pub mod mention;
mod parse;
mod rank;
mod search;

use crate::tui::app::state::App;

pub fn symbol_search_active(app: &App) -> bool {
    app.state.symbol_search.active
}

pub async fn refresh_symbol_search(app: &mut App) {
    app.state.symbol_search.loading = true;
    let root = match std::env::current_dir() {
        Ok(root) => root,
        Err(error) => {
            app.state.symbol_search.set_error(error.to_string());
            return;
        }
    };
    if app.state.symbol_search.language_files.is_empty() {
        app.state.symbol_search.language_files = languages::representatives(&root);
    }
    let files = app.state.symbol_search.language_files.clone();
    match search::workspace(&files, &app.state.symbol_search.query).await {
        Ok(results) => app.state.symbol_search.set_results(results),
        Err(error) => app.state.symbol_search.set_error(error.to_string()),
    }
}
