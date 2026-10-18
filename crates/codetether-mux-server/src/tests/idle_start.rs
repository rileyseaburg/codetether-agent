//! Start the idle program used by the manual benchmark.
use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest};

pub(super) async fn start(client: &mut MuxConnection) {
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
}
