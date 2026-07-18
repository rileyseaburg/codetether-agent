use super::super::{persistence, residency, store};
use super::subtree_test_support as support;

#[tokio::test]
async fn resume_reopens_only_root_and_preserves_lazy_descendant() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let child = support::entry("child", &owner, 1).await;
    let grandchild = support::entry("grandchild", child.id(), 2).await;
    support::register(&child).await;
    support::register(&grandchild).await;
    super::close(child.id(), Some(&owner)).await.unwrap();

    assert!(
        super::resume(child.id(), Some(&owner), Default::default())
            .await
            .unwrap()
            .success
    );
    assert!(store::get(child.id()).is_some());
    assert!(store::get(grandchild.id()).is_none());
    let opened = residency::open(grandchild.id(), Some(child.id()), Default::default())
        .await
        .unwrap();
    assert!(matches!(
        opened,
        residency::EnsureOpen::Ready { resumed: true, .. }
    ));
    support::cleanup(&[child, grandchild]).await;
}
