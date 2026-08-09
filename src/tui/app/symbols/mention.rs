use crate::tui::app::state::App;
use crate::tui::symbol_search::SymbolSearchMode;

/// Whether an `@` typed at the cursor starts a mention.
///
/// Only a word-initial `@` opens the picker, so `user@host`, `@decorator`
/// mid-word, and git refs insert a literal `@` instead.
pub fn starts_mention(input: &str, cursor: usize) -> bool {
    if cursor == 0 {
        return true;
    }
    input
        .chars()
        .nth(cursor - 1)
        .is_some_and(char::is_whitespace)
}

/// Open the picker, or insert a literal `@` when not at a word boundary.
///
/// Returns `true` when the mention picker was opened.
pub fn open(app: &mut App) -> bool {
    if !starts_mention(&app.state.input, app.state.input_cursor) {
        app.state.insert_char('@');
        return false;
    }
    let start = app.state.input_cursor;
    app.state.insert_char('@');
    app.state.symbol_search.open_mention(start);
    app.state.status = "Type a symbol name, then Enter to attach".to_string();
    true
}

/// Abandon an open mention, keeping the text the user already typed.
///
/// The `@` and query stay in the chat buffer, so a stray `@` never costs the
/// user their input.
pub fn dismiss(app: &mut App) {
    let typed = app.state.symbol_search.query.clone();
    super::cancel_refresh();
    app.state.symbol_search.close();
    app.state.insert_text(&typed);
    app.state.status.clear();
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
    super::cancel_refresh();
    app.state.symbol_search.close();
    app.state.status = format!("Attached symbol {}", symbol.name);
    true
}

#[cfg(test)]
mod tests;
