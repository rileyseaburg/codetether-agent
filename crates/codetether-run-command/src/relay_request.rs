//! Normalized relay command fields consumed by the native executor.
/// Validated relay execution request.
/// # Examples
/// ```
/// let request = codetether_run_command::parse_relay_request("/autochat 3 --no-prd repair", false).unwrap().unwrap();
/// assert_eq!(request.agent_count, 3);
/// ```
pub struct RelayRequest {
    /// Requested initial specialist count.
    pub agent_count: usize,
    /// Normalized task text.
    pub task: String,
    /// Whether objective approval and PRD execution are required.
    pub require_prd: bool,
    /// Whether go/team alias policy applies.
    pub easy_go_requested: bool,
    /// Whether the user supplied the specialist count.
    pub explicit_count: bool,
}
