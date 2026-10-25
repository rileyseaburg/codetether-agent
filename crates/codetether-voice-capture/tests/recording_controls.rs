//! Control transitions that do not require a physical microphone or API calls.
use codetether_voice_capture::{Controls, toggle_recording};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

#[test]
fn stopping_signals_capture_and_retains_pending_transcription() {
    let flag = Arc::new(AtomicBool::new(false));
    let slot = Arc::new(Mutex::new(None));
    let mut stop = Some(flag.clone());
    let mut pending = Some(slot.clone());
    let mut status = String::new();
    toggle_recording(Controls {
        is_chat: true,
        recording_stop_flag: &mut stop,
        pending_voice_text: &mut pending,
        status: &mut status,
    });
    assert!(flag.load(Ordering::Relaxed));
    assert!(stop.is_none());
    assert!(Arc::ptr_eq(pending.as_ref().unwrap(), &slot));
    assert_eq!(status, "Transcribing...");
}

#[test]
fn pending_work_blocks_restart_and_non_chat_controls_do_nothing() {
    for is_chat in [true, false] {
        let slot = Arc::new(Mutex::new(None));
        let mut stop = None;
        let mut pending = Some(slot.clone());
        let mut status = "unchanged".to_owned();
        toggle_recording(Controls {
            is_chat,
            recording_stop_flag: &mut stop,
            pending_voice_text: &mut pending,
            status: &mut status,
        });
        assert!(stop.is_none());
        assert!(Arc::ptr_eq(pending.as_ref().unwrap(), &slot));
        assert_eq!(
            status,
            if is_chat {
                "Still transcribing previous recording..."
            } else {
                "unchanged"
            }
        );
    }
}
