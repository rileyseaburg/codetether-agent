//! Periodic mux lease renewal for one active prompt.

pub(super) fn start<H: crate::LeaseHost>(
    host: std::sync::Arc<H>,
    owner: String,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            if let Err(error) = host.renew(&owner).await {
                tracing::warn!(target: "codetether_agent::session::helper::workspace_coordination::turn::renewal",owner, error = %error, "Mux lease renewal failed");
            }
        }
    })
}
