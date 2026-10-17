//! LSP document language IDs, distinct from language-server routing families.
//!
//! TSX/JSX share the TypeScript server but require React document IDs so the
//! server opens them with the JSX-capable parser. Unknown files stay plaintext.

use std::path::Path;

/// Resolve the protocol language ID without changing server-family routing.
///
/// # Arguments
/// * `path` — Document path whose extension selects a language.
/// # Returns
/// A protocol language ID, falling back to plaintext.
/// # Examples
/// ```
/// assert_eq!(codetether_lsp_core::document_language::id(std::path::Path::new("view.tsx")), "typescriptreact");
/// ```
pub fn id(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("tsx") => "typescriptreact",
        Some("jsx") => "javascriptreact",
        _ => {
            super::detect_language_from_path(path.to_string_lossy().as_ref()).unwrap_or("plaintext")
        }
    }
}

#[cfg(test)]
#[path = "document_language_tests.rs"]
mod tests;
