//! Injectable Morph editing contract for tools.
use anyhow::Result;
use async_trait::async_trait;
/// Applies Morph requests using caller-owned credentials and provider state.
///
/// # Implementors
/// Application adapters should delegate to their existing Morph entry point.
#[async_trait]
pub trait Editor: Send + Sync {
    /// Apply instructions and update context to the original code.
    ///
    /// # Arguments
    /// `code` is the original file; `instruction` and `update` describe changes.
    /// # Returns
    /// Updated file content.
    /// # Errors
    /// Propagates resolution, completion, transport and response failures.
    async fn apply(&self, code: &str, instruction: &str, update: &str) -> Result<String>;
}
