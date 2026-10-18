/// Borrowed worker settings supplied by the native argument adapter.
/// # Examples
/// ```
/// use codetether_a2a_worker_core::initialization::initialize::Options;
/// let token = None;
/// let options = Options { server: "http://server", token: &token, name: None,
///     workspaces: None, max_concurrent_tasks: 1, auto_approve: "safe" };
/// assert_eq!(options.server, "http://server");
/// ```
pub struct Options<'a> {
    /// Configured server URL before trailing-slash normalization.
    pub server: &'a str,
    /// Optional bearer token retained for environment and runtime setup.
    pub token: &'a Option<String>,
    /// Optional explicit worker name.
    pub name: Option<&'a str>,
    /// Optional workspace list in the existing CLI format.
    pub workspaces: Option<&'a str>,
    /// Configured concurrency before normalization.
    pub max_concurrent_tasks: usize,
    /// Automatic approval mode before parsing.
    pub auto_approve: &'a str,
}
