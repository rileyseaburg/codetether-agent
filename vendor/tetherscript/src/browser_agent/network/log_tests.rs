use crate::browser_agent::network::{
    RouteAction, RoutePattern, RouteRequest, RouteRule, RouteTable,
};

fn request(method: &str, url: &str) -> RouteRequest {
    RouteRequest::new(method, url)
}

#[test]
fn handle_logs_fulfill_abort_and_continue() {
    let mut table = RouteTable::default();
    table.add(
        RouteRule::new(RoutePattern::substring("/mock")),
        RouteAction::fulfill(201, "ok"),
    );
    assert_eq!(
        table.handle(request("GET", "/mock")),
        RouteAction::fulfill(201, "ok")
    );
    assert_eq!(table.handle(request("GET", "/live")), RouteAction::Continue);
    assert_eq!(table.log().len(), 2);
    assert_eq!(table.log()[0].sequence, 0);
    assert_eq!(table.log()[1].action, RouteAction::Continue);
}

#[test]
fn handle_logs_request_headers_and_body() {
    let mut table = RouteTable::default();
    let request = request("post", "/submit")
        .with_headers(vec![("x-test".into(), "yes".into())])
        .with_body("payload");

    assert_eq!(table.handle(request), RouteAction::Continue);
    let entry = table.log().last().unwrap();
    assert_eq!(entry.method, "POST");
    assert_eq!(entry.url, "/submit");
    assert_eq!(entry.headers, vec![("x-test".into(), "yes".into())]);
    assert_eq!(entry.body.as_deref(), Some("payload"));
}
