//! Explicit yes/no selection; defaults to No and ignores modified shortcuts.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) fn choose(selected_yes: &mut bool, key: KeyEvent) -> Option<bool> {
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
#[path = "answer_review_choice_tests.rs"]
mod tests;
