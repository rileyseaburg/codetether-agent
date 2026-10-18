//! Explicit yes/no selection; defaults to No and ignores modified shortcuts.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Interpret explicit answer-selection keys.
///
/// # Arguments
/// * `selected_yes` - Current selection, toggled by navigation keys.
/// * `key` - Terminal key event.
///
/// # Returns
/// An explicit yes/no intent, or none for navigation and unrelated keys.
///
/// # Examples
/// ```
/// use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};
/// let mut yes = false;
/// assert_eq!(codetether_answer_review_view::choose(&mut yes,KeyEvent::new(KeyCode::Enter,KeyModifiers::NONE)),Some(false));
/// ```
pub fn choose(selected_yes: &mut bool, key: KeyEvent) -> Option<bool> {
    if key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
    {
        return None;
    }
    match key.code {
        KeyCode::Left | KeyCode::Right | KeyCode::Tab | KeyCode::BackTab => {
            *selected_yes = !*selected_yes;
            None
        }
        KeyCode::Char('y' | 'Y') => Some(true),
        KeyCode::Char('n' | 'N') | KeyCode::Esc => Some(false),
        KeyCode::Enter => Some(*selected_yes),
        _ => None,
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
