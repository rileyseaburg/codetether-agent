//! Native source and storage bindings for shared proactive context.
mod store;
mod worker;
use crate::{provider::Provider, session::Session};
use std::sync::Arc;
pub(crate) use worker::prepared_index;
pub(crate) fn register(session: &Session, provider: Arc<dyn Provider>, model: &str) {
    codetether_proactive_context::register(session, provider, model);
}
pub(crate) fn prepare(session: &Session, provider: Arc<dyn Provider>, model: &str) {
    codetether_proactive_context::prepare(session, provider, model, worker::spawn);
}
pub(crate) fn schedule(session: &Session) {
    codetether_proactive_context::schedule(session, worker::spawn);
}
pub(crate) async fn remove(session_id: &str) {
    worker::remove(session_id).await;
}
