//! Mocked backend attempts exercise the production fallback loop and receipts.
use crate::{Backend, attempt::Attempt, quality, result::SearchResult};
fn attempt(backend: Backend, related: bool) -> Attempt {
    Attempt {
        backend,
        status: Some(200),
        request_id: None,
        outcome: quality::check(
            "Rockwall dinner",
            vec![SearchResult {
                title: if related {
                    "Rockwall dining"
                } else {
                    "Microsoft support"
                }
                .into(),
                url: "https://example.com/dining".into(),
                snippet: String::new(),
            }],
        ),
    }
}
#[tokio::test]
async fn unrelated_response_falls_back() {
    let result = crate::fallback::search(
        "Rockwall dinner",
        vec![Backend::Bing, Backend::DuckDuckGo],
        |b| std::future::ready(attempt(b, b == Backend::DuckDuckGo)),
    )
    .await;
    assert!(result.success);
    let evidence = &result.metadata["search_evidence"];
    assert_eq!(
        evidence["attempts"][0]["outcome"],
        "WEBSEARCH_UNRELATED_RESULTS"
    );
    assert_eq!(evidence["backend"], "duckduckgo");
    assert_eq!(evidence["sources"][0], "https://example.com/dining");
    assert!(!result.output.contains("Microsoft"));
}
#[tokio::test]
async fn all_bad_backends_fail_and_explicit_selection_stays_bounded() {
    for (setting, count) in [(None, 3), (Some("bing"), 1)] {
        let order = crate::backend_order(setting, true).unwrap();
        let result = crate::fallback::search("Rockwall dinner", order, |b| {
            std::future::ready(attempt(b, false))
        })
        .await;
        assert!(!result.success);
        assert!(!result.output.contains("No results found"));
        assert_eq!(result.metadata["error_code"], "WEBSEARCH_UNRELATED_RESULTS");
        let evidence = &result.metadata["search_evidence"];
        assert_eq!(evidence["attempts"].as_array().unwrap().len(), count);
        assert_eq!(evidence["sources"], serde_json::json!([]));
    }
}
