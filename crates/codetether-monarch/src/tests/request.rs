//! Read one local fixture HTTP request without contacting external services.
use serde_json::Value;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub(super) async fn read_request(stream: &mut TcpStream) -> (String, Value) {
    let mut bytes = Vec::new();
    loop {
        let mut chunk = [0; 4096];
        let n = stream.read(&mut chunk).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&chunk[..n]);
        if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            let header = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
            let length: usize = header
                .lines()
                .find_map(|line| line.strip_prefix("content-length:"))
                .map(|n| n.trim().parse().unwrap())
                .unwrap_or(0);
            if bytes.len() >= end + 4 + length {
                let body = if length == 0 {
                    Value::Null
                } else {
                    serde_json::from_slice(&bytes[end + 4..end + 4 + length]).unwrap()
                };
                return (header, body);
            }
        }
    }
}
