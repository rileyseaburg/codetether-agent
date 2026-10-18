//! Windows diagnostic CLI schema.
use clap::Subcommand;

/// Native diagnostics that never request an LLM completion.
///
/// # Examples
/// ```rust
/// use codetether_cli_options::WindowsCommand;
/// match (WindowsCommand::OcrStatus { require_ready: true }) {
///     WindowsCommand::OcrStatus { require_ready } => assert!(require_ready),
///     WindowsCommand::ComputerUseWorker => unreachable!(),
/// }
/// ```
#[derive(Clone, Debug, Subcommand)]
pub enum WindowsCommand {
    /// Internal isolated desktop-operation protocol; not an interactive agent.
    #[command(hide = true)]
    ComputerUseWorker,
    /// Report native OCR readiness as JSON.
    OcrStatus {
        /// Fail unless package identity, a language, and recognition are usable.
        #[arg(long)]
        require_ready: bool,
    },
}
