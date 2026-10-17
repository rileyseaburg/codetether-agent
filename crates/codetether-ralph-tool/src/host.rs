//! Application execution and checkout-policy boundary.

use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, RalphState};
use std::{
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};
/// Existing runtime and policy operations required by the Ralph tool.
///
/// # Implementors
/// Forward execution to the current Ralph loop and checkout through its policy guard.
#[async_trait]
pub trait Host: Send + Sync {
    /// Execute Ralph with the supplied provider and configuration.
    /// # Errors
    /// Returns initialization or execution errors with their original context.
    async fn run(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> Result<RalphState>;
    /// Request checkout through the existing access-policy guard.
    /// # Errors
    /// Returns a policy denial or git invocation error.
    fn checkout(cwd: &Path, branch: &str) -> Result<Output>;
}
