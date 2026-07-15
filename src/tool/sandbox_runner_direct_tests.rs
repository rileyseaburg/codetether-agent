use super::confined::confined_plan;
use super::{ENV, enabled, plan_with_override};
use crate::tool::sandbox::SandboxPolicy;

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
    assert!(!plan.network_isolated);
}

#[test]
fn confined_plan_uses_landlock_when_kernel_supports_it() {
    let policy = SandboxPolicy::default();
    let work_dir = std::env::temp_dir();
    let plan = confined_plan("sh", &args(), &policy, &work_dir, "bwrap_smoke_failed");
    if super::super::sandbox_landlock::kernel_available() {
        let plan = plan.expect("landlock-confined plan");
        assert_eq!(plan.program, "sh");
        assert!(plan.landlock.is_some());
        assert!(!plan.network_isolated);
    } else {
        assert!(plan.is_none(), "no Landlock kernel => fail closed");
    }
}

#[test]
fn unsafe_env_parser_accepts_only_truthy_values() {
    assert!(enabled(Some("1")));
    assert!(enabled(Some("true")));
    assert!(!enabled(Some("false")));
    assert!(!enabled(None));
}
