//! Prompt construction for model-authored swarm decomposition.

use crate::swarm::DecompositionStrategy;

pub(super) fn build(
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
