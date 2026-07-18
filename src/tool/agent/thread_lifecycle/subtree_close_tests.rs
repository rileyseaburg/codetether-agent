use super::super::{collaboration_runtime::thread_status, persistence, store};
use super::subtree_test_support as support;

#[tokio::test]
async fn close_is_repeatable_and_shuts_down_live_descendants() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let child = support::entry("child", &owner, 1).await;
    let grandchild = support::entry("grandchild", child.id(), 2).await;
    support::register(&child).await;
    support::register(&grandchild).await;

    assert!(
        super::close(child.id(), Some(&owner))
            .await
            .unwrap()
            .success
    );
    assert!(store::get(child.id()).is_none());
    assert!(store::get(grandchild.id()).is_none());
    assert_eq!(
        thread_status::get(child.id()),
        thread_status::ThreadStatus::Shutdown
    );
    assert_eq!(
        thread_status::get(grandchild.id()),
        thread_status::ThreadStatus::Shutdown
    );
    assert!(
        !persistence::is_open(Some(&owner), child.id())
            .await
            .unwrap()
    );
    assert!(
        persistence::is_open(Some(child.id()), grandchild.id())
            .await
            .unwrap()
    );
    assert!(
        super::close(child.id(), Some(&owner))
            .await
            .unwrap()
            .success
    );
    support::cleanup(&[child, grandchild]).await;
}
