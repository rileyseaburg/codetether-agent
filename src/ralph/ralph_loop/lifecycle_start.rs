//! Start the run, announce it, and select its feature branch.
use super::*;
impl RalphLoop {
    pub(super) fn start_run(&mut self) -> anyhow::Result<()> {
        self.state.status = RalphStatus::Running;
        crate::ralph::verifiability::warn_unverifiable_stories(
            &self.state.prd,
            self.config.quality_checks_enabled,
        );
        self.create_initial_run_record();
        // Emit Started event
        self.try_send_event(RalphEvent::Started {
            project: self.state.prd.project.clone(),
            feature: self.state.prd.feature.clone(),
            stories: Self::build_story_infos(&self.state.prd),
            max_iterations: self.state.max_iterations,
        });
        // Switch to feature branch
        if !self.state.prd.branch_name.is_empty() {
            info!(branch = %self.state.prd.branch_name, "Switching to branch");
            self.git_checkout(&self.state.prd.branch_name)?;
        }

        Ok(())
    }
}
