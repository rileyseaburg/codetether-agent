use codetether_companion_capture::{Options, Schedule};
use codetether_companion_protocol::CaptureTrigger::{Manual, Periodic, RightClick};
use std::time::Duration;

#[test]
fn remote_commands_and_clicks_cannot_start_or_resume_capture() {
    let mut schedule = Schedule::default();
    for _ in 0..2 {
        schedule.set_request(Some("opaque-request".into()));
        schedule.queue_click(RightClick);
        assert_eq!(schedule.due(Duration::ZERO), None);
        assert_eq!(schedule.request_id(), None);
        schedule.resume(Options::new(15, true, true, true).unwrap());
        assert_eq!(schedule.due(Duration::ZERO), Some(Periodic));
        schedule.set_request(Some("opaque-request".into()));
        assert_eq!(schedule.due(Duration::ZERO), Some(Manual));
        schedule.pause();
    }
}

#[test]
fn explicit_resume_clears_previous_work_and_cooldown() {
    let mut schedule = Schedule::default();
    schedule.resume(Options::new(15, true, true, true).unwrap());
    schedule.accepted(Duration::from_secs(40));
    schedule.queue_click(RightClick);
    schedule.set_request(Some("old-request".into()));
    schedule.failed(Duration::from_secs(41));
    schedule.resume(Options::new(30, false, false, false).unwrap());
    assert_eq!(schedule.request_id(), None);
    assert_eq!(schedule.due(Duration::from_secs(42)), None);
    schedule.resume(Options::new(30, true, false, false).unwrap());
    assert_eq!(schedule.due(Duration::from_secs(42)), Some(Periodic));
}
