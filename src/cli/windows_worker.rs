//! Hidden Windows desktop-worker entry point, dispatched before agent startup.
//!
//! This command must not initialize provider/auth configuration or stdout logging.

/// Run the computer-use worker protocol over standard input and output.
///
/// # Arguments
/// No CLI arguments; requests arrive as bounded JSON-lines on standard input.
/// # Returns
/// Returns normally when the parent closes its input stream.
/// # Errors
/// Returns an error on malformed or broken protocol streams; print it to stderr.
/// # Examples
/// ```text
/// codetether windows computer-use-worker
/// ```
pub async fn run() -> anyhow::Result<()> {
    crate::tool::computer_use::worker::serve().await
}
