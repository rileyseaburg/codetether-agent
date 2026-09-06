//! Model-free Windows readiness checks for automated installation.
//!
//! `codetether windows ocr-status --require-ready` returns nonzero unless a
//! packaged process can actually invoke a native OCR recognizer.

use crate::tool::computer_use::{input::ComputerUseInput, platform};
use clap::{Parser, Subcommand};

/// Arguments for native Windows diagnostics.
///
/// # Examples
/// ```rust
/// use clap::Parser;
/// use codetether_agent::cli::windows::{WindowsArgs, WindowsCommand};
/// let args = WindowsArgs::parse_from(["windows", "ocr-status"]);
/// assert!(matches!(args.command, WindowsCommand::OcrStatus { require_ready: false }));
/// ```
#[derive(Clone, Debug, Parser)]
pub struct WindowsArgs {
    /// Requested diagnostic operation.
    #[command(subcommand)]
    pub command: WindowsCommand,
}

/// Native diagnostics that never request an LLM completion.
///
/// # Examples
/// ```rust
/// use codetether_agent::cli::windows::WindowsCommand;
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

/// Run a native readiness check and emit its JSON result.
///
/// # Arguments
/// * `args` — Parsed diagnostic command and readiness requirement.
/// # Returns
/// Success when the query succeeds and its readiness requirement is met.
/// # Examples
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::cli::windows::{run, WindowsArgs, WindowsCommand};
/// run(WindowsArgs { command: WindowsCommand::OcrStatus { require_ready: true } })
///     .await.expect("OCR is installed and usable in this packaged Windows process");
/// # });
/// ```
///
/// # Errors
/// Returns an error for unsupported platforms or missing required readiness.
pub async fn run(args: WindowsArgs) -> anyhow::Result<()> {
    let require_ready = match args.command {
        WindowsCommand::ComputerUseWorker => return super::windows_worker::run().await,
        WindowsCommand::OcrStatus { require_ready } => require_ready,
    };
    // This CLI process is already disposable; do not spawn a second worker.
    let input: ComputerUseInput = serde_json::from_value(serde_json::json!({"action":"ocr_status"}))?;
    let result = platform::dispatch(&input).await?;
    println!("{}", result.output);
    anyhow::ensure!(result.success, "Windows OCR status query failed");
    let value: serde_json::Value = serde_json::from_str(&result.output)?;
    anyhow::ensure!(!require_ready || value["available"] == true, "Windows OCR setup is not ready");
    Ok(())
}

#[cfg(test)]
#[path = "windows_tests.rs"]
mod tests;