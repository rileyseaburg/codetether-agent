use super::super::super::SandboxPolicy;
use super::super::super::sandbox_runner_select::Runner;

fn triplet(args: &[String], op: &str, src: &str, dst: &str) -> bool {
    args.windows(3)
        .any(|w| w[0].as_str() == op && w[1].as_str() == src && w[2].as_str() == dst)
}

#[test]
fn bwrap_mounts_readonly_workdir_when_no_writable_root_covers_it() {
    let args = vec!["-c".to_string(), "echo ok".to_string()];
    let policy = SandboxPolicy {
        allow_exec: true,
        ..SandboxPolicy::default()
    };
    let plan = Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &args, &policy, "/workspace/project".as_ref())
        .unwrap();
    assert!(triplet(
        &plan.args,
        "--ro-bind",
        "/workspace/project",
        "/workspace/project"
    ));
    assert!(!triplet(
        &plan.args,
        "--bind",
        "/workspace/project",
        "/workspace/project"
    ));
}
