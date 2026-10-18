//! Route modal keys and transfer completed forms to session-aware submission.
use crate::{FormAction, SpawnFormState};
use crossterm::event::KeyEvent;

/// Result of applying a key to the optional spawn form.
/// # Examples
/// ```
/// use codetether_spawn_form::key_dispatch::Outcome;
/// let outcome = Outcome::Inactive;
/// assert!(matches!(outcome, Outcome::Inactive));
/// ```
pub enum Outcome {
    /// No form is open; the caller should continue normal key handling.
    Inactive,
    /// The form consumed the key, including cancellation.
    Consumed,
    /// The removed form is ready for session-aware submission.
    Submit(SpawnFormState),
}

/// Apply a modal key and take the form when submission is requested.
/// # Arguments
/// Receives the optional form slot, status text and keyboard event.
/// # Returns
/// Whether the key was consumed or a form is ready to submit.
/// # Examples
/// ```
/// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// use codetether_spawn_form::key_dispatch::{dispatch, Outcome};
/// let mut form = Some(Default::default());
/// let mut status = String::new();
/// assert!(matches!(dispatch(&mut form, &mut status,
///     KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)), Outcome::Consumed));
/// assert!(form.is_none());
/// ```
pub fn dispatch(form: &mut Option<SpawnFormState>, status: &mut String, key: KeyEvent) -> Outcome {
    let Some(active) = form.as_mut() else {
        return Outcome::Inactive;
    };
    match active.handle_key(key) {
        FormAction::Cancel => {
            crate::lifecycle::close(form, status);
            Outcome::Consumed
        }
        FormAction::Submit => Outcome::Submit(form.take().unwrap()),
        FormAction::Continue => Outcome::Consumed,
    }
}
