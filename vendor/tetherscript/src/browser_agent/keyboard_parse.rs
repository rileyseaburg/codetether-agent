//! String and character parsing for keyboard keys.

use crate::browser_agent::keyboard::KeyboardKey;

impl TryFrom<&str> for KeyboardKey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(key) = printable(value) {
            return Ok(key);
        }
        match value {
            "Enter" => Ok(Self::Enter),
            "Space" | "Spacebar" => Ok(Self::Space),
            "Backspace" => Ok(Self::Backspace),
            "Tab" => Ok(Self::Tab),
            "Escape" | "Esc" => Ok(Self::Escape),
            other => Err(unsupported(other)),
        }
    }
}

impl TryFrom<String> for KeyboardKey {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<char> for KeyboardKey {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_control() {
            Err(unsupported(&value.to_string()))
        } else {
            Ok(Self::Character(value))
        }
    }
}

fn printable(value: &str) -> Option<KeyboardKey> {
    let mut chars = value.chars();
    let ch = chars.next()?;
    (chars.next().is_none() && !ch.is_control()).then_some(KeyboardKey::Character(ch))
}

fn unsupported(value: &str) -> String {
    format!(
        "unsupported keyboard key {:?}; expected Enter, Space, Backspace, Tab, Escape, or one printable character",
        value
    )
}
