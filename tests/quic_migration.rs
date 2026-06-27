//! Phase 6 acceptance experiment: QUIC connection migration.
//!
//! Plan acceptance (docs/transport-first-class-plan.md, Phase 6): "Force a
//! path/IP change; confirm connection migration keeps the session alive
//! without a cold resync." This rebinds the client endpoint to a new local
//! UDP port mid-session and confirms an already-open connection still carries
//! a fresh bidirectional stream to completion.

use codetether_agent::a2a::stream::quic::{QuicStreamClient, QuicStreamServer};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn quic_connection_survives_local_path_change() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = rustls::pki_types::CertificateDer::from(cert.cert.der().to_vec());
    let key_der = rustls::pki_types::PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der()).into();

    let server = QuicStreamServer::bind(
        "127.0.0.1:0".parse().unwrap(),
        vec![cert_der.clone()],
        key_der,
    )
    .unwrap();
    let addr = server.local_addr().unwrap();

    // Server: accept connection, then echo every bi stream that arrives.
    let server_task = tokio::spawn(async move {
        let conn = server
            .accept()
            .await
            .unwrap()
            .accept()
            .unwrap()
            .await
            .unwrap();
        loop {
            match conn.accept_bi().await {
                Ok((mut send, mut recv)) => {
                    let buf = recv.read_to_end(64).await.unwrap();
                    send.write_all(&buf).await.unwrap();
                    send.finish().unwrap();
                }
                Err(_) => break,
            }
        }
    });

    let client = QuicStreamClient::with_roots(vec![cert_der]).unwrap();
    let conn = client.connect(addr, "localhost").await.unwrap();

    // First round-trip on the original path.
    let (mut s1, mut r1) = conn.open_bi().await.unwrap();
    s1.write_all(b"pre-migrate").await.unwrap();
    s1.finish().unwrap();
    assert_eq!(r1.read_to_end(64).await.unwrap(), b"pre-migrate");

    // Force a local path change (new source UDP port) mid-session.
    client.migrate_local_port().unwrap();

    // The same connection must keep working without a cold resync.
    let (mut s2, mut r2) = conn.open_bi().await.unwrap();
    s2.write_all(b"post-migrate").await.unwrap();
    s2.finish().unwrap();
    let echoed = timeout(Duration::from_secs(5), r2.read_to_end(64))
        .await
        .expect("connection must survive the local path change")
        .unwrap();
    assert_eq!(echoed, b"post-migrate");

    conn.close(0u32.into(), b"done");
    let _ = server_task.await;
}
