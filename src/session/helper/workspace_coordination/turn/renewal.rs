//! Periodic mux lease renewal for one active prompt.

pub(super) fn start(owner: String) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            if let Err(error) = crate::mux::coordination::renew(&owner).await {
                tracing::warn!(owner, error = %error, "Mux lease renewal failed");
            }
        }
    })
}
