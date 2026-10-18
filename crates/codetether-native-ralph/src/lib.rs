//! Native services composing Ralph's shared execution runtime.
//!
//! [Host] selects the native provider registry and the application's tools,
//! agent and [relay::Session]. [delegation] owns provider selection and
//! [verification] runs policy-gated story checks.
pub mod delegation;
pub mod relay;
pub mod verification;
/// Ralph runtime specialization for tool application `T`, agent `A` and session `S`.
///
/// # Examples
/// The application selects this marker as its shared Ralph runtime host.
/// `Host<T, A, S>` retains the application's existing tool and agent identities.
pub type Host<T, A, S> = codetether_ralph_runtime::application::ApplicationHost<
    codetether_native_providers::ProviderRegistry,
    T,
    codetether_ralph_agent::application::ApplicationAgent<T, A>,
    relay::Native<S>,
>;
