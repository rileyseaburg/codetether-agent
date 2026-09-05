//! Pinned child checkout survives close/reload while parent policy refreshes.

use super::{EnsureOpen, ResumeConfig, open, overlay_test_support};
use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::agent::{persistence, store, thread_lifecycle};

#[tokio::test]
async fn reload_preserves_pinned_checkout_and_refreshes_parent_settings() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let checkout = _temp.path().join(".codetether-worktrees/child");
    let id = overlay_test_support::persisted_with_pin(&owner, checkout.clone(), true).await;
    persistence::hydrate_parent(Some(&owner)).await.unwrap();
    assert!(
        thread_lifecycle::close(&id, Some(&owner))
            .await
            .unwrap()
            .success
    );
    assert!(store::get(&id).is_none());
    let parent = _temp.path().join("new-parent");
    let config = ResumeConfig::new(Some("live/model".into()), Some(parent.clone()), Some(false));
    let EnsureOpen::Ready { resumed: true, .. } = open(&id, Some(&owner), config).await.unwrap()
    else {
        panic!("closed child should reload");
    };
    let loaded = store::get(&id).unwrap();
    let persisted = Session::load(&id).await.unwrap();
    for session in [&loaded.session, &persisted] {
        let metadata = &session.metadata;
        assert!(metadata.workspace_pinned);
        assert_eq!(metadata.directory.as_ref(), Some(&checkout));
        assert_eq!(metadata.model.as_deref(), Some("live/model"));
        assert_eq!(metadata.inherited_prior_context_allowed, Some(false));
        let ContentPart::Text { text } = &session.messages[0].content[0] else {
            panic!("expected child system text");
        };
        assert!(text.contains(&checkout.display().to_string()));
        assert!(!text.contains(&parent.display().to_string()));
    }
    assert_eq!(loaded.model_id.as_deref(), Some("live/model"));
    persistence::remove(&loaded).await.unwrap();
    store::remove(&id);
}
