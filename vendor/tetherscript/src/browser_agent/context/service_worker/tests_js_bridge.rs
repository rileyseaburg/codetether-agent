use crate::browser_agent::{BrowserContext, BrowserPage, CacheResponse, Locator};

fn page(context: &mut BrowserContext, url: &str) -> usize {
    context.new_page(BrowserPage::from_html(url, "<p id='out'></p>"))
}

#[test]
fn js_register_updates_context_and_resolves_registration() {
    let mut context = BrowserContext::new();
    let index = page(&mut context, "https://app.test/index.html");
    let page = context.page_mut(index).unwrap();
    page.eval_js("navigator.serviceWorker.register('/sw.js',{scope:'/app/'}).then(function(r){document.getElementById('out').textContent=r.origin+'|'+r.scope+'|'+r.scriptURL+'|'+r.state;});").unwrap();
    assert_eq!(
        page.service_worker_registrations().unwrap()[0].scope,
        "https://app.test/app/"
    );
    page.expect_text(
        &Locator::css("#out"),
        "https://app.test|https://app.test/app/|https://app.test/sw.js|installing",
    )
    .unwrap();
}

#[test]
fn js_ready_uses_active_registration_metadata() {
    let mut context = BrowserContext::new();
    let index = page(&mut context, "https://app.test/app/");
    let page = context.page_mut(index).unwrap();
    page.service_worker_register("/app/", "/sw.js").unwrap();
    page.service_worker_activate("/app/").unwrap();
    page.eval_js("navigator.serviceWorker.ready.then(function(r){document.getElementById('out').textContent=r.state+':'+r.scope;});").unwrap();
    page.expect_text(&Locator::css("#out"), "active:https://app.test/app/")
        .unwrap();
}

#[test]
fn js_cache_match_and_delete_are_backed_by_context_cache() {
    let mut context = BrowserContext::new();
    let index = page(&mut context, "https://app.test/");
    let page = context.page_mut(index).unwrap();
    page.cache_put("v1", "/api/data", CacheResponse::text(201, "cached"))
        .unwrap();
    page.eval_js("caches.open('v1').then(function(c){c.keys().then(function(keys){c.match('/api/data').then(function(r){r.text().then(function(t){document.getElementById('out').textContent=keys.length+':'+r.status+':'+r.url+':'+t;});});});});").unwrap();
    page.expect_text(
        &Locator::css("#out"),
        "1:201:https://app.test/api/data:cached",
    )
    .unwrap();
    page.eval_js("caches.delete('v1');").unwrap();
    assert!(page.cache_match("v1", "/api/data").unwrap().is_none());
}
