//! Default editing behavior for keyboard actions.

use crate::browser_agent::keyboard::KeyboardKey;
use crate::browser_agent::keyboard_escape::quote;

pub(crate) fn body(key: &KeyboardKey, replacement: Option<&str>) -> String {
    match key {
        KeyboardKey::Enter => enter_body(),
        KeyboardKey::Space => click_body(),
        KeyboardKey::Backspace => replacement.map(input_body).unwrap_or_default(),
        KeyboardKey::Character(ch) => type_body(*ch),
        KeyboardKey::Tab | KeyboardKey::Escape => String::new(),
    }
}

fn click_body() -> String {
    "if(n.click){n.click();}".into()
}

fn enter_body() -> String {
    "if(n.tagName=='INPUT'){let f=n.closest('form');if(f){f.requestSubmit();}else if(n.click){n.click();}}else if(n.click){n.click();}".into()
}

fn input_body(value: &str) -> String {
    format!("if(n.inputText){{n.inputText({});}}", quote(value))
}

fn type_body(ch: char) -> String {
    format!("if(n.typeText){{n.typeText({});}}", quote(&ch.to_string()))
}
