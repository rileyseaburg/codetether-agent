use super::super::{AgentEntry, insert, remove};
use super::lineage_for_session;
use crate::session::Session;

#[test]
fn root_session_produces_direct_child_depth() {
    let root = uuid::Uuid::new_v4().to_string();
    assert_eq!(lineage_for_session(Some(&root)), (None, 1));
}

#[tokio::test]
async fn derives_child_lineage_from_parent_session() {
    let name = format!("parent-{}", uuid::Uuid::new_v4());
    let session = Session::new().await.expect("parent session");
    let session_id = session.id.clone();
    insert(AgentEntry {
        name: name.clone(),
        instructions: "parent task".into(),
        session,
        parent: None,
        owner_session_id: None,
        depth: 2,
        model_id: None,
    });
    assert_eq!(
        lineage_for_session(Some(&session_id)),
        (Some(name.clone()), 3)
    );
    remove(&session_id);
}
