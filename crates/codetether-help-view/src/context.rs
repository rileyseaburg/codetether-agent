//! Application values shown in help.
/// Borrowed session, environment and worker labels for help rendering.
/// # Examples
/// ```
/// let session = None;
/// let worker = None;
/// let cwd = String::from(".");
/// let context = codetether_help_view::HelpContext {
///     session_id: &session, cwd_display: &cwd, auto_apply_edits: false,
///     allow_network: false, slash_autocomplete: true, worker_id: &worker,
///     worker_name: &worker, a2a_connected: false,
/// };
/// assert_eq!(context.cwd_display, ".");
/// ```
pub struct HelpContext<'a> {
    /// Current session ID.
    pub session_id: &'a Option<String>,
    /// Display form of the workspace directory.
    pub cwd_display: &'a String,
    /// Whether edit previews apply automatically.
    pub auto_apply_edits: bool,
    /// Whether sandbox networking is enabled.
    pub allow_network: bool,
    /// Whether slash autocomplete is enabled.
    pub slash_autocomplete: bool,
    /// Connected worker ID, if available.
    pub worker_id: &'a Option<String>,
    /// Friendly name of the worker.
    pub worker_name: &'a Option<String>,
    /// Whether the worker is connected over A2A.
    pub a2a_connected: bool,
}
