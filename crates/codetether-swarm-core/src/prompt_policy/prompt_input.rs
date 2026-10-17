//! Typed inputs for swarm sub-agent system prompts.

/// Borrowed task, workspace and capability inputs for a delegated system prompt.
///
/// # Examples
/// ```no_run
/// use codetether_swarm_core::prompt_policy::{SystemPromptInput, system_prompt};
/// let input = SystemPromptInput { specialty: "review", subtask_id: "task", working_dir: ".", model: "model", instruction: "inspect", context: "", line_limit: None, read_only: true, expects_changes: false };
/// let prompt = system_prompt(input, std::convert::identity);
/// assert!(prompt.contains("READ-ONLY TASK"));
/// ```
pub struct SystemPromptInput<'a> {
    pub specialty: &'a str,
    pub subtask_id: &'a str,
    pub working_dir: &'a str,
    pub model: &'a str,
    pub instruction: &'a str,
    pub context: &'a str,
    pub line_limit: Option<usize>,
    pub read_only: bool,
    pub expects_changes: bool,
}
