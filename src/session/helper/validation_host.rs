//! Bind shared validation to the existing native LSP and tool policies.
use anyhow::Result;
use async_trait::async_trait;
use codetether_validation::ValidationReport;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
pub(super) struct NativeHost;
#[async_trait]
impl codetether_validation::Host for NativeHost {
    type Client = crate::lsp::client::LspClient;
    async fn refactor(&self, root: &Path, paths: &[PathBuf]) -> Result<Option<ValidationReport>> {
        super::super::refactor_guard::evaluate(root, paths).await
    }
    async fn manager(
        &self,
        root_uri: String,
    ) -> Arc<codetether_lsp_core::manager::LspManager<Self::Client>> {
        crate::tool::lsp::LspTool::with_root(root_uri)
            .get_manager()
            .await
    }
    async fn eslint(&self, root: &Path, path: &Path) -> Vec<String> {
        super::validation_eslint::collect(root, path).await
    }
}
