//! Regression tests for capture callback error classification and sharing.
use super::capture_errors::CaptureErrors;
use cpal::{Error, ErrorKind};

#[test]
fn callback_failure_is_shared_with_recorder() {
    let errors = CaptureErrors::default();
    let callback = errors.clone();
    std::thread::spawn(move || {
        callback.report(Error::with_message(
            ErrorKind::BackendError,
            "WASAPI startup failed",
        ));
    })
    .join()
    .unwrap();
    let reason = errors.check().unwrap_err().to_string();
    assert!(reason.contains("Audio capture failed"));
    assert!(reason.contains("WASAPI startup failed"));
}

#[test]
fn first_terminal_failure_is_preserved() {
    let errors = CaptureErrors::default();
    errors.report(Error::with_message(
        ErrorKind::PermissionDenied,
        "first failure",
    ));
    let first = errors.check().unwrap_err().to_string();
    errors.report(Error::new(ErrorKind::StreamInvalidated));
    assert_eq!(errors.check().unwrap_err().to_string(), first);
    assert!(first.contains("first failure"));
}

#[test]
fn recoverable_notifications_do_not_abort_capture() {
    let errors = CaptureErrors::default();
    for kind in [
        ErrorKind::DeviceChanged,
        ErrorKind::RealtimeDenied,
        ErrorKind::Xrun,
    ] {
        errors.report(Error::with_message(kind, "recoverable notification"));
        assert!(errors.check().is_ok());
    }
}

#[test]
fn new_recording_does_not_inherit_previous_failure() {
    let previous = CaptureErrors::default();
    previous.report(Error::with_message(ErrorKind::BackendError, "old failure"));
    let next = CaptureErrors::default();
    assert!(previous.check().is_err());
    assert!(next.check().is_ok());
}
