//! Status label and color helpers for the summary panel.

use ratatui::style::Color;

pub(super) fn worker(connected: bool) -> (&'static str, Color) {
    if connected {
        ("connected", Color::Green)
    } else {
        ("offline", Color::DarkGray)
    }
}

pub(super) fn peer(ready: bool) -> (&'static str, Color) {
    if ready {
        ("ready", Color::Cyan)
    } else {
        ("off", Color::DarkGray)
    }
}

pub(super) fn processing(processing: Option<bool>) -> (&'static str, Color) {
    match processing {
        Some(true) => ("processing", Color::Yellow),
        Some(false) => ("idle", Color::Green),
        None => ("unknown", Color::DarkGray),
    }
}
