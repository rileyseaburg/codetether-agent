//! An interactive deadline is not proof that a language server has failed.

#[derive(Debug, thiserror::Error)]
#[error(
    "Automatic LSP preflight budget expired for {path}; diagnostics continue in the background and the server is retained"
)]
/// Caller deadline elapsed while background diagnostics retain the server.
///
/// # Examples
/// ```
/// let error = codetether_lsp_preflight::budget::Expired { path: "file.ts".into() };
/// assert!(error.to_string().contains("server is retained"));
/// ```
pub struct Expired {
    /// Proposed file whose diagnostics outlived the interactive wait.
    pub path: std::path::PathBuf,
}
