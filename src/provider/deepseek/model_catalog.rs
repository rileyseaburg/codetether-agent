//! Provider-bound model catalog access for DeepSeek.

use crate::provider::ModelInfo;

use super::{DeepSeekProvider, models};

impl DeepSeekProvider {
    pub(crate) fn available_models(&self) -> Vec<ModelInfo> {
        models::list()
    }
}
