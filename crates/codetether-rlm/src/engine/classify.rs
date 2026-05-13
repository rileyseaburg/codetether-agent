//! Query classification for the evidence-first engine.

use crate::oracle::{GrepOracle, QueryType};

/// Engine route chosen before evidence collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryKind {
    /// Deterministic line-oriented pattern search.
    Pattern,
    /// Deterministic tree-sitter structural search.
    Structural,
    /// Model synthesis over curated evidence.
    Semantic,
}

/// Classify a query using oracle-compatible categories.
pub fn classify(tool_id: &str, query: &str) -> QueryKind {
    if matches!(
        tool_id,
        "session_context" | "context_reset" | "summary_index"
    ) {
        return QueryKind::Semantic;
    }
    match GrepOracle::classify_query(query) {
        QueryType::PatternMatch => QueryKind::Pattern,
        QueryType::Structural => QueryKind::Structural,
        QueryType::Semantic => QueryKind::Semantic,
    }
}
