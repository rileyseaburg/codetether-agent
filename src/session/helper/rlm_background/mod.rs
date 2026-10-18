//! Native type bindings for application-owned background RLM workers.
pub(super) use codetether_rlm_background::resolve_pending;
pub(crate) type Background = codetether_application_providers::background::Background<
    crate::provider::registry::application::Native,
    crate::session::Session,
>;
pub(crate) type Notify =
    codetether_application_providers::background::Notify<crate::session::Session>;
