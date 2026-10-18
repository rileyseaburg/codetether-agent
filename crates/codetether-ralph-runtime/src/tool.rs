//! Ralph tool integration with runtime execution and the runtime Git policy.
//!
//! [ApplicationHost] supplies the tool lifecycle using the selected runtime [Host].
use crate::{Host, RalphLoop};
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, RalphState};
use std::{
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};
/// Ralph tool host using the selected runtime and policy-checked restoration.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_ralph_runtime::Host>() {
/// let _tool = codetether_ralph_tool::RalphTool::<
///     codetether_ralph_runtime::tool::ApplicationHost<H>>::new();
/// # }
/// ```
pub struct ApplicationHost<H: Host>(std::marker::PhantomData<fn() -> H>);
#[async_trait]
impl<H: Host> codetether_ralph_tool::Host for ApplicationHost<H> {
    async fn run(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> Result<RalphState> {
        let mut ralph = RalphLoop::<H>::new(prd_path, provider, model, config)
            .await
            .context("Failed to initialize Ralph")?;

        ralph.run().await.context("Ralph loop failed")
    }
    fn checkout(cwd: &Path, branch: &str) -> Result<Output> {
        codetether_ralph_git::checkout::<H::Git>(cwd, branch)
    }
}
