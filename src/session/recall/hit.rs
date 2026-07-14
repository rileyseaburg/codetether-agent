//! Ranked evidence returned from the local recall index.

/// One provenance-bearing local recall match.
#[derive(Debug, Clone)]
pub(crate) struct RecallHit {
    /// Source session UUID.
    pub session_id: String,
    /// Optional source session title.
    pub title: Option<String>,
    /// Inclusive source message index.
    pub start: usize,
    /// Exclusive source message index.
    pub end: usize,
    /// Combined lexical and semantic relevance score.
    pub score: f32,
    /// Bounded source evidence.
    pub excerpt: String,
}
