//! Provider regression coverage.

#[test]
fn detects_minimax_chat_setting_error_variants() {
    assert!(super::super::minimax_error::is_minimax_chat_setting_error(
        "bad_request_error: invalid params, invalid chat setting (2013)"
    ));
    assert!(super::super::minimax_error::is_minimax_chat_setting_error(
        "code: 2013 invalid params"
    ));
    assert!(!super::super::minimax_error::is_minimax_chat_setting_error(
        "rate limit exceeded"
    ));
}
