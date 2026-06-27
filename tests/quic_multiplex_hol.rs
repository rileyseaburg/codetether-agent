//! Phase 6 acceptance experiment: QUIC multiplex head-of-line freedom.
//!
//! Plan acceptance (docs/transport-first-class-plan.md, Phase 6): "multiplex
//! two logical streams; stall one (app-level), confirm the other keeps
//! delivering (no HoL)." This runs the experiment on a loopback QUIC endpoint
//! with a self-signed certificate.

use codetether_agent::a2a::stream::quic::{QuicStreamClient, QuicStreamServer};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn quic_multiplex_has_no_head_of_line_blocking() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Self-signed loopback cert.
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = rustls::pki_types::CertificateDer::from(cert.cert.der().to_vec());
    let key_der = rustls::pki_types::PrivatePkcs8KeyDer::from(
        cert.key_pair.serialize_der(),
    )
    .into();

    let server = QuicStreamServer::bind("127.0.0.1:0".parse().unwrap(), vec![cert_der.clone()], key_der)
        .unwrap();
    let addr = server.local_addr().unwrap();

    // Server: accept one connection, accept two bi streams, echo on each.
    let server_task = tokio::spawn(async move {
        let conn = server.accept().await.unwrap().accept().unwrap().await.unwrap();
        for _ in 0..2 {
            let (mut send, mut recv) = conn.accept_bi().await.unwrap();
            tokio::spawn(async move {
                let buf = recv.read_to_end(64).await.unwrap();
                send.write_all(&buf).await.unwrap();
                send.finish().unwrap();
            });
        }
        // Keep the connection alive until the client drives both streams.
        conn.closed().await;
    });

    let client = QuicStreamClient::with_roots(vec![cert_der]).unwrap();
    let conn = client.connect(addr, "localhost").await.unwrap();

    // Stream A: open but deliberately stall (no write) — must not block B.
    let (_send_a, mut _recv_a) = conn.open_bi().await.unwrap();

    // Stream B: write + read echo. Independent of A's stall.
    let (mut send_b, mut recv_b) = conn.open_bi().await.unwrap();
    send_b.write_all(b"hello-B").await.unwrap();
    send_b.finish().unwrap();
    let echoed = timeout(Duration::from_secs(5), recv_b.read_to_end(64))
        .await
        .expect("stream B must deliver even while stream A is stalled")
        .unwrap();
    assert_eq!(echoed, b"hello-B");

    conn.close(0u32.into(), b"done");
    let _ = server_task.await;
}
