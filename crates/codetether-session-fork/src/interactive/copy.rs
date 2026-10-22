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
    let (pages, corrections) = crate::constraints::retained(session, 0, session_cut);
    child.install_history(messages, pages);
    metadata::copy(session.metadata(), child.metadata_mut());
    child.metadata_mut().constraint_corrections = corrections;
    child.set_title(
        session
            .title()
            .map(|t| format!("{t} (fork)"))
            .or_else(|| Some("fork".to_string())),
    );
    tui_cut
}

#[cfg(test)]
mod tests {
    #[test]
    fn interactive_forks_keep_source_pins_and_rebased_corrections() {
        let parent = crate::constraint_tests::fixture::parent();
        let mut child = crate::constraint_tests::fixture::Session::default();
        super::apply(&parent, &mut child, &[], 0);
        assert_eq!(child.metadata.constraint_corrections[0].source_turn, 0);
        assert_eq!(child.metadata.constraint_corrections[0].replacement_turn, 1);
        assert_eq!(
            child.pages[0],
            codetether_context_policy::pages::PageKind::Constraint
        );
        let mut earlier = crate::constraint_tests::fixture::Session::default();
        super::apply(&parent, &mut earlier, &[], 2);
        assert_eq!(earlier.messages.len(), 1);
        assert!(earlier.metadata.constraint_corrections.is_empty());
        assert_eq!(
            earlier.pages[0],
            codetether_context_policy::pages::PageKind::Constraint
        );
    }
}
