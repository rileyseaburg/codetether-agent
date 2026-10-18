//! Save and cancel handling for an approval-backed editor session.

use super::Host;
use codetether_editor::EditorInput;

/// Handle save, cancel and file-switch actions for an approval editor.
/// # Arguments
/// * `host` — Frontend session, buffer and view bindings.
/// * `action` — Editor action to dispatch.
/// # Returns
/// True for an approval-specific action; false for ordinary editing actions.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_approval_edit::interactive::Host) {
/// assert!(codetether_approval_edit::interactive::handle(host, &codetether_editor::EditorInput::OpenFinder));
/// # }
/// ```
pub fn handle(host: &mut impl Host, action: &EditorInput) -> bool {
    match action {
        EditorInput::Save => save(host),
        EditorInput::Quit => cancel(host),
        EditorInput::OpenFinder => {
            host.set_status(
                "File switching is disabled while revising an approval proposal".into(),
            );
        }
        _ => return false,
    }
    host.redraw();
    true
}

fn save(host: &mut impl Host) {
    let Some(text) = host.editor().as_ref().map(|buffer| buffer.text()) else {
        return cancel(host);
    };
    let Some(mut session) = host.session().take() else {
        return;
    };
    if session.store(text) {
        let buffer = session.buffer();
        let path = buffer.path().display().to_string();
        let (current, total) = session.progress();
        *host.editor() = Some(buffer);
        *host.session() = Some(session);
        host.reset_scroll();
        host.set_status(format!(
            "Editing proposed file {current}/{total}: {path} · Ctrl+S next/apply · Esc cancel"
        ));
        return;
    }
    let patch = session.revised_patch();
    match super::finish::apply(&session.id, &patch) {
        Ok(status) => super::close::close(host, status),
        Err(error) => {
            *host.session() = Some(session);
            host.set_status(format!("Could not apply edited proposal: {error}"));
        }
    }
}

fn cancel(host: &mut impl Host) {
    super::close::close(
        host,
        "Approval edit cancelled; original proposal is still pending".into(),
    );
}
