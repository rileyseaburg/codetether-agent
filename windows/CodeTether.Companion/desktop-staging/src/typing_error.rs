/// Redacted keyboard-delivery failures. A failure may follow partial insertion.
/// # Examples
/// ```text
/// match result { Ok(()) => show_sent(), Err(reason) => show_stopped(reason) }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypingError {
    /// No native Windows input support on this platform.
    Unsupported,
    /// Empty/oversized reply or a control character that could submit/navigate.
    InvalidText,
    /// Locked desktop, changed monitor, missing COM/UIA or no editable input.
    Unavailable,
    /// Foreground/focused element changed or a modifier/Escape was pressed.
    TargetChanged,
    /// Pause/Stop or the bounded input deadline cancelled delivery.
    Cancelled,
    /// Windows rejected all or part of an input batch (including UIPI denial).
    InputRejected,
}
impl std::fmt::Display for TypingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unsupported => "Keyboard delivery requires Windows",
            Self::InvalidText => {
                "Reply must be single-line plain text of at most 2000 UTF-16 units"
            }
            Self::Unavailable => "Focused editable input unavailable on the shared monitor",
            Self::TargetChanged => "Typing stopped because focus or keyboard state changed",
            Self::Cancelled => "Typing cancelled; input may be partial",
            Self::InputRejected => "Windows rejected keyboard input; input may be partial",
        })
    }
}
impl std::error::Error for TypingError {}
