//! Typed inputs for swarm sub-agent system prompts.

pub(crate) struct SystemPromptInput<'a> {
    pub specialty: &'a str,
    pub subtask_id: &'a str,
    pub working_dir: &'a str,
    pub model: &'a str,
    pub instruction: &'a str,
    pub context: &'a str,
    pub line_limit: Option<usize>,
    pub read_only: bool,
}
