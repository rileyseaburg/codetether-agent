//! Fork activation status and parent/child identity notice.
use super::View;
pub(super) fn show<V: View>(view: &mut V, child_id: &str, parent_id: &str, drop_last_n: usize) {
    let drop_note = if drop_last_n == 0 {
        String::new()
    } else {
        format!(" (dropped last {drop_last_n} turn(s) from fork)")
    };
    view.push_system_message(format!(
        "Forked session {}{}.\n  parent: {}\n  fork:   {}",
        &child_id[..8.min(child_id.len())],
        drop_note,
        parent_id,
        child_id,
    ));
    view.set_status(format!("Forked → {}", &child_id[..8.min(child_id.len())]));
}
