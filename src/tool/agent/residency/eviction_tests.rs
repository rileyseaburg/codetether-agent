use super::{EnsureOpen, ResumeConfig, open};
use crate::session::Session;
use crate::tool::agent::collaboration_runtime::thread_status::{self, ThreadStatus};
use crate::tool::agent::{persistence, store, thread_lifecycle};

#[tokio::test]
async fn full_residency_evicts_the_oldest_safe_child() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let target = entry("target", &owner).await;
    persistence::save(&target.name, &target).await.unwrap();
    store::insert(target.clone());
    thread_lifecycle::close(target.id(), Some(&owner))
        .await
        .unwrap();
    let mut residents = Vec::new();
    for index in 0..6 {
        let resident = entry(&format!("resident-{index}"), &owner).await;
        persistence::save(&resident.name, &resident).await.unwrap();
        thread_status::set(resident.id(), ThreadStatus::Completed(None));
        residents.push(resident.id().to_string());
        store::insert(resident);
    }
    super::touch(&residents[0]);
    let EnsureOpen::Ready { .. } = open(target.id(), Some(&owner), ResumeConfig::default())
        .await
        .unwrap()
    else {
        panic!("target should reload");
    };
    assert!(store::get(&residents[0]).is_some());
    assert!(store::get(&residents[1]).is_none());
    for resident in store::entries_for_parent(Some(&owner)) {
        persistence::remove(&resident).await.unwrap();
        store::remove(resident.id());
    }
}

async fn entry(name: &str, owner: &str) -> store::AgentEntry {
    let session = Session::new().await.unwrap();
    session.save().await.unwrap();
    store::AgentEntry {
        name: format!("{name}-{}", session.id),
        instructions: "test".into(),
        session,
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 0,
        model_id: None,
    }
}
