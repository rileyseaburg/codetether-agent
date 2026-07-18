use super::{EnsureOpen, ResumeConfig, open};
use crate::session::Session;
use crate::tool::agent::{persistence, store, thread_lifecycle};

#[tokio::test]
async fn closed_child_is_reloaded_with_the_same_identity() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let child = Session::new().await.unwrap();
    child.save().await.unwrap();
    let id = child.id.clone();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let name = format!("child-{id}");
    let entry = crate::tool::agent::store::AgentEntry {
        name: name.clone(),
        instructions: "resume".into(),
        session: child,
        parent: None,
        owner_session_id: Some(owner.clone()),
        depth: 0,
        model_id: None,
    };
    persistence::save(&name, &entry).await.unwrap();
    store::insert(entry.clone());
    assert!(
        thread_lifecycle::close(&id, Some(&owner))
            .await
            .unwrap()
            .success
    );
    assert!(store::get(&id).is_none());
    let EnsureOpen::Ready {
        agent_id, resumed, ..
    } = open(&id, Some(&owner), ResumeConfig::default())
        .await
        .unwrap()
    else {
        panic!("closed child should reload");
    };
    assert!(resumed);
    assert_eq!(agent_id, id);
    let loaded = store::get(&id).unwrap();
    persistence::remove(&loaded).await.unwrap();
    store::remove(&id);
}
