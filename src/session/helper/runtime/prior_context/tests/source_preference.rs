use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn repository_source_of_truth_survives_followups() {
    for instruction in [
        "its in the docs",
        "use the repository docs",
        "documentation is the source of truth",
    ] {
        let messages = [user(instruction), user("implement the next part")];
        assert!(!prior_context_allowed(&messages), "{instruction}");
    }
}

#[test]
fn repository_docs_win_over_incidental_history_names() {
    for instruction in [
        "use repository docs, not the session history",
        "use repository docs for the session recall implementation",
    ] {
        assert!(
            !prior_context_allowed(&[user(instruction)]),
            "{instruction}"
        );
    }
}

#[test]
fn explicit_session_request_can_override_docs_for_one_turn() {
    let messages = [
        user("use the repository docs"),
        user("use session recall this time even though its in the docs"),
    ];
    assert!(prior_context_allowed(&messages));
}
