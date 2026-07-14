//! Continuous anticipatory RLM preparation for session context.

mod capacity;
mod commit;
mod fingerprint;
mod freshness;
#[cfg(test)]
mod freshness_tests;
mod inherit;
mod merge;
mod ranges;
#[cfg(test)]
mod ranges_tests;
mod registry;
mod registry_status;
mod runtime;
mod schedule;
mod slot;
#[cfg(test)]
mod slot_tests;
mod store;
#[cfg(test)]
mod store_tests;
mod summarize;
mod types;
mod worker;

use crate::provider::Provider;
use crate::session::Session;
use std::sync::Arc;

pub(crate) use merge::prepared_index;

pub(crate) fn register(session: &Session, provider: Arc<dyn Provider>, model: &str) {
    registry::register(
        session.id.clone(),
        runtime::resolve(session, provider, model),
    );
}

pub(crate) fn prepare(session: &Session, provider: Arc<dyn Provider>, model: &str) {
    register(session, provider, model);
    schedule(session);
}

pub(crate) fn schedule(session: &Session) {
    schedule::run(session);
}

pub(crate) async fn remove(session_id: &str) {
    registry_status::remove(session_id);
    store::remove(session_id).await;
}
