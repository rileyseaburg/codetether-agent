//! A zero redirect limit still records the first response and its cookies.
use std::io::{BufRead, BufReader, Write};

#[test]
fn zero_redirect_limit_keeps_the_first_hop_and_marks_truncation() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/login", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(&stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if line == "\r\n" || line.is_empty() {
                break;
            }
        }
        stream.write_all(b"HTTP/1.1 302 Found\r\nLocation: /next\r\nSet-Cookie: a=1; Path=/\r\nContent-Length: 0\r\n\r\n").unwrap();
    });
    let raw = super::run(&url, 0).unwrap();
    server.join().unwrap();
    let trace: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(trace["final_url"], url);
    assert_eq!(trace["redirect_count"], 0);
    assert_eq!(trace["truncated"], true);
    assert_eq!(trace["steps"].as_array().unwrap().len(), 1);
    assert_eq!(trace["steps"][0]["status"], 302);
    assert_eq!(trace["steps"][0]["location"], "/next");
    assert!(trace["steps"][0].get("cookie_header_sent").is_none());
    assert_eq!(trace["cookies_after"][0]["name"], "a");
}
