//! Save the parent and build a fresh interactive fork before activation.
use super::{Session, View, copy};
pub(super) async fn create<S: Session, V: View>(
    session: &S,
    view: &mut V,
    drop_last_n: usize,
) -> Option<(S, String, usize)> {
    if let Err(error) = session.save().await {
        view.set_status(format!(
            "Fork aborted: failed to save current session: {error}"
        ));
        return None;
    }
    let parent_id = session.id().to_string();
    let mut child = match S::new().await {
        Ok(s) => s,
        Err(err) => {
            view.set_status(format!("Fork failed: {err}"));
            return None;
        }
    };
    let tui_cut = copy::apply(session, &mut child, view.messages(), drop_last_n);
    Some((child, parent_id, tui_cut))
}
