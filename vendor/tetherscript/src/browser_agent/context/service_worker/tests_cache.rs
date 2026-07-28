use crate::browser_agent::{BrowserContext, CacheResponse};

#[test]
fn cache_put_match_delete_and_keys_are_origin_scoped() {
    let mut context = BrowserContext::new();
    context.cache_put(
        "https://app.test/a",
        "v1",
        "/api/data",
        CacheResponse::text(200, "one"),
    );
    context.cache_put(
        "https://other.test/a",
        "v1",
        "/api/data",
        CacheResponse::text(200, "two"),
    );

    let hit = context.cache_match("https://app.test/b", "v1", "/api/data");
    assert_eq!(hit.map(|response| response.body), Some("one".into()));
    assert_eq!(context.cache_keys("https://app.test", "v1").len(), 1);
    assert!(context.cache_delete("https://app.test", "v1", "/api/data"));
    assert_eq!(
        context.cache_match("https://app.test", "v1", "/api/data"),
        None
    );
}
