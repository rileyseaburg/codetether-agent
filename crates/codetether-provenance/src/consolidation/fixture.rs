//! Provenance fixture contract.
use crate::{ExecutionOrigin, ExecutionProvenance};

pub(super) fn run(repo: &std::path::Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

pub(super) fn repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    run(dir.path(), &["init", "-q"]);
    run(dir.path(), &["config", "core.hooksPath", ".disabled-hooks"]);
    run(dir.path(), &["config", "core.fsmonitor", "false"]);
    run(dir.path(), &["config", "user.name", "Fixture"]);
    run(
        dir.path(),
        &["config", "user.email", "fixture@example.test"],
    );
    std::fs::write(dir.path().join("file.txt"), "first").unwrap();
    run(dir.path(), &["add", "file.txt"]);
    dir
}

pub(super) fn provenance() -> ExecutionProvenance {
    let mut p = ExecutionProvenance::for_operation("test", ExecutionOrigin::Ralph);
    p.provenance_id = "fixed-provenance".into();
    p.issued_at = "2026-01-01T00:00:00Z".parse().unwrap();
    p.identity.agent_identity_id = Some("fixture-agent".into());
    p
}
