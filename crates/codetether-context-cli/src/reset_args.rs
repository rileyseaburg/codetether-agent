//! Optional summary for a manual context reset.
use clap::Parser;

/// Summary to place in the reset marker.
/// # Examples
/// ```
/// let args = codetether_context_cli::ContextResetArgs { summary: Some("Resume here".into()) };
/// assert_eq!(args.summary.as_deref(), Some("Resume here"));
/// ```
#[derive(Parser, Debug)]
pub struct ContextResetArgs {
    /// Optional summary for the reset marker
    #[arg(long)]
    pub summary: Option<String>,
}
