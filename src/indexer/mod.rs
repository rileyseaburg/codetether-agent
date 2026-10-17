//! Application adapters for the standalone indexer.
//!
//! The core owns scanning, graph assembly, persistence, and embedding batches.
//! This module retains CLI output and resolves application providers.

use anyhow::{Context, Result};
pub use codetether_indexer::*;
use std::path::{Path, PathBuf};
mod cli;
mod provider_adapter;
mod resolver;
pub use cli::run;

/// Build an index using application-configured embedding providers.
///
/// # Arguments
/// * `root` - Workspace directory to scan.
/// * `options` - Filtering and embedding settings.
/// # Returns
/// A sorted source index and knowledge graph.
/// # Errors
/// Returns scanning, provider resolution, or embedding errors.
/// # Examples
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let index = codetether_agent::indexer::build_index(std::path::Path::new("."), &Default::default()).await.unwrap();
/// assert!(!index.root.is_empty());
/// # });
/// ```
pub async fn build_index(root: &Path, options: &BuildOptions) -> Result<CodebaseIndex> {
    build_index_with_resolver(root, options, &resolver::ApplicationResolver).await
}
