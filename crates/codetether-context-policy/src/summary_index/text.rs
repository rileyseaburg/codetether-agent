//! Shared post-processing of summary and recall text.

/// Remove the router stats banner from memory summaries.
pub fn strip_stats_header(text: &str) -> &str {
    let Some(rest) = text.strip_prefix("[RLM: ") else {
        return text;
    };
    rest.split_once("\n\n").map_or(text, |(_, body)| body)
}
