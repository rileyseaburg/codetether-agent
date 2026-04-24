use super::prompt_sections::{coordination_prompt, workflow_prompt};

pub struct SystemPromptInput<'a> {
    pub specialty: &'a str,
    pub subtask_id: &'a str,
    pub working_dir: &'a str,
    pub model: &'a str,
    pub prd_filename: &'a str,
    pub agents_md: &'a str,
    pub read_only: bool,
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

pub fn system_prompt(input: SystemPromptInput<'_>) -> String {
    let coordination = coordination_prompt(input.read_only, input.model);
    let workflow = workflow_prompt(input.read_only, input.prd_filename);
    format!(
        "You are a {} specialist sub-agent (ID: {}). You have access to tools to complete your task.\n\nWORKING DIRECTORY: {}\nAll file operations should be relative to this directory.\n\n{}\n\n{}\n\n{}\n\n{}\n\nWhen done, provide a brief summary of what you accomplished.{}",
        input.specialty,
        input.subtask_id,
        input.working_dir,
        mode_prompt(input.read_only),
        tools_prompt(input.read_only),
        coordination,
        workflow,
        input.agents_md,
    )
}
