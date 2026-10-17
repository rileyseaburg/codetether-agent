//! Modal question input; never sends answer keystrokes into the chat composer.
use crate::tui::app::state::App;
use codetether_question::{Action, runtime};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, key: KeyEvent) -> bool {
    let Some(id) = app.state.session_id.as_deref() else {
        return false;
    };
    let Some(draft) = runtime::pending(id) else {
        return false;
    };
    // Preserve the global turn-cancel and quit keys.
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return !matches!(key.code, KeyCode::Char('c' | 'q'));
    }
    app.state.needs_redraw = true;
    let action = match key.code {
        KeyCode::Esc => {
            runtime::dismiss(id);
            return true;
        }
        KeyCode::Up => Action::Previous,
        KeyCode::Down | KeyCode::Tab => Action::Next,
        KeyCode::BackTab => Action::Back,
        KeyCode::Enter => Action::Submit,
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Char(' ')
            if draft.cursor() < draft.question().options.len() && draft.question().multiple =>
        {
            Action::Toggle
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::ALT) => {
            Action::Text(c.to_string())
        }
        _ => return true,
    };
    runtime::act(id, action);
    app.state.main_last_event_at = Some(std::time::Instant::now());
    true
}

pub(super) fn paste(app: &mut App, text: &str) -> bool {
    let handled = app
        .state
        .session_id
        .as_deref()
        .is_some_and(|id| runtime::act(id, Action::Text(text.to_owned())));
    app.state.needs_redraw |= handled;
    handled
}

#[cfg(test)]
#[path = "question_key_tests.rs"]
mod tests;
