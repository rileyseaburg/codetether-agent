//! Reproduce Gemma's star query without hiding tools behind literal-star matches.
mod support;
use serde_json::json;
use support::catalog;
#[test]
fn star_and_empty_queries_list_the_entire_catalog() {
    for query in [
        json!({}),
        json!({"query":""}),
        json!({"query":"  "}),
        json!({"query":"*"}),
        json!({"query":" * "}),
    ] {
        let result = catalog(query);
        assert_eq!(result["count"], 4);
        assert_eq!(result["total_count"], 4);
        assert_eq!(result["filtered"], false);
        assert!(result["query"].is_null());
        assert_eq!(result["tools"][2]["name"], "webfetch");
        assert_eq!(result["tools"][3]["name"], "websearch");
    }
}
#[test]
fn substring_queries_remain_literal_and_report_the_filter() {
    for (query, normalized, count) in [
        (" WEB ", "web", 2),
        ("*.rs", "*.rs", 1),
        ("absent", "absent", 0),
    ] {
        let result = catalog(json!({"query":query}));
        assert_eq!(result["count"], count);
        assert_eq!(result["total_count"], 4);
        assert_eq!(result["filtered"], true);
        assert_eq!(result["query"], normalized);
    }
    let exact = catalog(json!({"tool":"websearch","query":"*"}));
    assert_eq!(exact["mode"], "tool");
    assert_eq!(exact["tool"]["name"], "websearch");
}
