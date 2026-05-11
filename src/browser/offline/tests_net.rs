//! Live-HTTP integration tests using an in-process TCP listener.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

use super::auth_trace;

#[test]
fn auth_trace_follows_redirect_and_captures_cookies() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || serve_two_hop(listener));

    let trace_json = auth_trace::run(&format!("http://127.0.0.1:{port}/login"), 5).unwrap();
    let trace: serde_json::Value = serde_json::from_str(&trace_json).unwrap();
    assert_eq!(trace["redirect_count"], 1);
    assert_eq!(trace["final_url"], format!("http://127.0.0.1:{port}/dashboard"));
    let steps = trace["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0]["status"], 302);
    assert_eq!(steps[1]["status"], 200);
    let cookies = trace["cookies_after"].as_array().unwrap();
    assert!(cookies.iter().any(|c| c["name"] == "session"));
}

fn serve_two_hop(listener: TcpListener) {
    for _ in 0..2 {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        loop {
            let mut h = String::new();
            reader.read_line(&mut h).unwrap();
            if h == "\r\n" || h.is_empty() {
                break;
            }
        }
        let response = if line.contains("/login") {
            "HTTP/1.1 302 Found\r\nLocation: /dashboard\r\nSet-Cookie: session=abc123; Path=/\r\nContent-Length: 0\r\n\r\n"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nhello!\n"
        };
        stream.write_all(response.as_bytes()).unwrap();
    }
}
