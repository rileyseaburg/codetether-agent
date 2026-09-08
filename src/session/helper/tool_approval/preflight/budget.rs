//! An interactive deadline is not proof that a language server has failed.

#[derive(Debug, thiserror::Error)]
#[error(
    "Automatic LSP preflight budget expired for {path}; diagnostics continue in the background and the server is retained"
)]
pub(super) struct Expired {
    pub path: std::path::PathBuf,
}
