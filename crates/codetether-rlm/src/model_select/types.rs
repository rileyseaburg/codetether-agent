//! Types used by RLM model selection.

/// RLM call surface that needs a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RlmModelPurpose {
    /// Top-level RLM analysis.
    Root,
    /// Session history/context compaction.
    Compression,
    /// Background tool-output summarisation.
    Background,
    /// Recursive subcall analysis.
    Subcall,
}

/// Source that won model selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RlmModelSource {
    /// [`crate::RlmConfig::root_model`] was set.
    RootConfig,
    /// [`crate::RlmConfig::subcall_model`] was set.
    SubcallConfig,
    /// [`super::RLM_MODEL_ENV`] was set.
    Env,
    /// Foreground caller model was reused.
    Caller,
}

/// Selected model plus provenance for logging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RlmModelChoice {
    /// Provider-qualified or bare model reference.
    pub model: String,
    /// Selection source.
    pub source: RlmModelSource,
}

impl RlmModelChoice {
    pub(crate) fn new(model: impl Into<String>, source: RlmModelSource) -> Self {
        Self {
            model: model.into(),
            source,
        }
    }
}
