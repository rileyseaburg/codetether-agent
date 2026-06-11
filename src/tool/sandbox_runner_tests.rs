use super::super::sandbox_runner_select::Runner;
use super::*;

#[path = "sandbox_runner_bwrap_path_tests.rs"]
mod bwrap_path_tests;
#[path = "sandbox_runner_bwrap_protected_tests.rs"]
mod bwrap_protected_tests;
#[path = "sandbox_runner_bwrap_readonly_tests.rs"]
mod bwrap_readonly_tests;
#[path = "sandbox_runner_bwrap_tests.rs"]
mod bwrap_tests;

fn policy(allow_network: bool) -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: vec!["/workspace".into()],
        allow_network,
        allow_exec: true,
        ..SandboxPolicy::default()
    }
}

#[test]
fn bwrap_runner_records_kernel_fallbacks() {
    let args = vec!["-c".to_string(), "echo ok".to_string()];
    let plan = Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &args, &policy(false), "/workspace".as_ref())
        .unwrap();
    assert_eq!(plan.unsafe_fallbacks, expected_kernel_fallbacks());
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn expected_kernel_fallbacks() -> Vec<String> {
    if super::super::sandbox_landlock::prepare(&policy(false), "/workspace".as_ref())
        .rules
        .is_some()
    {
        Vec::new()
    } else {
        vec!["landlock_inactive:kernel_unavailable".to_string()]
    }
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn expected_kernel_fallbacks() -> Vec<String> {
    vec![
        "seccomp_inactive:not_configured".to_string(),
        "landlock_inactive:not_configured".to_string(),
    ]
}
