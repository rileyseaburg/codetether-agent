use super::super::store;
use super::test_support as support;

#[tokio::test]
async fn continued_parent_recovers_children_under_its_new_id() {
    let (_temp, _guard) = support::isolate();
    let child = crate::session::Session::new().await.unwrap();
    child.save().await.unwrap();
    let old = format!("old-{}", uuid::Uuid::new_v4());
    let new = format!("new-{}", uuid::Uuid::new_v4());
    let name = format!("child-{}", uuid::Uuid::new_v4());
    let entry = support::entry(&name, child.clone(), &old);
    super::save(&name, &entry).await.unwrap();

    assert_eq!(super::reparent_owner(&old, &new).await.unwrap(), 1);
    assert_eq!(super::hydrate_parent(Some(&new)).await.unwrap(), 1);
    let restored = store::get_for_parent(&name, Some(&new)).unwrap();
    assert_eq!(restored.session.id, child.id);
    assert!(store::get_for_parent(&name, Some(&old)).is_none());

    super::remove(&restored).await.unwrap();
    store::remove(&child.id);
}
