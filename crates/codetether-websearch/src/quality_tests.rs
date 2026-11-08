//! Conservative relevance checks with deliberate semantic limitations.
use crate::{SearchFailure, quality, result::SearchResult};
fn result(title: &str) -> Vec<SearchResult> {
    vec![SearchResult {
        title: title.into(),
        url: "https://example.com/".into(),
        snippet: String::new(),
    }]
}
#[test]
fn unrelated_results_are_not_success() {
    let error = quality::check(
        "Rockwall TX best restaurants dinner",
        result("Microsoft support"),
    )
    .unwrap_err();
    assert!(matches!(error, SearchFailure::Unrelated(_)));
    assert_eq!(error.code(), "WEBSEARCH_UNRELATED_RESULTS");
    assert!(
        error
            .detail()
            .contains("Do not treat these results as evidence")
    );
}
#[test]
fn partial_unicode_empty_and_short_queries_are_preserved() {
    for (query, title) in [
        (
            "Rockwall TX best restaurants dinner",
            "Rockwall restaurants guide",
        ),
        ("rust", "Rust programming language"),
        ("café Zürich", "ZÜRICH dining guide"),
        ("旅行 東京", "東京"),
        ("!!!", "Something"),
        ("near me", "Something"),
        ("C", "Language"),
    ] {
        assert!(quality::check(query, result(title)).is_ok(), "{query}");
    }
    assert!(quality::check("Rockwall", vec![]).unwrap().is_empty());
    assert!(quality::check("Rockwall", result("Rockwallish")).is_err());
    let mut results = result("Dining guide");
    results[0].url = "https://example.com/rockwall".into();
    assert!(quality::check("Rockwall restaurants", results).is_ok());
}
