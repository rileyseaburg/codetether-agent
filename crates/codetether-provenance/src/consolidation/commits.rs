//! Provenance commits contract.
use super::fixture::{provenance, repo, run};

#[test]
fn blocking_commit_uses_identity_and_trailers() {
    let dir = repo();
    let p = provenance();
    assert!(
        crate::git_commit_with_provenance_blocking(dir.path(), "test: provenance", Some(&p))
            .unwrap()
            .status
            .success()
    );
    let log = run(dir.path(), &["log", "-1", "--format=%an%n%ae%n%B"]);
    assert!(log.contains("CodeTether Agent [fixture-agent]"));
    assert!(log.contains("agent+fixture-agent@codetether.run"));
    assert!(log.contains("CodeTether-Provenance-ID: fixed-provenance"));
    println!("COMMIT={}", serde_json::to_string(&log).unwrap());
}

#[tokio::test]
async fn async_commit_uses_same_message() {
    let dir = repo();
    let p = provenance();
    assert!(
        crate::git_commit_with_provenance(dir.path(), "test: provenance", Some(&p))
            .await
            .unwrap()
            .status
            .success()
    );
    let log = run(dir.path(), &["log", "-1", "--format=%an%n%ae%n%B"]);
    assert!(log.contains("CodeTether-Provenance-ID: fixed-provenance"));
    println!("COMMIT={}", serde_json::to_string(&log).unwrap());
}
