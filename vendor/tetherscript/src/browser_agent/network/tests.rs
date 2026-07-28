use super::*;

#[path = "log_tests.rs"]
mod log_tests;
#[path = "runtime_abort_tests.rs"]
mod runtime_abort_tests;
#[path = "runtime_precedence_tests.rs"]
mod runtime_precedence_tests;
#[path = "runtime_tests.rs"]
mod runtime_tests;
#[path = "runtime_xhr_tests.rs"]
mod runtime_xhr_tests;
#[path = "security_runtime_tests.rs"]
mod security_runtime_tests;

fn request(method: &str, url: &str) -> RouteRequest {
    RouteRequest::new(method, url)
}

#[test]
fn matches_method_and_url_patterns() {
    let post_api = RouteRule::new(RoutePattern::substring("/api/")).method("post");
    assert!(post_api.matches(&request("POST", "https://x.test/api/items")));
    assert!(!post_api.matches(&request("GET", "https://x.test/api/items")));
    assert!(RoutePattern::glob("**/assets/*.js").matches("https://x/assets/app.js"));
    assert!(!RoutePattern::glob("**/assets/*.js").matches("https://x/assets/app.css"));
}

#[test]
fn newest_matching_route_wins() {
    let mut table = RouteTable::default();
    table.add(
        RouteRule::new(RoutePattern::substring("/api/")),
        RouteAction::abort("generic"),
    );
    let specific = table.add(
        RouteRule::new(RoutePattern::glob("**/api/users")),
        RouteAction::fulfill(200, "users"),
    );
    let hit = table.match_request(&request("GET", "https://x.test/api/users"));
    assert_eq!(hit.map(|route| route.id), Some(specific));
}

#[test]
fn remove_restores_older_route() {
    let mut table = RouteTable::default();
    let first = table.add(RouteRule::new(RoutePattern::Any), RouteAction::Continue);
    let second = table.add(
        RouteRule::new(RoutePattern::Any),
        RouteAction::abort("offline"),
    );
    assert_eq!(table.remove(second).map(|route| route.id), Some(second));
    let hit = table.match_request(&request("GET", "https://x.test"));
    assert_eq!(hit.map(|route| route.id), Some(first));
}
