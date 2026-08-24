use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

pub(super) fn listener() -> TcpListener {
    let listener = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    listener
}

pub(super) fn command(listener: &TcpListener) -> String {
    format!(
        "printf probe >/dev/tcp/127.0.0.1/{}",
        listener.local_addr().expect("listener address").port()
    )
}

pub(super) async fn received(listener: &TcpListener) -> bool {
    let deadline = tokio::time::Instant::now() + Duration::from_millis(300);
    while tokio::time::Instant::now() < deadline {
        if let Ok((mut stream, _)) = listener.accept() {
            return payload(&mut stream) == b"probe";
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    false
}

fn payload(stream: &mut TcpStream) -> Vec<u8> {
    let mut payload = Vec::new();
    stream.read_to_end(&mut payload).expect("probe payload");
    payload
}