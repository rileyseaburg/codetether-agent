use crate::browser_agent::{BrowserPage, RouteAction, RoutePattern, RouteRule};
use crate::js::JsValue;

#[test]
fn newest_route_precedence_applies_inside_page_runtime() {
    let mut page = BrowserPage::from_html("https://app.test/", "<main></main>");
    page.route(RouteRule::new(RoutePattern::Any), RouteAction::abort("old"));
    page.route(
        RouteRule::new(RoutePattern::substring("/api/new")),
        RouteAction::fulfill(203, "new"),
    );
    page.eval_js("window.out=''; fetch('/api/new').then(function(r){ r.text().then(function(t){ window.out=r.status + ':' + t; }); });").unwrap();
    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("203:new".into())
    );
    assert_eq!(
        page.route_log().last().unwrap().action,
        RouteAction::fulfill(203, "new")
    );
}
