//! Explicit model selection from [`crate::RlmConfig`].

use crate::config::RlmConfig;

use super::{RlmModelPurpose, RlmModelSource};

pub(super) fn configured(
    config: &RlmConfig,
    purpose: RlmModelPurpose,
) -> Option<(&str, RlmModelSource)> {
    let root = clean(config.root_model.as_deref());
    let sub = clean(config.subcall_model.as_deref());
    match purpose {
        RlmModelPurpose::Subcall => sub
            .map(|m| (m, RlmModelSource::SubcallConfig))
            .or_else(|| root.map(|m| (m, RlmModelSource::RootConfig))),
        RlmModelPurpose::Root | RlmModelPurpose::Compression | RlmModelPurpose::Background => {
            root.map(|m| (m, RlmModelSource::RootConfig))
        }
    }
}

fn clean(model: Option<&str>) -> Option<&str> {
    model.filter(|m| !m.trim().is_empty())
}
