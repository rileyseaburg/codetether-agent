//! Serve one response per connection and retain the received requests.

use super::*;
use std::io::Write;

pub(super) fn serve(replies: Vec<Reply>) -> (String, std::thread::JoinHandle<Vec<Recorded>>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/v1", listener.local_addr().unwrap());
    let handle = std::thread::spawn(move || {
        let mut records = Vec::new();
        for reply in replies {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .unwrap();
            records.push(super::receive::receive(&mut socket));
            let length = reply.declared_len.unwrap_or(reply.body.len());
            let headers = format!(
                "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n",
                reply.status, reply.content_type
            );
            socket
                .write_all(format!("{headers}{}", reply.body).as_bytes())
                .unwrap();
        }
        records
    });
    (url, handle)
}
