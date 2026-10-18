//! Native specialization of shared startup session resolution.
use crate::{
    bus::AgentBus,
    session::{Session, TailLoad},
};
use anyhow::Result;
use std::sync::Arc;
pub(super) type Resolved = codetether_tui_startup::Resolved<Session>;
pub(super) async fn resolve(
    scan: Option<Result<TailLoad>>,
    bus: &Arc<AgentBus>,
) -> Result<Resolved> {
    codetether_tui_startup::resolve::resolve::<super::session_resolve_host::Native>(scan, bus).await
}
