//! Ctrl+R handler: toggle voice recording from the TUI.

#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use codetether_voice_capture::max_duration;
use voice_worker::spawn_voice_worker;

mod voice_drain;
mod voice_worker;

/// Handle Ctrl+R keypress — toggle voice recording.
pub(super) fn handle_voice_input(app: &mut App) {
    if app.state.view_mode != ViewMode::Chat {
        return;
    }
    match &app.state.recording_stop_flag {
        Some(flag) => {
            flag.store(true, Ordering::Relaxed);
            app.state.status = "Transcribing...".into();
            app.state.recording_stop_flag = None;
        }
        None => {
            if app.state.pending_voice_text.is_some() {
                app.state.status = "Still transcribing previous recording...".into();
                return;
            }
            let flag = Arc::new(AtomicBool::new(false));
            app.state.recording_stop_flag = Some(flag.clone());
            app.state.status = "Recording... (Ctrl+R to stop)".into();
            let slot = Arc::new(std::sync::Mutex::new(None));
            app.state.pending_voice_text = Some(slot.clone());
            spawn_voice_worker(max_duration(), flag, slot);
        }
    }
}

pub use voice_drain::drain_voice_transcription;
