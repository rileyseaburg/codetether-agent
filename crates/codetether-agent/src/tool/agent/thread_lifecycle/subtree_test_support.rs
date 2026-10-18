use super::super::{collaboration_runtime::thread_status, persistence, store};
use crate::session::Session;

/// Create a persisted-session fixture with the requested parent edge.
pub(super) async fn entry(name: &str, owner: &str, depth: u8) -> store::AgentEntry {
    let session = Session::new().await.unwrap();
    session.save().await.unwrap();
    store::AgentEntry {
        name: name.into(),
        instructions: format!("run {name}"),
        session,
        parent: None,
        owner_session_id: Some(owner.into()),
        depth,
        model_id: Some("test/model".into()),
    }
}

/// Persist and register an open fixture entry.
pub(super) async fn register(entry: &store::AgentEntry) {
    persistence::save(&entry.name, entry).await.unwrap();
    store::insert(entry.clone());
    thread_status::restored(entry.id());
}

/// Remove fixture persistence, runtime state, and status channels.
pub(super) async fn cleanup(entries: &[store::AgentEntry]) {
    for entry in entries {
        persistence::remove(entry).await.unwrap();
        store::remove(entry.id());
        super::super::execution_state::reopen(entry.id());
        thread_status::remove(entry.id());
    }
}
