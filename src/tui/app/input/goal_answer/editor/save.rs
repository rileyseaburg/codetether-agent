//! Save goal text through human controls, keeping unsaved drafts on error.

use crate::session::tasks::{control, runtime::answer_review};
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
use anyhow::{Result, anyhow, ensure};

pub(super) async fn save(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
) -> Result<()> {
    let draft = app
        .state
        .goal_editor
        .as_ref()
        .ok_or_else(|| anyhow!("No goal draft"))?;
    ensure!(
        draft.session_id == slot.view().id(),
        "Session changed; draft retained. Esc to close."
    );
    let text = app
        .state
        .editor
        .as_ref()
        .ok_or_else(|| anyhow!("Goal editor buffer missing"))?
        .text();
    if text == draft.original.objective {
        super::close(app);
        app.state.status = "Goal unchanged".into();
        return Ok(());
    }
    let state = answer_review::read(&draft.session_id)?;
    let request = draft.request(&state, text)?;
    control::update_user(&draft.session_id, request).await?;
    let id = draft.session_id.clone();
    super::close(app);
    super::super::commands::restart(app, &id, runtime);
    app.state.status = "Goal saved — recorded usage, criteria, and session tasks preserved".into();
    Ok(())
}
