//! Native proactive services with application session access.
mod session;
pub(crate) use codetether_native_proactive::{prepare, prepared_index, register, schedule};
pub(crate) async fn remove(session_id: &str) {
    codetether_native_proactive::remove::<crate::session::Session>(session_id).await;
}
