//! Bind shared jobs to native model selection and session event callbacks.
use crate::{
    provider::Provider,
    rlm::{RlmConfig, RlmModelPurpose},
    session::SessionEvent,
};
use std::sync::Arc;
pub(super) fn resolve<'a>(
    provider: Arc<dyn Provider>,
    model: &'a str,
    config: &'a RlmConfig,
    purpose: RlmModelPurpose,
) -> codetether_rlm_background::Resolution<'a> {
    Box::pin(super::super::rlm_model::resolve(
        provider, model, config, purpose,
    ))
}
pub(super) fn notify(notify: Option<super::Notify>) -> Option<codetether_rlm_background::Notify> {
    notify.map(|notify| {
        Arc::new(move |progress| notify(SessionEvent::RlmProgress(progress)))
            as codetether_rlm_background::Notify
    })
}
