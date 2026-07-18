use super::{registry::Registry, reservation::Reservation};
use std::sync::Arc;
use tokio::sync::Barrier;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn same_owner_and_name_can_only_be_reserved_once() {
    let registry = Arc::new(Registry::default());
    let barrier = Arc::new(Barrier::new(3));
    let first = attempt(Arc::clone(&registry), Arc::clone(&barrier));
    let second = attempt(Arc::clone(&registry), Arc::clone(&barrier));
    barrier.wait().await;
    let (first, second) = tokio::join!(first, second);
    let accepted = usize::from(first.unwrap().is_some()) + usize::from(second.unwrap().is_some());
    assert_eq!(accepted, 1);
    assert!(Reservation::try_with(registry, "worker", Some("owner")).is_some());
}

fn attempt(
    registry: Arc<Registry>,
    barrier: Arc<Barrier>,
) -> tokio::task::JoinHandle<Option<Reservation>> {
    tokio::spawn(async move {
        barrier.wait().await;
        Reservation::try_with(registry, "worker", Some("owner"))
    })
}

#[test]
fn distinct_owners_can_reserve_the_same_name() {
    let registry = Arc::new(Registry::default());
    let first = Reservation::try_with(Arc::clone(&registry), "worker", Some("owner-a"));
    let second = Reservation::try_with(registry, "worker", Some("owner-b"));
    assert!(first.is_some() && second.is_some());
}
