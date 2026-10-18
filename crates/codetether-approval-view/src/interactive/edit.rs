//! Opens a pending patch proposal in the in-TUI source editor.

use std::path::Path;

use super::Host;
use codetether_approval_queue as approval_queue;

use codetether_approval_edit::ApprovalEditSession;

/// Open the active patch proposal for editing.
/// # Arguments
/// * `host` — Frontend state and command bindings.
/// * `cwd` — Workspace root for patch editing.
/// # Returns
/// Whether the action was consumed.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_approval_view::interactive::Host) {
/// let _ = codetether_approval_view::interactive::open(host, std::path::Path::new("."));
/// # }
/// ```
pub fn open(host: &mut impl Host, cwd: &Path) -> bool {
    let Some(item) = approval_queue::active() else {
        return false;
    };
    if item.tool != "apply_patch" {
        host.set_status("Only patch approvals can be opened in the code editor".into());
        return true;
    }
    let Some(patch) = item.preview.as_deref() else {
        host.set_status("This approval has no editable patch preview".into());
        return true;
    };
    match ApprovalEditSession::from_patch(cwd, item.id, patch) {
        Ok(session) => activate(host, session),
        Err(error) => host.set_status(format!("Cannot edit approval patch: {error}")),
    }
    true
}

fn activate(host: &mut impl Host, session: ApprovalEditSession) {
    let buffer = session.buffer();
    let (current, total) = session.progress();
    let path = buffer.path().display().to_string();
    *host.editor() = Some(buffer);
    *host.session() = Some(session);
    host.reset_editor_scroll();
    host.show_editor();
    host.set_status(format!(
        "Editing proposed file {current}/{total}: {path} · Ctrl+S next/apply · Esc cancel"
    ));
}
