//! Native specialization of shared thinker client ownership and dispatch.
mod host;
/// Thinker client using the application's model loaders and provider registry.
pub type ThinkerClient = codetether_cognition_core::thinker::client::Client<
    codetether_thinker_service::Service<host::Native>,
>;
#[cfg(test)]
#[path = "client/native_tests.rs"]
mod native_tests;
