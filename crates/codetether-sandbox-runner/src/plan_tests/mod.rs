//! Regression coverage for shared sandbox runner plans.
use crate::{Runner, RunnerPlan};
use codetether_sandbox_policy::SandboxPolicy;

#[path = "sandbox_runner_bwrap_path_tests.rs"]
mod bwrap_path_tests;
#[path = "sandbox_runner_bwrap_protected_tests.rs"]
mod bwrap_protected_tests;
#[path = "sandbox_runner_bwrap_readonly_tests.rs"]
mod bwrap_readonly_tests;
#[path = "sandbox_runner_bwrap_tests.rs"]
mod bwrap_tests;
#[path = "sandbox_runner_bwrap_toolchain_tests.rs"]
mod bwrap_toolchain_tests;

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

fn expected_kernel_fallbacks() -> Vec<String> {
    let landlock = crate::sandbox_landlock::prepare(&policy(false), "/workspace".as_ref());
    let mut expected = Vec::new();
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    expected.push("seccomp_inactive:not_configured".to_string());
    if landlock.rules.is_none() {
        expected.push("landlock_inactive:not_configured".to_string());
    }
    expected.extend(landlock.fallback);
    expected
}
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn lock_env() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
