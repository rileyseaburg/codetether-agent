//! Whitespace-tokenized model filtering for the picker.
//!
//! Users typically type a provider + model phrase such as `bedrock fable`.
//! A single substring match would fail because the canonical id is
//! `bedrock/us.anthropic.claude-fable-5` (the terms are non-adjacent).
//! We split the filter on whitespace and require every term to match.

/// Return the subset of `models` matching every whitespace-separated term
/// in `filter` (case-insensitive). An empty filter matches everything.
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
mod tests {
    use super::matching;

    fn catalog() -> Vec<String> {
        vec![
            "bedrock/global.anthropic.claude-fable-5".to_string(),
            "bedrock/us.anthropic.claude-fable-5".to_string(),
            "zai/glm-5".to_string(),
        ]
    }

    #[test]
    fn multi_term_filter_matches_non_adjacent_tokens() {
        let models = catalog();
        let hits = matching(&models, "bedrock fable");
        assert_eq!(
            hits,
            vec![
                "bedrock/global.anthropic.claude-fable-5",
                "bedrock/us.anthropic.claude-fable-5"
            ]
        );
    }

    #[test]
    fn empty_filter_matches_all_and_extra_spaces_ignored() {
        let models = catalog();
        assert_eq!(matching(&models, "   ").len(), 3);
        assert_eq!(matching(&models, "zai   glm"), vec!["zai/glm-5"]);
    }
}
