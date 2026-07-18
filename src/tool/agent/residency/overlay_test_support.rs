use crate::tool::agent::{persistence, session_factory, store, thread_lifecycle};
use std::path::PathBuf;

pub(super) async fn closed(owner: &str, workspace: PathBuf) -> String {
    let id = persisted(owner, workspace).await;
    super::super::persistence::hydrate_parent(Some(owner))
        .await
        .unwrap();
    thread_lifecycle::close(&id, Some(owner)).await.unwrap();
    id
}

pub(super) async fn persisted(owner: &str, workspace: PathBuf) -> String {
    let session = session_factory::create_agent_session(
        "worker",
        "test runtime overlay",
        "old/model",
        Some(workspace),
        true,
    )
    .await
    .unwrap();
    session.save().await.unwrap();
    let id = session.id.clone();
    let entry = store::AgentEntry {
        name: "worker".into(),
        instructions: "test runtime overlay".into(),
        session,
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 0,
        model_id: Some("old/model".into()),
    };
    persistence::save(&entry.name, &entry).await.unwrap();
    id
}
