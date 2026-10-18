//! Native command-policy-aware shell execution.
use codetether_ralph_core::verification::{ShellExecutor, ShellOutput};
use std::path::Path;
/// Stateless native command-policy binding for quality gates.
#[derive(Default)]
pub struct QualityHost;
#[async_trait::async_trait]
impl ShellExecutor for QualityHost {
    async fn execute(&self, cwd: &Path, command: &str) -> anyhow::Result<ShellOutput> {
        let result = crate::ralph::quality_shell::run(cwd, command).await?;
        Ok(ShellOutput {
            success: result.success,
            output: result.combined,
        })
    }
}
