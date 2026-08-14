use super::super::SandboxPolicy;
use super::super::sandbox_runner_select::Runner;
use super::exec_args;
use std::path::{Path, PathBuf};

fn policy() -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: vec![std::env::temp_dir()],
        allow_exec: true,
        ..SandboxPolicy::default()
    }
}

#[test]
fn exec_args_pass_profile_by_file_then_the_command() {
    let args = exec_args(
        Path::new("/tmp/profile.sb"),
        "sh",
        &["-c".to_string(), "echo ok".to_string()],
    );
    assert_eq!(
        args,
        vec!["-f", "/tmp/profile.sb", "sh", "-c", "echo ok"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
}

#[test]
fn seatbelt_plan_stages_a_profile_and_reports_read_gap() {
    let work_dir = std::env::temp_dir();
    let plan = Runner::Seatbelt(PathBuf::from("/usr/bin/sandbox-exec"))
        .plan(
            "sh",
            &["-c".to_string(), "echo ok".to_string()],
            &policy(),
            &work_dir,
        )
        .expect("seatbelt plan");
    assert_eq!(plan.program, "/usr/bin/sandbox-exec");
    assert_eq!(plan.args.first().map(String::as_str), Some("-f"));
    let profile = Path::new(&plan.args[1]);
    assert!(profile.exists(), "profile must be staged on disk");
    let text = std::fs::read_to_string(profile).expect("read staged profile");
    assert!(text.contains("(deny default)"));
    assert!(plan.network_isolated);
    assert!(
        plan.unsafe_fallbacks
            .contains(&"seatbelt_read_unconfined".to_string())
    );
    let _ = std::fs::remove_file(profile);
}
