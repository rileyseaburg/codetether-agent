//! Public scheduling and result-application surface for symbol search.

use crate::tui::app::state::App;

pub fn active(app: &App) -> bool {
    app.state.symbol_search.active
}

/// Schedule a cancellable, trailing-debounced symbol refresh.
pub fn schedule(app: &mut App) {
    let query = app.state.symbol_search.query.clone();
    if !super::gate::is_searchable(&query) {
        super::runtime::cancel();
        app.state.symbol_search.set_results(Vec::new());
        app.state.symbol_search.loading = false;
        return;
    }
    app.state.symbol_search.loading = true;
    let root = match std::env::current_dir() {
        Ok(root) => root,
        Err(error) => {
            app.state.symbol_search.set_error(error.to_string());
            return;
        }
    };
    let files = app.state.symbol_search.language_files.clone();
    super::runtime::schedule(query, files, root);
}

/// Apply the newest completed search when it still matches the UI query.
pub fn drain(app: &mut App) -> bool {
    let Some(done) = super::runtime::take() else {
        return false;
    };
    if !active(app) || app.state.symbol_search.query != done.query {
        return false;
    }
    app.state.symbol_search.language_files = done.files;
    match done.result {
        Ok(results) => app.state.symbol_search.set_results(results),
        Err(error) => app.state.symbol_search.set_error(error.to_string()),
    }
    true
}

/// Close the picker and cancel its pending background query.
pub fn close(app: &mut App) {
    super::runtime::cancel();
    app.state.symbol_search.close();
}
