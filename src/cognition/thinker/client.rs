//! Native specialization of shared thinker client ownership and dispatch.
/// Thinker client using the application's model loaders and provider registry.
pub type ThinkerClient = codetether_cognition_core::thinker::client::Client<Services>;
/// Backend services selected for native cognition clients and runtimes.
pub(in crate::cognition) type Services = codetether_thinker_service::Service<
    codetether_application_providers::thinker::ApplicationHost<
        crate::provider::registry::application::Native,
    >,
>;
#[cfg(test)]
#[path = "client/native_tests.rs"]
mod native_tests;
