use super::super::super::SandboxPolicy;
use super::super::super::sandbox_runner_select::Runner;
use super::policy;

fn pair(args: &[String], op: &str, path: &str) -> bool {
    args.windows(2)
        .any(|w| w[0].as_str() == op && w[1].as_str() == path)
}

fn triplet(args: &[String], op: &str, src: &str, dst: &str) -> bool {
    args.windows(3)
        .any(|w| w[0].as_str() == op && w[1].as_str() == src && w[2].as_str() == dst)
}

#[test]
fn bwrap_binds_writable_roots_without_rebinding_child_workdir() {
    let args = vec!["-c".to_string(), "echo ok".to_string()];
    let plan = Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &args, &policy(false), "/workspace/project".as_ref())
        .unwrap();
    assert!(triplet(&plan.args, "--bind", "/workspace", "/workspace"));
    assert!(!triplet(
        &plan.args,
        "--bind",
        "/workspace/project",
        "/workspace/project"
    ));
    assert!(!triplet(
        &plan.args,
        "--ro-bind",
        "/workspace/project",
        "/workspace/project"
    ));
}

#[test]
fn bwrap_keeps_tmp_workdir_on_tmpfs_without_allowed_path() {
    let args = vec!["-c".to_string(), "echo ok".to_string()];
    let policy = SandboxPolicy {
        allow_exec: true,
        ..SandboxPolicy::default()
    };
    let plan = Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &args, &policy, "/tmp".as_ref())
        .unwrap();
    assert!(pair(&plan.args, "--tmpfs", "/tmp"));
    assert!(!triplet(&plan.args, "--bind", "/tmp", "/tmp"));
}
