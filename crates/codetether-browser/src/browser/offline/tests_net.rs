//! Live-HTTP integration tests using an in-process TCP listener.

use std::net::TcpListener;
use std::thread;

use super::auth_trace;
mod server;
use server::serve_two_hop;

#[test]
fn auth_trace_follows_redirect_and_captures_cookies() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let port = listener.local_addr().expect("local addr").port();
    thread::spawn(move || serve_two_hop(listener));

    let trace_json =
        auth_trace::run(&format!("http://127.0.0.1:{port}/login"), 5).expect("auth_trace run");
    let trace: serde_json::Value = serde_json::from_str(&trace_json).expect("parse trace json");
    assert_eq!(trace["redirect_count"], 1);
    assert_eq!(trace["truncated"], false);
    assert_eq!(
        trace["final_url"],
        format!("http://127.0.0.1:{port}/dashboard")
    );
    let steps = trace["steps"].as_array().expect("steps array");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0]["status"], 302);
    assert_eq!(steps[1]["status"], 200);
    let sent_to_dashboard = steps[1]["cookie_header_sent"].as_str().unwrap_or("");
    assert!(
        sent_to_dashboard.contains("session=abc123"),
        "second hop should send session cookie back: got {sent_to_dashboard:?}"
    );
    let cookies = trace["cookies_after"]
        .as_array()
        .expect("cookies_after array");
    assert!(cookies.iter().any(|c| c["name"] == "session"));
}
