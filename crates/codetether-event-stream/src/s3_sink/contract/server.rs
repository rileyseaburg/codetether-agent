//! server contract support.
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};
pub(super) fn start(status: &str) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let response =
        format!("HTTP/1.1 {status}\r\nContent-Length: 6\r\nConnection: close\r\n\r\nresult");
    let handle = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(10)))
            .unwrap();
        let mut bytes = Vec::new();
        let header_end = loop {
            let mut byte = [0];
            socket.read_exact(&mut byte).unwrap();
            bytes.push(byte[0]);
            if bytes.ends_with(b"\r\n\r\n") {
                break bytes.len();
            }
        };
        let headers = String::from_utf8(bytes.clone()).unwrap();
        let length: usize = headers
            .lines()
            .find_map(|line| {
                let (key, value) = line.split_once(':')?;
                key.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse().unwrap())
            })
            .unwrap_or(0);
        bytes.resize(header_end + length, 0);
        socket.read_exact(&mut bytes[header_end..]).unwrap();
        socket.write_all(response.as_bytes()).unwrap();
        String::from_utf8(bytes).unwrap()
    });
    (endpoint, handle)
}
