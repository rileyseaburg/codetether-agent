//! Durable save and persistence status for a completed undo.
use super::{Session, View};
pub(super) async fn save<S: Session, V: View>(session: &S, view: &mut V, undo_count: usize) {
    if let Err(error) = session.save().await {
        tracing::warn!(error = %error, "Failed to save session after undo");
        view.set_status(format!(
            "Undid {undo_count} turn(s) (not persisted: {error})"
        ));
    } else {
        view.set_status(format!("Undid {undo_count} turn(s)"));
    }
}
