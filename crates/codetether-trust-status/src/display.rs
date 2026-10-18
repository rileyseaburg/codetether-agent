//! UI-facing trust summaries and badge styling.
use crate::{current_status, guidance};
use codetether_config::TrustPolicyStatus;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

/// Format a full settings/status message.
///
/// # Arguments
/// * `status` - Snapshot whose effective settings are described.
///
/// # Returns
/// A full trust, approval, sandbox and profile description.
///
/// # Examples
/// ```
/// let config = codetether_config::Config::default();
/// let status = codetether_config::TrustPolicyStatus::from_config(&config);
/// assert!(codetether_trust_status::format_status(&status).contains("Trust:"));
/// ```
pub fn format_status(status: &TrustPolicyStatus) -> String {
    guidance::summary(status)
}

/// Explain why approval prompts are appearing and how to change them.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// Approval behavior and settings guidance for the current snapshot.
///
/// # Panics
/// Panics if the shared status mutex is poisoned.
///
/// # Examples
/// ```
/// let _text = codetether_trust_status::approval_guidance();
/// ```
pub fn approval_guidance() -> String {
    guidance::approval(current_status())
}

/// Return a short startup policy summary for the status bar.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// A startup summary when a snapshot has been stored.
///
/// # Panics
/// Panics if the shared status mutex is poisoned.
///
/// # Examples
/// ```
/// let _summary = codetether_trust_status::startup_summary();
/// ```
pub fn startup_summary() -> Option<String> {
    current_status().map(guidance::startup)
}

/// Render the compact status-bar trust badge.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// A styled compact badge when a snapshot is available.
///
/// # Panics
/// Panics if the shared status mutex is poisoned.
///
/// # Examples
/// ```
/// let _badge = codetether_trust_status::trust_status_badge_span();
/// ```
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
