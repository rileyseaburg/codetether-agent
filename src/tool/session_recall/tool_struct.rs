//! Construction and runtime dependencies for `SessionRecallTool`.

use crate::provider::Provider;
use crate::rlm::RlmConfig;
use std::sync::Arc;

pub(super) use super::description::DESCRIPTION;

/// Local-first recall tool over persisted session evidence.
pub struct SessionRecallTool {
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) config: RlmConfig,
}

impl SessionRecallTool {
    pub fn new(provider: Arc<dyn Provider>, model: String, config: RlmConfig) -> Self {
        Self {
            provider,
            model,
            config,
        }
    }
}
