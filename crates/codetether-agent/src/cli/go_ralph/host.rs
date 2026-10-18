//! Bind the shared approved-task pipeline to the application's existing runtime.
/// Application provider registry and Ralph loop used by CLI and task callers.
pub type AppHost = codetether_go_ralph::application::ApplicationHost<
    crate::provider::ProviderRegistry,
    crate::ralph::RalphLoop,
>;
