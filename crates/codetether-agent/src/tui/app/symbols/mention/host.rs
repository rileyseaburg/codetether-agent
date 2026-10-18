//! Borrow the native editor and picker for shared mention operations.
use crate::tui::app::state::App;
use codetether_composer_view::ComposerState;
use codetether_symbol_mention::ComposerView;

pub(super) fn view(app: &mut App) -> ComposerView<'_, ComposerState> {
    ComposerView {
        editor: &mut app.state.composer,
        picker: &mut app.state.symbol_search,
        status: &mut app.state.status,
        input: |editor| &editor.input,
        cursor: |editor| editor.cursor,
        insert_char: ComposerState::insert_char,
        insert_text: ComposerState::insert_text,
        replace: ComposerState::replace_input_chars,
    }
}
