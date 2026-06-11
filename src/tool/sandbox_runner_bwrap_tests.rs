use super::super::super::sandbox_runner_select::Runner;
use super::super::RunnerPlan;
use super::policy;

fn shell_args() -> Vec<String> {
    vec!["-c".to_string(), "echo ok".to_string()]
}

fn make_plan(allow_network: bool, cwd: &str) -> RunnerPlan {
    Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &shell_args(), &policy(allow_network), cwd.as_ref())
        .unwrap()
}

fn pair(args: &[String], op: &str, path: &str) -> bool {
    args.windows(2)
        .any(|w| w[0].as_str() == op && w[1].as_str() == path)
}

fn triplet(args: &[String], op: &str, src: &str, dst: &str) -> bool {
    args.windows(3)
        .any(|w| w[0].as_str() == op && w[1].as_str() == src && w[2].as_str() == dst)
}

#[test]
fn bwrap_runner_wraps_command_and_network_namespace() {
    let plan = make_plan(false, "/workspace");
    assert_eq!(plan.program, "/usr/bin/bwrap");
    assert!(plan.args.contains(&"--seccomp".to_string()));
    assert!(plan.args.contains(&"--unshare-net".to_string()));
    assert_eq!(
        plan.args[plan.args.len() - 4..].join("\0"),
        "--\0sh\0-c\0echo ok"
    );
    assert!(plan.network_isolated);
    assert!(
        !make_plan(true, "/workspace")
            .args
            .contains(&"--unshare-net".to_string())
    );
}

#[test]
fn bwrap_mounts_system_paths_readonly_and_tmpfs() {
    let args = make_plan(false, "/workspace").args;
    assert!(triplet(&args, "--ro-bind", "/usr", "/usr"));
    assert!(triplet(&args, "--ro-bind-try", "/etc", "/etc"));
    assert!(pair(&args, "--tmpfs", "/tmp"));
    assert!(!triplet(&args, "--bind", "/usr", "/usr"));
}
