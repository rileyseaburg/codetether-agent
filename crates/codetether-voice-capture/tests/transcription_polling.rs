//! Polling unfinished or locked worker slots does not disturb active recording.
use codetether_voice_capture::{RecordingState, TranscriptionInput, drain_voice_transcription};
use std::sync::{Arc, Mutex, atomic::AtomicBool};

#[test]
fn missing_or_incomplete_results_leave_state_untouched() {
    for has_slot in [false, true] {
        let flag = Arc::new(AtomicBool::new(false));
        let mut recording = RecordingState {
            stop_flag: Some(flag.clone()),
            pending_text: has_slot.then(|| Arc::new(Mutex::new(None))),
        };
        let mut status = "Recording...".to_owned();
        drain_voice_transcription(&mut TranscriptionInput {
            recording: &mut recording,
            status: &mut status,
            insert: |_: &str| panic!("no transcription is ready"),
        });
        assert!(Arc::ptr_eq(recording.stop_flag.as_ref().unwrap(), &flag));
        assert_eq!(recording.pending_text.is_some(), has_slot);
        assert_eq!(status, "Recording...");
    }
}

#[test]
fn locked_result_is_left_for_a_later_tick() {
    let flag = Arc::new(AtomicBool::new(false));
    let slot = Arc::new(Mutex::new(Some(Ok("hello".to_owned()))));
    let mut recording = RecordingState {
        stop_flag: Some(flag.clone()),
        pending_text: Some(slot.clone()),
    };
    let mut status = "Transcribing...".to_owned();
    let guard = slot.lock().unwrap();
    drain_voice_transcription(&mut TranscriptionInput {
        recording: &mut recording,
        status: &mut status,
        insert: |_: &str| panic!("the result is locked"),
    });
    assert_eq!(guard.as_ref().unwrap().as_ref().unwrap(), "hello");
    assert!(Arc::ptr_eq(recording.stop_flag.as_ref().unwrap(), &flag));
    assert!(Arc::ptr_eq(recording.pending_text.as_ref().unwrap(), &slot));
    assert_eq!(status, "Transcribing...");
}
