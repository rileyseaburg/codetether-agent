//! Summarize lookup logic for [`ContextSummarizeTool`].

use crate::session::Session;
use crate::session::index::types::{SummaryNode, SummaryRange};

/// Look up a cached summary from the session index.
pub fn lookup_cached(session: &Session, range: SummaryRange) -> Option<&SummaryNode> {
    session.summary_index.get(range)
}

/// Build the "not cached" response message.
pub fn not_cached_msg(start: usize, end: usize, target: usize) -> String {
    format!(
        "No cached summary for turns [{start},{end}). \
         Requesting one at target_tokens={target}. \
         It will be available after the next derivation cycle.",
    )
}
