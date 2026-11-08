//! Opt-in chat sessions cannot inherit a persisted coding-project binding.
use crate::session::Session;
use codetether_request_preparation::general_chat;

pub(super) fn prepare(session: &mut Session) -> anyhow::Result<()> {
    if !general_chat::enabled(&session.agent) {
        return Ok(());
    }
    let (workspace, _, _) = general_chat::context();
    std::fs::create_dir_all(&workspace)?;
    session.set_agent_name("chat");
    session.metadata.directory = Some(workspace);
    Ok(())
}
