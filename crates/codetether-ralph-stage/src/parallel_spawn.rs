//! Prepare shared stage inputs and spawn story tasks in source order.
use super::parallel_stage_state::ParallelStage;
use super::*;
pub(crate) fn spawn_parallel_stage<H: Host>(
    host: &H,
    stories: Vec<UserStory>,
    worktree_mgr: &Option<Arc<WorktreeManager>>,
    mut worktrees_by_story: HashMap<String, WorktreeInfo>,
) -> ParallelStage {
    // Execute stories in parallel
    let semaphore = Arc::new(tokio::sync::Semaphore::new(
        host.inputs().config.max_concurrent_stories,
    ));
    let provider = Arc::clone(host.inputs().provider);
    let model = host.inputs().model.clone();
    let prd_info = (
        host.inputs().state.prd.project.clone(),
        host.inputs().state.prd.feature.clone(),
    );
    let working_dir = host.inputs().state.working_dir.clone();
    let progress_path = host.inputs().config.progress_path.clone();

    let mut handles = Vec::new();

    // Collect accumulated learnings from previous stages via bus
    let accumulated_learnings = bus_collect_learnings(host);

    for story in stories {
        let task = prepare_parallel_task(
            host,
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
