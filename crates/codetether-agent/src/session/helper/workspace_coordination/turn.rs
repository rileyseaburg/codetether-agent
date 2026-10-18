//! Native mux services for the shared prompt lease lifetime.
pub(in crate::session::helper) struct LeaseTurn(
    codetether_workspace_coordination::LeaseTurn<super::host::NativeHost>,
);
impl LeaseTurn {
    pub(in crate::session::helper) fn open(owner: &str) -> Self {
        Self(codetether_workspace_coordination::LeaseTurn::open(
            std::sync::Arc::new(super::host::NativeHost),
            owner,
        ))
    }
    pub(in crate::session::helper) async fn close(self) {
        self.0.close().await;
    }
}
