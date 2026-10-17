//! Ranked evidence returned from the local recall index.

/// One provenance-bearing local recall match.
///
/// # Examples
/// ```rust
/// let hit=codetether_session_recall::RecallHit {
///     session_id:"id".into(),title:None,start:0,end:1,score:0.8,excerpt:"decision".into(),
/// };
/// assert_eq!(hit.start,0);
/// ```
#[derive(Debug, Clone)]
pub struct RecallHit {
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
