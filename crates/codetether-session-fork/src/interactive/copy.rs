//! Copy the retained history and selected metadata into a fresh fork.
use super::{Session, cuts, metadata};
use codetether_chat_model::ChatMessage;
pub(super) fn apply<S: Session>(
    session: &S,
    child: &mut S,
    ui: &[ChatMessage],
    drop_last_n: usize,
) -> usize {
    let session_cut = cuts::session(session.messages(), drop_last_n);
    let tui_cut = cuts::ui(ui, drop_last_n);
    let messages = session.messages()[..session_cut].to_vec();
    let pages = if session.pages().len() >= session_cut {
        session.pages()[..session_cut].to_vec()
    } else {
        codetether_context_policy::pages::classify_all(&messages)
    };
    child.install_history(messages, pages);
    metadata::copy(session.metadata(), child.metadata_mut());
    child.set_title(
        session
            .title()
            .map(|t| format!("{t} (fork)"))
            .or_else(|| Some("fork".to_string())),
    );
    tui_cut
}
