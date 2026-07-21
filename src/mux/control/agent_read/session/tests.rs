use serde_json::json;

#[test]
fn reads_latest_semantic_excerpts() {
    let indexed = json!({"documents": [
        {"excerpt": "old"}, {"excerpt": "user request"}, {"excerpt": "assistant reply"}
    ]});

    assert_eq!(super::excerpts(&indexed), "user request\nassistant reply");
}
