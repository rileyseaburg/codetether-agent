use super::{Action, Host};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Select chat scrolling shortcuts while leaving other keys unconsumed.
/// # Arguments
/// * `host` - Native chat state and scroll operations.
/// * `key` - Key with its modifier flags.
/// # Returns
/// Some(false) for a consumed scroll key, otherwise None.
/// # Examples
/// ```text
/// scroll::dispatch(&mut native_host, key);
/// ```
pub fn dispatch(host: &mut impl Host, key: KeyEvent) -> Option<bool> {
    if !host.in_chat() {
        return None;
    }

    let alt = key.modifiers.contains(KeyModifiers::ALT);
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('j') if alt => host.apply(Action::Down(1)),
        KeyCode::Char('k') if alt => host.apply(Action::Up(1)),
        KeyCode::Char('d') if alt => host.apply(Action::PageDown),
        KeyCode::Char('u') if alt => host.apply(Action::PageUp),
        KeyCode::Char('g') if ctrl => host.apply(Action::Top),
        KeyCode::Char('G') if ctrl => host.apply(Action::Bottom),
        _ => return None,
    }
    Some(false)
}
