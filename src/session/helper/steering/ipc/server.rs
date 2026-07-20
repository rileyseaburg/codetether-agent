//! Accept session-addressed steering connections.

use tokio::net::UnixListener;

pub(super) async fn run(listener: UnixListener, session_id: String) {
    loop {
        let Ok((stream, _)) = listener.accept().await else {
            return;
        };
        let session_id = session_id.clone();
        tokio::spawn(async move {
            if let Err(error) = super::connection::handle(stream, session_id).await {
                tracing::warn!(%error, "Session steering connection failed");
            }
        });
    }
}
