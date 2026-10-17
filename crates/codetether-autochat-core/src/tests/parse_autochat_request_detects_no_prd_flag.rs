//! Existing auto-chat parsing and policy regression.
use super::*;

#[test]
fn parse_autochat_request_detects_no_prd_flag() {
    let parsed = parse_autochat_request(
        &format!("{AUTOCHAT_NO_PRD_FLAG} 4 build relay"),
        AUTOCHAT_DEFAULT_AGENTS,
        AUTOCHAT_QUICK_DEMO_TASK,
    );
    assert_eq!(
        parsed,
        Some(ParsedAutochatRequest {
            agent_count: 4,
            task: "build relay".to_string(),
            bypass_prd: true,
            explicit_count: true,
        })
    );
}
