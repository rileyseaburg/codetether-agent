use super::{ENV, enabled, plan_with_override};

fn args() -> Vec<String> {
    vec!["-c".to_string(), "echo ok".to_string()]
}

#[test]
fn direct_runner_fails_closed_without_unsafe_override() {
    let err = plan_with_override("sh", &args(), "bwrap_not_found", false).unwrap_err();
    assert!(err.to_string().contains("OS sandbox unavailable"));
    assert!(err.to_string().contains(ENV));
}

#[test]
fn direct_runner_records_explicit_unsafe_env_override() {
    let args = args();
    let plan = plan_with_override("sh", &args, "bwrap_not_found", true).unwrap();
    assert_eq!(plan.program, "sh");
    assert_eq!(plan.args, args);
    assert!(
        plan.unsafe_fallbacks
            .contains(&format!("unsafe_sandbox_fallback_allowed:{ENV}"))
    );
    assert!(
        plan.unsafe_fallbacks
            .contains(&"os_sandbox_unavailable:bwrap_not_found".to_string())
    );
    assert!(!plan.network_isolated);
}

#[test]
fn unsafe_env_parser_accepts_only_truthy_values() {
    assert!(enabled(Some("1")));
    assert!(enabled(Some("true")));
    assert!(!enabled(Some("false")));
    assert!(!enabled(None));
}
