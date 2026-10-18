//! Existing TUI regression cases moved beside their implementation.
use crate::{is_easy_go_command, next_go_model};

#[test]
fn easy_go_command_detects_go_and_team_aliases() {
    assert!(is_easy_go_command("/go build indexing"));
    assert!(is_easy_go_command("/team 4 implement auth"));
    assert!(!is_easy_go_command("/autochat build indexing"));
}

#[test]
fn next_go_model_toggles_between_glm_and_minimax() {
    assert_eq!(next_go_model(Some("zai/glm-5")), "minimax/MiniMax-M3");
    assert_eq!(next_go_model(Some("z-ai/glm-5")), "minimax/MiniMax-M3");
    assert_eq!(next_go_model(Some("minimax/MiniMax-M3")), "zai/glm-5");
    assert_eq!(next_go_model(Some("unknown/model")), "minimax/MiniMax-M3");
}
