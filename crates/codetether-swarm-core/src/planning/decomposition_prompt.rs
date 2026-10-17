//! Prompt construction for model-authored swarm decomposition.

use crate::DecompositionStrategy;

/// Build the initial decomposition prompt.
///
/// # Arguments
/// * `task` - Overall task to decompose.
/// * `strategy` - Selected decomposition policy.
/// * `maximum` - Maximum requested worker count.
/// * `parallel_required` - Whether concurrent root tasks are required.
///
/// # Returns
/// The prompt text sent to the decomposition model.
///
/// # Panics
/// Panics when `strategy` is `DecompositionStrategy::None`; callers handle this mode without decomposition.
///
/// # Examples
/// ```
/// let prompt = codetether_swarm_core::planning::decomposition_prompt::build(
///     "inspect source", codetether_swarm_core::DecompositionStrategy::Automatic, 4, true,
/// );
/// assert!(prompt.contains("inspect source"));
/// ```
pub fn build(
    task: &str,
    strategy: DecompositionStrategy,
    maximum: usize,
    parallel_required: bool,
) -> String {
    let strategy = match strategy {
        DecompositionStrategy::Automatic => "Find independently executable parallel work.",
        DecompositionStrategy::ByDomain => "Split work by domain expertise.",
        DecompositionStrategy::ByData => "Split work by files, sections, or datasets.",
        DecompositionStrategy::ByStage => "Split work into workflow stages.",
        DecompositionStrategy::None => unreachable!(),
    };
    let parallel = if parallel_required {
        "- Create at least two dependency-free subtasks that can start immediately\n\
         - Do not make every subtask depend on the previous subtask"
    } else {
        "- Use dependencies only when one subtask truly requires another's result"
    };
    format!(
        r#"You are the parent swarm orchestrator. Decompose the task for distinct worker agents.

TASK: {task}
STRATEGY: {strategy}

CONSTRAINTS:
- Maximum {maximum} subtasks
- Give each worker a distinct, independently executable instruction
- Assign a specialty to each worker
{parallel}
- Dependencies must exactly match earlier subtask names
- Set needs_worktree=true only for workers that edit or create files

Return JSON only:
{{"subtasks":[{{"name":"name","instruction":"detailed work","specialty":"role",\
"dependencies":[],"priority":1,"needs_worktree":false}}]}}"#
    )
}
