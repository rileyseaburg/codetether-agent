//! Native services used by the shared validation sequence.
use crate::ValidationReport;
use anyhow::Result;
use async_trait::async_trait;
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
/// Provide refactor checks, an initialized LSP manager and optional ESLint output.
/// # Implementors
/// Implement using the existing application's tools and access policies.
#[async_trait]
pub trait Host: Sync {
    /// Client identity retained by the native manager.
    type Client: ManagedClient;
    /// Run the native refactor guard before language diagnostics.
    /// # Errors
    /// Propagates guard scanning or hook failures.
    async fn refactor(&self, root: &Path, paths: &[PathBuf]) -> Result<Option<ValidationReport>>;
    /// Obtain the native manager for the workspace URI.
    async fn manager(&self, root_uri: String) -> Arc<LspManager<Self::Client>>;
    /// Collect native sandboxed ESLint diagnostics for a file.
    async fn eslint(&self, root: &Path, path: &Path) -> Vec<String>;
}
