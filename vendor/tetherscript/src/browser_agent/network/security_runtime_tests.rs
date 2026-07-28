use crate::browser_agent::{BrowserPage, RouteAction, RoutePattern, RouteRule};
use crate::js::JsValue;

const CORS_REASON: &str =
    "CORS blocked cross-origin request from https://app.test to https://api.test";

fn page() -> BrowserPage {
    BrowserPage::from_html("https://app.test/", "<main></main>")
}

#[test]
fn cross_origin_xhr_is_blocked_and_logged() {
    let mut page = page();
    page.eval_js(
        "window.err=''; let xhr=XMLHttpRequest(); xhr.onerror=function(){ window.err=xhr.statusText; }; xhr.open('GET','https://api.test/data'); xhr.send();",
    )
    .unwrap();

    assert_eq!(
        page.eval_js("window.err").unwrap(),
        JsValue::String(CORS_REASON.into())
    );
    let event = page.session.network.last().unwrap();
    assert_eq!(event.status, None);
    assert_eq!(event.route_result.as_deref(), Some("blocked"));
    let entry = page.route_log().last().unwrap().clone();
    assert_eq!(entry.action, RouteAction::abort(CORS_REASON));
    let security = entry.security.as_ref().unwrap();
    assert!(!security.same_origin);
    assert_eq!(security.referrer.as_deref(), Some("https://app.test/"));
}

#[test]
fn allowed_origin_bypasses_cors_and_keeps_metadata() {
    let mut page = page();
    page.allow_origin("https://api.test");
    page.route(
        RouteRule::new(RoutePattern::substring("api.test/data")),
        RouteAction::fulfill(200, "ok"),
    );

    page.eval_js("window.out=''; fetch('https://api.test/data').then(function(r){ r.text().then(function(t){ window.out=r.status + ':' + t; }); });").unwrap();

    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("200:ok".into())
    );
    assert_eq!(
        page.session.network.last().unwrap().route_result.as_deref(),
        Some("fulfill")
    );
    let security = page.route_log().last().unwrap().security.clone().unwrap();
    assert!(security.allowed_by_policy);
}
