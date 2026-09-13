//! TetherScript language-server registration.
//!
//! `tetherscript lsp` is a diagnostics-only language server: it advertises
//! `textDocumentSync` and publishes lex/parse errors via
//! `textDocument/publishDiagnostics`, but implements no definition, hover,
//! reference, or symbol requests.
//!
//! It is registered on **both** resolution paths:
//!
//! * as a **linter** (`[lsp.linters.tetherscript]`), so `.tether` edits flow
//!   through the same post-edit verification hook that runs eslint, ruff, and
//!   biome;
//! * as the **language server** for the `tetherscript` language id, because the
//!   automatic pre-approval diagnostics pass resolves servers by language, not
//!   linter name — without this a `.tether` `write` was never checked.
//!
//! Non-diagnostic `lsp` tool actions (hover, definition, …) are refused up
//! front by `tool::lsp::capability_gate` from the advertised capabilities.
//!
//! Verified against `tetherscript 0.1.0-alpha.31`, which returns
//! `serverInfo.name = "tetherscript-lsp"` and reports, for example,
//! `parse error: expected parameter name, got LBrace` with an exact range.

use super::types::LspConfig;

#[cfg(test)]
#[path = "tetherscript_language_tests.rs"]
mod language_tests;

/// Linter name used in `[lsp.linters]` config and diagnostics output.
pub const TETHERSCRIPT_LINTER: &str = "tetherscript";

/// Language id reported by [`super::detect_language_from_path`] for `.tether`
/// and `.kl` files. Same string as the linter name so one `[lsp.servers]` or
/// `[lsp.linters]` entry configures both paths.
pub const TETHERSCRIPT_LANGUAGE: &str = "tetherscript";

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
    Some(server_config())
}

/// Returns the builtin language-server config for the `tetherscript` language.
///
/// This is what lets `.tether` writes reach the automatic pre-approval
/// diagnostics pass, which resolves servers by language id rather than by
/// linter name.
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::tetherscript::{TETHERSCRIPT_LANGUAGE, language_server_config};
///
/// assert!(language_server_config(TETHERSCRIPT_LANGUAGE).is_some());
/// assert!(language_server_config("rust").is_none());
/// ```
pub fn language_server_config(language: &str) -> Option<LspConfig> {
    (language == TETHERSCRIPT_LANGUAGE).then(server_config)
}

fn server_config() -> LspConfig {
    LspConfig {
        command: TETHERSCRIPT_COMMAND.to_string(),
        args: tetherscript_args(),
        file_extensions: TETHERSCRIPT_EXTENSIONS
            .iter()
            .map(|ext| (*ext).to_string())
            .collect(),
        ..Default::default()
    }
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
