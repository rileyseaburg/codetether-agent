//! Application shell policy and UI callbacks for shared quality sequencing.
use super::*;
use codetether_ralph_core::verification::{ShellExecutor, ShellOutput};
pub(super) struct QualityHost;
#[async_trait::async_trait]
impl ShellExecutor for QualityHost {
    async fn execute(&self, cwd: &Path, command: &str) -> anyhow::Result<ShellOutput> {
        let result = quality_shell::run(cwd, command).await?;
        Ok(ShellOutput {
            success: result.success,
            output: result.combined,
        })
    }
}
impl RalphLoop {
    pub(super) async fn run_quality_gates_in_dir_with_events(
        &self,
        dir: &Path,
        story_id: &str,
    ) -> anyhow::Result<bool> {
        codetether_ralph_core::quality::run_quality_gates(
            dir,
            &self.state.prd.quality_checks,
            &QualityHost,
            |name, passed| {
                self.try_send_event(RalphEvent::StoryQualityCheck {
                    story_id: story_id.to_string(),
                    check_name: name.to_string(),
                    passed,
                });
            },
        )
        .await
    }

    /// Run quality gates and emit events for each check
    pub(super) async fn run_quality_gates_with_events(
        &self,
        story_id: &str,
    ) -> anyhow::Result<bool> {
        self.run_quality_gates_in_dir_with_events(&self.state.working_dir, story_id)
            .await
    }
}
