use super::*;
/// Stage tasks and borrowed host metadata needed before backend execution.
///
/// # Examples
/// ```
/// use codetether_swarm_core::stage_runtime::StageInput;
/// let failed = std::collections::HashSet::new();
/// let input = StageInput { tasks: vec![], failed: &failed, provider: "provider" };
/// assert!(input.tasks.is_empty());
/// ```
pub struct StageInput<'a> {
    /// Tasks assigned to this stage, in the host's order.
    pub tasks: Vec<SubTask>,
    /// Failed task IDs used to block dependent tasks.
    pub failed: &'a HashSet<String>,
    /// Provider label used in stage logging.
    pub provider: &'a str,
}
