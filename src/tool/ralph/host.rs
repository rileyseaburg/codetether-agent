//! Delegate Ralph execution and checkout to their existing application paths.
use crate::{
    provider::Provider,
    ralph::{RalphConfig, RalphLoop, RalphState},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::{
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};
/// Existing Ralph runtime with policy-checked branch restoration.
pub struct ApplicationHost;
#[async_trait]
impl codetether_ralph_tool::Host for ApplicationHost {
    async fn run(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> Result<RalphState> {
        let mut ralph = RalphLoop::new(prd_path, provider, model, config)
            .await
            .context("Failed to initialize Ralph")?;

        ralph.run().await.context("Ralph loop failed")
    }
    fn checkout(cwd: &Path, branch: &str) -> Result<Output> {
        crate::ralph::git_policy::checkout(cwd, branch)
    }
}
