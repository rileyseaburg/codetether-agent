//! Save semantic edits without losing drafts on conflicts or errors.
use crate::{Journal, Read, Session, Write};
use anyhow::{Result, anyhow, ensure};
pub(super) async fn save<S: Session>(session: &mut S) -> Result<()> {
    let draft = session
        .editor()
        .draft()
        .ok_or_else(|| anyhow!("No goal draft"))?;
    ensure!(
        draft.session_id == session.session_id(),
        "Session changed; draft retained. Esc to close."
    );
    let text = session
        .editor()
        .buffer()
        .ok_or_else(|| anyhow!("Goal editor buffer missing"))?
        .text();
    if text == draft.original.objective {
        super::close(session.editor_mut());
        session.editor_mut().status("Goal unchanged".into());
        return Ok(());
    }
    let state = <<S::Editor as Read>::Journal as Journal>::read(&draft.session_id)?;
    let request = draft.request(&state, text)?;
    <<S::Editor as Read>::Journal as Journal>::update(&draft.session_id, request).await?;
    let id = draft.session_id.clone();
    super::close(session.editor_mut());
    session.restart(&id);
    session
        .editor_mut()
        .status("Goal saved — recorded usage, criteria, and session tasks preserved".into());
    Ok(())
}
