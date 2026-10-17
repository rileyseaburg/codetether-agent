//! Existing indexed retrieval and RLM synthesis supplied by the application.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmResult};
use std::{path::Path, sync::Arc};
/// Search parameters passed to the indexed recall backend.
///
/// # Examples
/// ```rust
/// let options=codetether_session_recall::SearchOptions {
///     session_id:None,excluded_session:None,limit:3,minimum_score:0.2,
/// };
/// assert_eq!(options.limit,3);
/// ```
pub struct SearchOptions<'a> {
    /// Restrict results to one session when provided.
    pub session_id: Option<&'a str>,
    /// Omit one session when provided.
    pub excluded_session: Option<&'a str>,
    /// Maximum ranked evidence matches.
    pub limit: usize,
    /// Minimum accepted relevance score.
    pub minimum_score: f32,
}
/// Backend operations shared with the application's existing recall machinery.
///
/// # Implementors
/// Delegate to indexed local retrieval and the existing RLM router.
#[async_trait]
pub trait Host: Send + Sync {
    /// Retrieve bounded local evidence under the existing workspace policy.
    /// # Errors
    /// Returns indexed retrieval errors.
    async fn search(
        workspace: &Path,
        query: &str,
        options: SearchOptions<'_>,
    ) -> Result<Vec<crate::RecallHit>>;
    /// Run explicit answer synthesis over the already retrieved evidence.
    /// # Errors
    /// Returns errors from the existing RLM router.
    async fn synthesize(
        framed: &str,
        query: &str,
        provider: Arc<dyn Provider>,
        model: &str,
        config: &RlmConfig,
    ) -> Result<RlmResult>;
}
