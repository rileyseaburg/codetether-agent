//! Linux cancellation evidence for RustyRoad's owned process lifecycle.
#![cfg(target_os = "linux")]

use super::super::process::Process;
use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

#[tokio::test]
async fn rustyroad_cancellation_kills_and_reaps_owned_child() {
    let pid = Arc::new(AtomicU32::new(0));
    let observed = Arc::clone(&pid);
    let result = tokio::time::timeout(Duration::from_secs(1), async move {
        let mut command = tokio::process::Command::new("sh");
        command.args(["-c", "exec sleep 30"]);
        let process = Process::spawn_command(command).await.unwrap();
        observed.store(process.child.id().unwrap(), Ordering::SeqCst);
        std::future::pending::<()>().await;
        drop(process);
    })
    .await;
    assert!(result.is_err());
    let pid = pid.load(Ordering::SeqCst);
    assert_ne!(pid, 0, "fixture process must start before cancellation");
    let path = format!("/proc/{pid}");
    for _ in 0..100 {
        if !std::path::Path::new(&path).exists() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    panic!("RustyRoad child {pid} survived cancellation or was not reaped");
}
