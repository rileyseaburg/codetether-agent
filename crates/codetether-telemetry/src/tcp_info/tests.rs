//! Tests for [`super::sample`] against a real loopback socket (Linux).

use super::sample;

#[test]
fn samples_a_live_loopback_connection() {
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::os::fd::AsRawFd;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let mut client = TcpStream::connect(addr).unwrap();
    let (mut server, _) = listener.accept().unwrap();
    // Push a little data so the kernel has populated TCP state.
    client.write_all(b"ping").unwrap();
    let _ = server.write(b"pong");

    let snap = sample(client.as_raw_fd());
    assert!(
        snap.is_some(),
        "TCP_INFO should be readable on an open conn"
    );
    // cwnd is always >= 1 segment on an established connection.
    assert!(snap.unwrap().snd_cwnd >= 1);
}

#[test]
fn invalid_fd_returns_none() {
    assert!(sample(-1).is_none());
}
