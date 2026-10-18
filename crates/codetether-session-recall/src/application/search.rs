//! Indexed local retrieval supplied by the embedding application.
use crate::{RecallHit, SearchOptions};
use std::path::Path;
/// Indexed-search operations used before optional answer synthesis.
/// # Implementors
/// Delegate to the application's existing recall query and storage adapters.
#[async_trait::async_trait]
pub trait Search: Send + Sync {
    /// Retrieve bounded evidence under the application's workspace policy.
    /// # Errors
    /// Returns the existing indexed retrieval errors.
    async fn search(
        workspace: &Path,
        query: &str,
        options: SearchOptions<'_>,
    ) -> anyhow::Result<Vec<RecallHit>>;
}
