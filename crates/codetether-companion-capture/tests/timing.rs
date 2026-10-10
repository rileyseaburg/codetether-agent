use codetether_companion_capture::{Options, Schedule};
use codetether_companion_protocol::CaptureTrigger::{Manual, Periodic, RightClick};
use std::time::Duration;

#[test]
fn click_and_periodic_deadlines_are_inclusive() {
    for interval in [15, 300] {
        let mut schedule = Schedule::default();
        schedule.resume(Options::new(interval, true, true, false).unwrap());
        schedule.accepted(Duration::from_secs(10));
        assert_eq!(schedule.due(Duration::from_secs(9)), None);
        schedule.queue_click(RightClick);
        assert_eq!(schedule.due(Duration::from_millis(14_999)), None);
        assert_eq!(schedule.due(Duration::from_secs(15)), Some(RightClick));
        schedule.accepted(Duration::from_secs(15));
        assert_eq!(
            schedule.due(Duration::from_millis((15 + interval) * 1000 - 1)),
            None
        );
        assert_eq!(
            schedule.due(Duration::from_secs(15 + interval)),
            Some(Periodic)
        );
    }
}

#[test]
fn remote_requests_bypass_cooldown_but_not_retry_delay() {
    let mut schedule = Schedule::default();
    schedule.resume(Options::new(15, true, true, true).unwrap());
    schedule.accepted(Duration::from_secs(10));
    schedule.set_request(Some("request".into()));
    assert_eq!(schedule.due(Duration::from_secs(11)), Some(Manual));
    schedule.failed(Duration::from_secs(11));
    assert_eq!(schedule.due(Duration::from_millis(15_999)), None);
    assert_eq!(schedule.request_id(), Some("request"));
    assert_eq!(schedule.due(Duration::from_secs(16)), Some(Manual));
    schedule.accepted(Duration::from_secs(16));
    assert_eq!(schedule.request_id(), None);
    assert_eq!(schedule.due(Duration::from_secs(30)), None);
    assert_eq!(schedule.due(Duration::from_secs(31)), Some(Periodic));
}
