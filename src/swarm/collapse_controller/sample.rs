use super::git::{collect_changed_files, collect_changed_lines};
use super::*;
impl CollapseController {
    pub fn sample(&mut self, branches: &[BranchRuntimeState]) -> Result<CollapseTick> {
        if branches.is_empty() {
            return Ok(CollapseTick::default());
        }

        let mut probe_handles = Vec::with_capacity(branches.len());
        for branch in branches {
            let subtask_id = branch.subtask_id.clone();
            let branch_name = branch.branch.clone();
            let worktree_path = branch.worktree_path.clone();
            let handle = std::thread::spawn(move || -> Result<BranchObservation> {
                Ok(BranchObservation {
                    subtask_id,
                    branch: branch_name,
                    compile_ok: collapse_git_probe::static_clean(&worktree_path)?,
                    changed_files: collect_changed_files(&worktree_path)?,
                    changed_lines: collect_changed_lines(&worktree_path)?,
                    resource_health_score: 1.0,
                    infra_unhealthy_signals: 0,
                })
            });
            probe_handles.push(handle);
        }

        let mut observations = Vec::with_capacity(branches.len());
        for handle in probe_handles {
            let observation = handle
                .join()
                .map_err(|_| anyhow!("Branch sampling thread panicked"))??;
            observations.push(observation);
        }
        Ok(self.sample_observations(&observations))
    }
}
