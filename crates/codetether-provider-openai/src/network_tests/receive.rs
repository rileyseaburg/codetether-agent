//! Read the JSON request submitted to a loopback fixture.

use super::*;
use std::io::Read;

pub(super) fn receive(socket: &mut std::net::TcpStream) -> Recorded {
    let mut bytes = Vec::new();
    let (end, length) = loop {
        let mut chunk = [0; 1024];
        let read = socket.read(&mut chunk).unwrap();
        assert!(read > 0, "request ended early");
        bytes.extend_from_slice(&chunk[..read]);
        if let Some(offset) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            let end = offset + 4;
            let headers = String::from_utf8_lossy(&bytes[..end]);
            let length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().unwrap())
                })
                .unwrap_or(0);
            if bytes.len() >= end + length {
                break (end, length);
            }
        }
    };
    let headers = String::from_utf8(bytes[..end].to_vec()).unwrap();
    let path = headers.split_whitespace().nth(1).unwrap().to_owned();
    let body = if length == 0 {
        Value::Null
    } else {
        serde_json::from_slice(&bytes[end..end + length]).unwrap()
    };
    Recorded {
        path,
        headers,
        body,
    }
}
