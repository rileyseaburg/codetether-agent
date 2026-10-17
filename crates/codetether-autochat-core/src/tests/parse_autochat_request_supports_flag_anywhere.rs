//! Existing auto-chat parsing and policy regression.
use super::*;

#[test]
fn parse_autochat_request_supports_flag_anywhere() {
    let parsed = parse_autochat_request(
        "build relay --no-prd now",
        AUTOCHAT_DEFAULT_AGENTS,
        AUTOCHAT_QUICK_DEMO_TASK,
    );
    assert_eq!(
        parsed,
        Some(ParsedAutochatRequest {
            agent_count: AUTOCHAT_DEFAULT_AGENTS,
            task: "build relay now".to_string(),
            bypass_prd: true,
            explicit_count: false,
        })
    );
}
