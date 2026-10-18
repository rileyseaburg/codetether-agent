//! Ralph command composition with application providers and a loop runner.
//!
//! [`ApplicationHost`] loads the provider registry and applies the shared model
//! selection policy, then delegates execution to the supplied runtime loop.
use crate::{Application, Provider, ProviderRegistry};
use codetether_ralph_cli::LoopRunner;
use std::{marker::PhantomData, path::PathBuf, sync::Arc};

/// Bind a provider registry and runtime loop to the Ralph command interface.
/// # Examples
/// ```no_run
/// # fn example<P: codetether_application_providers::Application, L: codetether_ralph_cli::LoopRunner>() {
/// type Bound<P, L> = codetether_application_providers::cli::ralph::ApplicationHost<
///     codetether_application_providers::ProviderRegistry<P>, L>;
/// fn accepts_host<H: codetether_ralph_cli::Host>() {}
/// accepts_host::<Bound<P, L>>();
/// # }
/// ```
pub struct ApplicationHost<P, L>(PhantomData<fn() -> (P, L)>);

#[async_trait::async_trait(?Send)]
impl<P: Application, L: LoopRunner> codetether_ralph_cli::Host
    for ApplicationHost<ProviderRegistry<P>, L>
{
    async fn load_provider(model: Option<String>) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        let registry = ProviderRegistry::<P>::from_vault().await?;
        codetether_ralph_cli::select_provider(model, &|name| registry.get(name))
    }
    async fn execute_loop(
        prd: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: codetether_ralph_core::RalphConfig,
        json: bool,
    ) -> anyhow::Result<()> {
        L::execute(prd, provider, model, config, json).await
    }
}
