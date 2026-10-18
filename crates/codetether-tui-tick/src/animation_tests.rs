use std::time::Duration;

use super::animation_epoch_at;

#[test]
fn idle_animation_epoch_is_stable() {
    assert_eq!(animation_epoch_at(false, false, Duration::ZERO), 0);
    assert_eq!(animation_epoch_at(false, false, Duration::from_secs(99)), 0);
}

#[test]
fn processing_animation_advances_once_per_second() {
    let first = animation_epoch_at(true, false, Duration::from_millis(1_000));
    let same_second = animation_epoch_at(true, false, Duration::from_millis(1_999));
    let next_second = animation_epoch_at(true, false, Duration::from_millis(2_000));

    assert_eq!(first, same_second);
    assert_ne!(first, next_second);
}

#[test]
fn play_break_animation_advances_four_times_per_second() {
    assert_ne!(
        animation_epoch_at(true, true, Duration::ZERO),
        animation_epoch_at(true, true, Duration::from_millis(250))
    );
}
