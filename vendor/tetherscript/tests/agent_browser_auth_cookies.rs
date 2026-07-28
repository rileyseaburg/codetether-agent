use tetherscript::browser_agent::{
    BrowserPage, RouteAction, RouteFulfillment, RoutePattern, RouteRule,
};
use tetherscript::js::JsValue;

fn login_response() -> RouteAction {
    RouteAction::Fulfill(RouteFulfillment {
        status: 200,
        headers: vec![(
            "set-cookie".into(),
            "sid=abc; Path=/; HttpOnly; Secure; SameSite=Lax".into(),
        )],
        body: "login-ok".into(),
    })
}

fn profile_response() -> RouteAction {
    RouteAction::Fulfill(RouteFulfillment {
        status: 200,
        headers: vec![("content-type".into(), "text/plain".into())],
        body: "profile-ok".into(),
    })
}

#[test]
fn fetch_set_cookie_authenticates_following_fetch() {
    let mut page = BrowserPage::from_html("https://app.test/login", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::substring("/api/login")),
        login_response(),
    );
    page.route(
        RouteRule::new(RoutePattern::substring("/api/profile")),
        profile_response(),
    );

    page.eval_js("window.out=''; fetch('/api/login',{method:'post'}).then(function(){ fetch('/api/profile').then(function(r){ r.text().then(function(t){ window.out=t; }); }); });").unwrap();

    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("profile-ok".into())
    );
    assert_eq!(
        page.eval_js("document.cookie").unwrap(),
        JsValue::String(String::new())
    );
    assert_eq!(
        page.session.cookie_header("https://app.test/api/profile"),
        "sid=abc"
    );
    assert!(profile_cookie_header(&page).contains(&("cookie".into(), "sid=abc".into())));
}

#[test]
fn xhr_set_cookie_authenticates_following_xhr() {
    let mut page = BrowserPage::from_html("https://app.test/login", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::substring("/api/login")),
        login_response(),
    );
    page.route(
        RouteRule::new(RoutePattern::substring("/api/profile")),
        profile_response(),
    );

    page.eval_js("window.out=''; let a=XMLHttpRequest(); a.onload=function(){ let b=XMLHttpRequest(); b.onload=function(){ window.out=b.responseText; }; b.open('get','/api/profile'); b.send(); }; a.open('post','/api/login'); a.send('body');").unwrap();

    assert_eq!(
        page.eval_js("window.out").unwrap(),
        JsValue::String("profile-ok".into())
    );
    assert!(profile_cookie_header(&page).contains(&("cookie".into(), "sid=abc".into())));
}

#[test]
fn document_cookie_write_authenticates_following_fetch() {
    let mut page = BrowserPage::from_html("https://app.test/login", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::substring("/api/profile")),
        profile_response(),
    );

    page.eval_js("document.cookie='sid=doc; Path=/; Secure'; fetch('/api/profile');")
        .unwrap();

    assert!(profile_cookie_header(&page).contains(&("cookie".into(), "sid=doc".into())));
}

fn profile_cookie_header(page: &BrowserPage) -> Vec<(String, String)> {
    page.route_log()
        .into_iter()
        .find(|entry| entry.url.ends_with("/api/profile"))
        .map(|entry| entry.headers)
        .unwrap_or_default()
}
