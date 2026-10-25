use crate::images::http;
use std::{
    io::{Read, Write},
    net::TcpListener,
    time::Duration,
};

fn response(wire: &'static [u8]) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let _ = stream.read(&mut [0; 4096]);
        let _ = stream.write_all(wire);
    });
    format!("http://{address}/")
}

#[tokio::test]
async fn bounded_reader_rejects_chunked_and_declared_oversize() {
    let client = reqwest::Client::new();
    for wire in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 8\r\nConnection: close\r\n\r\n12345678".as_slice(),
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n4\r\n1234\r\n4\r\n5678\r\n0\r\n\r\n".as_slice(),
    ] {
        let reply = client.get(response(wire)).send().await.unwrap();
        let error = http::body(reply, 6).await.err().unwrap().to_string();
        assert_eq!(error, "image_response_too_large");
    }
}

#[test]
fn append_bound_is_exact_and_preserves_existing_bytes_on_rejection() {
    let mut bytes = vec![1, 2];
    http::append(&mut bytes, &[3, 4], 4).unwrap();
    assert!(http::append(&mut bytes, &[5], 4).is_err());
    assert_eq!(bytes, [1, 2, 3, 4]);
}
