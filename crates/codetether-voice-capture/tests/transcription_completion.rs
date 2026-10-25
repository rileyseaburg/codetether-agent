//! Completed transcriptions release recording state and are consumed once.
use codetether_voice_capture::{RecordingState, TranscriptionInput, drain_voice_transcription};
use std::sync::{Arc, Mutex, atomic::AtomicBool};

#[test]
fn completed_outcomes_release_recording_state_once() {
    for (outcome, expected_text, expected_status) in [
        (Ok("hello".to_owned()), "hello", "Voice transcribed ✓"),
        (
            Err("capture failed".to_owned()),
            "",
            "Voice: capture failed",
        ),
        (
            Ok(String::new()),
            "",
            "Voice transcription produced no text",
        ),
    ] {
        let flag = Arc::new(AtomicBool::new(false));
        let slot = Arc::new(Mutex::new(Some(outcome)));
        let mut recording = RecordingState {
            stop_flag: Some(flag),
            pending_text: Some(slot.clone()),
        };
        let mut status = "Transcribing...".to_owned();
        let mut text = "> ".to_owned();
        let mut insertions = 0;
        {
            let mut input = TranscriptionInput {
                recording: &mut recording,
                status: &mut status,
                insert: |value: &str| {
                    text.push_str(value);
                    insertions += 1;
                },
            };
            drain_voice_transcription(&mut input);
            drain_voice_transcription(&mut input);
        }
        assert_eq!(text, format!("> {expected_text}"));
        assert_eq!(insertions, usize::from(!expected_text.is_empty()));
        assert_eq!(status, expected_status);
        assert!(recording.stop_flag.is_none());
        assert!(recording.pending_text.is_none());
        assert!(slot.lock().unwrap().is_none());
    }
}
