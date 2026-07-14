use chrono::Utc;
use tokio::net::TcpListener;

use crate::mux::model::MuxSnapshot;
use crate::mux::protocol::{ClientRequest, ServerResponse, read_frame, write_frame};
use crate::mux::registry::MuxRecord;

#[tokio::test]
async fn protocol_one_server_can_be_stopped() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        assert!(matches!(
            read_frame(&mut stream).await.unwrap(),
            Some(ClientRequest::Authenticate { .. })
        ));
        write_frame(&mut stream, &ServerResponse::Authenticated { version: 1 })
            .await
            .unwrap();
        assert!(matches!(
            read_frame(&mut stream).await.unwrap(),
            Some(ClientRequest::Shutdown)
        ));
        write_frame(&mut stream, &ServerResponse::ShuttingDown)
            .await
            .unwrap();
    });
    let record = MuxRecord {
        name: "legacy".into(),
        address,
        token: "secret".into(),
        pid: 1,
        started_at: Utc::now(),
        state: MuxSnapshot::new("legacy".into(), std::env::temp_dir()),
    };
    let response = super::shutdown::request(&record).await.unwrap();
    assert!(matches!(response, ServerResponse::ShuttingDown));
    server.await.unwrap();
}
