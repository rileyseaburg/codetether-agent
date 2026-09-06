//! LSP document language IDs, distinct from language-server routing families.
//!
//! TSX/JSX share the TypeScript server but require React document IDs so the
//! server opens them with the JSX-capable parser. Unknown files stay plaintext.

use std::path::Path;

pub(super) fn id(path: &Path) -> &'static str {
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
