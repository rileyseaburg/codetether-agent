//! Best-effort release of one prompt's mux leases.

pub(super) async fn run(owner: &str) {
    if let Err(error) = crate::mux::coordination::release(owner).await {
        tracing::warn!(owner, error = %error, "Mux lease release failed");
    }
}

pub(super) fn spawn(owner: String) {
    let Ok(runtime) = tokio::runtime::Handle::try_current() else {
        tracing::warn!(
            owner,
            "Mux lease release deferred to expiry without a runtime"
        );
        return;
    };
    runtime.spawn(async move { run(&owner).await });
}
