//! Phase 6 integration: Phase 1 resume semantics carried over QUIC.
//!
//! Confirms the framed QUIC session (length-delimited `id`/`event`/`data`)
//! reuses the resumable event-id contract: a reader opened with a resume floor
//! skips already-processed seqs and delivers only newer frames — the same
//! at-least-once guarantee Phase 1 proved over SSE, now on a QUIC stream.

use codetether_agent::a2a::stream::event_id::EventId;
use codetether_agent::a2a::stream::frame::ParsedFrame;
use codetether_agent::a2a::stream::quic::{
    QuicFrameReader, QuicFrameWriter, QuicStreamClient, QuicStreamServer,
};
use std::time::Duration;
use tokio::time::timeout;

fn frame(seq: u64, data: &str) -> ParsedFrame {
    ParsedFrame {
        id: Some(EventId {
            epoch: "e1".into(),
            seq,
        }),
        event: "progress".into(),
        data: data.into(),
    }
}

#[tokio::test]
async fn quic_framed_session_resumes_past_cursor() {
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

    // Server: emit frames seq 1..=4 on the first bidi stream.
    let server_task = tokio::spawn(async move {
        let conn = server
            .accept()
            .await
            .unwrap()
            .accept()
            .unwrap()
            .await
            .unwrap();
        let (send, _recv) = conn.accept_bi().await.unwrap();
        let mut w = QuicFrameWriter::new(send);
        for seq in 1..=4 {
            w.write_frame(&frame(seq, &format!("v{seq}")))
                .await
                .unwrap();
        }
        w.finish().unwrap();
        assert_eq!(w.last_seq(), Some(4));
        conn.closed().await;
    });

    let client = QuicStreamClient::with_roots(vec![cert_der]).unwrap();
    let conn = client.connect(addr, "localhost").await.unwrap();
    let (mut s, recv) = conn.open_bi().await.unwrap();
    // Open the stream so the server's accept_bi resolves.
    s.write_all(b"\x00\x00\x00\x00").await.ok();

    // Resume floor = 2: expect to receive only seq 3 and 4.
    let mut reader = QuicFrameReader::new(recv, Some(2));
    let mut got = Vec::new();
    while let Some(f) = timeout(Duration::from_secs(5), reader.next_frame())
        .await
        .expect("frame within timeout")
        .unwrap()
    {
        got.push(f.id.unwrap().seq);
    }
    assert_eq!(
        got,
        vec![3, 4],
        "frames at/below resume floor must be skipped"
    );

    conn.close(0u32.into(), b"done");
    let _ = server_task.await;
}
