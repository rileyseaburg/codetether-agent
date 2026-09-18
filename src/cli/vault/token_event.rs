//! Hidden prompt editing, including bracketed paste and cancellation.
use anyhow::{Result, ensure};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
pub(super) fn apply(event: Event, value: &mut String) -> Result<bool> {
    match event {
        Event::Paste(text) => {
            let text = text.trim();
            ensure!(
                !text.chars().any(char::is_control),
                "Token paste contains control characters"
            );
            value.push_str(text);
        }
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                anyhow::bail!("Vault login cancelled");
            }
            match key.code {
                KeyCode::Enter => return Ok(true),
                KeyCode::Backspace => {
                    value.pop();
                }
                KeyCode::Char(c)
                    if !c.is_control()
                        && !key
                            .modifiers
                            .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
                {
                    value.push(c)
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(false)
}
