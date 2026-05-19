//! Tool state for `context_summarize`.

use std::sync::Arc;

use crate::provider::Provider;

/// Summarize (read cached) or produce summary for a turn range.
pub struct ContextSummarizeTool {
    pub(crate) provider: Option<Arc<dyn Provider>>,
    pub(crate) model: Option<String>,
}

impl ContextSummarizeTool {
    pub fn cached_only() -> Self {
        Self {
            provider: None,
            model: None,
        }
    }

    pub fn new(provider: Arc<dyn Provider>, model: String) -> Self {
        Self {
            provider: Some(provider),
            model: Some(model),
        }
    }
}
