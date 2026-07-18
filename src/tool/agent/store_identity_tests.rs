use super::{AgentEntry, insert, remove, scope::resolve_for_parent};
use crate::session::Session;

#[tokio::test]
async fn duplicate_nicknames_are_isolated_by_parent_and_thread_id() {
    let nickname = format!("reviewer-{}", uuid::Uuid::new_v4());
    let owner_a = format!("owner-{}", uuid::Uuid::new_v4());
    let owner_b = format!("owner-{}", uuid::Uuid::new_v4());
    let first = entry(&nickname, &owner_a, Session::new().await.unwrap());
    let second = entry(&nickname, &owner_b, Session::new().await.unwrap());
    let first_id = first.session.id.clone();
    let second_id = second.session.id.clone();
    insert(first);
    insert(second);

    assert_eq!(
        resolve_for_parent(&nickname, Some(&owner_a)).unwrap().0,
        first_id
    );
    assert_eq!(
        resolve_for_parent(&nickname, Some(&owner_b)).unwrap().0,
        second_id
    );
    assert!(resolve_for_parent(&nickname, None).is_none());
    let first_guard = super::super::execution_state::try_start(&first_id).unwrap();
    let second_guard = super::super::execution_state::try_start(&second_id).unwrap();
    drop((first_guard, second_guard));
    remove(&first_id);
    remove(&second_id);
}

fn entry(name: &str, owner: &str, session: Session) -> AgentEntry {
    AgentEntry {
        name: name.into(),
        instructions: "test".into(),
        session,
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 0,
        model_id: None,
    }
}
