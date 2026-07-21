//! Resilient lifecycle for one mux output subscription.

use tokio::sync::mpsc::Sender;

use super::MuxLiveOutput;

pub(super) async fn follow(name: String, sender: Sender<MuxLiveOutput>) {
    while !sender.is_closed() {
        if let Err(error) = super::connection::run(&name, &sender).await {
            tracing::warn!(session = name, %error, "Mux realtime stream reconnecting");
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
