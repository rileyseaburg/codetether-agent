//! Auth debugging unit tests.

use super::cookie_tracker::CookieTracker;
use super::flow_recorder::AuthFlowRecorder;
use super::jwt_decode::decode_jwt;
use super::session_state::CookieSnapshot;
use super::token_refresh::TokenRefreshTracker;

fn jwt() -> &'static str { "eyJhbGciOiJub25lIn0.eyJpc3MiOiJpZHAiLCJzdWIiOiJ1MSIsImV4cCI6MjAwfQ." }

#[test]
fn decodes_jwt_claims() {
    let t = decode_jwt("test", jwt()).unwrap();
    assert_eq!(t.issuer.as_deref(), Some("idp"));
    assert_eq!(t.subject.as_deref(), Some("u1"));
    assert_eq!(t.expires_at, Some(200));
}

#[test]
fn diffs_cookie_login_state() {
    let mut ct = CookieTracker::new();
    ct.capture_before(vec![]);
    ct.capture_after(vec![CookieSnapshot {
        name: "sid".into(), value: "1".into(), domain: "a".into(),
        path: "/".into(), secure: true, httponly: true, samesite: None, expires: None,
    }]);
    assert_eq!(ct.diff().len(), 1);
}

#[test]
fn records_redirect_flow() {
    let mut f = AuthFlowRecorder::new();
    f.login_form("/login", vec!["user".into(), "pass".into()]);
    f.redirect("/login", "/callback", 302, vec![]);
    f.final_page("/app", true);
    assert_eq!(f.redirects().len(), 1);
    assert_eq!(f.events().len(), 3);
}

#[test]
fn detects_refresh() {
    let mut tr = TokenRefreshTracker::new();
    let old = decode_jwt("auth", jwt()).unwrap();
    tr.observe(vec![old]);
    let new = decode_jwt("auth", "eyJhbGciOiJub25lIn0.eyJpc3MiOiJpZHAiLCJzdWIiOiJ1MSIsImV4cCI6MzAwfQ.").unwrap();
    assert_eq!(tr.observe(vec![new]).len(), 1);
}
