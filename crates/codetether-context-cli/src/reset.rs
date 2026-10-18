//! Load the current session, append a reset marker, and persist it.
use crate::{ContextResetArgs, ResetSession};
use anyhow::Result;
use codetether_context_reset::format_reset_marker;
use codetether_provider_core::{ContentPart, Message, Role};

const DEFAULT_RESET_SUMMARY: &str = "Manual context reset requested from the CLI.";

pub(crate) async fn reset<S: ResetSession>(args: ContextResetArgs) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let mut session = S::last_for_directory(Some(cwd.as_path())).await?;
    let summary = args
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(DEFAULT_RESET_SUMMARY);
    let marker = format_reset_marker(summary, summary.len());
    session.add_message(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: marker.clone(),
        }],
    });
    session.save().await?;
    println!("{marker}");
    Ok(())
}
