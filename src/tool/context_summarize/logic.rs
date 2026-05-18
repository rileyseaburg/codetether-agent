//! Summarize lookup logic for [`ContextSummarizeTool`].

use crate::session::Session;
use crate::session::index::types::{SummaryNode, SummaryRange};

/// Look up a cached summary from the session index.
pub fn lookup_cached(session: &Session, range: SummaryRange) -> Option<&SummaryNode> {
    session.summary_index.get(range)
}
