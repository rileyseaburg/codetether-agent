use super::super::super::SandboxPolicy;
use super::super::super::sandbox_runner_select::Runner;

fn pair(args: &[String], op: &str, path: &str) -> bool {
    args.windows(2)
        .any(|w| w[0].as_str() == op && w[1].as_str() == path)
}

fn triplet(args: &[String], op: &str, path: &str) -> bool {
    args.windows(3)
        .any(|w| w[0].as_str() == op && w[1].as_str() == path && w[2].as_str() == path)
}

#[test]
fn bwrap_protects_metadata_under_writable_root() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir(temp.path().join(".git")).expect("git dir");
    let policy = SandboxPolicy {
        allowed_paths: vec![temp.path().into()],
        allow_exec: true,
        ..SandboxPolicy::default()
    };
    let plan = Runner::Bubblewrap("/usr/bin/bwrap".into())
        .plan("sh", &["-c".into(), "echo ok".into()], &policy, temp.path())
        .unwrap();
    let git = temp.path().join(".git").display().to_string();
    let codex = temp.path().join(".codex").display().to_string();
    assert!(triplet(&plan.args, "--ro-bind", &git));
    assert!(pair(&plan.args, "--dir", &codex));
}
