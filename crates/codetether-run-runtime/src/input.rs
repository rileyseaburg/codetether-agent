//! Prepared input for the ordinary run execution path.
/// Data prepared by CLI dispatch before creating the session.
/// # Examples
/// ```
/// # fn example(args: codetether_cli_options::RunArgs) {
/// let input = codetether_run_runtime::Input { args, config: Default::default(), workspace_dir: ".".into(), knowledge_snapshot: None, message: "task".into(), jsonl_output: false };
/// assert_eq!(input.message, "task");
/// # }
/// ```
pub struct Input {
    /// Original command options.
    pub args: codetether_cli_options::RunArgs,
    /// Loaded and policy-adjusted configuration.
    pub config: codetether_config::Config,
    /// Working directory used by resume and checkpoint operations.
    pub workspace_dir: std::path::PathBuf,
    /// Optional precomputed knowledge snapshot.
    pub knowledge_snapshot: Option<std::path::PathBuf>,
    /// Prepared message including file attachments.
    pub message: String,
    /// Whether live JSONL output is requested.
    pub jsonl_output: bool,
}
