//! Host actions returned by form key handling.
//! Form state and terminal presentation require no application runtime.

/// Outcome of handling one key in an open form.
///
/// # Examples
/// ```
/// use codetether_spawn_form::FormAction;
/// let action = FormAction::Continue;
/// assert!(matches!(action, FormAction::Continue));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormAction {
    /// Keep the form open after editing or ignoring the key.
    Continue,
    /// Ask the host to close the form.
    Cancel,
    /// Ask the host to submit the current values.
    Submit,
}
