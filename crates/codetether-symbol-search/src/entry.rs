//! Symbol picker entry.
use std::path::PathBuf;

/// One workspace symbol returned by a language server.
///
/// # Examples
/// ```
/// use codetether_symbol_search::SymbolEntry;
/// let entry = SymbolEntry {
///     name: "main".into(), kind: "Function".into(), path: "src/main.rs".into(),
///     uri: None, line: Some(1), container: None,
/// };
/// assert_eq!(entry.mention(), "@main (`src/main.rs:1`)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolEntry {
    /// Source-level symbol name.
    pub name: String,
    /// LSP kind, such as `Function` or `Struct`.
    pub kind: String,
    /// Definition file path.
    pub path: PathBuf,
    /// Original language-server URI when available.
    pub uri: Option<String>,
    /// One-based definition line when available.
    pub line: Option<u32>,
    /// Parent symbol or namespace when available.
    pub container: Option<String>,
}

impl SymbolEntry {
    /// Formats a chat mention containing the symbol and source location.
    /// # Arguments
    /// * `self` - Symbol whose name and location should be attached.
    /// # Returns
    /// A mention containing the name, path and optional line number.
    /// # Examples
    /// See the construction example on [SymbolEntry].
    pub fn mention(&self) -> String {
        let line = self
            .line
            .map(|value| format!(":{value}"))
            .unwrap_or_default();
        format!("@{} (`{}{line}`)", self.name, self.path.display())
    }
}
