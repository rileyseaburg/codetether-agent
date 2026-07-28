//! Network replay and CORS debugging unit tests.

use super::*;
use super::capture::CapturedExchange;
use super::cors_debug::{analyze_cors, CorsBlockReason};
use super::diff::diff_captures;
use super::har::export_har;
use super::mock::{MockRequest, MockServer};
use super::replay::{ReplayOptions, ReplayPlan};

fn ex() -> CapturedExchange {
    CapturedExchange::new(1, "GET", "https://api/x")
        .with_response(200, vec![("access-control-allow-origin".into(), "https://app".into())], b"ok".to_vec())
}

#[test]
fn cors_reports_mismatch() {
    let mut e = ex(); e.request.headers.push(("origin".into(), "https://evil".into()));
    let c = analyze_cors(&e, None);
    assert!(c.blocked); assert_eq!(c.reason, CorsBlockReason::AllowOriginMismatch);
}

#[test]
fn replay_is_deterministic_and_mutable() {
    let mut p = ReplayPlan::new(vec![ex()]);
    let r = p.next(ReplayOptions { headers: vec![("x-test".into(), "1".into())],
        body: Some(b"b".to_vec()), delay_ms: 7 }).unwrap();
    assert_eq!(r.delay_ms, 7);
}

#[test]
fn diff_finds_response_body_change() {
    let a = vec![ex()]; let mut b = ex(); b.response.as_mut().unwrap().body = b"new".to_vec();
    let d = diff_captures(&a, &[b]); assert_eq!(d.changed[0].fields, vec!["response.body"]);
}

#[test]
fn mock_serves_capture() {
    let s = MockServer::new(vec![ex()]);
    let r = s.respond_or_404(&MockRequest { method: "GET".into(), url: "https://api/x".into(), headers: vec![], body: vec![] });
    assert_eq!(r.status, 200);
}

#[test]
fn har_contains_url() { assert!(export_har(&[ex()]).contains("https://api/x")); }
