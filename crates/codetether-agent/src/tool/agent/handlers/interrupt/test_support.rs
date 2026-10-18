use super::super::super::{collaboration_runtime::thread_status, execution_state, store};
use crate::session::Session;

pub(super) async fn child(owner: &str) -> store::AgentEntry {
    let session = Session::new().await.unwrap();
    session.save().await.unwrap();
    let entry = store::AgentEntry {
        name: "interrupt-child".into(),
        instructions: "test interruption".into(),
        session,
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 1,
        model_id: Some("test/model".into()),
    };
    store::insert(entry.clone());
    entry
}

pub(super) fn start(agent_id: &str) {
    let guard = execution_state::try_start(agent_id).unwrap();
    let handle = tokio::spawn(async move {
        let _guard = guard;
        std::future::pending::<()>().await;
    });
    execution_state::register(agent_id, &handle);
}

pub(super) fn cleanup(agent_id: &str) {
    store::remove(agent_id);
    execution_state::reopen(agent_id);
    thread_status::remove(agent_id);
}
