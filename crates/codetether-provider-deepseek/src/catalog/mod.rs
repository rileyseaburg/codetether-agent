//! Ordered model metadata and provider catalog access.
//!
//! `list` assembles the existing Flash, Pro, and legacy model entries in order.
//! `DeepSeekProvider::available_models` supplies them to the provider contract.
use super::DeepSeekProvider;
use codetether_provider_core::ModelInfo;
mod flash;
mod legacy;
mod pro;
use flash::flash_model;
use legacy::{chat_legacy, reasoner_legacy};
use pro::pro_model;

pub(crate) fn list() -> Vec<ModelInfo> {
    vec![flash_model(), pro_model(), chat_legacy(), reasoner_legacy()]
}

impl DeepSeekProvider {
    pub(crate) fn available_models(&self) -> Vec<ModelInfo> {
        list()
    }
}
