//! Manual end-to-end benchmark for an idle attached PTY.

use std::time::{Duration, Instant};

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest};

#[tokio::test]
#[ignore = "manual performance benchmark"]
async fn idle_read_requests_one_second() {
    let workspace = tempfile::tempdir().unwrap();
    let (target, context, server) = super::pty_support::server(workspace.path().into()).await;
    let mut client = MuxConnection::connect(&target).await.unwrap();
    super::idle_start::start(&mut client).await;
    let started = Instant::now();
    let mut requests = 0_u64;
    while started.elapsed() < Duration::from_secs(1) {
        client
            .request(ClientRequest::Program {
                request: ProgramRequest::Read {
                    window_id: 0,
                    offset: 0,
                },
            })
            .await
            .unwrap();
        requests += 1;
        tokio::time::sleep(Duration::from_millis(16)).await;
    }
    println!(
        "BENCH mux_idle elapsed_ms={} read_requests={requests}",
        started.elapsed().as_millis()
    );
    client.request(ClientRequest::Detach).await.unwrap();
    let mut closer = MuxConnection::connect(&target).await.unwrap();
    closer.request(ClientRequest::Detach).await.unwrap();
    context.programs.stop_all();
    server.await.unwrap();
}
