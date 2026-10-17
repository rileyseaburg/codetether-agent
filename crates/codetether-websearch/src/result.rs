//! result for web search.
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct SearchResult {
    pub(crate) title: String,
    pub(crate) url: String,
    pub(crate) snippet: String,
}
