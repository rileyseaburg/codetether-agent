//! CLI syntax for WorkerServerArgs.
use clap::Parser;
/// Parsed options for the worker server args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::WorkerServerArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

/// Arguments for standalone worker HTTP server (testing/debugging)
#[derive(Parser, Debug, Clone)]
pub struct WorkerServerArgs {
    /// Hostname to bind
    #[arg(long, default_value = "0.0.0.0")]
    pub hostname: String,

    /// Port to bind
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
