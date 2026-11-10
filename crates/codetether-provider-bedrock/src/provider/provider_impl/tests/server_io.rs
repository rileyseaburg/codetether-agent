//! Blocking HTTP fixture writer; the tail remains gated until the reader allows it.
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc::Receiver,
    time::Duration,
};

pub(super) fn serve(listener: TcpListener, first: Vec<u8>, tail: Vec<u8>, gate: Receiver<()>) {
    let (mut socket, _) = listener.accept().unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    socket
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut request = Vec::new();
    while !request.windows(4).any(|p| p == b"\r\n\r\n") {
        let mut bytes = [0; 1024];
        let n = socket.read(&mut bytes).unwrap();
        assert!(n > 0);
        request.extend_from_slice(&bytes[..n]);
    }
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/vnd.amazon.eventstream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        first.len() + tail.len()
    );
    socket.write_all(header.as_bytes()).unwrap();
    let split = first.len() / 2;
    socket.write_all(&first[..split]).unwrap();
    socket.flush().unwrap();
    socket.write_all(&first[split..]).unwrap();
    if gate.recv_timeout(Duration::from_secs(5)).is_ok() {
        socket.write_all(&tail).unwrap();
    }
}
