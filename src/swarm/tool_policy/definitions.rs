use crate::provider::ToolDefinition;
use crate::swarm::SubTask;
use crate::tool::readonly::is_read_only;

pub fn is_read_only_task(task: &SubTask) -> bool {
    !task.needs_worktree()
}

pub fn definitions(all: &[ToolDefinition], read_only: bool) -> Vec<ToolDefinition> {
    all.iter()
        .filter(|tool| tool.name != "question")
        .filter(|tool| !read_only || is_read_only(&tool.name))
        .cloned()
        .collect()
}
