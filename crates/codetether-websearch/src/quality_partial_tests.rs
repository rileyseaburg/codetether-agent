//! Regression for a location-only match masking an unrelated topic.
use crate::{quality, result::SearchResult};
fn result(title: &str) -> SearchResult {
    SearchResult {
        title: title.into(),
        url: "https://example.com/".into(),
        snippet: String::new(),
    }
}
#[test]
fn partial_matches_do_not_mask_wrong_city_or_topic() {
    for (query, title) in [
        ("weather in Lavon Texas today", "Chicago weather forecast"),
        (
            "Wylie TX restaurants dinner",
            "Wylie Funeral Homes Baltimore",
        ),
    ] {
        assert!(quality::check(query, vec![result(title)]).is_err());
    }
    let mixed = vec![result("Wylie Funeral Homes"), result("Wylie restaurants")];
    let accepted = quality::check("Wylie TX restaurants dinner", mixed).unwrap();
    assert_eq!(accepted.len(), 1);
    assert_eq!(accepted[0].title, "Wylie restaurants");
    // This is lexical overlap, not a guarantee of geographic correctness.
    assert!(quality::check("weather Lavon Texas", vec![result("Texas weather")]).is_ok());
}
