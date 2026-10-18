use super::{EnsureOpen, ResumeConfig, open};
use crate::tool::agent::{persistence, store};
use std::path::PathBuf;

#[tokio::test]
async fn first_access_refreshes_a_disk_restored_child_once() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let id = super::overlay_test_support::persisted(&owner, PathBuf::from("/tmp/old")).await;
    assert_eq!(persistence::hydrate_parent(Some(&owner)).await.unwrap(), 1);
    let first = ResumeConfig::new(Some("live/model".into()), None, Some(false));
    let EnsureOpen::Ready { resumed: false, .. } = open(&id, Some(&owner), first).await.unwrap()
    else {
        panic!("restored child should remain resident")
    };
    let loaded = store::get(&id).unwrap();
    assert_eq!(loaded.session.metadata.model.as_deref(), Some("live/model"));
    assert_eq!(
        loaded.session.metadata.inherited_prior_context_allowed,
        Some(false)
    );
    let second = ResumeConfig::new(Some("later/model".into()), None, Some(true));
    open(&id, Some(&owner), second).await.unwrap();
    let unchanged = store::get(&id).unwrap();
    assert_eq!(
        unchanged.session.metadata.model.as_deref(),
        Some("live/model")
    );
    persistence::remove(&unchanged).await.unwrap();
    store::remove(&id);
}
