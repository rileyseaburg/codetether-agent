use crate::browser_agent::{BrowserContext, BrowserPage, CacheResponse};
use crate::js::JsValue;

#[test]
fn active_worker_fulfills_pass_through_fetch_from_cache() {
    let mut context = BrowserContext::new();
    let index = context.new_page(BrowserPage::from_html("https://app.test/", ""));
    let page = context.page_mut(index).unwrap();
    page.service_worker_register("/", "/sw.js").unwrap();
    page.service_worker_activate("/").unwrap();
    page.cache_put("v1", "/api/data", CacheResponse::text(200, "cached"))
        .unwrap();

    page.eval_js("window.out=''; fetch('/api/data').then(function(r){ r.text().then(function(t){ window.out=r.status + ':' + t; }); });")
        .unwrap();

    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("200:cached".into())
    );
    let log = page.service_worker_fetch_log().unwrap();
    assert!(log[0].matched);
    assert_eq!(log[0].cache_name.as_deref(), Some("v1"));
}
