//! Activity-age badge shared by the terminal chat layouts.
use crate::tui::app::state::App;
use ratatui::{style::Stylize, text::Span};
use std::time::SystemTime;

/// Render time since the newest runtime event or displayed message.
pub(crate) fn badge(app: &App) -> Span<'static> {
    let (latest, kind) = app
        .state
        .messages
        .last()
        .filter(|message| message.timestamp > app.state.last_activity_at)
        .map_or(
            (app.state.last_activity_at, app.state.last_activity_kind),
            |message| (message.timestamp, "message"),
        );
    format!("{} ({kind})", label(latest, SystemTime::now())).dim()
}

fn label(latest: SystemTime, now: SystemTime) -> String {
    let seconds = now.duration_since(latest).unwrap_or_default().as_secs();
    let age = if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3600 {
        format!("{}m {:02}s", seconds / 60, seconds % 60)
    } else {
        format!(
            "{}h {:02}m {:02}s",
            seconds / 3600,
            seconds / 60 % 60,
            seconds % 60
        )
    };
    format!("Last activity: {age}")
}

#[cfg(test)]
#[path = "activity_tests.rs"]
mod tests;
