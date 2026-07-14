//! Proof-daemon process startup and registry discovery.

use std::path::Path;
use std::process::Stdio;

use crate::mux::registry::MuxRecord;

pub(super) struct ProofProcess {
    pub child: tokio::process::Child,
    pub record: MuxRecord,
}

pub(super) async fn start(name: &str, workspace: &Path, root: &Path) -> ProofProcess {
    let mut child = tokio::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "mux::tests::multiprocess::child::daemon",
            "--ignored",
            "--exact",
        ])
        .env("MUX_PROOF_NAME", name)
        .env("MUX_PROOF_WORKSPACE", workspace)
        .env("CODETETHER_DATA_DIR", root)
        .env(
            crate::mux::token::BOOTSTRAP_ENV,
            format!("proof-token-{name}"),
        )
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let path = root.join("mux").join(format!("{name}.json"));
    for _ in 0..100 {
        assert!(
            child.try_wait().unwrap().is_none(),
            "proof daemon exited early"
        );
        if let Ok(bytes) = tokio::fs::read(&path).await
            && let Ok(record) = serde_json::from_slice(&bytes)
        {
            return ProofProcess { child, record };
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    panic!("proof daemon did not publish {}", path.display());
}
