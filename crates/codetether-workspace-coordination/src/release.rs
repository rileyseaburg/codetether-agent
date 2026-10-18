//! Best-effort release of one prompt's mux leases.

pub(super) async fn run<H: crate::LeaseHost>(host: &H, owner: &str) {
    if let Err(error) = host.release(owner).await {
        tracing::warn!(target: "codetether_agent::session::helper::workspace_coordination::turn::release",owner, error = %error, "Mux lease release failed");
    }
}

pub(super) fn spawn<H: crate::LeaseHost>(host: std::sync::Arc<H>, owner: String) {
    let Ok(runtime) = tokio::runtime::Handle::try_current() else {
        tracing::warn!(target: "codetether_agent::session::helper::workspace_coordination::turn::release",
            owner,
            "Mux lease release deferred to expiry without a runtime"
        );
        return;
    };
    runtime.spawn(async move { run(host.as_ref(), &owner).await });
}
