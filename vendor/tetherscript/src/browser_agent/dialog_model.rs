//! Dialog data types exposed by browser pages.

/// Browser JavaScript dialog kind.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::DialogKind;
///
/// assert_eq!(DialogKind::Alert.as_str(), "alert");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogKind {
    /// `window.alert(message)`.
    Alert,
    /// `window.confirm(message)`.
    Confirm,
    /// `window.prompt(message, defaultValue)`.
    Prompt,
}

impl DialogKind {
    /// Return the stable lowercase dialog kind.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Alert => "alert",
            Self::Confirm => "confirm",
            Self::Prompt => "prompt",
        }
    }
}

/// Queued response used by the deterministic dialog bridge.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::DialogDecision;
///
/// let decision = DialogDecision::Prompt("Grace".into());
/// assert!(matches!(decision, DialogDecision::Prompt(_)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogDecision {
    /// Accept the next dialog. Prompts use their default value.
    Accept,
    /// Accept the next prompt with a replacement value.
    Prompt(String),
    /// Dismiss the next dialog.
    Dismiss,
}

/// A dialog observed by an agent page.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{DialogKind, DialogRecord};
///
/// let record = DialogRecord {
///     sequence: 0,
///     kind: DialogKind::Alert,
///     message: "ready".into(),
///     default_value: None,
///     accepted: Some(true),
///     response: None,
/// };
/// assert_eq!(record.kind, DialogKind::Alert);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogRecord {
    /// Monotonic page-local sequence number.
    pub sequence: u64,
    /// Dialog kind.
    pub kind: DialogKind,
    /// Dialog message.
    pub message: String,
    /// Prompt default value, when present.
    pub default_value: Option<String>,
    /// Whether the dialog was accepted by a queued decision.
    pub accepted: Option<bool>,
    /// Prompt response text, when accepted with a value.
    pub response: Option<String>,
}
