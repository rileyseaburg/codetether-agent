//! Status-bar formatting for effective trust and tool policy settings.

#[path = "trust_status_guidance.rs"]
mod guidance;
#[path = "trust_status_labels.rs"]
mod labels;

use crate::config::{Config, TrustPolicyStatus};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};
use std::sync::{LazyLock, Mutex};

static STATUS: LazyLock<Mutex<Option<TrustPolicyStatus>>> = LazyLock::new(|| Mutex::new(None));

/// Store the loaded config's effective trust policy status for TUI rendering.
pub fn set_from_config(config: &Config) {
    set_status(TrustPolicyStatus::from_config(config));
}

/// Store an explicit trust policy status.
pub fn set_status(status: TrustPolicyStatus) {
    *STATUS.lock().expect("trust status mutex poisoned") = Some(status);
}

/// Return the last loaded trust policy status.
pub fn current_status() -> Option<TrustPolicyStatus> {
    *STATUS.lock().expect("trust status mutex poisoned")
}

/// Format a full settings/status message.
pub fn format_status(status: &TrustPolicyStatus) -> String {
    guidance::summary(status)
}

/// Explain why approval prompts are appearing and how to change them.
pub fn approval_guidance() -> String {
    guidance::approval(current_status())
}

/// Return a short startup policy summary for the status bar.
pub fn startup_summary() -> Option<String> {
    current_status().map(guidance::startup)
}

/// Render the compact status-bar trust badge.
pub fn trust_status_badge_span() -> Option<Span<'static>> {
    let status = current_status()?;
    let color = if status.project_trusted {
        Color::Green
    } else {
        Color::Yellow
    };
    Some(Span::styled(
        guidance::badge(&status),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

#[cfg(test)]
#[path = "trust_status_tests.rs"]
mod tests;
