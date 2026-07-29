//! TetherScript linter-server registration.
//!
//! `tetherscript lsp` is a diagnostics-only language server: it advertises
//! `textDocumentSync` and publishes lex/parse errors via
//! `textDocument/publishDiagnostics`, but implements no definition, hover,
//! reference, or symbol requests.
//!
//! It is therefore registered on the **linter** path rather than as a general
//! language server, so `.tether` edits flow through the same post-edit
//! verification hook that already runs eslint, ruff, biome, and stylelint. This
//! keeps `AGENTS.md`'s "test it through the tool runtime" rule enforceable for
//! plugin authors: a syntax error in `examples/tetherscript/*.tether` surfaces
//! immediately instead of at execution time.
//!
//! Verified against `tetherscript 0.1.0-alpha.26`, which returns
//! `serverInfo.name = "tetherscript-lsp"` and reports, for example,
//! `parse error: expected parameter name, got LBrace` with an exact range.

use super::types::LspConfig;

/// Linter name used in `[lsp.linters]` config and diagnostics output.
pub const TETHERSCRIPT_LINTER: &str = "tetherscript";

/// Executable that hosts the TetherScript language server.
pub const TETHERSCRIPT_COMMAND: &str = "tetherscript";

/// Source extensions handled by the TetherScript language server.
///
/// `.kl` is the legacy plugin extension still accepted during migration.
pub const TETHERSCRIPT_EXTENSIONS: &[&str] = &["tether", "kl"];

/// Returns the argv that starts the TetherScript language server over stdio.
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::tetherscript::tetherscript_args;
///
/// assert_eq!(tetherscript_args(), vec!["lsp".to_string()]);
/// ```
pub fn tetherscript_args() -> Vec<String> {
    vec!["lsp".to_string()]
}

/// Returns the builtin linter config for TetherScript, or `None` for other names.
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::tetherscript::{TETHERSCRIPT_LINTER, linter_config};
///
/// assert!(linter_config(TETHERSCRIPT_LINTER).is_some());
/// assert!(linter_config("eslint").is_none());
/// ```
pub fn linter_config(name: &str) -> Option<LspConfig> {
    if name != TETHERSCRIPT_LINTER {
        return None;
    }
    Some(LspConfig {
        command: TETHERSCRIPT_COMMAND.to_string(),
        args: tetherscript_args(),
        file_extensions: TETHERSCRIPT_EXTENSIONS
            .iter()
            .map(|ext| (*ext).to_string())
            .collect(),
        ..Default::default()
    })
}

/// Builtin linter names probed when no `[lsp.linters]` config is present.
///
/// Owning this list here keeps the auto-detect set in one place instead of
/// duplicating it inside the oversized client module.
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::tetherscript::LINTER_CANDIDATES;
///
/// assert!(LINTER_CANDIDATES.contains(&"tetherscript"));
/// assert!(LINTER_CANDIDATES.contains(&"ruff"));
/// ```
pub const LINTER_CANDIDATES: &[&str] =
    &["eslint", "biome", "ruff", "stylelint", TETHERSCRIPT_LINTER];

/// Returns extensions for the TetherScript linter, or an empty slice otherwise.
///
/// Serves as the fallback arm of [`super::types::linter_extensions`].
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::tetherscript::linter_extensions;
///
/// assert!(linter_extensions("tetherscript").contains(&"tether"));
/// assert!(linter_extensions("unknown").is_empty());
/// ```
pub fn linter_extensions(name: &str) -> &'static [&'static str] {
    if name == TETHERSCRIPT_LINTER {
        TETHERSCRIPT_EXTENSIONS
    } else {
        &[]
    }
}
