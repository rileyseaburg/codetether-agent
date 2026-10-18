//! Validated inputs for PRD-backed CLI execution.
/// Request fields consumed by the objective and Ralph workflow.
/// # Examples
/// ```
/// let input = codetether_go_cli::Input { task: "repair".into(), model: "provider/model".into(), agent_count: 2, easy_go_requested: true, explicit_count: false, format: "json".into() };
/// assert_eq!(input.agent_count, 2);
/// ```
pub struct Input {
    /// User task.
    pub task: String,
    /// Selected model reference.
    pub model: String,
    /// Requested team size.
    pub agent_count: usize,
    /// Whether go/team default concurrency applies.
    pub easy_go_requested: bool,
    /// Whether the team size was explicit.
    pub explicit_count: bool,
    /// Output format.
    pub format: String,
}
