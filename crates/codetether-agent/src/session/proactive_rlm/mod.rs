//! Native proactive services with application session access.
mod session;
#[cfg(test)]
pub(crate) async fn remove(session_id: &str) {
    codetether_native_proactive::remove::<crate::session::Session>(session_id).await;
}
#[cfg(not(test))]
pub(crate) use codetether_native_proactive::{prepare, prepared_index};

#[cfg(test)]
include!("mod_test_imports.rs");
