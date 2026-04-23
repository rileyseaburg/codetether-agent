//! Ctrl+V handler: toggle voice recording from the TUI.
//!
//! First press starts recording, second press stops and transcribes.
//! Auto-stops after `CODETETHER_VOICE_INPUT_MAX_SECS` (default 60).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::tool::voice_input::{encoder, recorder};
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Maximum recording duration from env (default 60s).
fn max_duration() -> u64 {
    std::env::var("CODETETHER_VOICE_INPUT_MAX_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60)
        .min(300)
}

/// Handle Ctrl+V keypress — toggle voice recording.
pub(super) fn handle_voice_input(app: &mut App) {
    if app.state.view_mode != ViewMode::Chat {
        return;
    }

    match &app.state.recording_stop_flag {
        Some(flag) => {
            // Already recording — signal stop
            flag.store(true, Ordering::Relaxed);
            app.state.status = "Transcribing...".into();
            app.state.recording_stop_flag = None;
        }
        None => {
            // Start recording in background thread
            let flag = Arc::new(AtomicBool::new(false));
            app.state.recording_stop_flag = Some(flag.clone());
            app.state.status = "Recording... (Ctrl+V to stop)".into();

            let flag_clone = flag.clone();
            let max = max_duration();
            std::thread::spawn(move || match recorder::record(max, flag_clone) {
                Ok(samples) => {
                    if let Ok(wav) = encoder::encode_wav(&samples) {
                        tracing::info!("Voice recording captured {} bytes", wav.len());
                    }
                }
                Err(e) => {
                    tracing::error!("Recording failed: {e}");
                }
            });
        }
    }
}
