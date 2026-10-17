//! Prepare shared stage inputs and spawn story tasks in source order.
use super::parallel_stage_state::ParallelStage;
use super::*;
impl RalphLoop {
    pub(super) fn spawn_parallel_stage(
        &self,
        stories: Vec<UserStory>,
        worktree_mgr: &Option<Arc<WorktreeManager>>,
        mut worktrees_by_story: HashMap<String, crate::worktree::WorktreeInfo>,
    ) -> ParallelStage {
        // Execute stories in parallel
        let semaphore = Arc::new(tokio::sync::Semaphore::new(
            self.config.max_concurrent_stories,
        ));
        let provider = Arc::clone(&self.provider);
        let model = self.model.clone();
        let prd_info = (
            self.state.prd.project.clone(),
            self.state.prd.feature.clone(),
        );
        let working_dir = self.state.working_dir.clone();
        let progress_path = self.config.progress_path.clone();

        let mut handles = Vec::new();

        // Collect accumulated learnings from previous stages via bus
        let accumulated_learnings = self.bus_collect_learnings();

        for story in stories {
            let task = self.prepare_parallel_task(
                &semaphore,
                &provider,
                &model,
                &prd_info,
                &working_dir,
                worktree_mgr,
                &mut worktrees_by_story,
                &progress_path,
                &accumulated_learnings,
                story,
            );
            let handle = tokio::spawn(task.run());
            handles.push(handle);
        }

        ParallelStage {
            handles,
            provider,
            model,
            working_dir,
            _semaphore: semaphore,
        }
    }
}
