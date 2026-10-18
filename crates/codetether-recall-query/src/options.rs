//! Selection and score controls for one local recall query.

/// Fully typed controls for one local recall query.
/// # Examples
/// ```
/// let options = codetether_recall_query::SearchOptions {
///     session_id: None, excluded_session: Some("current"), limit: 3, minimum_score: 0.45,
/// };
/// assert_eq!(options.limit, 3);
/// ```
pub struct SearchOptions<'a> {
    /// Restrict search to one session, permitting on-demand sidecar migration.
    pub session_id: Option<&'a str>,
    /// Exclude a session from returned evidence.
    pub excluded_session: Option<&'a str>,
    /// Maximum number of returned hits.
    pub limit: usize,
    /// Inclusive hybrid-score threshold.
    pub minimum_score: f32,
}
