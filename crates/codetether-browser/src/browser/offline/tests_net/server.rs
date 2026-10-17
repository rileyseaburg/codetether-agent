//! Two-hop loopback server for redirect and cookie propagation tests.
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
pub(super) fn serve_two_hop(listener: TcpListener) {
    for hop in 0..2 {
        let (mut stream, _) = listener.accept().expect("accept");
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line).expect("read request line");
        loop {
            let mut h = String::new();
            reader.read_line(&mut h).expect("read header line");
            if h == "\r\n" || h.is_empty() {
                break;
            }
        }
        let response = if line.contains("/login") && hop == 0 {
            "HTTP/1.1 302 Found\r\nLocation: /dashboard\r\nSet-Cookie: session=abc123; Path=/\r\nContent-Length: 0\r\n\r\n"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nhello!\n"
        };
        stream
            .write_all(response.as_bytes())
            .expect("write response");
    }
}
