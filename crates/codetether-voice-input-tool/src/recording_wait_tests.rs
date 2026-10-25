//! Synthetic stream failures and cancellation without microphone hardware.
use super::{capture_errors::CaptureErrors, recording_wait::for_stop};
use cpal::{Error, ErrorKind};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

#[test]
fn failure_wins_over_cancellation_and_deadline() {
    let errors = CaptureErrors::default();
    errors.report(Error::with_message(
        ErrorKind::StreamInvalidated,
        "startup failed",
    ));
    for stopped in [false, true] {
        let result = for_stop(Duration::ZERO, &AtomicBool::new(stopped), &errors);
        assert!(result.unwrap_err().to_string().contains("startup failed"));
    }
}

#[test]
fn cancellation_and_deadline_return_without_failure() {
    let errors = CaptureErrors::default();
    assert!(for_stop(Duration::ZERO, &AtomicBool::new(false), &errors).is_ok());
    assert!(for_stop(Duration::from_secs(5), &AtomicBool::new(true), &errors).is_ok());
}

#[test]
fn asynchronous_terminal_failure_interrupts_recording() {
    let errors = CaptureErrors::default();
    let callback_errors = errors.clone();
    let callback = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(25));
        callback_errors.report(Error::with_message(
            ErrorKind::DeviceNotAvailable,
            "disconnected",
        ));
    });
    let started = Instant::now();
    let result = for_stop(Duration::from_secs(5), &AtomicBool::new(false), &errors);
    callback.join().unwrap();
    assert!(result.unwrap_err().to_string().contains("disconnected"));
    assert!(started.elapsed() < Duration::from_secs(3));
}

#[test]
fn recoverable_notification_does_not_fail_the_wait() {
    let errors = CaptureErrors::default();
    errors.report(Error::new(ErrorKind::Xrun));
    assert!(for_stop(Duration::ZERO, &AtomicBool::new(false), &errors).is_ok());
    assert!(errors.check().is_ok());
}
