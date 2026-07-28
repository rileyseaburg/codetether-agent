use crate::browser_agent::{BrowserPage, RouteAction, RoutePattern, RouteRule};
use crate::js::JsValue;

#[test]
fn xhr_fulfilled_by_route_returns_status_and_body() {
    let mut page = BrowserPage::from_html("https://app.test/", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::substring("/api/xhr")),
        RouteAction::fulfill(204, "xhr-body"),
    );

    page.eval_js(
        "let xhr=XMLHttpRequest(); window.out=''; xhr.onload=function(){ window.out=xhr.status + ':' + xhr.responseText; }; xhr.open('post','/api/xhr'); xhr.setRequestHeader('X-Test','yes'); xhr.send('payload');",
    )
    .unwrap();

    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("204:xhr-body".into())
    );
    assert_eq!(
        page.session.network.last().unwrap().route_result.as_deref(),
        Some("fulfill")
    );
    let log = page.route_log();
    let entry = log.last().unwrap();
    assert_eq!(entry.method, "POST");
    assert_eq!(entry.url, "https://app.test/api/xhr");
    assert_eq!(entry.body.as_deref(), Some("payload"));
    assert_eq!(entry.headers, vec![("x-test".into(), "yes".into())]);
}
