//! Model-precedence regressions.
use crate::resolve_autochat_model;

#[test]
fn easy_go_defaults_to_minimax_when_model_not_set() {
    assert_eq!(
        resolve_autochat_model(None, None, Some("zai/glm-5"), true),
        "minimax/MiniMax-M3"
    );
}

#[test]
fn explicit_model_wins_over_easy_go_default() {
    assert_eq!(
        resolve_autochat_model(Some("zai/glm-5"), None, None, true),
        "zai/glm-5"
    );
}
