//! Prompt input builder for summary production.

use super::super::index::types::SummaryRange;
use anyhow::{Context as _, Result};
use crate::provider::Message;

/// Build the RLM input for a transcript range.
pub fn summary_input(messages: &[Message], range: SummaryRange, target: usize) -> Result<String> {
    let slice = messages
        .get(range.start..range.end)
        .context("summary_for range out of bounds")?;
    let context = crate::session::helper::error::messages_to_rlm_context(slice);
    Ok(format!(
        "Summarise the following transcript window (~{} messages, target ≤ {} tokens). \
         Preserve key decisions, file paths, error classes, and tool outcomes.\n\n{context}",
        slice.len(), target,
    ))
}
