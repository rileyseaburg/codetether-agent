use super::{EnsureOpen, ResumeConfig, open};
use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::agent::{persistence, store};
use std::path::PathBuf;

#[tokio::test]
async fn reload_applies_and_persists_live_parent_config() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let old_workspace = PathBuf::from("/tmp/old-child-workspace");
    let new_workspace = PathBuf::from("/tmp/live-parent-workspace");
    let id = super::overlay_test_support::closed(&owner, old_workspace.clone()).await;
    let config = ResumeConfig::new(
        Some("live/model".into()),
        Some(new_workspace.clone()),
        Some(false),
    );
    let EnsureOpen::Ready { resumed: true, .. } = open(&id, Some(&owner), config).await.unwrap()
    else {
        panic!("closed child should reload")
    };
    let loaded = store::get(&id).unwrap();
    assert_eq!(loaded.session.metadata.model.as_deref(), Some("live/model"));
    assert_eq!(
        loaded.session.metadata.directory,
        Some(new_workspace.clone())
    );
    assert_eq!(
        loaded.session.metadata.inherited_prior_context_allowed,
        Some(false)
    );
    let ContentPart::Text { text } = &loaded.session.messages[0].content[0] else {
        panic!("expected child system text")
    };
    assert!(text.contains(&new_workspace.display().to_string()));
    assert!(!text.contains(&old_workspace.display().to_string()));
    let persisted = Session::load(&id).await.unwrap();
    assert_eq!(persisted.metadata.model.as_deref(), Some("live/model"));
    persistence::remove(&loaded).await.unwrap();
    store::remove(&id);
}
