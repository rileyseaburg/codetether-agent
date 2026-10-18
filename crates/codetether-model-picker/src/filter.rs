//! Whitespace-tokenized model filtering for the picker.
//!
//! Users typically type a provider + model phrase such as `bedrock fable`.
//! A single substring match would fail because the canonical id is
//! `bedrock/us.anthropic.claude-fable-5` (the terms are non-adjacent).
//! We split the filter on whitespace and require every term to match.

/// Return the subset of `models` matching every whitespace-separated term
/// in `filter` (case-insensitive). An empty filter matches everything.
/// # Arguments
/// * `models` - Catalog whose order is preserved.
/// * `filter` - Whitespace-separated search terms.
/// # Returns
/// Borrowed model names containing every term, ignoring case.
/// # Examples
/// ```
/// let models = vec!["provider/model".into(), "other/model".into()];
/// assert_eq!(codetether_model_picker::matching(&models, "PROVIDER model"), vec!["provider/model"]);
/// ```
pub fn matching<'a>(models: &'a [String], filter: &str) -> Vec<&'a str> {
    let terms: Vec<String> = filter.split_whitespace().map(str::to_lowercase).collect();
    if terms.is_empty() {
        return models.iter().map(String::as_str).collect();
    }
    models
        .iter()
        .map(String::as_str)
        .filter(|model| {
            let haystack = model.to_lowercase();
            terms.iter().all(|term| haystack.contains(term))
        })
        .collect()
}
#[cfg(test)]
#[path = "filter_tests.rs"]
mod tests;
