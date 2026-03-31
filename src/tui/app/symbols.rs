use crate::tool::{Tool, lsp::LspTool};
use crate::tui::app::state::App;

pub fn symbol_search_active(app: &App) -> bool {
    !app.state.symbol_search.query.is_empty()
        || !app.state.symbol_search.results.is_empty()
        || app.state.symbol_search.loading
        || app.state.symbol_search.error.is_some()
}

pub async fn refresh_symbol_search(app: &mut App) {
    app.state.symbol_search.loading = true;
    let result = LspTool::new()
        .execute(serde_json::json!({
            "action": "workspaceSymbol",
            "query": app.state.symbol_search.query.clone(),
        }))
        .await;

    match result {
        Ok(tool_result) if tool_result.success => {
            let results = tool_result
                .output
                .lines()
                .skip(2)
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    Some(crate::tui::symbol_search::SymbolEntry {
                        name: trimmed.to_string(),
                        kind: "Symbol".to_string(),
                        path: std::path::PathBuf::from(trimmed),
                        uri: None,
                        line: None,
                        container: None,
                    })
                })
                .collect();
            app.state.symbol_search.set_results(results);
        }
        Ok(tool_result) => app.state.symbol_search.set_error(tool_result.output),
        Err(err) => app.state.symbol_search.set_error(err.to_string()),
    }
}
