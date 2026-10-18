/// Borrowed startup settings used to construct shared task runtime state.
/// # Examples
/// ```
/// let settings = codetether_a2a_worker_core::initialization::RuntimeConfig {
///     server: "http://server", token: None, worker_id: "worker", name: "agent",
///     max_concurrent_tasks: 1, auto_approve: "safe",
/// };
/// assert_eq!(settings.name, "agent");
/// ```
pub struct RuntimeConfig<'a> {
    /// Worker server URL.
    pub server: &'a str,
    /// Optional bearer token.
    pub token: Option<&'a str>,
    /// Stable worker identity.
    pub worker_id: &'a str,
    /// Advertised agent name.
    pub name: &'a str,
    /// Configured concurrency before normalization.
    pub max_concurrent_tasks: usize,
    /// Automatic approval policy before parsing.
    pub auto_approve: &'a str,
}
