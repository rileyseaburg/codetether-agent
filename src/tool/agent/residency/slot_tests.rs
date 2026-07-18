use super::{slot::Slot, state::ResidencyState};
use std::sync::Arc;
use tokio::sync::Barrier;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_reservations_cannot_oversubscribe() {
    let state = Arc::new(ResidencyState::default());
    let barrier = Arc::new(Barrier::new(3));
    let first = attempt(Arc::clone(&state), Arc::clone(&barrier));
    let second = attempt(Arc::clone(&state), Arc::clone(&barrier));
    barrier.wait().await;
    let (first, second) = tokio::join!(first, second);
    let accepted = usize::from(first.unwrap().is_some()) + usize::from(second.unwrap().is_some());
    assert_eq!(accepted, 1);
    assert_eq!(state.usage(), 0);
}

fn attempt(
    state: Arc<ResidencyState>,
    barrier: Arc<Barrier>,
) -> tokio::task::JoinHandle<Option<Slot>> {
    tokio::spawn(async move {
        barrier.wait().await;
        Slot::try_with(state, 1)
    })
}

#[test]
fn commit_materializes_and_drop_rolls_back() {
    let state = Arc::new(ResidencyState::default());
    Slot::try_with(Arc::clone(&state), 1)
        .unwrap()
        .commit("child-1");
    assert_eq!(state.usage(), 1);
    assert!(Slot::try_with(Arc::clone(&state), 1).is_none());
    state.forget("child-1");
    let pending = Slot::try_with(Arc::clone(&state), 1).unwrap();
    assert_eq!(state.usage(), 1);
    drop(pending);
    assert_eq!(state.usage(), 0);
}
