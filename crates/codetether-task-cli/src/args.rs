use std::path::PathBuf;

/// Inputs consumed by remote task submission.
///
/// # Examples
/// ```
/// let args = codetether_task_cli::TaskArgs {
///     prompt: "Review changes".into(), agent: "build".into(),
///     workspace_id: Some("workspace".into()), workspace: None,
///     file: vec![], title: None, model: None,
///     worker_personality: None, notify_email: None, format: "json".into(),
/// };
/// assert!(args.file.is_empty());
/// ```
pub struct TaskArgs {
    /// Prompt to submit.
    pub prompt: String,
    /// Requested agent name.
    pub agent: String,
    /// Explicit remote workspace identifier, skipping remote lookup.
    pub workspace_id: Option<String>,
    /// Local workspace path, taking precedence over the global project.
    pub workspace: Option<PathBuf>,
    /// Local files to attach as workspace-relative paths.
    pub file: Vec<PathBuf>,
    /// Explicit title, or a title derived from the prompt when absent.
    pub title: Option<String>,
    /// Optional model override.
    pub model: Option<String>,
    /// Optional worker personality.
    pub worker_personality: Option<String>,
    /// Optional notification email address.
    pub notify_email: Option<String>,
    /// Output format; exactly `json` selects JSON output.
    pub format: String,
}
