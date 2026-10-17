/// Identity and execution limits used when constructing an agent result.
///
/// # Examples
/// ```
/// let spec = codetether_swarm_core::agent_result::ExecutionSpec { subtask_id: "task", instruction: "inspect", max_steps: 10, timeout_secs: 60 };
/// assert_eq!(spec.max_steps, 10);
/// ```
pub struct ExecutionSpec<'a> {
    /// Task ID used for the task and derived agent identifiers.
    pub subtask_id: &'a str,
    /// Instruction used to infer verification requirements.
    pub instruction: &'a str,
    /// Configured step limit included in exhaustion errors.
    pub max_steps: usize,
    /// Configured timeout included in deadline errors.
    pub timeout_secs: u64,
}
