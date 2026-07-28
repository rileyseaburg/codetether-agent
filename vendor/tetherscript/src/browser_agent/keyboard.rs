//! Keyboard key values accepted by page keyboard actions.

#[path = "keyboard_default.rs"]
pub(crate) mod keyboard_default;
#[path = "keyboard_edit.rs"]
pub(crate) mod keyboard_edit;
#[path = "keyboard_parse.rs"]
mod keyboard_parse;
#[path = "keyboard_script.rs"]
pub(crate) mod keyboard_script;

/// A keyboard key that can be dispatched by [`BrowserPage::press`].
///
/// [`BrowserPage::press`]: crate::browser_agent::BrowserPage::press
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardKey {
    /// The Enter key.
    Enter,
    /// The Space key.
    Space,
    /// The Backspace key.
    Backspace,
    /// The Tab key.
    Tab,
    /// The Escape key.
    Escape,
    /// A single printable character.
    Character(char),
}

impl KeyboardKey {
    pub(crate) fn js_key(&self) -> String {
        match self {
            Self::Enter => "Enter".into(),
            Self::Space => " ".into(),
            Self::Backspace => "Backspace".into(),
            Self::Tab => "Tab".into(),
            Self::Escape => "Escape".into(),
            Self::Character(ch) => ch.to_string(),
        }
    }

    pub(crate) fn needs_editable(&self) -> bool {
        matches!(self, Self::Backspace | Self::Character(_))
    }
}
