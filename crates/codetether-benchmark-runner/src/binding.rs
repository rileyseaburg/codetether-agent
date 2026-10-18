//! Benchmark application composition using shared providers and Ralph runtime.
//!
//! [`ApplicationHost`] preserves provider loading and loop lifecycle operations;
//! [`Version`] supplies the embedding application's report identity.
use crate::Application;
use codetether_application_providers::{Application as Providers, ProviderRegistry};
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, RalphState};
use codetether_ralph_runtime::{Host, RalphLoop};
use std::{marker::PhantomData, path::PathBuf, sync::Arc};

/// Application identity recorded in benchmark reports.
/// # Implementors
/// The executable supplies its own package version.
pub trait Version: Send + Sync + 'static {
    /// Return the package version used by the embedding executable.
    fn version() -> &'static str;
}

/// Select the provider factory, Ralph runtime and report version.
/// # Examples
/// ```
/// # fn example<P, R, V>() where P: codetether_application_providers::Application,
/// # R: codetether_ralph_runtime::Host, V: codetether_benchmark_runner::binding::Version {
/// fn accepts_application<A: codetether_benchmark_runner::Application>() {}
/// accepts_application::<codetether_benchmark_runner::binding::ApplicationHost<P, R, V>>();
/// # }
/// ```
pub struct ApplicationHost<P, R, V>(PhantomData<fn() -> (P, R, V)>);

impl<P: Providers, R: Host, V: Version> Application for ApplicationHost<P, R, V> {
    type Registry = ProviderRegistry<P>;
    type Loop = RalphLoop<R>;
    fn version() -> &'static str {
        V::version()
    }
    async fn registry() -> anyhow::Result<ProviderRegistry<P>> {
        ProviderRegistry::<P>::from_vault().await
    }
    fn provider(registry: &ProviderRegistry<P>, name: &str) -> Option<Arc<dyn Provider>> {
        registry.get(name)
    }
    async fn start(
        path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> anyhow::Result<RalphLoop<R>> {
        RalphLoop::<R>::new(path, provider, model, config).await
    }
    async fn run(loop_state: &mut RalphLoop<R>) -> anyhow::Result<RalphState> {
        loop_state.run().await
    }
}
