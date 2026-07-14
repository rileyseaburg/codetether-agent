use std::path::PathBuf;

/// One workspace symbol returned by a language server.
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
    pub fn mention(&self) -> String {
        let line = self
            .line
            .map(|value| format!(":{value}"))
            .unwrap_or_default();
        format!("@{} (`{}{line}`)", self.name, self.path.display())
    }
}
