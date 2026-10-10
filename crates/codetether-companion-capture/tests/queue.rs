use codetether_companion_capture::{Options, Schedule};
use codetether_companion_protocol::CaptureTrigger::{DoubleClick, Manual, Periodic, RightClick};
use std::time::Duration;

#[test]
fn disabled_and_unsupported_clicks_are_ignored() {
    let mut schedule = Schedule::default();
    schedule.resume(Options::new(15, false, false, true).unwrap());
    for trigger in [RightClick, Periodic, Manual] {
        schedule.queue_click(trigger);
        assert_eq!(schedule.due(Duration::ZERO), None);
    }
    schedule.queue_click(DoubleClick);
    assert_eq!(schedule.due(Duration::ZERO), Some(DoubleClick));
    schedule.accepted(Duration::ZERO);
    assert_eq!(schedule.due(Duration::from_secs(30)), None);
}

#[test]
fn pending_work_is_coalesced_with_remote_then_click_priority() {
    let mut schedule = Schedule::default();
    schedule.resume(Options::new(15, true, true, true).unwrap());
    for _ in 0..100 {
        schedule.queue_click(RightClick);
        schedule.queue_click(DoubleClick);
    }
    schedule.set_request(Some("old-request".into()));
    schedule.set_request(Some("latest-request".into()));
    assert_eq!(schedule.request_id(), Some("latest-request"));
    assert_eq!(schedule.due(Duration::ZERO), Some(Manual));
    assert_eq!(schedule.due(Duration::ZERO), Some(Manual));
    schedule.set_request(None);
    assert_eq!(schedule.due(Duration::ZERO), Some(DoubleClick));
    schedule.set_request(Some("  ".into()));
    assert_eq!(schedule.request_id(), None);
    schedule.accepted(Duration::ZERO);
    assert_eq!(schedule.due(Duration::from_secs(5)), None);
    assert_eq!(schedule.due(Duration::from_secs(15)), Some(Periodic));
}
