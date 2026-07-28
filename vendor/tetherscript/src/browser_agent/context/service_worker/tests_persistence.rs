use crate::browser_agent::{BrowserContext, CacheResponse};

#[test]
fn context_snapshot_restores_workers_and_caches() {
    let mut context = BrowserContext::new();
    context.service_worker_register("https://app.test/", "/", "/sw.js");
    context.service_worker_activate("https://app.test/", "/");
    context.cache_put(
        "https://app.test/",
        "v1",
        "/api/data",
        CacheResponse::text(200, "cached"),
    );

    let snapshot = context.snapshot_state();
    let mut restored = BrowserContext::new();
    restored.restore_state(snapshot).unwrap();

    assert_eq!(
        restored
            .cache_match("https://app.test/", "v1", "/api/data")
            .map(|response| response.body),
        Some("cached".into())
    );
    assert_eq!(
        restored
            .service_worker_registrations("https://app.test/")
            .len(),
        1
    );
}
