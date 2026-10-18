//! Keyboard interaction modes shared by terminal views.
use serde::{Deserialize, Serialize};

/// Keyboard input mode currently used by the TUI.
///
/// The input mode determines how key presses are interpreted: as navigation,
/// text editing, or command entry. It is separate from [`ViewMode`] so every
/// screen can share consistent keyboard behavior.
///
/// # Examples
///
/// ```
/// use codetether_tui_models::InputMode;
///
/// let mode = InputMode::Normal;
/// assert_eq!(mode, InputMode::Normal);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    /// Navigation mode where keys control focus, scrolling, and view actions.
    Normal,
    /// Text-entry mode for composing or editing prompt input.
    Editing,
    /// Slash-command or command-line mode.
    Command,
}
