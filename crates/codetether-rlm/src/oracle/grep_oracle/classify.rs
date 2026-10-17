//! Classify pattern, structural, and semantic queries.

use super::GrepOracle;
use crate::oracle::QueryType;
impl GrepOracle {
    /// Classify the query type based on keywords.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepOracle};
    /// assert_eq!(GrepOracle::classify_query("find all functions"), codetether_rlm::oracle::QueryType::PatternMatch);
    /// ```
    pub fn classify_query(query: &str) -> QueryType {
        let lower = query.to_lowercase();

        // Pattern-match indicators
        if lower.contains("find all")
            || lower.contains("list all")
            || lower.contains("search for")
            || lower.contains("grep")
            || lower.contains("lines matching")
            || lower.contains("occurrences of")
            || lower.contains("count")
        {
            return QueryType::PatternMatch;
        }

        // Structural indicators
        if lower.contains("signature")
            || lower.contains("parameters")
            || lower.contains("return type")
            || lower.contains("fields of")
            || lower.contains("implements")
            || lower.contains("trait")
        {
            return QueryType::Structural;
        }

        QueryType::Semantic
    }
}
