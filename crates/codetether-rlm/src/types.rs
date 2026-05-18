//! Core RLM types shared across oracle, repl, and router.

use serde::{Deserialize, Serialize};

/// Result of RLM analysis.
///
/// # Examples
///
/// ```rust
/// use codetether_rlm::RlmAnalysisResult;
///
/// let r = RlmAnalysisResult {
///     answer: "summary".into(),
///     iterations: 3,
///     sub_queries: vec![],
///     stats: Default::default(),
/// };
/// assert_eq!(r.iterations, 3);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmAnalysisResult {
    pub answer: String,
    pub iterations: usize,
    pub sub_queries: Vec<SubQuery>,
    pub stats: super::RlmStats,
}

/// Record of a sub-LM call.
///
/// # Examples
///
/// ```rust
/// use codetether_rlm::SubQuery;
///
/// let sq = SubQuery {
///     query: "find errors".into(),
///     context_slice: None,
///     response: "found 3".into(),
///     tokens_used: 150,
/// };
/// assert_eq!(sq.tokens_used, 150);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubQuery {
    pub query: String,
    pub context_slice: Option<String>,
    pub response: String,
    pub tokens_used: usize,
}
