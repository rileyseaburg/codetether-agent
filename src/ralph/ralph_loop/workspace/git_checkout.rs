//! git_checkout application policy and provenance integration.
use super::*;
impl RalphLoop {
    /// Git checkout
    pub(in crate::ralph::ralph_loop) fn git_checkout(&self, branch: &str) -> anyhow::Result<()> {
        // Try to checkout, create if doesn't exist
        let output = git_policy::checkout(&self.state.working_dir, branch)?;

        if !output.status.success() {
            git_policy::guard(
                &self.state.working_dir,
                &format!("git checkout -b {branch}"),
            )?;
            Command::new("git")
                .args(["checkout", "-b", branch])
                .current_dir(&self.state.working_dir)
                .output()?;
        }

        Ok(())
    }
}
