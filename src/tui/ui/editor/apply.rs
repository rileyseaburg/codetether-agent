//! Applies [`EditorInput`] actions to a [`FileBuffer`].
//!
//! Separating application from key-mapping keeps both pieces small and lets the
//! mapper be tested without a buffer and vice versa. Returns whether the editor
//! should remain open (`true`) or close (`false`, on quit).

use super::edit::EditorEdit;
use super::file_buffer::FileBuffer;
use super::input::EditorInput;

/// Applies one action to `buf`, returning `false` when the editor should close.
///
/// # Errors
///
/// Returns an error only when [`EditorInput::Save`] fails to write the file.
pub fn apply(buf: &mut FileBuffer, action: EditorInput) -> std::io::Result<bool> {
    match action {
        EditorInput::Insert(c) => buf.backend_mut().insert_char(c),
        EditorInput::Newline => buf.backend_mut().insert_char('\n'),
        EditorInput::Backspace => buf.backend_mut().delete_back(),
        EditorInput::Move(dir) => buf.backend_mut().move_cursor(dir),
        EditorInput::Save => buf.save()?,
        EditorInput::Quit => return Ok(false),
    }
    Ok(true)
}
