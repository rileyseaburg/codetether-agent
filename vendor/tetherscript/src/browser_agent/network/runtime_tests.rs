use crate::browser_agent::{BrowserPage, RouteAction, RouteFulfillment, RoutePattern, RouteRule};
use crate::js::JsValue;

fn page() -> BrowserPage {
    BrowserPage::from_html("https://app.test/", "<main></main>")
}

#[test]
fn fetch_fulfilled_by_route_returns_status_headers_and_body() {
    let mut page = page();
    page.route(
        RouteRule::new(RoutePattern::substring("/api/data")),
        RouteAction::Fulfill(RouteFulfillment {
            status: 202,
            headers: vec![("x-route".into(), "yes".into())],
            body: "{\"ok\":true}".into(),
        }),
    );
    page.eval_js("window.out=''; fetch('/api/data', {method:'post', headers:{'X-Test':'yes'}, body:'payload'}).then(function(r){ r.text().then(function(t){ window.out=r.status + ':' + r.headers.get('x-route') + ':' + t; }); });").unwrap();
    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("202:yes:{\"ok\":true}".into())
    );
    assert_eq!(
        page.session.network.last().unwrap().route_result.as_deref(),
        Some("fulfill")
    );
    let log = page.route_log();
    let entry = log.last().unwrap();
    assert_eq!(entry.method, "POST");
    assert_eq!(entry.url, "https://app.test/api/data");
    assert_eq!(entry.body.as_deref(), Some("payload"));
    assert_eq!(entry.headers, vec![("x-test".into(), "yes".into())]);
}
