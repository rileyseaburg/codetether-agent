//! Model refresh result passed to frontends.
use crate::ModelRefreshSummary;

/// Completed model discovery delivered to a frontend.
/// # Examples
/// ```
/// use codetether_model_discovery::{ModelRefreshEvent,ModelRefreshSummary};
/// let event = ModelRefreshEvent::Loaded { models: vec![], summary: ModelRefreshSummary::empty() };
/// match event { ModelRefreshEvent::Loaded { models, .. } => assert!(models.is_empty()) }
/// ```
#[derive(Debug)]
pub enum ModelRefreshEvent {
    /// Final sorted model references and discovery counts.
    Loaded {
        /// Model references ready for the picker.
        models: Vec<String>,
        /// Provider success, failure and timeout counts.
        summary: ModelRefreshSummary,
    },
}
