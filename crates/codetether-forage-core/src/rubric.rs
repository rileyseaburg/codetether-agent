//! Shared forage rubric data.
/// Goals used to score alignment and optionally reject unrelated work.
///
/// # Examples
/// ```
/// let rubric = codetether_forage_core::MoonshotRubric::default();
/// assert!(rubric.goals.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct MoonshotRubric {
    /// Goals used for token-overlap alignment.
    pub goals: Vec<String>,
    /// Reject opportunities below the minimum alignment when true.
    pub required: bool,
    /// Minimum allowed alignment for required filtering.
    pub min_alignment: f64,
}
