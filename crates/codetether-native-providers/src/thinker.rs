//! Native thinker bindings sharing the application provider registry.
//!
//! [ThinkerClient] selects [Services] for model loading and credentialed
//! backends. Construct it with the shared cognition ThinkerConfig.
/// Thinker client using the application's model loaders and provider registry.
pub type ThinkerClient = codetether_cognition_core::thinker::client::Client<Services>;
/// Backend services selected for native cognition clients and runtimes.
pub type Services = codetether_thinker_service::Service<
    codetether_application_providers::thinker::ApplicationHost<
        crate::provider::registry::application::Native,
    >,
>;
