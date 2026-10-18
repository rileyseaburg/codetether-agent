//! Native provider and loop types for the shared Ralph command composition.
pub(super) type NativeHost = codetether_application_providers::cli::ralph::ApplicationHost<
    crate::provider::ProviderRegistry,
    crate::ralph::RalphLoop,
>;
