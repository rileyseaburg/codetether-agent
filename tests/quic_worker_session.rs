//! Phase 6 live integration: full worker↔server QUIC session path.
//!
//! Exercises the production wiring building blocks end-to-end on loopback:
//! `QuicListener` (server accept loop) ← `dial_session` (worker dial with resume
//! cursor) → `QuicFrameReader`. Proves the framed session delivers events and
//! honors the durable resume floor over a real QUIC connection.

use codetether_agent::a2a::stream::cursor::Cursor;
use codetether_agent::a2a::stream::event_id::EventId;
use codetether_agent::a2a::stream::frame::ParsedFrame;
use codetether_agent::a2a::stream::quic::{
    QuicFrameWriter, QuicListener, QuicStreamClient, QuicStreamServer, dial_session,
};
use std::time::Duration;
use tokio::time::timeout;

fn frame(seq: u64, data: &str) -> ParsedFrame {
    ParsedFrame {
        id: Some(EventId {
            epoch: "ep".into(),
            seq,
        }),
        event: "progress".into(),
        data: data.into(),
    }
}

#[tokio::test]
async fn quic_worker_session_streams_with_resume() {
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
    let listener = QuicListener::new(server);

    // Server: accept one connection via the listener loop, then stream seq 1..=5.
    let server_task = tokio::spawn(async move {
        let conn = listener.accept_connection().await.unwrap().unwrap();
        let (send, mut recv) = conn.accept_bi().await.unwrap();
        // Drain the worker's 4-byte prime.
        let mut prime = [0u8; 4];
        recv.read_exact(&mut prime).await.unwrap();
        let mut w = QuicFrameWriter::new(send);
        for seq in 1..=5 {
            w.write_frame(&frame(seq, &format!("v{seq}")))
                .await
                .unwrap();
        }
        w.finish().unwrap();
        conn.closed().await;
    });

    // Worker: durable cursor already committed seq 3 → expect only 4,5.
    let dir = tempfile::tempdir().unwrap();
    let mut cursor = Cursor::load(dir.path().join("cursor"));
    cursor
        .commit(&EventId {
            epoch: "ep".into(),
            seq: 3,
        })
        .unwrap();

    let client = QuicStreamClient::with_roots(vec![cert_der]).unwrap();
    let mut session = dial_session(&client, addr, "localhost", &cursor)
        .await
        .unwrap();

    let mut got = Vec::new();
    while let Some(f) = timeout(Duration::from_secs(5), session.reader.next_frame())
        .await
        .expect("frame within timeout")
        .unwrap()
    {
        got.push(f.id.unwrap().seq);
    }
    assert_eq!(got, vec![4, 5], "resume floor from cursor must skip seq<=3");

    session.connection.close(0u32.into(), b"done");
    let _ = server_task.await;
}
