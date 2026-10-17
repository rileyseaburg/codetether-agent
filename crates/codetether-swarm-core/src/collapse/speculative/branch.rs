/// Planned branch identity, strategy and task prompt.
///
/// # Examples
/// ```
/// let spec = codetether_swarm_core::collapse::speculative::BranchSpec { branch_name: "branch".into(), strategy_prompt: "inspect".into(), task_prompt: "task".into() };
/// assert_eq!(spec.branch_name, "branch");
/// ```

pub struct BranchSpec {
    pub branch_name: String,
    pub strategy_prompt: String,
    pub task_prompt: String,
}
