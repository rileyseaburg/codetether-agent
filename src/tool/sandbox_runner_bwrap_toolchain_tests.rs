use super::super::super::SandboxPolicy;
use super::super::super::sandbox_runner_select::Runner;
use super::super::super::sandbox_toolchain;

fn triplet(args: &[String], op: &str, path: &str) -> bool {
    args.windows(3)
        .any(|w| w[0].as_str() == op && w[1].as_str() == path && w[2].as_str() == path)
}

fn plan_args(policy: &SandboxPolicy) -> Vec<String> {
    let args = vec!["-c".to_string(), "node --version".to_string()];
    Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &args, policy, "/workspace/project".as_ref())
        .unwrap()
        .args
}

#[test]
fn bwrap_exposes_configured_toolchain_roots_readonly() {
    let _lock = crate::approval::test_env::lock_env();
    let toolchain = tempfile::tempdir().expect("toolchain");
    let root = toolchain.path().display().to_string();
    unsafe { std::env::set_var(sandbox_toolchain::ENV, &root) };
    let policy = SandboxPolicy {
        allowed_paths: vec!["/workspace".into()],
        allow_exec: true,
        ..SandboxPolicy::default()
    };

    let args = plan_args(&policy);
    unsafe { std::env::remove_var(sandbox_toolchain::ENV) };

    assert!(triplet(&args, "--ro-bind", &root));
    assert!(!triplet(&args, "--bind", &root));
}

#[test]
fn bwrap_skips_toolchain_roots_already_writable() {
    let _lock = crate::approval::test_env::lock_env();
    let workspace = tempfile::tempdir().expect("workspace");
    let tools = workspace.path().join("node_modules/.bin");
    std::fs::create_dir_all(&tools).expect("tools dir");
    unsafe { std::env::set_var(sandbox_toolchain::ENV, &tools) };
    let policy = SandboxPolicy {
        allowed_paths: vec![workspace.path().into()],
        allow_exec: true,
        ..SandboxPolicy::default()
    };

    let args = plan_args(&policy);
    unsafe { std::env::remove_var(sandbox_toolchain::ENV) };

    assert!(!triplet(&args, "--ro-bind", &tools.display().to_string()));
}
