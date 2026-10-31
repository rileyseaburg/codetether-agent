//! Goal command discovery and selection regressions.

use crate::{CompletionState, suggestions};

#[test]
fn goal_suggestions_include_supported_subcommands() {
    let matches = suggestions("/goal");
    assert!(matches.iter().any(|command| command == "/goal"));
    for verb in [
        "set", "auto", "edit", "override", "budget", "pause", "resume", "done", "clear", "show",
        "status", "reaffirm", "execute",
    ] {
        let command = format!("/goal {verb}");
        assert!(matches.contains(&command), "missing suggestion: {command}");
    }
    assert!(matches.iter().all(|command| command.starts_with("/goal")));
}

#[test]
fn goal_resume_is_selectable_from_completion_menu() {
    let mut state = CompletionState::default();
    state.refresh_slash_suggestions("/goal ");
    for _ in 0..state.slash_suggestions.len() {
        if state.selected_slash_suggestion() == Some("/goal resume") {
            return;
        }
        state.select_next_slash_suggestion();
    }
    panic!("/goal resume must be reachable in the completion menu");
}
