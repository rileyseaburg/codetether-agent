//! Manual end-to-end benchmark for an idle attached PTY.

use std::time::{Duration, Instant};

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest};

#[tokio::test]
#[ignore = "manual performance benchmark"]
async fn idle_read_requests_one_second() {
    let workspace = tempfile::tempdir().unwrap();
    let (record, context, server) = super::pty_support::server(workspace.path().into()).await;
    let mut client = MuxConnection::connect(&record).await.unwrap();
    client
        .request(ClientRequest::Program {
            request: ProgramRequest::Start {
                window_id: 0,
                command: "sleep 1".into(),
                columns: 80,
                rows: 24,
            },
        })
        .await
        .unwrap();
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
    let mut closer = MuxConnection::connect(&record).await.unwrap();
    closer.request(ClientRequest::Detach).await.unwrap();
    context.programs.stop_all();
    server.await.unwrap();
}
