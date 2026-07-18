use super::{EnsureOpen, ResumeConfig, open};
use crate::tool::agent::{persistence, store};
use std::path::PathBuf;

#[tokio::test]
async fn name_and_id_aliases_share_one_activation() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let id = super::overlay_test_support::closed(&owner, PathBuf::from("/tmp/work")).await;
    let by_id = open(&id, Some(&owner), ResumeConfig::default());
    let by_name = open("worker", Some(&owner), ResumeConfig::default());
    let (by_id, by_name) = tokio::join!(by_id, by_name);
    let outcomes = [by_id.unwrap(), by_name.unwrap()];
    let resumed = outcomes
        .iter()
        .filter(|outcome| matches!(outcome, EnsureOpen::Ready { resumed: true, .. }))
        .count();
    assert_eq!(resumed, 1);
    let loaded = store::get(&id).unwrap();
    persistence::remove(&loaded).await.unwrap();
    store::remove(&id);
}
