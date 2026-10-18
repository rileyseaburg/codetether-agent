//! Bind shared jobs to native model selection and session event callbacks.
use crate::{
    provider::Provider,
    rlm::{RlmConfig, RlmModelPurpose},
    session::SessionEvent,
};
use std::sync::Arc;
pub(crate) fn resolve<'a>(
    provider: Arc<dyn Provider>,
    model: &'a str,
    config: &'a RlmConfig,
    purpose: RlmModelPurpose,
) -> codetether_rlm_background::Resolution<'a> {
    Box::pin(
        codetether_application_providers::model_resolution::resolve::<
            crate::provider::registry::application::Native,
        >(provider, model, config, purpose),
    )
}
pub(super) fn notify(notify: Option<super::Notify>) -> Option<codetether_rlm_background::Notify> {
    notify.map(|notify| {
        Arc::new(move |progress| notify(SessionEvent::RlmProgress(progress)))
            as codetether_rlm_background::Notify
    })
}
