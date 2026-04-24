use super::SubTask;
use crate::provider::ToolDefinition;
use crate::tool::{ToolRegistry, readonly::is_read_only};

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

pub fn restrict_registry(registry: &mut ToolRegistry, read_only: bool) {
    if !read_only {
        registry.unregister("question");
        return;
    }
    let ids: Vec<String> = registry.list().into_iter().map(str::to_string).collect();
    for id in ids {
        if !is_read_only(&id) {
            registry.unregister(&id);
        }
    }
}

pub fn mode_prompt(read_only: bool) -> &'static str {
    if read_only {
        "READ-ONLY TASK: inspect, analyze, and report. Do not modify files, run shell commands, or start autonomous implementation workflows."
    } else {
        "IMPORTANT: You MUST use tools to make changes. Do not just describe what to do - actually do it using the tools available."
    }
}

pub fn tools_prompt(read_only: bool) -> &'static str {
    if read_only {
        "Available tools:\n- read: Read file contents\n- glob: Find files by pattern\n- grep: Search file contents\n- tree/fileinfo/headtail/diff: Inspect repository state\n- lsp/codesearch: Query code intelligence\n- webfetch/websearch: Research external context"
    } else {
        "Available tools:\n- read/write/edit/multiedit: Inspect and modify files\n- glob/grep: Search the repository\n- bash: Run shell commands with the task cwd\n- prd/ralph/go: Run autonomous implementation workflows\n- swarm_share/agent: Coordinate with helper agents"
    }
}

#[cfg(test)]
mod tests;
