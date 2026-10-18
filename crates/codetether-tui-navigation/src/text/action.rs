/// Native buffer or filter mutation selected by text routing.
/// # Examples
/// ```
/// use codetether_tui_navigation::text::Action;
/// assert!(matches!(Action::Insert('x'), Action::Insert('x')));
/// ```
pub enum Action {
    /// Dismiss the active mention search.
    DismissMention,
    /// Insert a literal character into chat input.
    Insert(char),
    /// Append a symbol-search character.
    SymbolChar(char),
    /// Delete a symbol-search character.
    SymbolBackspace,
    /// Schedule symbol results to refresh.
    RefreshSymbols,
    /// Append a bus-filter character.
    BusPush(char),
    /// Delete a bus-filter character.
    BusPop,
    /// Append a model-filter character.
    ModelPush(char),
    /// Delete a model-filter character.
    ModelBackspace,
    /// Append a file-filter character.
    FilePush(char),
    /// Delete a file-filter character.
    FileBackspace,
    /// Begin mention search, which owns insertion of the @ character.
    OpenMention,
    /// Delete a character from chat input.
    DeleteBackspace,
}
