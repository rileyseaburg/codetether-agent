//! Ownership of one prompt's renewal task and lease release lifetime.
use crate::LeaseHost;
use std::sync::Arc;
/// Keeps prompt leases renewed and releases them when closed or dropped.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_workspace_coordination::LeaseHost>(host: std::sync::Arc<H>) {
/// let turn = codetether_workspace_coordination::LeaseTurn::open(host, "owner");
/// turn.close().await;
/// # }
/// ```
pub struct LeaseTurn<H: LeaseHost> {
    pub(super) owner: Option<String>,
    pub(super) renewal: Option<tokio::task::JoinHandle<()>>,
    pub(super) host: Arc<H>,
}
