use crate::tui::app::state::App;
use crate::tui::symbol_search::SymbolSearchMode;

pub fn open(app: &mut App) {
    let start = app.state.input_cursor;
    app.state.insert_char('@');
    app.state.symbol_search.open_mention(start);
    app.state.status = "Type a symbol name, then Enter to attach".to_string();
}

pub fn confirm(app: &mut App) -> bool {
    let SymbolSearchMode::Mention { start } = app.state.symbol_search.mode else {
        return false;
    };
    let Some(symbol) = app.state.symbol_search.selected_symbol().cloned() else {
        return true;
    };
    let mention = symbol.mention();
    app.state.replace_input_chars(start..start + 1, &mention);
    app.state.symbol_search.close();
    app.state.status = format!("Attached symbol {}", symbol.name);
    true
}

#[cfg(test)]
mod tests;
