//! Tool state for `context_summarize`.

use std::sync::Arc;

use codetether_provider_core::Provider;

/// Read a cached summary or produce one for a requested turn range.
///
/// # Examples
/// ```rust
/// # fn example<S:codetether_context_summarize::SummarySource>() {
/// let _tool=codetether_context_summarize::ContextSummarizeTool::<S>::cached_only();
/// # }
/// ```
pub struct ContextSummarizeTool<S> {
    source: std::marker::PhantomData<fn() -> S>,
    pub(crate) provider: Option<Arc<dyn Provider>>,
    pub(crate) model: Option<String>,
}

impl<S: crate::SummarySource> ContextSummarizeTool<S> {
    /// Construct a tool that only returns already cached summaries.
    pub fn cached_only() -> Self {
        Self {
            source: std::marker::PhantomData,
            provider: None,
            model: None,
        }
    }

    /// Construct a tool that can produce missing summaries with this provider/model.
    pub fn new(provider: Arc<dyn Provider>, model: String) -> Self {
        Self {
            source: std::marker::PhantomData,
            provider: Some(provider),
            model: Some(model),
        }
    }
}
