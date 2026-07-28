use crate::browser_agent::{BrowserPage, RouteAction, RoutePattern, RouteRule};
use crate::js::JsValue;

#[test]
fn abort_route_rejects_fetch_and_logs_result() {
    let mut page = BrowserPage::from_html("https://app.test/", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::Any),
        RouteAction::abort("offline"),
    );

    page.eval_js("window.err='none'; fetch('/blocked').catch(function(e){ window.err=e; });")
        .unwrap();

    assert_eq!(
        page.eval_js("window.err").unwrap(),
        JsValue::String("offline".into())
    );
    let event = page.session.network.last().unwrap();
    assert_eq!(event.status, None);
    assert_eq!(event.route_result.as_deref(), Some("abort"));
}
