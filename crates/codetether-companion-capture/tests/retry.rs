use codetether_companion_capture::{Options, Schedule};
use codetether_companion_protocol::CaptureTrigger::{Periodic, RightClick};
use std::time::Duration;

#[test]
fn failure_gates_clicks_and_periodic_without_advancing_success_clock() {
    for click in [false, true] {
        let mut schedule = Schedule::default();
        schedule.resume(Options::new(15, true, true, false).unwrap());
        schedule.accepted(Duration::ZERO);
        if click {
            schedule.queue_click(RightClick);
        }
        schedule.failed(Duration::from_secs(16));
        assert_eq!(schedule.due(Duration::from_millis(20_999)), None);
        let expected = if click { RightClick } else { Periodic };
        assert_eq!(schedule.due(Duration::from_secs(21)), Some(expected));
        schedule.failed(Duration::from_secs(21));
        assert_eq!(schedule.due(Duration::from_millis(25_999)), None);
        assert_eq!(schedule.due(Duration::from_secs(26)), Some(expected));
    }
}

#[test]
fn paused_completions_and_extreme_times_are_safe() {
    let mut schedule = Schedule::default();
    schedule.failed(Duration::MAX);
    schedule.accepted(Duration::MAX);
    assert_eq!(schedule.due(Duration::MAX), None);
    schedule.resume(Options::new(15, true, false, false).unwrap());
    assert_eq!(schedule.due(Duration::ZERO), Some(Periodic));
    schedule.failed(Duration::MAX);
    assert_eq!(schedule.due(Duration::MAX - Duration::from_nanos(1)), None);
    assert_eq!(schedule.due(Duration::MAX), Some(Periodic));
    schedule.accepted(Duration::MAX);
    assert_eq!(schedule.due(Duration::ZERO), None);
    assert_eq!(schedule.due(Duration::MAX), None);
}

#[test]
fn intervals_are_bounded() {
    for interval in [0, 14, 301, u64::MAX] {
        assert!(Options::new(interval, true, true, true).is_none());
    }
    for interval in [15, 300] {
        assert!(Options::new(interval, true, true, true).is_some());
    }
}
