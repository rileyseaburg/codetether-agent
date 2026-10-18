//! Response extraction and clean proof-daemon shutdown.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

pub(super) fn state(response: ServerResponse) -> crate::mux::model::MuxSnapshot {
    let ServerResponse::Snapshot { state } = response else {
        panic!("expected snapshot");
    };
    state
}

pub(super) async fn shutdown(client: &mut MuxConnection, child: &mut tokio::process::Child) {
    assert!(matches!(
        client.request(ClientRequest::Shutdown).await.unwrap(),
        ServerResponse::ShuttingDown
    ));
    assert!(child.wait().await.unwrap().success());
}
